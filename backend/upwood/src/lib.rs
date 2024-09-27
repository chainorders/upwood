pub mod api_users;
pub mod db;
mod schema;

pub mod user_pool {
    use jsonwebtokens_cognito::KeySet;
    use serde::Deserialize;
    use tracing::debug;

    #[derive(Debug)]
    pub enum Error {
        KeySetCreate,
        FetchJwks,
        BuildVerifier,
        Verification(jsonwebtokens_cognito::Error),
        ClaimsDeserialization(serde_json::Error),
        UnVerifiedEmail,
    }
    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Clone)]
    pub struct UserPool {
        key_set:  KeySet,
        verifier: jsonwebtokens::Verifier,
    }

    impl UserPool {
        pub async fn new(
            user_pool_region: &str,
            user_pool_id: &str,
            user_pool_client_id: &str,
        ) -> Result<Self> {
            let key_set =
                KeySet::new(user_pool_region, user_pool_id).map_err(|_| Error::KeySetCreate)?;
            key_set
                .prefetch_jwks()
                .await
                .map_err(|_| Error::FetchJwks)?;
            let verifier = key_set
                .new_id_token_verifier(&[user_pool_client_id])
                .build()
                .map_err(|_| Error::BuildVerifier)?;
            Ok(Self { key_set, verifier })
        }

        pub async fn verify_decode_id_token(&self, token: &str) -> Result<Claims> {
            let claims = self
                .key_set
                .try_verify(token, &self.verifier)
                .map_err(Error::Verification)?;
            debug!("Claims: {:?}", claims.to_string());

            let claims: Claims =
                serde_json::from_value(claims).map_err(Error::ClaimsDeserialization)?;
            if !claims.email_verified {
                return Err(Error::UnVerifiedEmail);
            }
            Ok(claims)
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct Claims {
        pub sub:              String,
        #[serde(rename = "cognito:groups")]
        pub cognito_groups:   Option<Vec<String>>,
        #[serde(rename = "cognito:username")]
        pub cognito_username: String,
        pub email_verified:   bool,
        pub email:            String,
        pub given_name:       String,
        pub family_name:      String,
    }

    #[cfg(test)]
    mod test {
        // use super::*;

        // #[tracing_test::traced_test]
        // #[tokio::test]
        // /// Test that we can decode a token
        // /// To setup a user pool, you can use the aws cli:
        // /// - create a new user pool : execute command `cdk deploy` inside `cdk-deployment` dev container
        // /// - create a new user in user pool
        // /// ```bash
        // /// aws cognito-idp admin-create-user \
        // /// --user-pool-id <USER_POOL_ID> \
        // /// --temporary-password \|mW8wRHo \
        // /// --message-action SUPPRESS \
        // /// --user-attributes '[{"Name":"given_name","Value":"John"},{"Name":"family_name","Value":"Carter"},{"Name":"email","Value":"john-carter-fjuh765@yopmail.com"},{"Name":"email_verified","Value":"true"}]' \
        // /// --username john-carter-fjuh765@yopmail.com
        // /// ```
        // /// - change user password
        // /// ```bash
        // /// aws cognito-idp admin-set-user-password \
        // /// --user-pool-id <USER_POOL_ID> \
        // /// --username <USER NAME> \
        // /// --password \|mW8wRHo \
        // /// --permanent
        // /// ```
        // /// - initiate auth
        // /// ```
        // /// aws cognito-idp initiate-auth \
        // /// --auth-flow USER_PASSWORD_AUTH \
        // /// --auth-parameters USERNAME=<USER NAME/SUB>,PASSWORD=\|mW8wRHo \
        // /// --client-id <USER POOL CLIENT ID>
        // /// ```
        // /// - get id token
        // async fn decode_token() {
        //     let id_token = "";
        //     let user_pool = UserPool::new(
        //         "<USER POOL REGION>",
        //         "<USER POOL ID>",
        //         "<USER POOL CLIENT ID>",
        //     )
        //     .await
        //     .expect("Failed to create user pool");
        //     let user = user_pool
        //         .verify_decode_id_token(id_token)
        //         .await
        //         .expect("Failed to decode token");

        //     assert_eq!(user.email, "john-carter-fjuh765@yopmail.com");
        //     assert_eq!(user.given_name, "John");
        //     assert_eq!(user.family_name, "Carter");
        // }
    }
}

pub mod api {
    use poem::Error;
    use poem_openapi::auth::Bearer;
    use poem_openapi::SecurityScheme;

    use crate::api_users::ApiUsers;
    use crate::user_pool;
    pub type ApiType = ApiUsers;
    pub fn create_service() -> poem_openapi::OpenApiService<ApiType, ()> {
        poem_openapi::OpenApiService::new(
            ApiUsers {
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
}
