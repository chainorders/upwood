//! This module contains the transaction processor for the Concordium RWA backend.
//! It includes the definition of the database module, as well as the modules for the RWA identity registry,
//! RWA market, RWA security NFT, and RWA security SFT.
//! It also defines the listener and API configuration struct, as well as the contracts API configuration struct.
//! The module provides functions to run the contracts API server and listener, as well as to generate the API client.
//! It also includes helper functions to create the listener, server routes, and service for the contracts API.

mod db;
pub mod rwa_identity_registry;
pub mod rwa_market;
pub mod rwa_security_nft;
pub mod rwa_security_sft;

use self::db::ContractDb;
use crate::txn_listener::{EventsProcessor, TransactionsListener};
use clap::Parser;
use concordium_rust_sdk::{types::smart_contracts::OwnedContractName, v2::Endpoint};
use log::{debug, info};
use poem::{
    listener::TcpListener,
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use rwa_identity_registry::{db::*, processor::*};
use rwa_market::{api::*, db::*, processor::*};
use rwa_security_nft::{api::*, db::*, processor::*};
use rwa_security_sft::{api::*, db::*, processor::*};
use std::{io::Write, str::FromStr};
use tokio::{spawn, try_join};

/// Implementation of the RWA identity registry database trait for the contract database.
impl IRwaIdentityRegistryDb for ContractDb {}

/// Implementation of the RWA security NFT database trait for the contract database.
impl IRwaSecurityNftDb for ContractDb {}

/// Implementation of the RWA security SFT database trait for the contract database.
impl IRwaSecuritySftDb for ContractDb {}

/// Implementation of the RWA market database trait for the contract database.
impl IRwaMarketDb for ContractDb {}

/// Configuration struct for the listener and API.
#[derive(Parser, Debug, Clone)]
pub struct ListenerAndApiConfig {
    /// The MongoDB URI.
    #[clap(env)]
    pub mongodb_uri: String,
    /// The Concordium node URI.
    #[clap(env)]
    pub concordium_node_uri: String,
    /// The web server address.
    #[clap(env)]
    pub web_server_addr: String,
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
    pub starting_block_hash: String,
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
pub struct ContractsApiConfig {
    /// The URI of the MongoDB instance.
    pub mongodb_uri: String,
    /// The name of the RWA market contract.
    pub rwa_market_contract_name: String,
    /// The name of the RWA security NFT contract.
    pub rwa_security_nft_contract_name: String,
    /// The name of the RWA security SFT contract.
    pub rwa_security_sft_contract_name: String,
}

/// Conversion from ListenerAndApiConfig to ContractsApiConfig.
impl From<ListenerAndApiConfig> for ContractsApiConfig {
    fn from(config: ListenerAndApiConfig) -> Self {
        Self {
            mongodb_uri:                    config.mongodb_uri,
            rwa_market_contract_name:       config.rwa_market_contract_name,
            rwa_security_nft_contract_name: config.rwa_security_nft_contract_name,
            rwa_security_sft_contract_name: config.rwa_security_sft_contract_name,
        }
    }
}

/// Runs the contracts API server and contracts events processor.
pub async fn run_api_server_and_listener(config: ListenerAndApiConfig) -> anyhow::Result<()> {
    debug!("Starting contracts API server with config: {:?}", config);

    let mut listener = create_listener(config.to_owned()).await?;
    let starting_block_hash = if config.starting_block_hash.is_empty() {
        None
    } else {
        Some(config.starting_block_hash.parse()?)
    };
    let listener_handle = spawn(async move { listener.listen(starting_block_hash).await });
    info!("Listening for transactions...");

    let routes = create_server_routes(config.to_owned().into()).await?;
    let server_handle = spawn(async move {
        Server::new(TcpListener::bind(config.web_server_addr))
            .run(routes)
            .await
            .map_err(|e| e.into())
    });
    info!("Listening for web requests...");
    let (listener_handle, server_handle) = try_join!(listener_handle, server_handle)?;
    vec![listener_handle, server_handle].into_iter().for_each(|handle| {
        if let Err(err) = handle {
            log::error!("Error: {:?}", err);
        }
    });
    info!("Shutting down...");
    Ok(())
}

/// Creates the listener for processing transactions.
async fn create_listener(config: ListenerAndApiConfig) -> anyhow::Result<TransactionsListener> {
    let client = mongodb::Client::with_uri_str(&config.mongodb_uri).await?;
    let concordium_client =
        concordium_rust_sdk::v2::Client::new(Endpoint::from_str(&config.concordium_node_uri)?)
            .await?;
    let processors: Vec<Box<dyn EventsProcessor>> = vec![
        Box::new(RwaIdentityRegistryProcessor {
            db:         ContractDb {
                client:        client.to_owned(),
                contract_name: config.rwa_identity_registry_contract_name.try_into()?,
            },
            module_ref: config.rwa_identity_registry_module_ref.parse()?,
        }),
        Box::new(RwaSecurityNftProcessor {
            db:         ContractDb {
                client:        client.to_owned(),
                contract_name: config.rwa_security_nft_contract_name.try_into()?,
            },
            module_ref: config.rwa_security_nft_module_ref.parse()?,
        }),
        Box::new(RwaSecuritySftProcessor {
            db:         ContractDb {
                client:        client.to_owned(),
                contract_name: config.rwa_security_sft_contract_name.try_into()?,
            },
            module_ref: config.rwa_security_sft_module_ref.parse()?,
        }),
        Box::new(RwaMarketProcessor {
            db:         ContractDb {
                client:        client.to_owned(),
                contract_name: config.rwa_market_contract_name.try_into()?,
            },
            module_ref: config.rwa_market_module_ref.parse()?,
        }),
    ];

    let listener =
        TransactionsListener::new(concordium_client, client.to_owned(), processors).await?;
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
) -> Result<
    OpenApiService<
        (RwaMarketApi<ContractDb>, RwaSecurityNftApi<ContractDb>, RwaSecuritySftApi<ContractDb>),
        (),
    >,
    anyhow::Error,
> {
    let mongo_client = mongodb::Client::with_uri_str(&config.mongodb_uri).await?;
    let api_service = OpenApiService::new(
        (
            RwaMarketApi {
                db: ContractDb {
                    client:        mongo_client.to_owned(),
                    contract_name: OwnedContractName::new(
                        config.rwa_market_contract_name.to_owned(),
                    )?,
                },
            },
            RwaSecurityNftApi {
                db: ContractDb {
                    client:        mongo_client.to_owned(),
                    contract_name: OwnedContractName::new(
                        config.rwa_security_nft_contract_name.to_owned(),
                    )?,
                },
            },
            RwaSecuritySftApi {
                db: ContractDb {
                    client:        mongo_client,
                    contract_name: OwnedContractName::new(
                        config.rwa_security_sft_contract_name.to_owned(),
                    )?,
                },
            },
        ),
        "RWA Contracts API",
        "1.0.0",
    );
    Ok(api_service)
}
