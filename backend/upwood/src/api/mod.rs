pub mod files;
pub mod tree_nft_contract;
pub mod tree_nft_metadata;
pub mod user;

use std::sync::Arc;
use std::time::Duration;

use aws::s3;
use concordium::chain::concordium_global_context;
use concordium_rust_sdk::types::{ContractAddress, WalletAccount};
use concordium_rust_sdk::v2;
use concordium_rust_sdk::web3id::did::Network;
use diesel::r2d2::ConnectionManager;
use events_listener::txn_processor::identity_registry::model::IdentityRegistry;
use poem::http::StatusCode;
use poem::middleware::{AddData, Cors, Tracing};
use poem::{EndpointExt, Route};
use poem_openapi::auth::Bearer;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, SecurityScheme};
use r2d2::Pool;
use secure_string::SecureString;
use serde::Deserialize;
use sha2::Digest;
use shared::db::DbPool;

use crate::db;
use crate::utils::{self, *};
pub type OpenApiServiceType = poem_openapi::OpenApiService<
    (
        user::Api,
        user::AdminApi,
        tree_nft_metadata::Api,
        tree_nft_contract::Api,
        files::Api,
    ),
    (),
>;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub api_socket_port: u32,
    pub api_socket_address: String,
    pub postgres_user: String,
    pub postgres_password: SecureString,
    pub postgres_host: String,
    pub postgres_port: u32,
    pub postgres_db: String,
    pub db_pool_max_size: u32,
    pub aws_user_pool_id: String,
    pub aws_user_pool_client_id: String,
    pub aws_user_pool_region: String,
    pub user_challenge_expiry_duration_mins: i64,
    pub concordium_node_uri: String,
    pub concordium_network: String,
    pub tree_nft_agent_wallet_json_str: String,
    pub identity_registry_contract_index: u64,
    pub files_bucket_name: String,
    pub files_presigned_url_expiry_secs: u64,
    pub filebase_s3_endpoint_url: String,
    pub filebase_access_key_id: SecureString,
    pub filebase_secret_access_key: SecureString,
    pub filebase_bucket_name: String,
}

pub async fn create_web_app(config: &Config) -> Route {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres_user,
        config.postgres_password,
        config.postgres_host,
        config.postgres_port,
        config.postgres_db
    );
    // Database Dependencies
    db::db_setup::run_migrations(&database_url);
    let db_pool: DbPool = Pool::builder()
        .max_size(config.db_pool_max_size)
        .build(ConnectionManager::new(&database_url))
        .expect("Failed to create database connection pool");

    // AWS Dependencies
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let user_pool = utils::aws::cognito::UserPool::new(
        &sdk_config,
        &config.aws_user_pool_id,
        &config.aws_user_pool_client_id,
        &config.aws_user_pool_region,
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
    let global_context = concordium_global_context(&mut concordium_client).await;
    let network: Network = config
        .concordium_network
        .parse()
        .expect("Failed to parse Concordium Network");
    let tree_nft_agent_wallet =
        WalletAccount::from_json_str(&config.tree_nft_agent_wallet_json_str)
            .expect("Failed to parse Tree NFT Agent Wallet JSON");
    let identity_registry = IdentityRegistry::new(ContractAddress::new(
        config.identity_registry_contract_index,
        0,
    ));

    let api = create_service();
    let ui = api.swagger_ui();
    let api = api
        .with(AddData::new(db_pool))
        .with(AddData::new(s3::FilesBucket::new(
            &sdk_config,
            config.files_bucket_name.to_owned(),
            Duration::from_secs(config.files_presigned_url_expiry_secs))))
        .with(AddData::new(ipfs::filebase::FilesBucket::new(
            &config.filebase_s3_endpoint_url,
            config.filebase_access_key_id.unsecure(),
            config.filebase_secret_access_key.unsecure(),
            &config.filebase_bucket_name,
            Duration::from_secs(config.files_presigned_url_expiry_secs))))
        .with(AddData::new(user_pool))
        .with(AddData::new(global_context))
        // Enhancements : Make an Object Pool for Concordium Client. So that connections to the node can be tracked
        .with(AddData::new(concordium_client))
        .with(AddData::new(network))
        .with(AddData::new(identity_registry))
        .with(AddData::new(user::UserChallengeConfig {
            challenge_expiry_duration: chrono::Duration::minutes(
                config.user_challenge_expiry_duration_mins,
            ),
        }))
        .with(AddData::new(tree_nft_contract::TreeNftConfig {
            agent: Arc::new(tree_nft_contract::TreeNftAgent(tree_nft_agent_wallet)),
        }))
        .with(Cors::new())
        .with(Tracing);

    Route::new().nest("/", api).nest("/ui", ui)
}

pub fn create_service() -> OpenApiServiceType {
    poem_openapi::OpenApiService::new(
        (
            user::Api,
            user::AdminApi,
            tree_nft_metadata::Api,
            tree_nft_contract::Api,
            files::Api,
        ),
        "Upwood API",
        "1.0.0",
    )
}
pub const PAGE_SIZE: i64 = 20;

/// ApiKey authorization
#[derive(SecurityScheme, Clone)]
#[oai(
    ty = "bearer",
    key_in = "header",
    bearer_format = "bearer",
    key_name = "Authorization",
    checker = "decode_token"
)]
pub struct BearerAuthorization(pub aws::cognito::Claims);

/// Verifies and decodes the claims in the Identity Token from the Cognito User Pool.
/// Returns the claims if the token is valid, otherwise returns an error.
async fn decode_token(req: &poem::Request, bearer: Bearer) -> poem::Result<aws::cognito::Claims> {
    req.data::<aws::cognito::UserPool>()
        .ok_or(poem::Error::from_status(
            poem::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))?
        .verify_decode_id_token(&bearer.token)
        .await
        .map_err(|e| match e {
            aws::cognito::Error::CognitoVerification(error) => poem::Error::from_string(
                format!("Cognito verification error: {}", error),
                poem::http::StatusCode::UNAUTHORIZED,
            ),
            aws::cognito::Error::ClaimsDeserialization(error) => poem::Error::from_string(
                format!("Claims Deserialization Error: {}", error),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            _ => unreachable!(),
        })
}

/// Ensure that the account is an admin
pub fn ensure_is_admin(claims: &aws::cognito::Claims) -> Result<()> {
    if !claims.is_admin() {
        return Err(Error::UnAuthorized(PlainText(
            "Account is not an admin".to_string(),
        )));
    }
    Ok(())
}

pub fn ensure_registered(claims: &aws::cognito::Claims) -> Result<()> {
    if !claims.email_verified() {
        return Err(Error::BadRequest(PlainText(
            "Account not registered".to_owned(),
        )));
    }

    Ok(())
}

pub fn ensure_account_registered(
    claims: &aws::cognito::Claims,
) -> Result<concordium_rust_sdk::id::types::AccountAddress> {
    let account = claims.account().ok_or(Error::BadRequest(PlainText(
        "Account not registered".to_string(),
    )))?;
    Ok(account)
}

#[derive(Debug, ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    #[oai(status = 500)]
    InternalServer(PlainText<String>),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
    #[oai(status = 401)]
    UnAuthorized(PlainText<String>),
}

impl From<r2d2::Error> for Error {
    fn from(error: r2d2::Error) -> Self { Self::InternalServer(PlainText(error.to_string())) }
}
impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Self::InternalServer(PlainText(error.to_string()))
    }
}
impl From<aws::cognito::Error> for Error {
    fn from(e: aws::cognito::Error) -> Self {
        Self::InternalServer(PlainText(format!("User pool error: {}", e)))
    }
}
impl From<v2::QueryError> for Error {
    fn from(_: v2::QueryError) -> Self {
        Self::InternalServer(PlainText("Concordium Query error".to_string()))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
pub type JsonResult<T> = Result<Json<T>>;
pub type NoResResult = Result<()>;

pub fn hasher(data: Vec<u8>) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hash.into()
}

#[derive(poem_openapi::Tags)]
enum ApiTags {
    /// Operations about user
    User,
    /// Operations about tree nft metadata & contract
    TreeNft,
    //// Operations about forest project & contract
    ForestProject,
}
