//! This module contains the transaction processor for the Concordium RWA
//! backend. It includes the definition of the database module, as well as the
//! modules for the RWA identity registry, RWA market, RWA security NFT, and RWA
//! security SFT. It also defines the listener and API configuration struct, as
//! well as the contracts API configuration struct. The module provides
//! functions to run the contracts API server and listener, as well as to
//! generate the API client. It also includes helper functions to create the
//! listener, server routes, and service for the contracts API.

pub mod rwa_identity_registry;
pub mod rwa_market;
pub mod rwa_security_nft;
pub mod rwa_security_sft;

use crate::txn_listener::{DatabaseClient, EventsProcessor, TransactionsListener};
use clap::Parser;
use concordium_rust_sdk::{
    types::{smart_contracts::OwnedContractName, AbsoluteBlockHeight},
    v2::{self, Endpoint},
};
use futures::TryFutureExt;
use log::{debug, info};
use poem::{
    listener::TcpListener,
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use rwa_identity_registry::processor::*;
use rwa_market::{api::*, processor::*};
use rwa_security_nft::{api::*, processor::*};
use rwa_security_sft::{api::*, processor::*};
use std::{io::Write, str::FromStr, time::Duration};
use tokio::spawn;

#[derive(Parser, Debug, Clone)]
pub struct ListenerConfig {
    /// The MongoDB URI.
    #[clap(env)]
    pub mongodb_uri: String,
    /// The Concordium node URI.
    #[clap(env)]
    pub concordium_node_uri: String,
    /// The reference to the RWA identity registry module.
    #[clap(env)]
    pub rwa_identity_registry_module_ref: String,
    /// The reference to the RWA security NFT module.
    #[clap(env)]
    pub rwa_security_nft_module_ref: String,
    /// The reference to the RWA security SFT module.
    #[clap(env)]
    pub rwa_security_sft_module_ref: String,
    /// The reference to the RWA market module.
    #[clap(env)]
    pub rwa_market_module_ref: String,
    /// The starting block hash.
    #[clap(env, default_value = "")]
    pub default_block_height: u64,
    #[clap(env, default_value = "100")]
    pub node_rate_limit: u64,
    #[clap(env, default_value = "1")]
    pub node_rate_limit_duration_secs: u64,
    /// The name of the RWA security NFT contract.
    #[clap(env, default_value = "init_rwa_security_nft")]
    pub rwa_security_nft_contract_name: String,
    /// The name of the RWA security SFT contract.
    #[clap(env, default_value = "init_rwa_security_sft")]
    pub rwa_security_sft_contract_name: String,
    /// The name of the RWA identity registry contract.
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    /// The name of the RWA market contract.
    #[clap(env, default_value = "init_rwa_market")]
    pub rwa_market_contract_name: String,
}

/// Configuration struct for the contracts API.
/// Configuration options for the Contracts API.
#[derive(Parser, Debug, Clone)]
pub struct ContractsApiConfig {
    /// The URI of the MongoDB instance.
    #[clap(env)]
    pub mongodb_uri:                    String,
    #[clap(env, default_value = "init_rwa_market")]
    pub rwa_market_contract_name:       String,
    /// The name of the RWA security NFT contract.
    #[clap(env, default_value = "init_rwa_security_nft")]
    pub rwa_security_nft_contract_name: String,
    /// The name of the RWA security SFT contract.
    #[clap(env, default_value = "init_rwa_security_sft")]
    pub rwa_security_sft_contract_name: String,
    #[clap(env)]
    pub web_server_addr:                String,
}

pub async fn run_listener(config: ListenerConfig) -> anyhow::Result<()> {
    debug!("Starting contracts listener with config: {:?}", config);

    let listener_handle = spawn(create_listener(config).and_then(TransactionsListener::listen));
    info!("Listening for transactions...");
    listener_handle.await?
}

pub async fn run_api(config: ContractsApiConfig) -> anyhow::Result<()> {
    let routes = create_server_routes(config.to_owned()).await?;
    let web_server_addr = config.web_server_addr.clone();
    let server_handle = spawn(Server::new(TcpListener::bind(web_server_addr)).run(routes));
    info!("Listening for web requests at {}...", config.web_server_addr);
    server_handle.await?.map_err(|e| e.into())
}

/// Creates the listener for processing transactions.
async fn create_listener(config: ListenerConfig) -> anyhow::Result<TransactionsListener> {
    let client = mongodb::Client::with_uri_str(&config.mongodb_uri).await?;
    let processors: Vec<Box<dyn EventsProcessor>> = vec![
        Box::new(RwaIdentityRegistryProcessor {
            module_ref:    config.rwa_identity_registry_module_ref.parse()?,
            contract_name: OwnedContractName::new(config.rwa_identity_registry_contract_name)?,
            client:        client.to_owned(),
        }),
        Box::new(RwaSecurityNftProcessor {
            module_ref:    config.rwa_security_nft_module_ref.parse()?,
            client:        client.clone(),
            contract_name: OwnedContractName::new(config.rwa_security_nft_contract_name)?,
        }),
        Box::new(RwaSecuritySftProcessor {
            module_ref:    config.rwa_security_sft_module_ref.parse()?,
            client:        client.clone(),
            contract_name: OwnedContractName::new(config.rwa_security_sft_contract_name)?,
        }),
        Box::new(RwaMarketProcessor {
            module_ref:    config.rwa_market_module_ref.parse()?,
            client:        client.clone(),
            contract_name: OwnedContractName::new(config.rwa_market_contract_name)?,
        }),
    ];

    let endpoint = Endpoint::from_str(&config.concordium_node_uri)?;
    let endpoint = endpoint.rate_limit(
        config.node_rate_limit,
        Duration::from_secs(config.node_rate_limit_duration_secs),
    );

    let listener = TransactionsListener::new(
        v2::Client::new(endpoint).await?,
        DatabaseClient::init(client).await?,
        processors,
        AbsoluteBlockHeight {
            height: config.default_block_height,
        },
    )
    .await?;
    Ok(listener)
}

/// Creates the server routes for the contracts API.
async fn create_server_routes(config: ContractsApiConfig) -> anyhow::Result<CorsEndpoint<Route>> {
    let api_service = create_service(config).await?;
    let ui = api_service.swagger_ui();
    let routes = Route::new().nest("/", api_service).nest("/ui", ui).with(Cors::new());

    Ok(routes)
}

/// Configuration struct for generating the API client.
#[derive(Parser, Debug, Clone)]
/// Configuration struct for OpenAPI.
pub struct OpenApiConfig {
    /// The output file path for the OpenAPI specification.
    #[clap(env, default_value = "processor-openapi-spec.json")]
    pub output: String,
    /// The URI for the MongoDB connection.
    #[clap(env, default_value = "mongodb://root:example@localhost:27017")]
    pub mongodb_uri: String,
    /// The contract name for the RWA identity registry.
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    /// The contract name for the RWA security NFT.
    #[clap(env, default_value = "init_rwa_security_nft")]
    pub rwa_security_nft_contract_name: String,
    /// The contract name for the RWA security SFT.
    #[clap(env, default_value = "init_rwa_security_sft")]
    pub rwa_security_sft_contract_name: String,
    /// The contract name for the RWA market.
    #[clap(env, default_value = "init_rwa_market")]
    pub rwa_market_contract_name: String,
}

/// Conversion from OpenApiConfig to ContractsApiConfig.
impl From<OpenApiConfig> for ContractsApiConfig {
    fn from(config: OpenApiConfig) -> Self {
        Self {
            mongodb_uri:                    config.mongodb_uri,
            rwa_market_contract_name:       config.rwa_market_contract_name,
            rwa_security_nft_contract_name: config.rwa_security_nft_contract_name,
            rwa_security_sft_contract_name: config.rwa_security_sft_contract_name,
            web_server_addr:                "anything.com".to_owned(),
        }
    }
}

/// Generates the API client based on the OpenAPI configuration.
pub async fn generate_api_client(config: OpenApiConfig) -> anyhow::Result<()> {
    let api_service = create_service(config.to_owned().into()).await?;
    let spec_json = api_service.spec();
    let mut file = std::fs::File::create(config.output)?;
    file.write_all(spec_json.as_bytes())?;
    Ok(())
}

/// Creates the service for the contracts API.
async fn create_service(
    config: ContractsApiConfig,
) -> Result<OpenApiService<(RwaMarketApi, RwaSecurityNftApi, RwaSecuritySftApi), ()>, anyhow::Error>
{
    let mongo_client = mongodb::Client::with_uri_str(&config.mongodb_uri).await?;
    let api_service = OpenApiService::new(
        (
            RwaMarketApi {
                client:        mongo_client.to_owned(),
                contract_name: config.rwa_market_contract_name,
            },
            RwaSecurityNftApi {
                client:        mongo_client.to_owned(),
                contract_name: config.rwa_security_nft_contract_name,
            },
            RwaSecuritySftApi {
                client:        mongo_client.to_owned(),
                contract_name: config.rwa_security_sft_contract_name,
            },
        ),
        "RWA Contracts API",
        "1.0.0",
    );
    Ok(api_service)
}
