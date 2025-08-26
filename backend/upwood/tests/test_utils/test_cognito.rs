use aws_config::SdkConfig;
use aws_sdk_cognitoidentityprovider::types::{AuthFlowType, ChallengeNameType, UserType};

pub struct CognitoTestClient {
    pub client:              aws_sdk_cognitoidentityprovider::Client,
    pub user_pool_id:        String,
    pub user_pool_client_id: String,
}

impl CognitoTestClient {
    pub fn new(
        config: &SdkConfig,
        aws_user_pool_id: String,
        aws_user_pool_client_id: String,
    ) -> Self {
        let client = aws_sdk_cognitoidentityprovider::Client::new(config);
        Self {
            client,
            user_pool_id: aws_user_pool_id.to_owned(),
            user_pool_client_id: aws_user_pool_client_id.to_owned(),
        }
    }

    pub async fn admin_set_user_password(&mut self, username: &str, password: &str) {
        self.client
            .admin_set_user_password()
            .user_pool_id(self.user_pool_id.to_owned())
            .username(username.to_owned())
            .password(password.to_owned())
            .permanent(false)
            .send()
            .await
            .expect("Failed to set user password");
    }

    pub async fn user_change_password(
        &mut self,
        email: &str,
        password_temp: &str,
        password: &str,
    ) -> String {
        let auth_response = self.user_initiate_auth_req(email, password_temp).await;
        assert!(auth_response
            .challenge_name
            .is_some_and(|c| c.eq(&ChallengeNameType::NewPasswordRequired)));

        let res = self
            .client
            .respond_to_auth_challenge()
            .client_id(self.user_pool_client_id.to_owned())
            .challenge_name(ChallengeNameType::NewPasswordRequired)
            .challenge_responses("USERNAME", email)
            .challenge_responses("NEW_PASSWORD", password)
            .set_session(auth_response.session)
            .send()
            .await
            .expect("Failed to respond to new password required challenge");
        res.authentication_result
            .expect("Failed to get auth response")
            .id_token
            .expect("Failed to get id token")
    }

    pub async fn user_initiate_auth_req(
        &self,
        email: &str,
        password: &str,
    ) -> aws_sdk_cognitoidentityprovider::operation::initiate_auth::InitiateAuthOutput {
        self.client
            .initiate_auth()
            .client_id(self.user_pool_client_id.to_owned())
            .auth_flow(AuthFlowType::UserPasswordAuth)
            .auth_parameters("USERNAME", email.to_owned())
            .auth_parameters("PASSWORD", password.to_owned())
            .send()
            .await
            .expect("Failed to initiate auth")
    }

    pub async fn user_login(&self, email: &str, password: &str) -> String {
        let auth_response = self.user_initiate_auth_req(email, password).await;
        assert!(auth_response.challenge_name.is_none());

        let auth_response = auth_response
            .authentication_result
            .expect("Failed to get auth response");
        auth_response.id_token.expect("Failed to get id token")
    }

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

    pub async fn admin_remove_from_admin_group(&mut self, username: &str) {
        self.client
            .admin_remove_user_from_group()
            .user_pool_id(self.user_pool_id.to_owned())
            .username(username.to_owned())
            .group_name("admin".to_owned())
            .send()
            .await
            .expect("Failed to remove user from admin group");
    }

    pub async fn find_by_email(&self, email: &str) -> Option<String> {
        let res = self
            .client
            .list_users()
            .user_pool_id(self.user_pool_id.to_owned())
            .filter(format!("email = \"{}\"", email))
            .send()
            .await
            .expect("Failed to list users");
        res.users
            .and_then(|users| users.into_iter().next())
            .and_then(|user: UserType| user.username)
    }
}
