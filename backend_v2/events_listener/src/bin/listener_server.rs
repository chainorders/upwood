use std::{sync::Arc, time::Duration};

use clap::Parser;
use concordium_rust_sdk::{base::smart_contracts::OwnedContractName, v2};
use concordium_rwa_events_listener::{
    txn_listener::{EventsProcessor, TransactionsListener},
    txn_processor::{
        rwa_identity_registry::processor::RwaIdentityRegistryProcessor,
        rwa_market::processor::RwaMarketProcessor,
        rwa_security_cis2::processor::RwaSecurityCIS2Processor,
    },
};
use concordium_rwa_security_sft::types::NftTokenId;
use concordium_rwa_utils::cis2_types::{NftTokenAmount, SftTokenAmount, SftTokenId};
use diesel::{r2d2::ConnectionManager, PgConnection};
use log::{debug, info};
use r2d2::Pool;
use tokio::sync::RwLock;

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
    /// The reference to the RWA identity registry module.
    #[clap(env, long)]
    pub rwa_identity_registry_module_ref: String,
    /// The reference to the RWA security NFT module.
    #[clap(env, long)]
    pub rwa_security_nft_module_ref: String,
    /// The reference to the RWA security SFT module.
    #[clap(env, long)]
    pub rwa_security_sft_module_ref: String,
    /// The reference to the RWA market module.
    #[clap(env, long)]
    pub rwa_market_module_ref: String,
    /// The starting block hash.
    #[clap(env, long)]
    pub default_block_height: u64,
    #[clap(env, long)]
    pub node_rate_limit: u64,
    #[clap(env, long)]
    pub node_rate_limit_duration_secs: u64,
    /// The name of the RWA security NFT contract.
    #[clap(env, long)]
    pub rwa_security_nft_contract_name: String,
    /// The name of the RWA security SFT contract.
    #[clap(env, long)]
    pub rwa_security_sft_contract_name: String,
    /// The name of the RWA identity registry contract.
    #[clap(env, long)]
    pub rwa_identity_registry_contract_name: String,
    /// The name of the RWA market contract.
    #[clap(env, long)]
    pub rwa_market_contract_name: String,
}

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".listener.env").ok();
    env_logger::init();
    let config = Config::parse();
    info!("Contracts Listener: Starting");
    debug!("{:#?}", config);

    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(manager)
        .expect("Failed to create connection pool");
    let processors: Vec<Arc<RwLock<dyn EventsProcessor>>> = vec![
        Arc::new(RwLock::new(RwaIdentityRegistryProcessor {
            module_ref:    config
                .rwa_identity_registry_module_ref
                .parse()
                .expect("Invalid identity registry module ref"),
            contract_name: OwnedContractName::new(config.rwa_identity_registry_contract_name)
                .expect("Invalid identity registry contract name"),
            pool:          pool.clone(),
        })),
        Arc::new(RwLock::new(RwaSecurityCIS2Processor::<NftTokenId, NftTokenAmount>::new(
            pool.clone(),
            config.rwa_security_nft_module_ref.parse().expect("Invalid security NFT module ref"),
            OwnedContractName::new(config.rwa_security_nft_contract_name)
                .expect("Invalid security NFT contract name"),
        ))),
        Arc::new(RwLock::new(RwaSecurityCIS2Processor::<SftTokenId, SftTokenAmount>::new(
            pool.clone(),
            config.rwa_security_sft_module_ref.parse().expect("Invalid security SFT module ref"),
            OwnedContractName::new(config.rwa_security_sft_contract_name)
                .expect("Invalid security SFT contract name"),
        ))),
        Arc::new(RwLock::new(RwaMarketProcessor {
            module_ref:    config.rwa_market_module_ref.parse().expect("Invalid market module ref"),
            contract_name: OwnedContractName::new(config.rwa_market_contract_name)
                .expect("Invalid market contract name"),
            pool:          pool.clone(),
        })),
    ];

    let endpoint: v2::Endpoint =
        config.concordium_node_uri.parse().expect("Failed to parse Concordium node URI");
    let endpoint = endpoint.rate_limit(
        config.node_rate_limit,
        Duration::from_secs(config.node_rate_limit_duration_secs),
    );
    let concordium_client =
        v2::Client::new(endpoint).await.expect("Failed to create Concordium client");

    let listener = TransactionsListener::new(
        concordium_client,
        pool.clone(),
        processors,
        config.default_block_height.into(),
    );

    listener.listen().await.expect("Listener runtime error");
}
