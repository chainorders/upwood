pub mod carbon_credits;
pub mod files;
pub mod forest_project;
pub mod identity_registry;
pub mod investment_portfolio;
pub mod tree_nft;
pub mod user;
pub mod user_communication;

use std::sync::Arc;
use std::time::Duration;

use aws::s3;
use concordium::chain::concordium_global_context;
use concordium_rust_sdk::types::{ContractAddress, WalletAccount};
use concordium_rust_sdk::web3id::did::Network;
use concordium_rust_sdk::{cis2, v2};
use diesel::r2d2::ConnectionManager;
use events_listener::processors::cis2_utils::Cis2TokenIdToDecimal;
use poem::http::StatusCode;
use poem::middleware::{AddData, Cors, Tracing};
use poem::{EndpointExt, Route};
use poem_openapi::auth::Bearer;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, Object, SecurityScheme};
use r2d2::Pool;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use secure_string::SecureString;
use serde::Deserialize;
use sha2::Digest;
use shared::db_setup;
use shared::db_shared::DbPool;
use tracing::error;

use crate::utils::{self, *};
pub type OpenApiServiceType = poem_openapi::OpenApiService<
    (
        user::UserApi,
        tree_nft::Api,
        files::Api,
        identity_registry::Api,
        carbon_credits::Api,
        forest_project::ForestProjectApi,
        forest_project::ForestProjectAdminApi,
        investment_portfolio::Api,
        user_communication::Api,
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
    pub identity_registry_contract_index: Decimal,
    pub compliance_contract_index: Decimal,
    pub carbon_credit_contract_index: Decimal,
    pub euro_e_contract_index: Decimal,
    pub tree_ft_contract_index: Decimal,
    pub tree_nft_contract_index: Decimal,
    pub offchain_rewards_contract_index: Decimal,
    pub offchain_rewards_agent_wallet_json_str: String,
    pub files_bucket_name: String,
    pub files_presigned_url_expiry_secs: u64,
    pub filebase_s3_endpoint_url: String,
    pub filebase_access_key_id: SecureString,
    pub filebase_secret_access_key: SecureString,
    pub filebase_bucket_name: String,
    /// Default commission for affiliates
    pub affiliate_commission: Decimal,
}

impl Config {
    /// Load configuration from .env files
    /// The .env file should be in the root of the project
    pub fn load() -> Self {
        dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
        dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("secure.env"))
            .ok();

        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("Failed to build config")
            .try_deserialize()
            .expect("Failed to deserialize config")
    }

    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password.unsecure(),
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }

    pub fn concordium_node_endpoint(&self) -> v2::Endpoint {
        self.concordium_node_uri
            .parse()
            .expect("Failed to parse Concordium Node URI")
    }

    pub fn concordium_network(&self) -> Network {
        self.concordium_network
            .parse()
            .expect("Failed to parse Concordium Network")
    }

    pub async fn concordium_client(&self) -> v2::Client {
        v2::Client::new(self.concordium_node_endpoint())
            .await
            .expect("Failed to create Concordium Client")
    }

    pub fn db_pool(&self) -> DbPool {
        let database_url = self.db_url();
        Pool::builder()
            .max_size(self.db_pool_max_size)
            .build(ConnectionManager::new(database_url))
            .expect("Failed to create database connection pool")
    }

    pub async fn aws_cognito_user_pool(
        &self,
        aws_sdk_config: &aws_config::SdkConfig,
    ) -> aws::cognito::UserPool {
        utils::aws::cognito::UserPool::new(
            aws_sdk_config,
            self.aws_user_pool_id.clone(),
            self.aws_user_pool_client_id.clone(),
            self.aws_user_pool_region.clone(),
        )
        .await
        .expect("Failed to create user pool")
    }

    pub fn tree_nft_agent_wallet(&self) -> WalletAccount {
        WalletAccount::from_json_str(&self.tree_nft_agent_wallet_json_str)
            .expect("Failed to parse Tree NFT Agent Wallet JSON")
    }

    pub fn tree_nft_agent(&self) -> tree_nft::TreeNftAgent {
        tree_nft::TreeNftAgent(self.tree_nft_agent_wallet())
    }

    pub fn tree_nft_config(&self) -> tree_nft::TreeNftConfig {
        tree_nft::TreeNftConfig {
            agent: Arc::new(self.tree_nft_agent()),
        }
    }

    pub fn offchain_rewards_agent_wallet(&self) -> WalletAccount {
        WalletAccount::from_json_str(&self.offchain_rewards_agent_wallet_json_str)
            .expect("Failed to parse Offchain Rewards Agent Wallet JSON")
    }

    pub fn offchain_rewards_agent(&self) -> user::OffchainRewardsAgent {
        user::OffchainRewardsAgent(self.offchain_rewards_agent_wallet())
    }

    pub fn offchain_rewards_config(&self) -> user::OffchainRewardsConfig {
        user::OffchainRewardsConfig {
            agent: Arc::new(self.offchain_rewards_agent()),
        }
    }

    pub fn ipfs_files_bucket(&self) -> ipfs::filebase::FilesBucket {
        ipfs::filebase::FilesBucket::new(
            self.filebase_s3_endpoint_url.clone(),
            self.filebase_access_key_id.unsecure().to_string(),
            self.filebase_secret_access_key.unsecure().to_string(),
            self.filebase_bucket_name.clone(),
            Duration::from_secs(self.files_presigned_url_expiry_secs),
        )
    }

    pub fn files_bucket(&self, aws_sdk_config: &aws_config::SdkConfig) -> s3::FilesBucket {
        s3::FilesBucket::new(
            aws_sdk_config,
            self.files_bucket_name.clone(),
            Duration::from_secs(self.files_presigned_url_expiry_secs),
        )
    }

    pub fn contracts_config(&self) -> SystemContractsConfig {
        SystemContractsConfig {
            identity_registry_contract_index: self.identity_registry_contract_index,
            compliance_contract_index:        self.compliance_contract_index,
            carbon_credit_contract_index:     self.carbon_credit_contract_index,
            carbon_credit_token_id:           cis2::TokenId::new_unchecked(vec![]).to_decimal(),
            euro_e_contract_index:            self.euro_e_contract_index,
            euro_e_token_id:                  cis2::TokenId::new_unchecked(vec![]).to_decimal(),
            tree_ft_contract_index:           self.tree_ft_contract_index,
            tree_nft_contract_index:          self.tree_nft_contract_index,
            offchain_rewards_contract_index:  self.offchain_rewards_contract_index,
        }
    }

    pub fn user_challenge_config(&self) -> user::UserChallengeConfig {
        user::UserChallengeConfig {
            challenge_expiry_duration: chrono::Duration::minutes(
                self.user_challenge_expiry_duration_mins,
            ),
        }
    }

    /// Default commission for affiliates
    pub fn affiliate_commission(&self) -> AffiliateCommission {
        AffiliateCommission {
            commission: self.affiliate_commission,
        }
    }
}

pub async fn create_web_app(config: Config) -> Route {
    let database_url = config.db_url();
    // Database Dependencies
    db_setup::run_migrations(&database_url);
    let db_pool = config.db_pool();

    // AWS Dependencies
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    // Concordium Dependencies
    let mut concordium_client = config.concordium_client().await;
    let global_context = concordium_global_context(&mut concordium_client).await;
    let api = create_service();
    let ui = api.swagger_ui();
    let api = api
        .with(AddData::new(db_pool))
        .with(AddData::new(config.files_bucket(&sdk_config)))
        .with(AddData::new(config.ipfs_files_bucket()))
        .with(AddData::new(config.aws_cognito_user_pool(&sdk_config).await))
        .with(AddData::new(global_context))
        // Enhancements : Make an Object Pool for Concordium Client. So that connections to the node can be tracked
        .with(AddData::new(concordium_client))
        .with(AddData::new(config.concordium_network()))
        .with(AddData::new(config.contracts_config()))
        .with(AddData::new(config.user_challenge_config()))
        .with(AddData::new(config.tree_nft_config()))
        .with(AddData::new(config.offchain_rewards_config()))
        .with(AddData::new(config.affiliate_commission()))
        .with(Cors::new())
        .after(|f| async move {
            match f {
                Ok(res) => Ok(res),
                Err(error) => {
                    error!("Api error: {:?}", error.to_string());
                    Err(error)
                }
            }
        })
        .with(Tracing);

    Route::new().nest("/", api).nest("/ui", ui)
}

pub fn create_service() -> OpenApiServiceType {
    poem_openapi::OpenApiService::new(
        (
            user::UserApi,
            tree_nft::Api,
            files::Api,
            identity_registry::Api,
            carbon_credits::Api,
            forest_project::ForestProjectApi,
            forest_project::ForestProjectAdminApi,
            investment_portfolio::Api,
            user_communication::Api,
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
    /// Operations about s3 & IPFS files
    Files,
    /// Operations about user
    User,
    /// Operations about carbon credits
    CarbonCredits,
    /// Operations about identity registry contract
    IdentityRegistry,
    /// Operations about tree fungible token contract
    TreeFT,
    /// Operations about tree nft contract
    TreeNft,
    /// Operations about tree nft metadata
    TreeNftMetadata,
    //// Operations about forest project & contract
    ForestProject,
    InvestmentPortfolio,
    UserCommunication,
    Wallet,
}

#[derive(Clone, Object, serde::Serialize, serde::Deserialize)]
pub struct SystemContractsConfig {
    pub identity_registry_contract_index: Decimal,
    pub compliance_contract_index:        Decimal,
    pub carbon_credit_contract_index:     Decimal,
    pub carbon_credit_token_id:           Decimal,
    pub euro_e_contract_index:            Decimal,
    pub euro_e_token_id:                  Decimal,
    pub tree_ft_contract_index:           Decimal,
    pub tree_nft_contract_index:          Decimal,
    pub offchain_rewards_contract_index:  Decimal,
}

impl SystemContractsConfig {
    pub fn identity_registry(&self) -> ContractAddress {
        ContractAddress::new(self.identity_registry_contract_index.to_u64().unwrap(), 0)
    }

    pub fn compliance(&self) -> ContractAddress {
        ContractAddress::new(self.compliance_contract_index.to_u64().unwrap(), 0)
    }

    pub fn carbon_credit(&self) -> ContractAddress {
        ContractAddress::new(self.carbon_credit_contract_index.to_u64().unwrap(), 0)
    }

    pub fn euro_e(&self) -> ContractAddress {
        ContractAddress::new(self.euro_e_contract_index.to_u64().unwrap(), 0)
    }

    pub fn tree_ft(&self) -> ContractAddress {
        ContractAddress::new(self.tree_ft_contract_index.to_u64().unwrap(), 0)
    }

    pub fn tree_nft(&self) -> ContractAddress {
        ContractAddress::new(self.tree_nft_contract_index.to_u64().unwrap(), 0)
    }

    pub fn offchain_rewards(&self) -> ContractAddress {
        ContractAddress::new(self.offchain_rewards_contract_index.to_u64().unwrap(), 0)
    }
}

#[derive(Clone, Object)]
pub struct AffiliateCommission {
    pub commission: Decimal,
}
