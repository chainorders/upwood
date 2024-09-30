use std::path::Path;

use clap::Parser;
use poem::listener::TcpListener;
use poem::middleware::{AddData, Cors};
use poem::{EndpointExt, Route, Server};
use tracing::{debug, info};

#[derive(Parser, Debug, Clone)]
struct Config {
    /// Postgres Database Url
    #[clap(env, long)]
    pub database_url:            String,
    #[clap(env, long)]
    pub db_pool_max_size:        u32,
    #[clap(env, long)]
    pub web_server_addr:         String,
    #[clap(env, long)]
    pub aws_user_pool_region:    String,
    #[clap(env, long)]
    pub aws_user_pool_id:        String,
    #[clap(env, long)]
    pub aws_user_pool_client_id: String,
}

#[tokio::main]
async fn main() {
    dotenvy::from_filename(Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();

    let config = Config::parse();
    info!("Contracts API: Starting Server");
    debug!("{:#?}", config);

    let db_pool = upwood::db::setup_database(&config.database_url, config.db_pool_max_size);
    let api_service = upwood::api::create_service();
    let ui = api_service.swagger_ui();
    let user_pool = upwood::user_pool::UserPool::new(
        &config.aws_user_pool_region,
        &config.aws_user_pool_id,
        &config.aws_user_pool_client_id,
    )
    .await
    .expect("Failed to create user pool");

    let routes = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .with(AddData::new(db_pool))
        .with(AddData::new(user_pool))
        .with(Cors::new());
    info!("Starting Server at {}", config.web_server_addr);
    Server::new(TcpListener::bind(config.web_server_addr))
        .run(routes)
        .await
        .expect("Server runtime error");
}
