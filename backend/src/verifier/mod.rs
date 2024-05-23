mod api;
mod db;
mod identity_registry_client;
mod web3_id_utils;
use self::{
    api::VerifierApi, db::Db, identity_registry_client::IdentityRegistryClient,
    web3_id_utils::CredStatement,
};
use chrono::Datelike;
use clap::Parser;
use concordium_rust_sdk::{
    id::{
        constants::AttributeKind,
        id_proof_types::{AtomicStatement, AttributeInRangeStatement, RevealAttributeStatement},
        types::AttributeTag,
    },
    types::{Address, ContractAddress, Energy, WalletAccount},
    v2::BlockIdentifier,
    web3id::{did::Network, Web3IdAttribute},
};
use futures::{StreamExt, TryStreamExt};
use log::{debug, info};
use poem::{
    listener::TcpListener,
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use std::{io::Write, path::PathBuf, str::FromStr};
use tokio::spawn;

#[derive(Parser, Debug, Clone)]
pub struct ApiConfig {
    #[clap(env)]
    pub concordium_node_uri: String,
    #[clap(env)]
    pub verifier_web_server_addr: String,
    #[clap(env)]
    pub mongodb_uri: String,
    #[clap(env)]
    pub identity_registry: String,
    #[clap(env)]
    pub agent_wallet_path: PathBuf,
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    #[clap(env, default_value = "30000")]
    pub register_identity_max_energy: String,
    #[clap(env, default_value = "testnet")]
    pub network: String,
}

pub async fn run_api_server(config: ApiConfig) -> anyhow::Result<()> {
    debug!("Starting Verifier API Server with config: {:?}", config);

    let routes = create_server_routes(config.to_owned()).await?;
    let web_server_addr = config.verifier_web_server_addr.clone();
    let server_handle =
        spawn(async move { Server::new(TcpListener::bind(web_server_addr)).run(routes).await });
    info!("Listening for web requests at {}", config.verifier_web_server_addr);
    server_handle.await??;
    info!("Shutting Down...");
    Ok(())
}

async fn create_server_routes(config: ApiConfig) -> anyhow::Result<CorsEndpoint<Route>> {
    let api_service = create_service(config).await?;
    let ui = api_service.swagger_ui();
    let routes = Route::new().nest("/", api_service).nest("/ui", ui).with(Cors::new());

    Ok(routes)
}

async fn create_service(
    config: ApiConfig,
) -> Result<OpenApiService<VerifierApi, ()>, anyhow::Error> {
    let mongo_client = mongodb::Client::with_uri_str(&config.mongodb_uri)
        .await
        .map_err(|_| anyhow::Error::msg("Failed to connect to MongoDB"))?;

    let mut concordium_client = concordium_rust_sdk::v2::Client::new(
        concordium_rust_sdk::v2::Endpoint::from_str(&config.concordium_node_uri)?,
    )
    .await
    .map_err(|_| anyhow::Error::msg("Failed to connect to Concordium Node"))?;

    let global_context =
        concordium_client.get_cryptographic_parameters(BlockIdentifier::LastFinal).await?.response;
    let agent_wallet = WalletAccount::from_json_file(config.agent_wallet_path)?;
    let identity_registry = ContractAddress::from_str(&config.identity_registry)?;

    let now = chrono::Utc::now();
    let year = u64::try_from(now.year()).ok().unwrap();
    let years_ago = year.checked_sub(18).unwrap();
    let date_years_ago = format!("{:04}{:02}{:02}", years_ago, now.month(), now.day());
    let upper = Web3IdAttribute::String(AttributeKind(date_years_ago));
    let lower = Web3IdAttribute::String(AttributeKind(String::from("18000101")));
    let id_statement = vec![
        AtomicStatement::AttributeInRange {
            statement: AttributeInRangeStatement {
                // date of birth
                attribute_tag: AttributeTag(3),
                lower,
                upper,
                _phantom: std::marker::PhantomData,
            },
        },
        AtomicStatement::RevealAttribute {
            statement: RevealAttributeStatement {
                // nationality
                attribute_tag: AttributeTag(5),
            },
        },
    ];
    let cred_statement: CredStatement = vec![AtomicStatement::RevealAttribute {
        statement: RevealAttributeStatement {
            // `degreeType` is being used to enable testing of the project using the [web3 id test tools](https://github.com/Concordium/concordium-web3id/blob/main/test-tools/issuer-front-end/README.md)
            attribute_tag: "degreeType".to_string(),
        },
    }];

    let mut identity_registry_client =
        IdentityRegistryClient::new(concordium_client.clone(), identity_registry);
    let issuers = identity_registry_client.issuers().await.map_err(|e| {
        anyhow::Error::msg(format!("Failed to retrieve issuers from identity registry: {:?}", e))
    })?;
    info!("Issuers: {:?}", issuers);

    let is_agent = identity_registry_client
        .is_agent(&Address::Account(agent_wallet.address))
        .await
        .map_err(|e| {
            anyhow::Error::msg(format!(
                "Failed to check if agent is an agent in identity registry: {:?}",
                e
            ))
        })?;
    assert!(is_agent, "provided agent wallet is not an agent in identity registry");

    let identity_providers = concordium_client
        .get_identity_providers(BlockIdentifier::LastFinal)
        .await?
        .response
        .map_ok(|ip_info| ip_info.ip_identity)
        .filter_map(|r| async move {
            match r {
                Ok(r) => Some(r),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>()
        .await;
    info!("Identity Providers: {:?}", identity_providers);

    let api_service = OpenApiService::new(
        VerifierApi {
            id_statement,
            cred_statement,
            identity_registry,
            db: Db {
                client: mongo_client.to_owned(),
                identity_registry,
                agent_address: agent_wallet.address,
            },
            concordium_client,
            agent_wallet,
            global_context,
            max_energy: Energy::from_str(&config.register_identity_max_energy)?,
            network: Network::from_str(&config.network)?,
            issuers,
            identity_providers,
        },
        "RWA Contracts API",
        "1.0.0",
    );
    Ok(api_service)
}

#[derive(Parser, Debug, Clone)]
pub struct OpenApiConfig {
    #[clap(env, default_value = "verifier-api-specs.json")]
    pub output: String,
    #[clap(env, default_value = "http://node.testnet.concordium.com:20000")]
    pub concordium_node_uri: String,
    #[clap(env, default_value = "0.0.0.0:3001")]
    pub verifier_web_server_addr: String,
    #[clap(env, default_value = "mongodb://root:example@localhost:27017")]
    pub mongodb_uri: String,
    /// Identity Registry Contract String
    #[clap(env, default_value = "<7762,0>")]
    pub identity_registry: String,
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    #[clap(env, default_value = "registerIdentity")]
    pub rwa_identity_registry_register_identity_fn_name: String,
    /// Identity Registry Agent Wallet Path
    #[clap(env, default_value = "agent_wallet.export")]
    pub agent_wallet_path: PathBuf,
    /// Max energy to use for register identity
    #[clap(env, default_value = "30000")]
    pub register_identity_max_energy: String,
    #[clap(env, default_value = "testnet")]
    pub network: String,
}

impl From<OpenApiConfig> for ApiConfig {
    fn from(config: OpenApiConfig) -> Self {
        Self {
            concordium_node_uri: config.concordium_node_uri,
            verifier_web_server_addr: config.verifier_web_server_addr,
            mongodb_uri: config.mongodb_uri,
            identity_registry: config.identity_registry,
            rwa_identity_registry_contract_name: config.rwa_identity_registry_contract_name,
            agent_wallet_path: config.agent_wallet_path,
            register_identity_max_energy: config.register_identity_max_energy,
            network: config.network,
        }
    }
}

pub async fn generate_api_client(config: OpenApiConfig) -> anyhow::Result<()> {
    let api_service = create_service(config.to_owned().into()).await?;
    let spec_json = api_service.spec();
    let mut file = std::fs::File::create(config.output)?;
    file.write_all(spec_json.as_bytes())?;
    Ok(())
}
