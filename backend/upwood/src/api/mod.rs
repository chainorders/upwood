mod users;

use poem::Error;
use poem_openapi::auth::Bearer;
use poem_openapi::SecurityScheme;

use crate::user_pool;
pub fn create_service() -> poem_openapi::OpenApiService<users::ApiUsers, ()> {
    poem_openapi::OpenApiService::new(
        users::ApiUsers {
            max_invitations_per_email: 20,
        },
        "Upwood API",
        "1.0.0",
    )
}

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    key_in = "header",
    bearer_format = "bearer",
    key_name = "Authorization",
    checker = "decode_token"
)]
pub struct BearerAuthorization(pub user_pool::Claims);

async fn decode_token(req: &poem::Request, bearer: Bearer) -> poem::Result<user_pool::Claims> {
    req.data::<user_pool::UserPool>()
        .ok_or(Error::from_status(
            poem::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))?
        .verify_decode_id_token(&bearer.token)
        .await
        .map_err(|_| Error::from_status(poem::http::StatusCode::UNAUTHORIZED))
}
