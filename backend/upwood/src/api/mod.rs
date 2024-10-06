pub mod tree_nft_metadata;
pub mod user;

use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::v2;
use poem_openapi::auth::Bearer;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, SecurityScheme};
use sha2::Digest;

use crate::utils::*;
pub type OpenApiServiceType = poem_openapi::OpenApiService<(user::Api, tree_nft_metadata::Api), ()>;
pub fn create_service() -> OpenApiServiceType {
    poem_openapi::OpenApiService::new((user::Api, tree_nft_metadata::Api), "Upwood API", "1.0.0")
}

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
        .map_err(|_| poem::Error::from_status(poem::http::StatusCode::UNAUTHORIZED))
}

/// Ensure that the account is an admin
pub fn ensure_is_admin(claims: &aws::cognito::Claims) -> Result<()> {
    if !claims.is_admin() {
        return Err(Error::Forbidden(PlainText(
            "Account is not an admin".to_string(),
        )));
    }
    Ok(())
}

///  Ensure that the Concordium account address is present in the Claims received from the User Pool Token
pub fn ensure_account_registered(claims: &aws::cognito::Claims) -> Result<AccountAddress> {
    let account_address: AccountAddress = claims
        .account_address
        .as_ref()
        .ok_or(Error::Forbidden(PlainText(
            "Account is not registered".to_string(),
        )))?
        .parse()
        .map_err(|_| {
            Error::InternalServer(PlainText("Failed to parse account address".to_string()))
        })?;
    Ok(account_address)
}

#[derive(Debug, ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    #[oai(status = 500)]
    InternalServer(PlainText<String>),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
    #[oai(status = 403)]
    Forbidden(PlainText<String>),
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
    fn from(_: aws::cognito::Error) -> Self {
        Self::InternalServer(PlainText("User pool error".to_string()))
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
