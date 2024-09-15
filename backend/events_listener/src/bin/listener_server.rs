use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use backon::{ExponentialBuilder, Retryable};
use clap::Parser;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{OwnedContractName, WasmModule};
use concordium_rust_sdk::types::AbsoluteBlockHeight;
use concordium_rust_sdk::v2;
use concordium_rwa_events_listener::txn_listener;
use concordium_rwa_events_listener::txn_listener::listener::{
    ListenerError, ProcessorError, ProcessorFnType, TransactionsListener,
};
use concordium_rwa_events_listener::txn_processor::{rwa_identity_registry, rwa_security_cis2};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use security_sft_rewards::types::{AgentRole, TokenAmount, TokenId};
use tracing::{debug, error, info, warn};
use tracing_subscriber::fmt::format::FmtSpan;
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
    pub node_rate_limit_duration_millis: u64,
    #[clap(env, long)]
    pub account: String,
    #[clap(env, long)]
    pub node_connect_timeout_millis: u64,
    #[clap(env, long)]
    pub node_request_timeout_millis: u64,
    #[clap(env, long)]
    pub listener_retry_times: usize,
    #[clap(env, long)]
    pub listener_retry_min_delay_millis: u64,
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
        .with_target(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let config = Config::parse();
    debug!("{:#?}", config);

    let db_pool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(ConnectionManager::<PgConnection>::new(&config.database_url))
        .expect("Failed to create connection pool");

    let endpoint = config
        .concordium_node_uri
        .parse::<v2::Endpoint>()?
        .rate_limit(
            config.node_rate_limit,
            Duration::from_millis(config.node_rate_limit_duration_millis),
        )
        .timeout(Duration::from_millis(config.node_request_timeout_millis))
        .connect_timeout(Duration::from_millis(config.node_connect_timeout_millis));
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

    let owner_account = config.account.parse().expect("Invalid account");
    let listener = TransactionsListener::new(
        concordium_client,
        db_pool,
        owner_account,
        processors,
        default_block_height,
    );
    let listen = || async {
        info!("Contracts Listener: Starting");
        txn_listener::listener::listen(listener.clone()).await?;
        Ok::<_, ListenerError>(())
    };
    let retry_policy = ExponentialBuilder::default()
        .with_max_times(config.listener_retry_times)
        .with_min_delay(Duration::from_millis(
            config.listener_retry_min_delay_millis,
        ))
        .with_max_delay(Duration::from_millis(10000));

    listen
    .retry(retry_policy)
    .sleep(tokio::time::sleep)
    // When to retry
    .when(|e: &ListenerError| match e {
        ListenerError::DatabaseError(_) => false,
        ListenerError::FinalizedBlockTimeout => true,
        ListenerError::FinalizedBlockStreamEnded => true,
        ListenerError::QueryError(_) => true,
        ListenerError::DatabasePoolError(_) => true,
        ListenerError::GrpcError(_) => true,
        ListenerError::ProcessorError(ProcessorError::EventsParseError(_)) => false,
        ListenerError::ProcessorError(ProcessorError::DatabaseError(_)) => false,
        ListenerError::ProcessorError(ProcessorError::DatabasePoolError(_)) => true,
    })
    // Notify when retrying
    .notify(|err: &_, dur: Duration| {
        warn!("retrying {:?} after {:?}", err, dur);
    })
    .await?;
    Err(Error::ListenerStopped)
}
