use std::sync::Arc;

use clap::Parser;
use concordium_rust_sdk::types::WalletAccount;
use concordium_rust_sdk::v2::{self, BlockIdentifier};
use concordium_rust_sdk::web3id::did::Network;
use diesel::r2d2::ConnectionManager;
use events_listener::txn_processor;
use poem::listener::TcpListener;
use poem::middleware::{AddData, Cors};
use poem::{EndpointExt, Route, Server};
use r2d2::Pool;
use shared::db::DbPool;
use tracing::{debug, info};
use upwood::api;

#[derive(Parser, Debug, Clone)]
struct Config {
    #[clap(env, long)]
    pub database_url: String,
    #[clap(env, long)]
    pub concordium_node_uri: String,
    #[clap(env, long)]
    pub concordium_network: String,
    #[clap(env, long)]
    pub db_pool_max_size: u32,
    #[clap(env, long)]
    pub web_server_addr: String,
    #[clap(env, long)]
    pub aws_user_pool_id: String,
    #[clap(env, long)]
    pub aws_user_pool_client_id: String,
    #[clap(env, long)]
    pub user_challenge_expiry_duration_mins: i64,
    #[clap(env, long)]
    pub account_address_attribute_name: String,
    #[clap(env, long)]
    tree_nft_contract: String,
    #[clap(env, long)]
    tree_nft_agent_wallet_json_str: String,
}

#[tokio::main]
async fn main() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    let config = Config::parse();
    info!("Contracts API: Starting Server");
    debug!("{:#?}", config);

    // Database Dependencies
    upwood::db::db_setup::run_migrations(&config.database_url);
    let db_pool: DbPool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(ConnectionManager::new(&config.database_url))
        .unwrap();

    // AWS Dependencies
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    debug!("Loaded AWS config: {:#?}", config);
    let user_pool = upwood::utils::aws::cognito::UserPool::new(
        sdk_config,
        &config.aws_user_pool_id,
        &config.aws_user_pool_client_id,
        &config.account_address_attribute_name,
    )
    .await
    .expect("Failed to create user pool");

    // Concordium Dependencies
    let endpoint: v2::Endpoint = config
        .concordium_node_uri
        .parse()
        .expect("Failed to parse Concordium Node URI");
    let mut concordium_client = v2::Client::new(endpoint)
        .await
        .expect("Failed to create Concordium Client");
    let global_context = concordium_client
        .get_cryptographic_parameters(BlockIdentifier::LastFinal)
        .await
        .expect("Failed to get concordium cryptographic parameters")
        .response;
    let network: Network = config
        .concordium_network
        .parse()
        .expect("Failed to parse Concordium Network");
    let tree_nft_agent_wallet =
        WalletAccount::from_json_str(&config.tree_nft_agent_wallet_json_str)
            .expect("Failed to parse Tree NFT Agent Wallet JSON");
    let api = upwood::api::create_service();
    let ui = api.swagger_ui();
    let api = api
        .with(AddData::new(db_pool))
        .with(AddData::new(user_pool))
        .with(AddData::new(global_context))
        .with(AddData::new(concordium_client))
        .with(AddData::new(network))
        .with(AddData::new(txn_processor::nft_multi_rewarded::api::Api))
        .with(AddData::new(api::user::UserChallengeConfig {
            challenge_expiry_duration: chrono::Duration::minutes(
                config.user_challenge_expiry_duration_mins,
            ),
        }))
        .with(AddData::new(api::tree_nft_metadata::TreeNftConfig {
            contract: config.tree_nft_contract,
            agent:    Arc::new(api::tree_nft_metadata::TreeNftAgent(tree_nft_agent_wallet)),
        }))
        .with(Cors::new());
    let routes = Route::new().nest("/", api).nest("/ui", ui);
    info!("Starting Server at {}", config.web_server_addr);
    Server::new(TcpListener::bind(config.web_server_addr))
        .run(routes)
        .await
        .expect("Server runtime error");
}
