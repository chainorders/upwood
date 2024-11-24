use chrono::NaiveDateTime;
use poem::http::{Method, StatusCode};
use poem::test::{TestClient, TestResponse};
use poem::Route;
use shared::api::PagedResponse;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectHolderRewardTotal, ForestProjectInvestor, ForestProjectPrice,
    ForestProjectSeller, ForestProjectState, ForestProjectUser, HolderReward, UserTransaction,
};
use upwood::api;
use upwood::api::investment_portfolio::InvestmentPortfolioUserAggregate;
use upwood::api::user::{
    AdminUser, ApiUser, UserRegisterReq, UserRegistrationInvitationSendReq,
    UserUpdateAccountAddressRequest,
};
use uuid::Uuid;

pub struct ApiTestClient {
    pub client: TestClient<Route>,
}

// Users Implmentation
impl ApiTestClient {
    pub async fn new(config: api::Config) -> Self {
        let api = api::create_web_app(config).await;
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

    pub async fn user_register(&mut self, id_token: String, req: &UserRegisterReq) -> ApiUser {
        let res = self
            .client
            .request(Method::POST, "/users")
            .body_json(req)
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse update user response")
    }

    pub async fn user_self_req(&self, id_token: String) -> poem::test::TestResponse {
        self.client
            .get("/users")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
    }

    pub async fn user_self(&self, id_token: String) -> ApiUser {
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
        id_token: String,
        cognito_user_id: String,
        account_address: String,
    ) -> TestResponse {
        self.client
            .put(format!("/admin/users/{}/account_address", cognito_user_id))
            .body_json(&UserUpdateAccountAddressRequest {
                account_address: account_address.to_owned(),
            })
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
    }

    pub async fn admin_list_users(&self, id_token: String, page: i64) -> PagedResponse<AdminUser> {
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

    pub async fn admin_user_delete(&mut self, id_token: String, cognito_user_id: String) {
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

    pub async fn txn_history_list(
        &self,
        id_token: String,
        page: i64,
    ) -> PagedResponse<UserTransaction> {
        let res = self
            .client
            .get(format!("/user/txn_history/list/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list txn history response")
    }
}

// Files Implementation
impl ApiTestClient {
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

// Forest Projects Admin Implementation
impl ApiTestClient {
    pub async fn admin_find_forest_project(
        &self,
        id_token: String,
        project_id: Uuid,
    ) -> ForestProject {
        let res = self
            .client
            .get(format!("/admin/forest_projects/{}", project_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse find forest project response")
    }

    pub async fn admin_create_forest_project(
        &mut self,
        id_token: String,
        req: ForestProject,
    ) -> ForestProject {
        let res = self
            .client
            .post("/admin/forest_projects")
            .body_json(&req)
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse create forest project response")
    }

    pub async fn admin_forest_projects_list(
        &self,
        id_token: String,
        page: i64,
        state: Option<ForestProjectState>,
    ) -> PagedResponse<ForestProject> {
        let res = self
            .client
            .get(format!("/admin/forest_projects/list/{}", page))
            .query("state", &state)
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest projects response")
    }

    pub async fn admin_update_forest_project(
        &mut self,
        id_token: String,
        req: ForestProject,
    ) -> ForestProject {
        let res = self
            .client
            .put("/admin/forest_projects")
            .body_json(&req)
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse update forest project response")
    }

    pub async fn admin_forest_project_create_price(
        &self,
        id_token: String,
        project_id: Uuid,
        price: &ForestProjectPrice,
    ) -> ForestProjectPrice {
        let res = self
            .client
            .post(format!("/admin/forest_projects/{}/price", project_id))
            .body_json(price)
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse create forest project price response")
    }

    pub async fn admin_forest_project_investor_list(
        &self,
        id_token: String,
        project_id: Uuid,
        page: i64,
    ) -> PagedResponse<ForestProjectInvestor> {
        let res = self
            .client
            .get(format!(
                "/admin/forest_projects/{}/fund/investor/list/{}",
                project_id, page
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest project investors response")
    }
}

// Forest Projects User Implementation
impl ApiTestClient {
    pub async fn forest_project_list_active(
        &self,
        id_token: String,
        page: i64,
    ) -> PagedResponse<ForestProjectUser> {
        let res = self
            .client
            .get(format!("/forest_projects/list/active/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse list active forest projects response")
    }

    pub async fn forest_project_list_owned(
        &self,
        id_token: String,
    ) -> PagedResponse<ForestProjectUser> {
        let res = self
            .client
            .get("/forest_projects/list/owned")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse list owned forest projects response")
    }

    pub async fn forest_project_find(
        &self,
        id_token: String,
        project_id: Uuid,
    ) -> ForestProjectUser {
        let res = self
            .client
            .get(format!("/forest_projects/{}", project_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse find forest project response")
    }

    pub async fn forest_project_rewards_total(
        &self,
        id_token: String,
    ) -> Vec<ForestProjectHolderRewardTotal> {
        let res = self
            .client
            .get("/forest_projects/rewards/total")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse forest project rewards total response")
    }

    pub async fn forest_project_rewards_claimable(&self, id_token: String) -> Vec<HolderReward> {
        let res = self
            .client
            .get("/forest_projects/rewards/claimable")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse forest project rewards claimable response")
    }

    pub async fn forest_project_p2p_trade_sellers_list(
        &self,
        id_token: String,
        project_id: Uuid,
        page: i64,
    ) -> PagedResponse<ForestProjectSeller> {
        let res = self
            .client
            .get(format!(
                "/forest_projects/{}/p2p-trade/sellers/list/{}",
                project_id, page
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse list p2p trade sellers response")
    }
}

// User Portfolio Implementation
impl ApiTestClient {
    pub async fn portfolio_aggreagte(
        &self,
        id_token: String,
        now: Option<NaiveDateTime>,
    ) -> InvestmentPortfolioUserAggregate {
        let res = self
            .client
            .get("/portfolio/aggregate")
            .header("Authorization", format!("Bearer {}", id_token))
            .query("now", &now)
            .send()
            .await;
        assert_eq!(res.0.status(), StatusCode::OK);
        res.0
            .into_body()
            .into_json()
            .await
            .expect("Failed to parse list portfolio response")
    }
}
