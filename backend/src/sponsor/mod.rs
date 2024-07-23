//! This module contains the implementation of the Sponsor API server.
//! It provides functionality for interacting with the Concordium node and
//! handling sponsorship-related operations. The API server is configured using
//! the `ApiConfig` struct. The server can be started by calling the
//! `run_api_server` function. The API server routes are created using the
//! `create_server_routes` function. The actual API service is created using the
//! `create_service` function. The `OpenApiConfig` struct is used for
//! configuring the OpenAPI specification generation. The `generate_api_client`
//! function can be used to generate an API client based on the OpenAPI
//! specification.

use self::api::Api;
use clap::Parser;
use concordium_rust_sdk::{
    base::contracts_common::NonZeroThresholdU8,
    id::types::{AccountAddress, AccountKeys, ACCOUNT_ADDRESS_SIZE},
    types::{ContractAddress, Energy, WalletAccount},
    v2,
};
use log::{debug, info};
use poem::{
    listener::TcpListener,
    middleware::{AddData, Cors},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use std::{collections::BTreeMap, io::Write, path::PathBuf, str::FromStr};
use tokio::spawn;

pub mod api;
pub mod sponsor_client;

#[derive(Parser, Debug, Clone)]
/// Configuration struct for the API.
pub struct ApiConfig {
    /// The URI of the Concordium node.
    #[clap(env)]
    pub concordium_node_uri: String,

    /// The address of the sponsor web server.
    #[clap(env)]
    pub sponsor_web_server_addr: String,

    /// The contract used for sponsorship.
    #[clap(env)]
    pub sponsor_contract: String,

    /// The path to the sponsor's wallet.
    #[clap(env)]
    pub sponsor_wallet_path: PathBuf,

    /// The maximum energy permitted for a transaction.
    #[clap(env, default_value = "30000")]
    pub permit_max_energy: String,
}

/// Runs the Sponsor API server with the given configuration.
///
/// # Arguments
///
/// * `config` - The configuration for the API server.
///
/// # Returns
///
/// Returns `Ok(())` if the server runs successfully, otherwise returns an
/// `anyhow::Result` with an error.
pub async fn run_api_server(config: ApiConfig) -> anyhow::Result<()> {
    info!("Sponsor API: Starting Server");
    debug!("{:#?}", config);

    let routes = create_server_routes(config.to_owned()).await?;
    let web_server_addr = config.sponsor_web_server_addr.clone();
    let server_handle =
        spawn(async move { Server::new(TcpListener::bind(web_server_addr)).run(routes).await });
    info!("Sponsor API: Listening for web requests at {}", config.sponsor_web_server_addr);
    server_handle.await??;
    info!("Sponsor API: Shutting Down");
    Ok(())
}

async fn create_server_routes(config: ApiConfig) -> anyhow::Result<impl poem::Endpoint> {
    let wallet = WalletAccount::from_json_file(config.sponsor_wallet_path)?;
    let endpoint: v2::Endpoint = config.concordium_node_uri.parse()?;
    let concordium_client = v2::Client::new(endpoint)
        .await
        .map_err(|_| anyhow::Error::msg("Failed to connect to Concordium Node"))?;

    let api_service = create_service(wallet);
    let ui = api_service.swagger_ui();
    let routes = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .with(Cors::new())
        .with(AddData::new(Energy::from_str(&config.permit_max_energy)?))
        .with(AddData::new(ContractAddress::from_str(&config.sponsor_contract)?))
        .with(AddData::new(concordium_client));

    Ok(routes)
}

fn create_service(wallet: WalletAccount) -> OpenApiService<Api, ()> {
    OpenApiService::new(
        Api {
            wallet,
        },
        "RWA Contracts API",
        "1.0.0",
    )
}

#[derive(Parser, Debug, Clone)]
/// Configuration struct for OpenAPI.
pub struct OpenApiConfig {
    /// Output file path for the generated OpenAPI specs
    #[clap(env, default_value = "sponsor-api-specs.json")]
    pub output: String,
}

/// Generates an API client based on the OpenAPI specification.
/// The client is generated based on the provided `OpenApiConfig`.
/// The generated client is written to the file specified in the
/// `OpenApiConfig`. The client is generated using the `create_service`
/// function.
pub async fn generate_api_client(config: OpenApiConfig) -> anyhow::Result<()> {
    let dummy_wallet = WalletAccount {
        address: AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        keys:    AccountKeys {
            threshold: NonZeroThresholdU8::ONE,
            keys:      BTreeMap::new(),
        },
    };

    let api_service = create_service(dummy_wallet);
    let spec_json = api_service.spec();
    let mut file = std::fs::File::create(config.output)?;
    file.write_all(spec_json.as_bytes())?;
    Ok(())
}
