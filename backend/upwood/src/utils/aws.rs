pub mod cognito {
    use aws_sdk_cognitoidentityprovider::error::SdkError;
    use aws_sdk_cognitoidentityprovider::operation::admin_create_user::AdminCreateUserError;
    use aws_sdk_cognitoidentityprovider::operation::admin_disable_user::AdminDisableUserError;
    use aws_sdk_cognitoidentityprovider::operation::admin_get_user::{
        AdminGetUserError, AdminGetUserOutput,
    };
    use aws_sdk_cognitoidentityprovider::operation::admin_set_user_password::AdminSetUserPasswordError;
    use aws_sdk_cognitoidentityprovider::operation::admin_update_user_attributes::AdminUpdateUserAttributesError;
    use aws_sdk_cognitoidentityprovider::operation::confirm_sign_up::ConfirmSignUpError;
    use aws_sdk_cognitoidentityprovider::operation::initiate_auth::{
        InitiateAuthError, InitiateAuthOutput,
    };
    use aws_sdk_cognitoidentityprovider::operation::list_users::ListUsersError;
    use aws_sdk_cognitoidentityprovider::types::{
        AttributeType, AuthFlowType, UserStatusType, UserType,
    };
    use concordium_rust_sdk::id::types::AccountAddress;
    use jsonwebtokens_cognito::KeySet;
    use poem_openapi::Object;
    use serde::{Deserialize, Serialize};
    use tracing::{instrument, trace};

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
        #[error("User disable error: {0}")]
        DisableUser(#[from] CognitoError<AdminDisableUserError>),
        #[error("User list error")]
        UserListRes,
        #[error("User update error: {0}")]
        UpdateAttributeError(#[from] CognitoError<AdminUpdateUserAttributesError>),
        #[error("User reset password error: {0}")]
        ResetPassword(#[from] CognitoError<AdminSetUserPasswordError>),
        #[error("Login error")]
        LoginError,
        #[error("Auth initiate error: {0}")]
        AuthInitError(#[from] SdkError<InitiateAuthError>),
        #[error("ConfirmSignUp error: {0}")]
        ConfirmSignUpError(#[from] SdkError<ConfirmSignUpError>),
        #[error("AdminGetUserError: {0}")]
        AdminGetUserError(#[from] SdkError<AdminGetUserError>),
    }
    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Clone)]
    pub struct UserPool {
        key_set:                 KeySet,
        verifier:                jsonwebtokens::Verifier,
        client:                  CognitoClient,
        pub user_pool_id:        String,
        pub user_pool_client_id: String,
    }

    impl UserPool {
        pub async fn new(
            sdk_config: &aws_config::SdkConfig,
            user_pool_id: &str,
            user_pool_client_id: &str,
            user_pool_region: &str,
        ) -> Result<Self> {
            let client = CognitoClient::new(sdk_config);
            let key_set =
                KeySet::new(user_pool_region, user_pool_id).map_err(Error::CognitoKeySet)?;
            key_set.prefetch_jwks().await.map_err(Error::CognitoJWKS)?;
            let verifier = key_set
                .new_id_token_verifier(&[user_pool_client_id])
                .ignore_iat()
                .build()
                .map_err(Error::JsonToken)?;
            Ok(Self {
                key_set,
                verifier,
                client,
                user_pool_id: user_pool_id.to_string(),
                user_pool_client_id: user_pool_client_id.to_string(),
            })
        }

        #[instrument(skip_all)]
        pub async fn find_user_by_email(&self, email: &str) -> Result<Option<UserType>> {
            let users = self
                .client
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

        /// Create a new user in the Cognito user pool and sets the permanent password.
        #[instrument(skip_all)]
        pub async fn admin_create_permanent_user(
            &self,
            username: &str,
            password: &str,
            attributes: Vec<AttributeType>,
        ) -> Result<UserType> {
            let user = self.admin_create_temp_user(username, attributes).await?;
            self.admin_set_permament_password(username, password)
                .await?;
            Ok(user)
        }

        pub async fn admin_set_permament_password(
            &self,
            username: &str,
            password: &str,
        ) -> Result<()> {
            self.client
                .admin_set_user_password()
                .user_pool_id(&self.user_pool_id)
                .username(username)
                .permanent(true)
                .password(password)
                .send()
                .await?;
            Ok(())
        }

        pub async fn admin_update_user_attributes(
            &self,
            username: &str,
            attributes: Vec<AttributeType>,
        ) -> Result<()> {
            self.client
                .admin_update_user_attributes()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .set_user_attributes(Some(attributes))
                .send()
                .await?;
            Ok(())
        }

        /// Confirm the user sign up
        /// * Updates the user attributes
        /// * Confirms the user sign up
        /// * Sets the permanent password
        /// * Returns the user
        pub async fn confirm_user(
            &self,
            username: &str,
            temp_password: &str,
            password: &str,
            attributes: Vec<AttributeType>,
        ) -> Result<AdminGetUserOutput> {
            self.client
                .admin_update_user_attributes()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .set_user_attributes(Some(attributes))
                .send()
                .await?;
            let user = self
                .client
                .admin_get_user()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .send()
                .await?;
            if let Some(UserStatusType::Unconfirmed) = user.user_status {
                self.client
                    .confirm_sign_up()
                    .client_id(&self.user_pool_client_id)
                    .confirmation_code(temp_password)
                    .username(username)
                    .send()
                    .await?;
            }
            self.client
                .admin_set_user_password()
                .user_pool_id(&self.user_pool_id)
                .username(username)
                .permanent(true)
                .password(password)
                .send()
                .await?;
            Ok(user)
        }

        /// Create a new user in the Cognito user pool
        pub async fn admin_create_temp_user(
            &self,
            username: &str,
            attributes: Vec<AttributeType>,
        ) -> Result<UserType> {
            let user = self
                .client
                .admin_create_user()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .set_user_attributes(Some(attributes))
                .force_alias_creation(true)
                .send()
                .await?
                .user
                .ok_or(Error::UserCreateRes)?;
            Ok(user)
        }

        #[instrument(skip_all)]
        pub async fn admin_add_to_admin_group(&mut self, username: &str) {
            self.client
                .admin_add_user_to_group()
                .user_pool_id(self.user_pool_id.to_owned())
                .username(username.to_owned())
                .group_name("admin".to_owned())
                .send()
                .await
                .expect("Failed to add user to admin group");
        }

        pub async fn user_initiate_auth_req(
            &self,
            email: &str,
            password: &str,
        ) -> Result<InitiateAuthOutput> {
            self.client
                .initiate_auth()
                .client_id(&self.user_pool_client_id)
                .auth_flow(AuthFlowType::UserPasswordAuth)
                .auth_parameters("USERNAME", email.to_owned())
                .auth_parameters("PASSWORD", password.to_owned())
                .send()
                .await
                .map_err(Error::AuthInitError)
        }

        #[instrument(skip_all)]
        pub async fn user_login(&self, email: &str, password: &str) -> Result<String> {
            let auth_response = self.user_initiate_auth_req(email, password).await?;
            debug_assert!(auth_response.challenge_name.is_none());
            let id_token = auth_response
                .authentication_result
                .and_then(|r| r.id_token)
                .ok_or(Error::LoginError)?;
            Ok(id_token)
        }

        #[instrument(skip_all)]
        pub async fn set_email_verified(&self, username: &str) -> Result<()> {
            self.client
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
            self.client
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
        pub async fn update_account_address(&self, username: &str, address: &str) -> Result<()> {
            self.client
                .admin_update_user_attributes()
                .user_pool_id(self.user_pool_id.clone())
                .username(username)
                .user_attributes(account_address_attribute(address))
                .send()
                .await?;
            Ok(())
        }

        #[instrument(skip_all)]
        pub async fn disable_user(&self, username: &str) -> Result<()> {
            self.client
                .admin_disable_user()
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
            trace!("Claims: {:?}", claims.to_string());

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
        pub sub:               String,
        #[serde(rename = "cognito:groups")]
        pub cognito_groups:    Option<Vec<String>>,
        /// Is the email address marked verified in Cognito
        /// When the user registers with Cognito, the email address is not verified by default.
        /// When the user registers via the API the email address in cognito is marked verified.
        pub email_verified:    Option<bool>,
        pub email:             String,
        /// Concordium account address for the user
        #[serde(rename = "custom:con_accnt")]
        pub account:           Option<String>,
        #[serde(rename = "custom:affiliate_con_accnt")]
        pub affiliate_account: Option<String>,
        #[serde(rename = "given_name")]
        pub first_name:        Option<String>,
        #[serde(rename = "family_name")]
        pub last_name:         Option<String>,
        #[serde(rename = "custom:nationallity")]
        pub nationality:       Option<String>,
        #[serde(rename = "custom:company_id")]
        pub company_id:        Option<uuid::Uuid>,
    }

    impl Claims {
        pub fn is_admin(&self) -> bool {
            self.cognito_groups
                .as_ref()
                .is_some_and(|groups| groups.iter().any(|group| group == "admin"))
        }

        pub fn email_verified(&self) -> bool { self.email_verified.unwrap_or(false) }

        pub fn account(&self) -> Option<AccountAddress> {
            self.account.as_ref().map(|a| a.parse().unwrap())
        }
    }

    #[inline]
    pub fn email_attribute(email: &str) -> AttributeType {
        AttributeType::builder()
            .name("email".to_string())
            .value(email.to_string())
            .build()
            .unwrap()
    }

    #[inline]
    pub fn email_verified_attribute(is_verified: bool) -> AttributeType {
        AttributeType::builder()
            .name("email_verified".to_string())
            .value(is_verified.to_string())
            .build()
            .unwrap()
    }

    #[inline]
    pub fn account_address_attribute(address: &str) -> AttributeType {
        AttributeType::builder()
            .name("custom:con_accnt".to_string())
            .value(address)
            .build()
            .unwrap()
    }

    #[inline]
    pub fn affiliate_account_address_attribute(address: &str) -> AttributeType {
        AttributeType::builder()
            .name("custom:affiliate_con_accnt".to_string())
            .value(address)
            .build()
            .unwrap()
    }

    pub fn first_name_attribute(first_name: &str) -> AttributeType {
        AttributeType::builder()
            .name("given_name".to_string())
            .value(first_name.to_string())
            .build()
            .unwrap()
    }

    pub fn last_name_attribute(last_name: &str) -> AttributeType {
        AttributeType::builder()
            .name("family_name".to_string())
            .value(last_name.to_string())
            .build()
            .unwrap()
    }

    pub fn name_attribute(first_name: &str, last_name: &str) -> AttributeType {
        AttributeType::builder()
            .name("name".to_string())
            .value(format!("{} {}", first_name, last_name))
            .build()
            .unwrap()
    }

    pub fn nationality_attribute(nationality: &str) -> AttributeType {
        AttributeType::builder()
            .name("custom:nationality".to_string())
            .value(nationality.to_string())
            .build()
            .unwrap()
    }

    pub fn company_id_attribute(company_id: Option<&uuid::Uuid>) -> AttributeType {
        AttributeType::builder()
            .name("custom:company_id".to_string())
            .set_value(company_id.map(|id| id.to_string()))
            .build()
            .unwrap()
    }
}

pub mod s3 {
    use aws_config::SdkConfig;
    use aws_sdk_s3::error::SdkError;
    use aws_sdk_s3::operation::delete_object::DeleteObjectError;
    use aws_sdk_s3::operation::put_object::PutObjectError;
    use aws_sdk_s3::presigning::{PresigningConfig, PresigningConfigError};
    use aws_sdk_s3::Client;

    use crate::utils::S3Client;

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("presign config: {0}")]
        PresigningConfig(#[from] PresigningConfigError),
        #[error("put object: {0}")]
        PutObject(#[from] SdkError<PutObjectError>),
        #[error("delete object: {0}")]
        DeleteObject(#[from] SdkError<DeleteObjectError>),
        #[error("head: {0}")]
        Head(#[from] SdkError<aws_sdk_s3::operation::head_object::HeadObjectError>),
    }

    #[derive(Debug, Clone)]
    pub struct FilesBucket {
        pub files_bucket_name: String,
        pub client:            Client,
        pub expires_in:        std::time::Duration,
    }

    impl FilesBucket {
        pub fn new(
            config: &SdkConfig,
            files_bucket_name: String,
            expires_in: std::time::Duration,
        ) -> Self {
            Self {
                files_bucket_name,
                client: S3Client::new(config),
                expires_in,
            }
        }

        pub async fn create_presigned_url(&self, file_name: &str) -> Result<String> {
            let expires_in: PresigningConfig = PresigningConfig::expires_in(self.expires_in)?;
            let req = self
                .client
                .put_object()
                .bucket(&self.files_bucket_name)
                .key(file_name)
                .presigned(expires_in)
                .await?;
            Ok(req.uri().into())
        }

        pub async fn exists(&self, file_name: &str) -> Result<bool> {
            let resp = self
                .client
                .head_object()
                .bucket(&self.files_bucket_name)
                .key(file_name)
                .send()
                .await?;
            Ok(resp.content_length().is_some())
        }

        pub async fn delete(&self, file_name: &str) -> Result<()> {
            self.client
                .delete_object()
                .bucket(&self.files_bucket_name)
                .key(file_name)
                .send()
                .await?;
            Ok(())
        }
    }
}

pub mod ses {
    use shared::db_app::users::Company;
    use tracing::info;
    use uuid::Uuid;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {}

    #[derive(Debug, Clone)]
    pub struct Emailer {
        pub client: aws_sdk_sesv2::Client,
        pub from_email: String,
        pub company_invitation_accept_url: String,
    }

    impl Emailer {
        pub async fn send_company_invitation_email(
            &self,
            invitation_id: &Uuid,
            invitee_email: &str,
            inviter_email: &str,
            company: &Company,
        ) -> Result<(), Error> {
            info!(
                "Sending company invitation email to {} from {} for company {}, invitation url: {}, reject url: {}",
                invitee_email,
                inviter_email,
                company.name,
                format!("{}/{}/accept", self.company_invitation_accept_url, invitation_id),
                format!("{}/{}/reject", self.company_invitation_accept_url, invitation_id),
            );
            Ok(())
        }
    }
}
