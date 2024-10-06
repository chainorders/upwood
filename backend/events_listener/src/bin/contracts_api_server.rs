use std::path::Path;

use clap::Parser;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use events_listener::{db_setup, txn_processor};
use poem::listener::TcpListener;
use poem::middleware::{AddData, Cors};
use poem::{EndpointExt, Route, Server};
use shared::db::DbPool;
use tracing::{debug, info};

/// Configuration struct for the contracts API.
/// Configuration options for the Contracts API.
#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// Postgres Database Url
    #[clap(env, long)]
    pub database_url:     String,
    #[clap(env, long)]
    pub db_pool_max_size: u32,
    #[clap(env, long)]
    pub web_server_addr:  String,
}

#[tokio::main]
async fn main() {
    dotenvy::from_filename(Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();

    let config = Config::parse();
    info!("Contracts API: Starting Server");
    debug!("{:#?}", config);

    db_setup::run_migrations(&config.database_url);
    let api_service = txn_processor::create_service();
    let ui = api_service.swagger_ui();
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool: DbPool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(manager)
        .unwrap();
    let routes = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .with(AddData::new(pool))
        .with(Cors::new());
    info!("Starting Server at {}", config.web_server_addr);
    Server::new(TcpListener::bind(config.web_server_addr))
        .run(routes)
        .await
        .expect("Server runtime error");
}
