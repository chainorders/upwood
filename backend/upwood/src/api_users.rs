use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, Object, OpenApi};

use crate::api::BearerAuthorization;
use crate::{db, user_pool};

#[derive(Clone, Copy)]
pub struct ApiUsers {
    pub max_invitations_per_email: u16,
}

#[derive(Debug, ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

impl From<r2d2::Error> for Error {
    fn from(error: r2d2::Error) -> Self { Self::InternalServerError(PlainText(error.to_string())) }
}
impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Self::InternalServerError(PlainText(error.to_string()))
    }
}
impl From<user_pool::Error> for Error {
    fn from(_: user_pool::Error) -> Self {
        Self::InternalServerError(PlainText("User pool error".to_string()))
    }
}
pub type Result<T> = std::result::Result<Json<T>, Error>;

#[derive(Object)]
pub struct User {
    pub id:              String,
    pub email:           String,
    pub groups:          Vec<String>,
    pub family_name:     String,
    pub given_name:      String,
    pub account_address: Option<String>,
}

#[OpenApi]
impl ApiUsers {
    #[oai(path = "/users", method = "get")]
    pub async fn get(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(pool): Data<&db::DbPool>,
    ) -> Result<User> {
        let mut conn = pool.get()?;
        let account_address =
            db::users::find_by_id(&mut conn, &claims.sub)?.and_then(|u| u.account_address);
        Ok(Json(User {
            id: claims.sub,
            email: claims.email,
            groups: claims.cognito_groups.unwrap_or_default(),
            family_name: claims.family_name,
            given_name: claims.given_name,
            account_address,
        }))
    }
}
