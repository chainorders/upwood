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
    InternalServer(PlainText<String>),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

impl From<r2d2::Error> for Error {
    fn from(error: r2d2::Error) -> Self { Self::InternalServer(PlainText(error.to_string())) }
}
impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Self::InternalServer(PlainText(error.to_string()))
    }
}
impl From<user_pool::Error> for Error {
    fn from(_: user_pool::Error) -> Self {
        Self::InternalServer(PlainText("User pool error".to_string()))
    }
}
pub type Result<T> = std::result::Result<Json<T>, Error>;

#[derive(Object)]
pub struct User {
    pub id:              String,
    pub email:           String,
    pub family_name:     String,
    pub given_name:      String,
    pub account_address: Option<String>,
    pub is_admin:        bool,
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
        let is_admin = claims.is_admin();
        Ok(Json(User {
            is_admin,
            id: claims.sub,
            email: claims.email,
            family_name: claims.family_name,
            given_name: claims.given_name,
            account_address,
        }))
    }

    #[oai(path = "/users/list", method = "get")]
    pub async fn get_all(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(pool): Data<&db::DbPool>,
        Data(user_pool): Data<&user_pool::UserPool>,
    ) -> Result<Vec<User>> {
        if !claims.is_admin() {
            return Err(Error::UnAuthorized(PlainText(
                "Only admins can list users".to_string(),
            )));
        }
    }
}
