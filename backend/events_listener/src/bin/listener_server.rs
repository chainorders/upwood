use std::path::Path;
use std::time::Duration;

use backon::{ExponentialBuilder, Retryable};
use clap::Parser;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::AbsoluteBlockHeight;
use concordium_rust_sdk::v2;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use events_listener::listener::{Listener, ListenerError};
use events_listener::processors::Processors;
use r2d2::Pool;
use shared::db_setup;
use tracing::{debug, error, info, warn};
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::TryInitError;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    #[clap(env = "POSTGRES_USER", long, default_value = "concordium_rwa_dev_user")]
    pub postgres_user: String,
    #[clap(env = "POSTGRES_PASSWORD", long, default_value = "concordium_rwa_dev_pswd")]
    pub postgres_password: String,
    #[clap(env = "POSTGRES_HOST", long, default_value = "localhost")]
    pub postgres_host: String,
    #[clap(env = "POSTGRES_PORT", long, default_value = "5432")]
    pub postgres_port: u16,
    #[clap(env = "POSTGRES_DB", long, default_value = "concordium_rwa_dev")]
    pub postgres_db: String,
    #[clap(env = "DB_POOL_MAX_SIZE", long, default_value = "10")]
    pub db_pool_max_size: u32,
    /// The Concordium node URI.
    #[clap(env = "CONCORDIUM_NODE_URI", long, default_value = "https://grpc.testnet.concordium.com:20000")]
    pub concordium_node_uri: String,
    /// The starting block hash.
    #[clap(env = "DEFAULT_BLOCK_HEIGHT", long)]
    pub default_block_height: Option<u64>,
    #[clap(env = "NODE_RATE_LIMIT", long, default_value = "1000")]
    pub node_rate_limit: u64,
    #[clap(env = "NODE_RATE_LIMIT_DURATION_MILLIS", long, default_value = "2000")]
    pub node_rate_limit_duration_millis: u64,
    #[clap(env = "ACCOUNT", long, default_value = "4fWTMJSAymJoFeTbohJzwejT6Wzh1dAa2BtnbDicgjQrc94TgW")]
    pub account: String,
    #[clap(env = "NODE_CONNECT_TIMEOUT_MILLIS", long, default_value = "10000")]
    pub node_connect_timeout_millis: u64,
    #[clap(env = "NODE_REQUEST_TIMEOUT_MILLIS", long, default_value = "10000")]
    pub node_request_timeout_millis: u64,
    #[clap(env = "LISTENER_RETRY_TIMES", long, default_value = "10")]
    pub listener_retry_times: usize,
    #[clap(env = "LISTENER_RETRY_MIN_DELAY_MILLIS", long, default_value = "500")]
    pub listener_retry_min_delay_millis: u64,
    #[clap(env = "LISTENER_RETRY_MAX_DELAY_MILLIS", long, default_value = "10000")]
    pub listener_retry_max_delay_millis: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid node URI: {0}")]
    InvalidNodeUri(#[from] concordium_rust_sdk::endpoints::Error),
    #[error("Listener Error: {0}")]
    ListenerError(#[from] Box<ListenerError>),
    #[error("Listener stopped")]
    ListenerStopped,
    #[error("Tracing subscriber error: {0}")]
    TracingSubscriberError(#[from] TryInitError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load local .env file if it exists (for local development)
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

    let owner_account: AccountAddress = config.account.parse().expect("Invalid account");
    let retry_policy = ExponentialBuilder::default()
        .with_max_times(config.listener_retry_times)
        .with_min_delay(Duration::from_millis(
            config.listener_retry_min_delay_millis,
        ))
        .with_max_delay(Duration::from_millis(
            config.listener_retry_max_delay_millis,
        ));
    let db_pool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(ConnectionManager::<PgConnection>::new(&database_url))
        .expect("Failed to create connection pool");
    let listen = || async {
        let processors = Processors::new(vec![owner_account.to_string()]);
        let mut listener_blocks = Listener::new(
            concordium_client.clone(),
            db_pool.clone(),
            processors,
            default_block_height,
        );
        info!("Contracts Listener: Starting");
        listener_blocks.listen().await?;
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
