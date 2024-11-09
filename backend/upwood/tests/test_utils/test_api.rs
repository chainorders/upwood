use poem::http::StatusCode;
use poem::test::TestClient;
use poem::Route;
use shared::api::PagedResponse;
use upwood::api;
use upwood::api::user::{
    AdminUser, ApiUser, UserRegisterReq, UserRegistrationInvitationSendReq,
    UserUpdateAccountAddressRequest,
};
use uuid::Uuid;

pub struct TestApi {
    pub client: TestClient<Route>,
}

// Users Implmentation
impl TestApi {
    pub async fn new(config: api::Config) -> Self {
        let api = api::create_web_app(&config).await;
        let api = TestClient::new(api);
        Self { client: api }
    }

    pub async fn user_send_invitation(&mut self, email: &str) -> String {
        let mut invitation_res = self
            .client
            .post("/users/invitation")
            .body_json(&UserRegistrationInvitationSendReq {
                email:                     email.to_owned(),
                affiliate_account_address: None,
            })
            .send()
            .await
            .0;
        match invitation_res.status() {
            StatusCode::OK => {
                let id: String = invitation_res
                    .into_body()
                    .into_json()
                    .await
                    .expect("Failed to parse invitation response");
                id.to_owned()
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                let res = invitation_res
                    .take_body()
                    .into_string()
                    .await
                    .expect("Failed to parse invitation response");
                panic!("Failed to send invitation: {}", res);
            }
            res => panic!("Unexpected response: {}", res),
        }
    }

    pub async fn user_register(&mut self, id_token: &str, req: &UserRegisterReq) -> ApiUser {
        let res = self
            .client
            .post("/users")
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(req)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse update user response")
    }

    pub async fn user_self_req(&self, id_token: &str) -> poem::test::TestResponse {
        self.client
            .get("/users")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
    }

    pub async fn user_self(&self, id_token: &str) -> ApiUser {
        let mut res = self.user_self_req(id_token).await.0;
        match res.status() {
            StatusCode::OK => {
                let res: ApiUser = res
                    .into_body()
                    .into_json()
                    .await
                    .expect("Failed to parse get user response");
                res
            }
            status_code => {
                let res = res
                    .take_body()
                    .into_string()
                    .await
                    .expect("Failed to parse get user response");
                panic!("Failed to get user: {} {}", status_code, res);
            }
        }
    }

    pub async fn admin_user_update_account_address(
        &mut self,
        id_token: &str,
        cognito_user_id: &str,
        address: &str,
    ) {
        let res = self
            .client
            .put(format!("/admin/users/{}/account_address", cognito_user_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(&UserUpdateAccountAddressRequest {
                account_address: address.to_owned(),
            })
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
    }

    pub async fn admin_list_users(&self, id_token: &str, page: i64) -> PagedResponse<AdminUser> {
        let res = self
            .client
            .get(format!("/admin/users/list/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list users response")
    }

    pub async fn admin_user_delete(&mut self, id_token: &str, cognito_user_id: &str) {
        let mut res = self
            .client
            .delete(format!("/admin/users/{}", cognito_user_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        match res.status() {
            StatusCode::OK => {}
            status_code => {
                let res = res
                    .take_body()
                    .into_string()
                    .await
                    .expect("Failed to parse delete user response");
                panic!("Failed to delete user: {} {}", status_code, res);
            }
        }
    }
}

// Files Implementation
impl TestApi {
    pub async fn admin_file_upload_url_s3(&self, id_token: &str) -> api::files::UploadUrlResponse {
        let mut res = self
            .client
            .post("/admin/files/s3/upload_url")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;

        match res.status() {
            StatusCode::OK => {
                let res: api::files::UploadUrlResponse = res
                    .into_body()
                    .into_json()
                    .await
                    .expect("Failed to parse s3 upload url response");
                res
            }
            status_code => {
                let res = res
                    .take_body()
                    .into_string()
                    .await
                    .expect("Failed to parse s3 upload url response");
                panic!("Failed to get s3 upload url: {} {}", status_code, res);
            }
        }
    }

    pub async fn admin_delete_file_s3(&mut self, id_token: &str, file_name: &Uuid) {
        let res = self
            .client
            .delete(format!("/admin/files/s3/{}", file_name))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
    }

    pub async fn admin_file_upload_url_ipfs(
        &self,
        id_token: &str,
    ) -> api::files::UploadUrlResponse {
        let mut res = self
            .client
            .post("/admin/files/ipfs/upload_url")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;

        match res.status() {
            StatusCode::OK => {
                let res: api::files::UploadUrlResponse = res
                    .into_body()
                    .into_json()
                    .await
                    .expect("Failed to parse s3 upload url response");
                res
            }
            status_code => {
                let res = res
                    .take_body()
                    .into_string()
                    .await
                    .expect("Failed to parse s3 upload url response");
                panic!("Failed to get s3 upload url: {} {}", status_code, res);
            }
        }
    }

    pub async fn admin_delete_file_ipfs(&mut self, id_token: &str, file_name: &Uuid) {
        let res = self
            .client
            .delete(format!("/admin/files/ipfs/{}", file_name))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
