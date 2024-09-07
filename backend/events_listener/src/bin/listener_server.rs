use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use concordium_rust_sdk::base::smart_contracts::{OwnedContractName, WasmModule};
use concordium_rust_sdk::types::AbsoluteBlockHeight;
use concordium_rust_sdk::v2;
use concordium_rwa_events_listener::txn_listener::{
    EventsProcessor, ListenerError, TransactionsListener,
};
use concordium_rwa_events_listener::txn_processor::rwa_identity_registry::processor::RwaIdentityRegistryProcessor;
use concordium_rwa_events_listener::txn_processor::rwa_security_cis2::processor::RwaSecurityCIS2Processor;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use tracing::{debug, error, info};
use r2d2::Pool;
use security_sft_rewards::types::{AgentRole, TokenAmount, TokenId};
use tokio::sync::RwLock;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// Postgres Database Url
    #[clap(env, long)]
    pub database_url:                  String,
    #[clap(env, long)]
    pub db_pool_max_size:              u32,
    /// The Concordium node URI.
    #[clap(env, long)]
    pub concordium_node_uri:           String,
    /// The starting block hash.
    #[clap(env, long)]
    pub default_block_height:          Option<u64>,
    #[clap(env, long)]
    pub node_rate_limit:               u64,
    #[clap(env, long)]
    pub node_rate_limit_duration_secs: u64,
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
    env_logger::init();

    let config = Config::parse();
    info!("Contracts Listener: Starting");
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

    // Parsing modules to get module references
    let ir_module = WasmModule::from_slice(include_bytes!(
        "../../../../contracts/identity-registry/contract.wasm.v1"
    ))
    .expect("Failed to parse identity-registry module")
    .get_module_ref();
    info!("Identity Registry Module Reference: {:?}", ir_module);
    let security_sft_rewards_module = WasmModule::from_slice(include_bytes!(
        "../../../../contracts/security-sft-rewards/contract.wasm.v1"
    ))
    .expect("Failed to parse security-sft-rewards module")
    .get_module_ref();
    info!(
        "Security SFT Rewards Module Reference: {:?}",
        security_sft_rewards_module
    );
    let identity_registry_processor = RwaIdentityRegistryProcessor {
        module_ref:    ir_module,
        contract_name: OwnedContractName::new_unchecked("init_rwa_identity_registry".to_string()),
        pool:          db_pool.clone(),
    };
    let security_sft_processor = RwaSecurityCIS2Processor::<TokenId, TokenAmount, AgentRole>::new(
        db_pool.clone(),
        security_sft_rewards_module,
        OwnedContractName::new("init_security_sft_rewards".to_string())
            .expect("Invalid contract name"),
    );
    let processors: Vec<Arc<RwLock<dyn EventsProcessor>>> = vec![
        Arc::new(RwLock::new(identity_registry_processor)),
        Arc::new(RwLock::new(security_sft_processor)),
    ];
    let listener = TransactionsListener::new(
        concordium_client,
        db_pool.clone(),
        processors,
        default_block_height,
    );

    listener.listen().await?;
    error!("Contracts Listener: Stopped");
    Err(Error::ListenerStopped)
}
