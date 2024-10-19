pub mod cognito {
    use aws_sdk_cognitoidentityprovider::operation::admin_create_user::AdminCreateUserError;
    use aws_sdk_cognitoidentityprovider::operation::admin_delete_user::AdminDeleteUserError;
    use aws_sdk_cognitoidentityprovider::operation::admin_set_user_password::AdminSetUserPasswordError;
    use aws_sdk_cognitoidentityprovider::operation::admin_update_user_attributes::AdminUpdateUserAttributesError;
    use aws_sdk_cognitoidentityprovider::operation::list_users::ListUsersError;
    use aws_sdk_cognitoidentityprovider::types::UserType;
    use concordium_rust_sdk::id::types::AccountAddress;
    use jsonwebtokens_cognito::KeySet;
    use poem_openapi::Object;
    use serde::{Deserialize, Serialize};
    use tracing::{debug, instrument};

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
        ClaimsDeserialization(#[from] serde_json::Error),
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
        #[error("User reset password error: {0}")]
        ResetPassword(#[from] CognitoError<AdminSetUserPasswordError>),
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

        #[instrument(skip_all)]
        pub async fn find_user_by_email(&self, email: &str) -> Result<Option<UserType>> {
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
            Ok(user)
        }

        #[instrument(skip_all)]
        pub async fn create_user(&self, email: &str) -> Result<UserType> {
            let user = self
                .cognito_client
                .admin_create_user()
                .user_pool_id(self.user_pool_id.clone())
                .username(email)
                .user_attributes(email_attribute(email))
                .force_alias_creation(true)
                .send()
                .await?
                .user
                .ok_or(Error::UserCreateRes)?;
            Ok(user)
        }

        #[instrument(skip_all)]
        pub async fn set_email_verified(&self, username: &str) -> Result<()> {
            self.cognito_client
                .admin_update_user_attributes()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .user_attributes(email_verified_attribute(true))
                .send()
                .await?;
            Ok(())
        }

        #[instrument(skip_all)]
        pub async fn reset_password(&self, username: &str) -> Result<()> {
            self.cognito_client
                .admin_set_user_password()
                .user_pool_id(&self.user_pool_id)
                .username(username)
                .permanent(false)
                .password(generate_temp_password())
                .send()
                .await?;
            Ok(())
        }

        #[instrument(skip_all)]
        pub async fn update_account_address(
            &self,
            username: &str,
            address: &AccountAddress,
        ) -> Result<()> {
            self.cognito_client
                .admin_update_user_attributes()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .user_attributes(account_address_attribute(address))
                .send()
                .await?;
            Ok(())
        }

        #[instrument(skip_all)]
        pub async fn delete_user(&self, username: &str) -> Result<()> {
            self.cognito_client
                .admin_delete_user()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .send()
                .await?;
            Ok(())
        }

        #[instrument(skip(self, token))]
        pub async fn verify_decode_id_token(&self, token: &str) -> Result<Claims> {
            let claims = self
                .key_set
                .try_verify(token, &self.verifier)
                .map_err(Error::CognitoVerification)?;
            debug!("Claims: {:?}", claims.to_string());

            let claims: Claims = serde_json::from_value(claims)?;
            Ok(claims)
        }
    }

    fn generate_temp_password() -> String {
        passwords::PasswordGenerator::new()
            .length(10)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .spaces(false)
            .strict(true)
            .generate_one()
            .expect("Failed to generate password")
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Object)]
    pub struct Claims {
        /// Cognito user id
        pub sub:            String,
        #[serde(rename = "cognito:groups")]
        pub cognito_groups: Option<Vec<String>>,
        /// Is the email address marked verified in Cognito
        /// When the user registers with Cognito, the email address is not verified by default.
        /// When the user registers via the API the email address in cognito is marked verified.
        pub email_verified: Option<bool>,
        pub email:          String,
        /// Concordium account address for the user
        #[serde(rename = "custom:con_accnt")]
        pub account:        Option<String>,
    }

    impl Claims {
        pub fn is_admin(&self) -> bool {
            self.cognito_groups
                .as_ref()
                .map_or(false, |groups| groups.iter().any(|group| group == "admin"))
        }

        pub fn email_verified(&self) -> bool { self.email_verified.unwrap_or(false) }

        pub fn account(&self) -> Option<AccountAddress> {
            self.account.as_ref().map(|a| a.parse().unwrap())
        }
    }

    #[inline]
    pub fn email_attribute(email: &str) -> aws_sdk_cognitoidentityprovider::types::AttributeType {
        aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
            .name("email".to_string())
            .value(email.to_string())
            .build()
            .unwrap()
    }

    #[inline]
    pub fn email_verified_attribute(
        is_verified: bool,
    ) -> aws_sdk_cognitoidentityprovider::types::AttributeType {
        aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
            .name("email_verified".to_string())
            .value(is_verified.to_string())
            .build()
            .unwrap()
    }

    #[inline]
    pub fn account_address_attribute(
        address: &AccountAddress,
    ) -> aws_sdk_cognitoidentityprovider::types::AttributeType {
        aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
            .name("custom:con_accnt".to_string())
            .value(address.to_string())
            .build()
            .unwrap()
    }
}
