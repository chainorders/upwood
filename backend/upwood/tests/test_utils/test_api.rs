use poem::http::StatusCode;
use poem::test::{TestClient, TestResponse};
use poem::Route;
use poem_openapi::types::ToJSON;
use rust_decimal::Decimal;
use shared::api::PagedResponse;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectMedia, ForestProjectPrice, ForestProjectState,
};
use shared::db_app::forest_project_crypto::{
    ForestProjectFundInvestor, ForestProjectFundsAffiliateRewardRecord, ForestProjectTokenContract,
    ForestProjectUserYieldsAggregate, ForestProjectUserYieldsForEachOwnedToken,
    SecurityTokenContractType, TokenMetadata,
};
use shared::db_app::portfolio::UserTransaction;
use shared::db_app::users::UserRegistrationRequest;
use upwood::api;
use upwood::api::files::UploadUrlResponse;
use upwood::api::forest_project::ForestProjectAggApiModel;
use upwood::api::investment_portfolio::{InvestmentPortfolioUserAggregate, PortfolioValue};
use upwood::api::user::{
    ApiUser, ClaimRequest, LoginReq, LoginRes, UserCreatePostReq, UserCreatePostReqAdmin,
    UserRegisterGetRes, UserRegistrationRequestApi,
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
    pub async fn user_self_req(&self, id_token: String) -> poem::test::TestResponse {
        self.client
            .get("/user")
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

    pub async fn admin_list_users(&self, id_token: String, page: i64) -> PagedResponse<ApiUser> {
        let res = self
            .client
            .get(format!("/admin/user/list/{}", page))
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

// User Registration Request Implementation
impl ApiTestClient {
    pub async fn user_registration_request(&self, req: UserRegistrationRequestApi) -> TestResponse {
        self.client
            .post("/user/registration-request")
            .body_json(&req)
            .send()
            .await
    }

    pub async fn admin_registration_request_list(
        &self,
        id_token: String,
        page: i64,
    ) -> PagedResponse<UserRegistrationRequest> {
        let res = self
            .client
            .get(format!("/admin/registration-request/list/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list registration requests response")
    }

    pub async fn admin_registration_request_get(
        &self,
        id_token: String,
        request_id: Uuid,
    ) -> UserRegistrationRequest {
        let res = self
            .client
            .get(format!("/admin/registration-request/{}", request_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse get registration request response")
    }

    pub async fn admin_registration_request_accept(
        &self,
        id_token: String,
        request_id: Uuid,
        is_accepted: bool,
    ) -> TestResponse {
        self.client
            .put(format!(
                "/admin/registration-request/{}/accept/{}",
                request_id, is_accepted
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
    }

    pub async fn get_user_register(&self, registration_request_id: Uuid) -> UserRegisterGetRes {
        let res = self
            .client
            .get(format!("/user/register/{}", registration_request_id))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse get user register response")
    }

    pub async fn post_user_register(
        &self,
        registration_request_id: Uuid,
        req: UserCreatePostReq,
    ) -> ApiUser {
        let res = self
            .client
            .post(format!("/user/register/{}", registration_request_id))
            .body_json(&req)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse post user register response")
    }

    pub async fn admin_user_register(
        &self,
        id_token: String,
        registration_request_id: Uuid,
        req: UserCreatePostReqAdmin,
    ) -> ApiUser {
        let res = self
            .client
            .post(format!("/admin/user/register/{}", registration_request_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(&req)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse admin user register response")
    }

    pub async fn user_login_request(&self, req: LoginReq) -> TestResponse {
        self.client.post("/user/login").body_json(&req).send().await
    }

    pub async fn user_login(&self, req: LoginReq) -> LoginRes {
        let res = self.user_login_request(req).await.0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse user login response")
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
    pub async fn forest_project_list_by_state(
        &self,
        id_token: String,
        state: ForestProjectState,
        page: i64,
    ) -> PagedResponse<ForestProjectAggApiModel> {
        let res = self
            .client
            .get(format!("/forest_projects/list/{}/{}", state, page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list forest projects by state response")
    }

    pub async fn forest_project_get(
        &self,
        id_token: String,
        project_id: Uuid,
    ) -> ForestProjectAggApiModel {
        let res = self
            .client
            .get(format!("/forest_projects/{}", project_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse get forest project response")
    }

    pub async fn forest_project_list_owned(
        &self,
        id_token: String,
    ) -> PagedResponse<ForestProjectAggApiModel> {
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

    pub async fn admin_find_forest_project_latest_price(
        &self,
        id_token: String,
        project_id: Uuid,
    ) -> ForestProjectPrice {
        let res = self
            .client
            .get(format!(
                "/admin/forest_projects/{}/price/latest",
                project_id
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse find forest project latest price response")
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
                project_id,
                contract_type.to_json_string()
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
    }

    pub async fn forest_project_token_contract_find(
        &self,
        id_token: String,
        project_id: Uuid,
        contract_type: SecurityTokenContractType,
    ) -> ForestProjectTokenContract {
        let res = self
            .client
            .get(format!(
                "/admin/forest_projects/{}/token_contract/{}",
                project_id, contract_type
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse forest project token contract response")
    }
}

// Investment Portfolio Implementation
impl ApiTestClient {
    pub async fn portfolio_aggregate(
        &self,
        id_token: String,
        now: Option<chrono::NaiveDateTime>,
    ) -> InvestmentPortfolioUserAggregate {
        let res = self
            .client
            .get("/portfolio/aggregate")
            .header("Authorization", format!("Bearer {}", id_token))
            .query("now", &now)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse portfolio aggregate response")
    }

    pub async fn portfolio_value_last_n_months(
        &self,
        id_token: String,
        months: u32,
        now: Option<chrono::NaiveDateTime>,
    ) -> Vec<PortfolioValue> {
        let res = self
            .client
            .get(format!("/portfolio/value_last_n_months/{}", months))
            .header("Authorization", format!("Bearer {}", id_token))
            .query("now", &now)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse portfolio value last n months response")
    }
}

// Token Metadata Implementation
impl ApiTestClient {
    pub async fn admin_create_token_metadata(
        &self,
        id_token: String,
        metadata: &TokenMetadata,
    ) -> TokenMetadata {
        let res = self
            .client
            .post("/admin/token_metadata")
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(metadata)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse create token metadata response")
    }

    pub async fn admin_find_token_metadata(
        &self,
        id_token: String,
        contract_address: Decimal,
        token_id: Decimal,
    ) -> TokenMetadata {
        let res = self
            .client
            .get(format!(
                "/admin/token_metadata/{}/{}",
                contract_address, token_id
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse find token metadata response")
    }

    pub async fn admin_update_token_metadata(
        &self,
        id_token: String,
        metadata: &TokenMetadata,
    ) -> TokenMetadata {
        let res = self
            .client
            .put("/admin/token_metadata")
            .header("Authorization", format!("Bearer {}", id_token))
            .body_json(metadata)
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse update token metadata response")
    }

    pub async fn admin_list_token_metadata(
        &self,
        id_token: String,
        page: i64,
    ) -> PagedResponse<TokenMetadata> {
        let res = self
            .client
            .get(format!("/admin/token_metadata/list/{}", page))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
        res.into_body()
            .into_json()
            .await
            .expect("Failed to parse list token metadata response")
    }

    pub async fn admin_delete_token_metadata(
        &self,
        id_token: String,
        contract_address: Decimal,
        token_id: Decimal,
    ) {
        let res = self
            .client
            .delete(format!(
                "/admin/token_metadata/{}/{}",
                contract_address, token_id
            ))
            .header("Authorization", format!("Bearer {}", id_token))
            .send()
            .await
            .0;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
