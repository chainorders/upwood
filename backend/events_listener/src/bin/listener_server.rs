use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use clap::Parser;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{OwnedContractName, WasmModule};
use concordium_rust_sdk::types::AbsoluteBlockHeight;
use concordium_rust_sdk::v2;
use concordium_rwa_events_listener::txn_listener::listener::{
    ListenerError, ProcessorFnType, TransactionsListener,
};
use concordium_rwa_events_listener::txn_processor::{rwa_identity_registry, rwa_security_cis2};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use security_sft_rewards::types::{AgentRole, TokenAmount, TokenId};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// Postgres Database Url
    #[clap(env, long)]
    pub database_url: String,
    #[clap(env, long)]
    pub db_pool_max_size: u32,
    /// The Concordium node URI.
    #[clap(env, long)]
    pub concordium_node_uri: String,
    /// The starting block hash.
    #[clap(env, long)]
    pub default_block_height: Option<u64>,
    #[clap(env, long)]
    pub node_rate_limit: u64,
    #[clap(env, long)]
    pub node_rate_limit_duration_secs: u64,
    #[clap(env, long)]
    pub account: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid node URI: {0}")]
    InvalidNodeUri(#[from] concordium_rust_sdk::endpoints::Error),
    #[error("Listener Error: {0}")]
    ListenerError(#[from] ListenerError),
    #[error("Listener stopped")]
    ListenerStopped,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::from_filename(Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let config = Config::parse();
    debug!("{:#?}", config);

    let db_pool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(ConnectionManager::<PgConnection>::new(&config.database_url))
        .expect("Failed to create connection pool");

    let endpoint: v2::Endpoint = config.concordium_node_uri.parse()?;
    let endpoint = endpoint.rate_limit(
        config.node_rate_limit,
        Duration::from_secs(config.node_rate_limit_duration_secs),
    );
    let mut concordium_client = v2::Client::new(endpoint)
        .await
        .expect("Failed to create Concordium client");

    let default_block_height = match config.default_block_height {
        Some(height) => AbsoluteBlockHeight { height },
        None => {
            debug!("Fetching last finalized block height");
            concordium_client
                .get_consensus_info()
                .await
                .expect("Failed to get consensus info")
                .last_finalized_block_height
        }
    };
    info!("default block height: {}", default_block_height);
    let processors = {
        let mut map: BTreeMap<(ModuleReference, OwnedContractName), ProcessorFnType> =
            BTreeMap::new();
        map.insert(
            (
                WasmModule::from_slice(include_bytes!(
                    "../../../../contracts/security-sft-rewards/contract.wasm.v1"
                ))
                .expect("Failed to parse security-sft-rewards module")
                .get_module_ref(),
                OwnedContractName::new_unchecked("init_security_sft_rewards".to_string()),
            ),
            rwa_security_cis2::processor::process_events::<TokenId, TokenAmount, AgentRole>,
        );
        map.insert(
            (
                WasmModule::from_slice(include_bytes!(
                    "../../../../contracts/identity-registry/contract.wasm.v1"
                ))
                .expect("Failed to parse identity-registry module")
                .get_module_ref(),
                OwnedContractName::new_unchecked("init_rwa_identity_registry".to_string()),
            ),
            rwa_identity_registry::processor::process_events,
        );
        map
    };
    let listener = TransactionsListener::new(
        concordium_client,
        db_pool.clone(),
        config.account.parse().expect("Invalid account"),
        processors,
        default_block_height,
    );

    info!("Contracts Listener: Starting");
    listener.listen().await?;
    error!("Contracts Listener: Stopped");
    Err(Error::ListenerStopped)
}
