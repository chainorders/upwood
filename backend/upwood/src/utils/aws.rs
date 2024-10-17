pub mod cognito {
    use aws_sdk_cognitoidentityprovider::operation::admin_create_user::AdminCreateUserError;
    use aws_sdk_cognitoidentityprovider::operation::admin_delete_user::AdminDeleteUserError;
    use aws_sdk_cognitoidentityprovider::operation::admin_update_user_attributes::AdminUpdateUserAttributesError;
    use aws_sdk_cognitoidentityprovider::operation::list_users::ListUsersError;
    use jsonwebtokens_cognito::KeySet;
    use poem_openapi::Object;
    use serde::{Deserialize, Serialize};
    use tracing::debug;

    use crate::utils::CognitoClient;

    pub const ACCOUNT_ADDRESS_ATTRIBUTE_NAME: &str = "preffered_username";
    pub type CognitoError<T> = aws_sdk_cognitoidentityprovider::error::SdkError<T>;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Cognito error: {0}")]
        CognitoKeySet(jsonwebtokens_cognito::Error),
        #[error("Cognito JWKS error: {0}")]
        CognitoJWKS(jsonwebtokens_cognito::Error),
        #[error("Cognito error: {0}")]
        CognitoVerification(jsonwebtokens_cognito::Error),
        #[error("Json token error: {0}")]
        JsonToken(jsonwebtokens::error::Error),
        #[error("Claims deserialization error: {0}")]
        ClaimsDeserialization(serde_json::Error),
        #[error("Email Verification Error")]
        UnVerifiedEmail,
        #[error("Update account address error")]
        UpdateAccountAddress,
        #[error("List users error")]
        UserList(CognitoError<ListUsersError>),
        #[error("User create error: {0}")]
        UserCreate(#[from] CognitoError<AdminCreateUserError>),
        #[error("User create response error")]
        UserCreateRes,
        #[error("User delete error: {0}")]
        DeleteUser(#[from] CognitoError<AdminDeleteUserError>),
        #[error("User list error")]
        UserListRes,
        #[error("User update error: {0}")]
        UpdateAttributeError(#[from] CognitoError<AdminUpdateUserAttributesError>),
    }
    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Clone)]
    pub struct UserPool {
        key_set:          KeySet,
        verifier:         jsonwebtokens::Verifier,
        cognito_client:   CognitoClient,
        pub user_pool_id: String,
    }

    impl UserPool {
        pub async fn new(
            sdk_config: aws_config::SdkConfig,
            user_pool_id: &str,
            user_pool_client_id: &str,
            user_pool_region: &str,
        ) -> Result<Self> {
            let cognito_client = CognitoClient::new(&sdk_config);
            let key_set =
                KeySet::new(user_pool_region, user_pool_id).map_err(Error::CognitoKeySet)?;
            key_set.prefetch_jwks().await.map_err(Error::CognitoJWKS)?;
            let verifier = key_set
                .new_id_token_verifier(&[user_pool_client_id])
                .build()
                .map_err(Error::JsonToken)?;
            Ok(Self {
                key_set,
                verifier,
                cognito_client,
                user_pool_id: user_pool_id.to_string(),
            })
        }

        pub async fn find_user_by_email(&self, email: &str) -> Result<Option<String>> {
            let users = self
                .cognito_client
                .list_users()
                .user_pool_id(self.user_pool_id.clone())
                .filter(format!("email = \"{email}\""))
                .send()
                .await
                .map_err(Error::UserList)?
                .users
                .ok_or(Error::UserListRes)?;
            let user = users.into_iter().next();
            let user_id = user.and_then(|u| u.username);
            Ok(user_id)
        }

        pub async fn admin_create_user(&self, email: &str) -> Result<String> {
            let username = self
                .cognito_client
                .admin_create_user()
                .user_pool_id(self.user_pool_id.clone())
                .username(email)
                .user_attributes(email_attribute(email))
                .force_alias_creation(true)
                .send()
                .await?
                .user
                .ok_or(Error::UserCreateRes)?
                .username
                .ok_or(Error::UserCreateRes)?;
            Ok(username)
        }

        pub async fn admin_set_email_verified(&self, username: &str) -> Result<()> {
            self.cognito_client
                .admin_update_user_attributes()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .user_attributes(email_verified_attribute(true))
                .send()
                .await?;
            Ok(())
        }

        pub async fn admin_delete_user(&self, username: &str) -> Result<()> {
            self.cognito_client
                .admin_delete_user()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .send()
                .await?;
            Ok(())
        }

        pub async fn verify_decode_id_token(&self, token: &str) -> Result<Claims> {
            let claims = self
                .key_set
                .try_verify(token, &self.verifier)
                .map_err(Error::CognitoVerification)?;
            debug!("Claims: {:?}", claims.to_string());

            let claims: Claims =
                serde_json::from_value(claims).map_err(Error::ClaimsDeserialization)?;
            // if !claims.email_verified {
            //     return Err(Error::UnVerifiedEmail);
            // }
            Ok(claims)
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Object)]
    pub struct Claims {
        pub sub:              String,
        #[serde(rename = "cognito:groups")]
        pub cognito_groups:   Option<Vec<String>>,
        #[serde(rename = "cognito:username")]
        pub cognito_username: String,
        pub email_verified:   Option<bool>,
        pub email:            String,
    }

    impl Claims {
        pub fn is_admin(&self) -> bool {
            self.cognito_groups
                .as_ref()
                .map_or(false, |groups| groups.iter().any(|group| group == "admin"))
        }

        pub fn email_verified(&self) -> bool { self.email_verified.unwrap_or(false) }
    }

    pub fn email_attribute(email: &str) -> aws_sdk_cognitoidentityprovider::types::AttributeType {
        aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
            .name("email".to_string())
            .value(email.to_string())
            .build()
            .unwrap()
    }

    pub fn email_verified_attribute(
        is_verified: bool,
    ) -> aws_sdk_cognitoidentityprovider::types::AttributeType {
        aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
            .name("email_verified".to_string())
            .value(is_verified.to_string())
            .build()
            .unwrap()
    }
}
