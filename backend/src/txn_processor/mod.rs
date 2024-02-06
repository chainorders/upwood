pub mod api;
pub mod db;
mod rwa_identity_registry;
mod rwa_market;
mod rwa_security_nft;

use clap::Parser;
use concordium_rust_sdk::{types::smart_contracts::OwnedContractName, v2::Endpoint};
use log::{debug, info};
use poem::{
    listener::TcpListener,
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use std::{io::Write, str::FromStr};
use tokio::{spawn, try_join};

use rwa_identity_registry::{
    db::IContractDb as IRwaIdentityRegistryDb, processor::Processor as RwaIdentityRegistryProcessor,
};
use rwa_market::{
    api::Api as RwaMarketApi, db::IContractDb as IRwaMarketDb,
    processor::Processor as RwaMarketProcessor,
};
use rwa_security_nft::{
    api::Api as RwaSecurityNftApi, db::IContractDb as IRwaSecurityNftDb,
    processor::Processor as RwaSecurityNftProcessor,
};

use crate::txn_listener::{EventsProcessor, TransactionsListener};

use self::db::ContractDb;
impl IRwaIdentityRegistryDb for ContractDb {}
impl IRwaSecurityNftDb for ContractDb {}
impl IRwaMarketDb for ContractDb {}

#[derive(Parser, Debug, Clone)]
pub struct ContractsListenerAndApiConfig {
    #[clap(env)]
    pub mongodb_uri: String,
    #[clap(env)]
    pub concordium_node_uri: String,
    #[clap(env)]
    pub web_server_addr: String,
    #[clap(env)]
    pub rwa_identity_registry_module_ref: String,
    #[clap(env)]
    pub rwa_security_nft_module_ref: String,
    #[clap(env)]
    pub rwa_market_module_ref: String,
    #[clap(env, default_value = "init_rwa_security_nft")]
    pub rwa_security_nft_contract_name: String,
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    #[clap(env, default_value = "init_rwa_market")]
    pub rwa_market_contract_name: String,
}

pub struct ContractsApiConfig {
    pub mongodb_uri:                    String,
    pub rwa_market_contract_name:       String,
    pub rwa_security_nft_contract_name: String,
}

impl From<ContractsListenerAndApiConfig> for ContractsApiConfig {
    fn from(config: ContractsListenerAndApiConfig) -> Self {
        Self {
            mongodb_uri:                    config.mongodb_uri,
            rwa_market_contract_name:       config.rwa_market_contract_name,
            rwa_security_nft_contract_name: config.rwa_security_nft_contract_name,
        }
    }
}

/// Runs the contracts API server & Contracts events processor
pub async fn run_contracts_api_server(config: ContractsListenerAndApiConfig) -> anyhow::Result<()> {
    debug!("Starting contracts API server with config: {:?}", config);
    
    let mut listener = create_listener(config.to_owned()).await?;
    let listener_handle = spawn(async move { listener.listen().await });
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

async fn create_listener(
    config: ContractsListenerAndApiConfig,
) -> anyhow::Result<TransactionsListener> {
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

async fn create_server_routes(config: ContractsApiConfig) -> anyhow::Result<CorsEndpoint<Route>> {
    let api_service = create_service(config).await?;
    let ui = api_service.swagger_ui();
    let routes = Route::new().nest("/", api_service).nest("/ui", ui).with(Cors::new());

    Ok(routes)
}

#[derive(Parser, Debug, Clone)]
pub struct ContractsApiSwaggerConfig {
    #[clap(env, default_value = "processor-openapi-spec.json")]
    pub output: String,
    #[clap(env, default_value = "mongodb://root:example@localhost:27017")]
    pub mongodb_uri: String,
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    #[clap(env, default_value = "init_rwa_security_nft")]
    pub rwa_security_nft_contract_name: String,
    #[clap(env, default_value = "init_rwa_market")]
    pub rwa_market_contract_name: String,
}

impl From<ContractsApiSwaggerConfig> for ContractsApiConfig {
    fn from(config: ContractsApiSwaggerConfig) -> Self {
        Self {
            mongodb_uri:                    config.mongodb_uri,
            rwa_market_contract_name:       config.rwa_market_contract_name,
            rwa_security_nft_contract_name: config.rwa_security_nft_contract_name,
        }
    }
}

pub async fn generate_contracts_api_frontend_client(
    config: ContractsApiSwaggerConfig,
) -> anyhow::Result<()> {
    let api_service = create_service(config.to_owned().into()).await?;
    let spec_json = api_service.spec();
    let mut file = std::fs::File::create(config.output)?;
    file.write_all(spec_json.as_bytes())?;
    Ok(())
}

async fn create_service(
    config: ContractsApiConfig,
) -> Result<
    OpenApiService<(RwaMarketApi<ContractDb>, RwaSecurityNftApi<ContractDb>), ()>,
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
                    client:        mongo_client,
                    contract_name: OwnedContractName::new(
                        config.rwa_security_nft_contract_name.to_owned(),
                    )?,
                },
            },
        ),
        "RWA Contracts API",
        "1.0.0",
    );
    Ok(api_service)
}
