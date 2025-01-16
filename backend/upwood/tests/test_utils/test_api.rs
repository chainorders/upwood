use poem::http::{Method, StatusCode};
use poem::test::{TestClient, TestResponse};
use poem::Route;
use poem_openapi::types::ToJSON;
use shared::api::PagedResponse;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectMedia, ForestProjectPrice, ForestProjectState,
};
use shared::db_app::forest_project_crypto::{
    ActiveForestProjectUser, ForestProjectFundInvestor, ForestProjectFundsAffiliateRewardRecord, ForestProjectOwned, ForestProjectTokenContract, ForestProjectUserYieldsAggregate, ForestProjectUserYieldsForEachOwnedToken, FundedForestProjectUser, SecurityTokenContractType
};
use shared::db_app::portfolio::UserTransaction;
use upwood::api;
use upwood::api::files::UploadUrlResponse;
use upwood::api::user::{
    AdminUser, ApiUser, ClaimRequest, UserRegisterReq, UserRegistrationInvitationSendReq,
    UserUpdateAccountAddressRequest,
};
use uuid::Uuid;

pub struct ApiTestClient {
    pub client: TestClient<Route>,
}

impl ApiTestClient {
    pub async fn new(config: api::Config) -> Self {
        let api = api::create_web_app(config).await;
        let api = TestClient::new(api);
        Self { client: api }
    }
}

// Users Implementation
impl ApiTestClient {
    pub async fn user_send_invitation(
        &mut self,
        req: &UserRegistrationInvitationSendReq,
    ) -> String {
        let mut invitation_res = self
            .client
            .post("/users/invitation")
            .body_json(req)
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

    pub async fn admin_update_account_address(
        &mut self,
        id_token: String,
        cognito_user_id: String,
        req: &UserUpdateAccountAddressRequest,
    ) -> TestResponse {
        self.client
            .put(format!("/admin/users/{}/account_address", cognito_user_id))
            .body_json(req)
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

    pub async fn user_affiliate_rewards_list(
        &self,
        id_token: String,
        page: i64,
    ) -> PagedResponse<ForestProjectFundsAffiliateRewardRecord> {
        let res = self
            .client
            .get(format!("/user/affiliate/rewards/list/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list affiliate rewards response")
    }

    pub async fn user_affiliate_rewards_claim(
        &mut self,
        id_token: String,
        investment_record_id: Uuid,
    ) -> ClaimRequest {
        let res = self
            .client
            .get(format!(
                "/user/affiliate/rewards/claim/{}",
                investment_record_id
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse claim affiliate rewards response")
    }

    pub async fn user_transactions_list(
        &self,
        id_token: String,
        page: i64,
    ) -> PagedResponse<UserTransaction> {
        let res = self
            .client
            .get(format!("/user/transactions/list/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list user transactions response")
    }

    pub async fn system_config(&self) -> api::SystemContractsConfig {
        let res = self.client.get("/system_config").send().await.0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse system contracts config response")
    }
}

// Files Implementation
impl ApiTestClient {
    pub async fn admin_file_upload_url_s3(&self, id_token: String) -> UploadUrlResponse {
        let mut res = self
            .client
            .post("/admin/files/s3/upload_url")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;

        match res.status() {
            StatusCode::OK => {
                let res: UploadUrlResponse = res
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

    pub async fn admin_delete_file_s3(&mut self, id_token: String, file_name: Uuid) {
        let res = self
            .client
            .delete(format!("/admin/files/s3/{}", file_name))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
    }

    pub async fn admin_file_upload_url_ipfs(&self, id_token: String) -> UploadUrlResponse {
        let mut res = self
            .client
            .post("/admin/files/ipfs/upload_url")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;

        match res.status() {
            StatusCode::OK => {
                let res: UploadUrlResponse = res
                    .into_body()
                    .into_json()
                    .await
                    .expect("Failed to parse ipfs upload url response");
                res
            }
            status_code => {
                let res = res
                    .take_body()
                    .into_string()
                    .await
                    .expect("Failed to parse ipfs upload url response");
                panic!("Failed to get ipfs upload url: {} {}", status_code, res);
            }
        }
    }

    pub async fn admin_delete_file_ipfs(&mut self, id_token: String, file_name: Uuid) {
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

// Forest Projects Implementation
impl ApiTestClient {
    pub async fn forest_project_list_active(
        &self,
        id_token: String,
    ) -> PagedResponse<ActiveForestProjectUser> {
        let res = self
            .client
            .get("/forest_projects/list/active")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list active forest projects response")
    }

    pub async fn forest_project_get_active(
        &self,
        id_token: String,
        project_id: Uuid,
    ) -> ActiveForestProjectUser {
        let res = self
            .client
            .get(format!("/forest_projects/active/{}", project_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse get active forest project response")
    }

    pub async fn forest_project_list_funded(
        &self,
        id_token: String,
        page: i64,
    ) -> PagedResponse<FundedForestProjectUser> {
        let res = self
            .client
            .get(format!("/forest_projects/list/funded/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list funded forest projects response")
    }

    pub async fn forest_project_get_funded(
        &self,
        id_token: String,
        project_id: Uuid,
    ) -> FundedForestProjectUser {
        let res = self
            .client
            .get(format!("/forest_projects/funded/{}", project_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse get funded forest project response")
    }

    pub async fn forest_project_list_owned(
        &self,
        id_token: String,
    ) -> PagedResponse<ForestProjectOwned> {
        let res = self
            .client
            .get("/forest_projects/list/owned")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list owned forest projects response")
    }

    pub async fn forest_project_list_media(
        &self,
        id_token: String,
        project_id: Uuid,
        page: i64,
    ) -> PagedResponse<ForestProjectMedia> {
        let res = self
            .client
            .get(format!(
                "/forest_projects/{}/media/list/{}",
                project_id, page
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest project media response")
    }

    pub async fn forest_project_find_media(
        &self,
        id_token: String,
        project_id: Uuid,
        media_id: Uuid,
    ) -> ForestProjectMedia {
        let res = self
            .client
            .get(format!(
                "/forest_projects/{}/media/{}",
                project_id, media_id
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse find forest project media response")
    }

    pub async fn forest_project_yields_total(
        &self,
        id_token: String,
    ) -> Vec<ForestProjectUserYieldsAggregate> {
        let res = self
            .client
            .get("/forest_projects/yields/total")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest project yields total response")
    }

    pub async fn forest_project_yields_claimable(
        &self,
        id_token: String,
    ) -> Vec<ForestProjectUserYieldsForEachOwnedToken> {
        let res = self
            .client
            .get("/forest_projects/yields/claimable")
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest project yields claimable response")
    }

    // ForestProjectAdminApi Implementation
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
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse find forest project response")
    }

    pub async fn admin_list_forest_projects(
        &self,
        id_token: String,
        page: i64,
        state: Option<ForestProjectState>,
    ) -> PagedResponse<ForestProject> {
        let res = self
            .client
            .get(format!("/admin/forest_projects/list/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .query("state", &state)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest projects response")
    }

    pub async fn admin_create_forest_project(
        &self,
        id_token: String,
        project: &ForestProject,
    ) -> ForestProject {
        let res = self
            .client
            .post("/admin/forest_projects")
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(project)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse create forest project response")
    }

    pub async fn admin_update_forest_project(
        &self,
        id_token: String,
        project: &ForestProject,
    ) -> ForestProject {
        let res = self
            .client
            .put("/admin/forest_projects")
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(project)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse update forest project response")
    }

    pub async fn admin_create_forest_project_media(
        &self,
        id_token: String,
        project_id: Uuid,
        media: &ForestProjectMedia,
    ) -> ForestProjectMedia {
        let res = self
            .client
            .post(format!("/admin/forest_projects/{}/media", project_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(media)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse create forest project media response")
    }

    pub async fn admin_delete_forest_project_media(
        &self,
        id_token: String,
        project_id: Uuid,
        media_id: Uuid,
    ) -> ForestProjectMedia {
        let res = self
            .client
            .delete(format!(
                "/admin/forest_projects/{}/media/{}",
                project_id, media_id
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse delete forest project media response")
    }

    pub async fn admin_find_forest_project_price(
        &self,
        id_token: String,
        project_id: Uuid,
        price_at: chrono::NaiveDateTime,
    ) -> ForestProjectPrice {
        let res = self
            .client
            .get(format!(
                "/admin/forest_projects/{}/price/{}",
                project_id, price_at
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse find forest project price response")
    }

    pub async fn admin_list_forest_project_prices(
        &self,
        id_token: String,
        project_id: Uuid,
        page: i64,
    ) -> PagedResponse<ForestProjectPrice> {
        let res = self
            .client
            .get(format!(
                "/admin/forest_projects/{}/price/list/{}",
                project_id, page
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest project prices response")
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
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(price)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse create forest project price response")
    }

    pub async fn admin_forest_project_delete_price(
        &self,
        id_token: String,
        project_id: Uuid,
        price_at: chrono::NaiveDateTime,
    ) -> TestResponse {
        self.client
            .delete(format!(
                "/admin/forest_projects/{}/price/{}",
                project_id, price_at
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
    }

    pub async fn admin_forest_project_investor_list(
        &self,
        id_token: String,
        project_id: Uuid,
        page: i64,
    ) -> PagedResponse<ForestProjectFundInvestor> {
        let res = self
            .client
            .get(format!(
                "/admin/forest_projects/{}/fund/investor/list/{}",
                project_id, page
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest project investors response")
    }

    pub async fn forest_project_token_contract_create(
        &self,
        id_token: String,
        contract: &ForestProjectTokenContract,
    ) -> ForestProjectTokenContract {
        let res = self
            .client
            .post("/admin/forest_projects/token_contract")
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(contract)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse create forest project token contract response")
    }

    pub async fn forest_project_token_contract_update(
        &self,
        id_token: String,
        contract: &ForestProjectTokenContract,
    ) -> ForestProjectTokenContract {
        let res = self
            .client
            .put("/admin/forest_projects/token_contract")
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(contract)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse update forest project token contract response")
    }

    pub async fn forest_project_token_contract_delete(
        &self,
        id_token: String,
        project_id: Uuid,
        contract_type: SecurityTokenContractType,
    ) {
        let res = self
            .client
            .delete(format!(
                "/admin/forest_projects/{}/token_contract/{}",
                project_id, contract_type.to_json_string()
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
