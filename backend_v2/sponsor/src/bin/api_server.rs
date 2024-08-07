use concordium_rwa_sponsor_api::api;

use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use concordium_rust_sdk::{
    types::{ContractAddress, Energy, WalletAccount},
    v2,
};
use dotenv::dotenv;
use log::{debug, info};
use poem::{
    listener::TcpListener,
    middleware::{AddData, Cors},
    EndpointExt, Route, Server,
};

#[derive(Parser, Debug, Clone)]
/// Configuration struct for the API.
pub struct Config {
    /// The URI of the Concordium node.
    #[clap(env, long)]
    pub concordium_node_uri: String,

    /// The address of the sponsor web server.
    #[clap(env, long)]
    pub web_server_addr: String,

    /// The contract used for sponsorship.
    #[clap(env, long)]
    pub contract: String,

    /// The path to the sponsor's wallet.
    #[clap(env, long)]
    pub wallet_path: PathBuf,

    /// The maximum energy permitted for a transaction.
    #[clap(env, long)]
    pub max_energy: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let config = Config::parse();
    info!("Sponsor API: Starting Server");
    debug!("{:#?}", config);

    let wallet = WalletAccount::from_json_file(config.wallet_path).expect("Failed to load wallet");
    let endpoint: v2::Endpoint =
        config.concordium_node_uri.parse().expect("Failed to parse Concordium Node URI");
    let concordium_client =
        v2::Client::new(endpoint).await.expect("Failed to create Concordium Client");
    let max_energy: Energy = config.max_energy.parse().expect("Failed to parse max energy");
    let contract_address: ContractAddress =
        config.contract.parse().expect("Failed to parse contract address");

    let api_service = api::create_service();
    let ui = api_service.swagger_ui();
    let routes = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .with(Cors::new())
        .with(AddData::new(max_energy))
        .with(AddData::new(contract_address))
        .with(AddData::new(concordium_client))
        .with(AddData::new(Arc::new(wallet)));

    info!("Starting Server at {}", config.web_server_addr);
    Server::new(TcpListener::bind(config.web_server_addr))
        .run(routes)
        .await
        .expect("Server runtime error");
}
