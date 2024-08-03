mod api;
mod db;
mod identity_registry_client;
mod web3_id_utils;
use crate::shared::db::DbPool;

use self::{
    api::VerifierApi, identity_registry_client::IdentityRegistryClient,
    web3_id_utils::CredStatement,
};
use chrono::Datelike;
use clap::Parser;
use concordium_rust_sdk::{
    base::contracts_common::NonZeroThresholdU8,
    id::{
        constants::AttributeKind,
        id_proof_types::{AtomicStatement, AttributeInRangeStatement, RevealAttributeStatement},
        types::{AccountAddress, AccountKeys, AttributeTag, ACCOUNT_ADDRESS_SIZE},
    },
    types::{Address, ContractAddress, Energy, WalletAccount},
    v2::BlockIdentifier,
    web3id::{did::Network, Web3IdAttribute},
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use futures::{StreamExt, TryStreamExt};
use log::{debug, info};
use poem::{
    listener::TcpListener,
    middleware::{AddData, Cors},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use std::{collections::BTreeMap, io::Write, path::PathBuf, str::FromStr};
use tokio::spawn;
use web3_id_utils::IdStatement;

#[derive(Parser, Debug, Clone)]
pub struct ApiConfig {
    #[clap(env, long)]
    pub concordium_node_uri: String,
    #[clap(env, long)]
    pub verifier_web_server_addr: String,
    #[clap(env, long)]
    pub database_url: String,
    #[clap(env, long)]
    pub db_pool_max_size: u32,
    #[clap(env, long)]
    pub identity_registry: String,
    #[clap(env, long)]
    pub agent_wallet_path: PathBuf,
    #[clap(env, long)]
    pub rwa_identity_registry_contract_name: String,
    #[clap(env, long)]
    pub register_identity_max_energy: String,
    #[clap(env, long)]
    pub network: String,
}

pub async fn run_api_server(config: ApiConfig) -> anyhow::Result<()> {
    info!("Verifier API: Starting  Server");
    debug!("{:#?}", config);

    let routes = create_server_routes(config.to_owned()).await?;
    let web_server_addr = config.verifier_web_server_addr.clone();
    let server_handle =
        spawn(async move { Server::new(TcpListener::bind(web_server_addr)).run(routes).await });
    info!("Verifier API: Listening for web requests at {}", config.verifier_web_server_addr);
    server_handle.await??;
    info!("Verifier API: Shutting Down...");
    Ok(())
}

async fn create_server_routes(config: ApiConfig) -> anyhow::Result<impl poem::Endpoint> {
    let agent_wallet = WalletAccount::from_json_file(config.agent_wallet_path)?;
    let agent_address = agent_wallet.address;
    let identity_registry = ContractAddress::from_str(&config.identity_registry)?;

    let mut concordium_client = concordium_rust_sdk::v2::Client::new(
        concordium_rust_sdk::v2::Endpoint::from_str(&config.concordium_node_uri)?,
    )
    .await
    .map_err(|_| anyhow::Error::msg("Failed to connect to Concordium Node"))?;
    let global_context =
        concordium_client.get_cryptographic_parameters(BlockIdentifier::LastFinal).await?.response;
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

    let now = chrono::Utc::now();
    let year = u64::try_from(now.year()).ok().unwrap();
    let years_ago = year.checked_sub(18).unwrap();
    let date_years_ago = format!("{:04}{:02}{:02}", years_ago, now.month(), now.day());
    let upper = Web3IdAttribute::String(AttributeKind(date_years_ago));
    let lower = Web3IdAttribute::String(AttributeKind(String::from("18000101")));
    let id_statement: IdStatement = vec![
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
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool: DbPool = Pool::builder().max_size(config.db_pool_max_size).build(manager).unwrap();

    let api_service = create_service(agent_wallet).await?;
    let ui = api_service.swagger_ui();

    let routes = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .with(AddData::new(pool))
        .with(AddData::new(global_context))
        .with(AddData::new(identity_providers))
        .with(AddData::new(issuers))
        .with(AddData::new(concordium_client))
        .with(AddData::new(id_statement))
        .with(AddData::new(cred_statement))
        .with(AddData::new(identity_registry))
        .with(AddData::new(Network::from_str(&config.network)?))
        .with(AddData::new(Energy::from_str(&config.register_identity_max_energy)?))
        .with(AddData::new(agent_address))
        .with(Cors::new());

    Ok(routes)
}

async fn create_service(
    agent_wallet: WalletAccount,
) -> Result<OpenApiService<VerifierApi, ()>, anyhow::Error> {
    let api_service = OpenApiService::new(
        VerifierApi {
            agent_wallet,
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
}

pub async fn generate_open_api_specs(config: OpenApiConfig) -> anyhow::Result<()> {
    let dummy_wallet = WalletAccount {
        address: AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        keys:    AccountKeys {
            threshold: NonZeroThresholdU8::ONE,
            keys:      BTreeMap::new(),
        },
    };
    let api_service = create_service(dummy_wallet).await?;
    let spec_json = api_service.spec();
    let mut file = std::fs::File::create(config.output)?;
    file.write_all(spec_json.as_bytes())?;
    Ok(())
}
