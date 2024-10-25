use poem::listener::TcpListener;
use poem::Server;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::TryInitError;
use upwood::api;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid node URI: {0}")]
    InvalidNodeUri(#[from] concordium_rust_sdk::endpoints::Error),
    #[error("API server error: {0}")]
    ApiStopped(#[from] std::io::Error),
    #[error("Tracing subscriber error: {0}")]
    TracingSubscriberError(#[from] TryInitError),
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .flatten_event(false)
                .with_current_span(false)
                .with_span_list(true)
                .with_target(false),
        )
        .try_init()?;

    // Load environment variables from .env file & parse them
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("secure.env"))
        .ok();
    let config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()?;
    info!("configuration: {:#?}", config);

    let web_server_addr = format!("{}:{}", config.api_socket_address, config.api_socket_port);
    info!("Starting Server at {}", web_server_addr);
    Server::new(TcpListener::bind(web_server_addr))
        .run(api::create_web_app(&config).await)
        .await?;

    Ok(())
}
