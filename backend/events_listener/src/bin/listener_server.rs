use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use backon::{ExponentialBuilder, Retryable};
use clap::Parser;
use concordium_rust_sdk::types::AbsoluteBlockHeight;
use concordium_rust_sdk::v2;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use events_listener::txn_listener::listener::{ListenerConfig, ListenerError, ProcessorFnType};
use events_listener::txn_processor::cis2_security::{security_sft_rewards, security_sft_single};
use events_listener::txn_processor::{identity_registry, nft_multi_rewarded, security_mint_fund};
use events_listener::{db_setup, txn_listener};
use r2d2::Pool;
use tracing::{debug, error, info, warn};
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::TryInitError;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    #[clap(env, long)]
    pub postgres_user: String,
    #[clap(env, long)]
    pub postgres_password: String,
    #[clap(env, long)]
    pub postgres_host: String,
    #[clap(env, long)]
    pub postgres_port: u16,
    #[clap(env, long)]
    pub postgres_db: String,
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
    #[clap(env, long)]
    pub listener_retry_max_delay_millis: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid node URI: {0}")]
    InvalidNodeUri(#[from] concordium_rust_sdk::endpoints::Error),
    #[error("Listener Error: {0}")]
    ListenerError(#[from] ListenerError),
    #[error("Listener stopped")]
    ListenerStopped,
    #[error("Tracing subscriber error: {0}")]
    TracingSubscriberError(#[from] TryInitError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::from_filename(Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    let subscriber = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(false)
        .with_current_span(false)
        .with_span_list(true)
        .with_target(false);
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(subscriber)
        .try_init()?;

    let config = Config::parse();
    debug!("{:#?}", config);
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres_user,
        config.postgres_password,
        config.postgres_host,
        config.postgres_port,
        config.postgres_db
    );
    db_setup::run_migrations(&database_url);
    let db_pool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(ConnectionManager::<PgConnection>::new(&database_url))
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
    let mut concordium_client = v2::Client::new(endpoint).await.unwrap_or_else(|_| {
        panic!(
            "Failed to connect to Concordium node at {}",
            config.concordium_node_uri
        )
    });

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
        let mut map = BTreeMap::new();
        map.insert(
            (
                security_sft_rewards::module_ref(),
                security_sft_rewards::contract_name(),
            ),
            security_sft_rewards::process_events as ProcessorFnType,
        );
        map.insert(
            (
                security_sft_single::module_ref(),
                security_sft_single::contract_name(),
            ),
            security_sft_single::process_events as ProcessorFnType,
        );
        map.insert(
            (
                identity_registry::module_ref(),
                identity_registry::contract_name(),
            ),
            identity_registry::processor::process_events as ProcessorFnType,
        );
        map.insert(
            (
                nft_multi_rewarded::module_ref(),
                nft_multi_rewarded::contract_name(),
            ),
            nft_multi_rewarded::processor::process_events as ProcessorFnType,
        );
        map.insert(
            (
                security_mint_fund::module_ref(),
                security_mint_fund::contract_name(),
            ),
            security_mint_fund::processor::process_events as ProcessorFnType,
        );
        map
    };

    let owner_account = config.account.parse().expect("Invalid account");
    let retry_policy = ExponentialBuilder::default()
        .with_max_times(config.listener_retry_times)
        .with_min_delay(Duration::from_millis(
            config.listener_retry_min_delay_millis,
        ))
        .with_max_delay(Duration::from_millis(
            config.listener_retry_max_delay_millis,
        ));

    let listener_config = ListenerConfig::new(
        concordium_client,
        db_pool,
        owner_account,
        processors,
        default_block_height,
    );
    let listen = || async {
        info!("Contracts Listener: Starting");
        txn_listener::listener::listen(listener_config.clone()).await?;
        Ok::<_, ListenerError>(())
    };

    listen
    .retry(retry_policy)
    .sleep(tokio::time::sleep)
    .when(|e: &ListenerError| e.is_retryable())
    // Notify when retrying
    .notify(|err: &_, dur: Duration| {
        warn!("retrying {:?} after {:?}", err, dur);
    })
    .await
    .expect("Listener stopped");
    Err(Error::ListenerStopped)
}
