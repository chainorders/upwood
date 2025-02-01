use std::collections::BTreeSet;

use aws::cognito::{
    account_address_attribute, affiliate_account_address_attribute, email_attribute,
    email_verified_attribute, first_name_attribute, last_name_attribute, nationality_attribute,
};
use aws_sdk_cognitoidentityprovider::types::MessageActionType;
use concordium::account::Signer;
use concordium::identity::{Presentation, VerifyPresentationResponse};
use concordium_cis2::{TokenAmountU64, TokenIdUnit};
use concordium_rust_sdk::base::contracts_common::AccountSignatures;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::Address;
use concordium_rust_sdk::v2::BlockIdentifier;
use concordium_rust_sdk::web3id::CredentialMetadata;
use concordium_rust_sdk::{v2, web3id};
use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use serde::Serialize;
use shared::api::PagedResponse;
use shared::db::identity_registry::Identity;
use shared::db::offchain_rewards::OffchainRewardee;
use shared::db::security_mint_fund::SecurityMintFundContract;
use shared::db_app::forest_project_crypto::{
    ForestProjectFundsAffiliateRewardRecord, ForestProjectFundsInvestmentRecord,
};
use shared::db_app::portfolio::UserTransaction;
use shared::db_app::users::{User, UserRegistrationRequest};

use crate::api::*;
use crate::utils::*;

#[derive(Clone, Copy)]
pub struct UserApi;

#[OpenApi]
impl UserApi {
    #[oai(
        path = "/user/registration-request",
        method = "post",
        tag = "ApiTags::User"
    )]
    pub async fn user_registration_request(
        &self,
        Data(db_pool): Data<&DbPool>,
        Json(req): Json<UserRegistrationRequestApi>,
    ) -> NoResResult {
        let mut conn = db_pool.get()?;
        if User::find_by_email(&mut conn, &req.email)?.is_some() {
            return Err(Error::BadRequest(PlainText(
                "User already registered".to_string(),
            )));
        }
        if UserRegistrationRequest::find_by_email(&mut conn, &req.email)?.is_some() {
            return Err(Error::BadRequest(PlainText(
                "Request already exists".to_string(),
            )));
        }

        UserRegistrationRequest {
            id: uuid::Uuid::new_v4(),
            email: req.email.clone(),
            affiliate_account_address: req.affiliate_account_address.clone(),
            is_accepted: false,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
        .insert(&mut conn)?;
        Ok(())
    }

    #[oai(
        path = "/admin/registration-request/list/:page",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn admin_registration_request_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<UserRegistrationRequest>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (requests, page_count) = UserRegistrationRequest::list(&mut conn, page, PAGE_SIZE)?;
        Ok(Json(PagedResponse {
            data: requests,
            page,
            page_count,
        }))
    }

    #[oai(
        path = "/admin/registration-request/:id",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn admin_registration_request_get(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(id): Path<uuid::Uuid>,
    ) -> JsonResult<UserRegistrationRequest> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let request = UserRegistrationRequest::find(&mut conn, id)?
            .ok_or_else(|| Error::NotFound(PlainText("Request not found".to_string())))?;
        Ok(Json(request))
    }

    /// Accept or reject a user registration request.
    /// If the request is accepted, the user is added to the Cognito user pool.
    #[oai(
        path = "/admin/registration-request/:id/accept/:is_accepted",
        method = "put",
        tag = "ApiTags::User"
    )]
    pub async fn admin_registration_request_accept(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(id): Path<uuid::Uuid>,
        Path(is_accepted): Path<bool>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let request = UserRegistrationRequest::find(&mut conn, id)?
            .ok_or_else(|| Error::NotFound(PlainText("Request not found".to_string())))?;
        if is_accepted {
            // Add the user to the cognito user pool. This will trigger an invitation email to the user.
            let mut attrs = vec![email_attribute(&request.email)];
            if let Some(affiliate_account_address) = request.affiliate_account_address {
                attrs.push(affiliate_account_address_attribute(
                    &affiliate_account_address,
                ));
            }
            user_pool
                .admin_upsert_user(&request.email, attrs, MessageActionType::Resend)
                .await
                .map_err(|e| {
                    error!("Error creating user in user pool: {:?}", e);
                    e
                })?;
        }

        UserRegistrationRequest::delete(&mut conn, id)?;
        Ok(())
    }

    #[oai(path = "/user/register", method = "get", tag = "ApiTags::User")]
    pub async fn get_user_register(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(id_statement): Data<&IdStatement>,
    ) -> JsonResult<UserRegisterGetRes> {
        let id_statement = serde_json::to_value(id_statement).map_err(|_| {
            Error::InternalServer(PlainText("Failed to serialize id statement".to_string()))
        })?;
        let challenge = concordium::identity::generate_challenge(claims.sub.as_str());
        let challenge = hex::encode(challenge);
        Ok(Json(UserRegisterGetRes {
            challenge,
            id_statement,
        }))
    }

    /// Registers a user in the Cognito user pool and in the database.
    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/user/register/:registration_request_id",
        method = "post",
        tag = "ApiTags::User"
    )]
    pub async fn post_user_register(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        Data(global_context): Data<&concordium::identity::GlobalContext>,
        Data(network): Data<&web3id::did::Network>,
        Data(concordium_client): Data<&v2::Client>,
        Data(affiliate_commission): Data<&AffiliateCommission>,
        Json(req): Json<UserCreatePostReq>,
    ) -> JsonResult<ApiUser> {
        let mut conn = db_pool.get()?;
        let verification_res = {
            let proof = req
                .proof()?
                .ok_or_else(|| Error::BadRequest(PlainText("Proof not provided".to_string())))?;
            let account_address = req
                .account_address
                .parse()
                .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))?;
            let challenge = concordium::identity::generate_challenge(claims.sub.as_str());

            verify_presentation(
                &mut concordium_client.clone(),
                proof,
                account_address,
                network,
                global_context,
                challenge,
            )
            .await?
        };
        user_pool
            .admin_update_user_attributes(&claims.email, vec![
                email_verified_attribute(true),
                account_address_attribute(&req.account_address),
                first_name_attribute(&verification_res.first_name),
                last_name_attribute(&verification_res.last_name),
                nationality_attribute(&verification_res.nationality),
            ])
            .await?;
        user_pool
            .admin_set_permament_password(&claims.email, &req.password)
            .await?;
        let user = User {
            account_address:           req.account_address.to_string(),
            cognito_user_id:           claims.sub.clone(),
            email:                     claims.email.clone(),
            first_name:                verification_res.first_name,
            last_name:                 verification_res.last_name,
            nationality:               verification_res.nationality,
            affiliate_commission:      affiliate_commission.commission,
            desired_investment_amount: req.desired_investment_amount,
            affiliate_account_address: claims.affiliate_account.clone(),
        }
        .upsert(&mut conn)?;
        Ok(Json(ApiUser::new(user, false, false)))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/admin/user/register", method = "post", tag = "ApiTags::User")]
    pub async fn admin_user_register(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        Data(affiliate_commission): Data<&AffiliateCommission>,
        Data(contracts): Data<&SystemContractsConfig>,
        Json(req): Json<UserCreatePostReqAdmin>,
    ) -> JsonResult<ApiUser> {
        ensure_is_admin(&claims)?;

        let mut conn = db_pool.get()?;
        let user = user_pool
            .admin_create_permanent_user(&req.email, &req.password, vec![
                email_attribute(&req.email),
                email_verified_attribute(true),
                account_address_attribute(&req.account_address),
                first_name_attribute(&req.first_name),
                last_name_attribute(&req.last_name),
                nationality_attribute(&req.nationality),
            ])
            .await
            .map_err(|e| {
                error!("Error creating user in user pool: {:?}", e);
                e
            })?;
        let user = User {
            account_address:           req.account_address.to_string(),
            cognito_user_id:           user
                .attributes
                .and_then(|a| a.iter().find(|a| a.name == "sub").unwrap().value.clone())
                .ok_or_else(|| {
                    Error::InternalServer(PlainText("Cognito user ID not found".to_string()))
                })?,
            email:                     req.email.clone(),
            first_name:                req.first_name,
            last_name:                 req.last_name,
            nationality:               req.nationality,
            affiliate_commission:      req
                .affiliate_commission
                .unwrap_or(affiliate_commission.commission),
            desired_investment_amount: req.desired_investment_amount,
            affiliate_account_address: req.affiliate_account_address.clone(),
        }
        .upsert(&mut conn)?;
        let is_kyc_verified = Identity::exists(
            &mut conn,
            contracts.identity_registry_contract_index,
            &user.account_address,
        )?;
        Ok(Json(ApiUser::new(user, false, is_kyc_verified)))
    }

    /// Retrieves the current user's information based on the provided bearer authorization token.
    ///
    /// This function fetches the user's information from the database using the Cognito user ID
    /// from the bearer authorization token. It also checks if the user's account is KYC verified
    /// by looking up the identity registry.
    ///
    /// # Arguments
    /// * `db_pool` - A reference to the database connection pool.
    /// * `identity_registry` - A reference to the identity registry.
    /// * `claims` - The bearer authorization token claims.
    ///
    /// # Returns
    /// A `JsonResult` containing the user's information.
    #[oai(path = "/user", method = "get", tag = "ApiTags::User")]
    pub async fn user_self(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
    ) -> JsonResult<ApiUser> {
        let mut conn = db_pool.get()?;
        let is_admin = claims.is_admin();
        let user = User::find(&mut conn, &claims.sub)
            .map_err(|_| Error::NotFound(PlainText("User not found".to_string())))?
            .unwrap_or_else(|| User {
                account_address:           claims
                    .account()
                    .map(|a| a.to_string())
                    .unwrap_or_default(),
                cognito_user_id:           claims.sub.clone(),
                email:                     claims.email,
                first_name:                claims.first_name.unwrap_or_default(),
                last_name:                 claims.last_name.unwrap_or_default(),
                nationality:               claims.nationality.unwrap_or_default(),
                affiliate_commission:      Decimal::ZERO,
                desired_investment_amount: None,
                affiliate_account_address: claims.affiliate_account.clone(),
            });
        let is_kyc_verified = Identity::exists(
            &mut conn,
            contracts.identity_registry_contract_index,
            &user.account_address,
        )
        .map_err(|_| Error::InternalServer(PlainText("Failed to check KYC status".to_string())))?;

        let user = ApiUser::new(user, is_admin, is_kyc_verified);
        Ok(Json(user))
    }

    /// Get a user by their Cognito user ID.
    ///
    /// This endpoint is only accessible to admin users.
    ///
    /// # Arguments
    /// - `db_pool`: A reference to the database connection pool.
    /// - `identity_registry`: A reference to the identity registry.
    /// - `claims`: The authorization claims of the requesting user.
    /// - `cognito_user_id`: The Cognito user ID of the user to retrieve.
    ///
    /// # Returns
    /// A JSON response containing the `AdminUser` for the specified Cognito user ID.
    #[oai(
        path = "/admin/user/:cognito_user_id",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn admin_get_by_cognito_user_id(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(cognito_user_id): Path<uuid::Uuid>,
    ) -> JsonResult<ApiUser> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let user = User::find(&mut conn, &cognito_user_id.to_string())?
            .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
        let user = ApiUser::new(
            user.clone(),
            false,
            Identity::exists(
                &mut conn,
                contracts.identity_registry_contract_index,
                &user.account_address,
            )?,
        );
        Ok(Json(user))
    }

    /// Get a user by their account address.
    ///
    /// This endpoint is only accessible to admin users.
    ///
    /// # Arguments
    /// - `db_pool`: A reference to the database connection pool.
    /// - `identity_registry`: A reference to the identity registry.
    /// - `claims`: The authorization claims of the requesting user.
    /// - `account_address`: The account address of the user to retrieve.
    ///
    /// # Returns
    /// A JSON response containing the `AdminUser` for the specified account address.
    #[oai(
        path = "/admin/user/account_address/:account_address",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn admin_get_by_account_address(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(account_address): Path<String>,
    ) -> JsonResult<ApiUser> {
        ensure_is_admin(&claims)?;
        let account_address = account_address
            .parse()
            .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))?;
        let mut conn = db_pool.get()?;
        let user = User::find_by_account_address(&mut conn, &account_address)?
            .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
        let user = ApiUser::new(
            user.clone(),
            false,
            Identity::exists(
                &mut conn,
                contracts.identity_registry_contract_index,
                &user.account_address,
            )?,
        );
        Ok(Json(user))
    }

    /// Get a list of all the users.
    ///
    /// This endpoint is only accessible to admin users.
    ///
    /// # Arguments
    /// - `db_pool`: A reference to the database connection pool.
    /// - `identity_registry`: A reference to the identity registry.
    /// - `claims`: The authorization claims of the requesting user.
    /// - `page`: The page number to retrieve.
    ///
    /// # Returns
    /// A JSON response containing a paged list of `AdminUser` objects.
    #[oai(path = "/admin/user/list/:page", method = "get", tag = "ApiTags::User")]
    pub async fn admin_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ApiUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (users, page_count) = User::list(&mut conn, page, PAGE_SIZE)?;
        let addresses = users
            .iter()
            .map(|user| user.account_address())
            .map(Address::Account)
            .collect::<Vec<_>>();
        let registered = Identity::exists_batch(
            &mut conn,
            contracts.identity_registry_contract_index,
            &addresses,
        )?
        .into_iter()
        .collect::<BTreeSet<_>>();
        let data = users
            .into_iter()
            .map(|user| ApiUser {
                kyc_verified: registered.contains(&Address::Account(user.account_address())),
                is_admin: false,
                user,
            })
            .collect();
        Ok(Json(PagedResponse {
            data,
            page,
            page_count,
        }))
    }

    #[oai(
        path = "/user/investments/list/:page",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn investments(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<PagedResponse<ForestProjectFundsInvestmentRecord>> {
        let mut conn = db_pool.get()?;
        let (users, page_count) = ForestProjectFundsInvestmentRecord::list(
            &mut conn,
            contracts.mint_funds_contract_index,
            &claims.sub,
            page,
            PAGE_SIZE,
        )?;

        Ok(Json(PagedResponse {
            data: users,
            page,
            page_count,
        }))
    }

    #[oai(
        path = "/user/affiliate/rewards/list/:page",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn user_affiliate_rewards_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectFundsAffiliateRewardRecord>> {
        let mut conn = db_pool.get()?;
        let (users, page_count) =
            ForestProjectFundsAffiliateRewardRecord::list(&mut conn, &claims.sub, page, PAGE_SIZE)
                .map_err(|e| {
                    error!("Error listing user affiliate rewards: {:?}", e);
                    e
                })?;

        Ok(Json(PagedResponse {
            data: users,
            page,
            page_count,
        }))
    }

    #[oai(
        path = "/user/affiliate/rewards/claim/:investment_record_id",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn user_affiliate_rewards_claim(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        Data(config): Data<&OffchainRewardsConfig>,
        Path(investment_record_id): Path<uuid::Uuid>,
    ) -> JsonResult<ClaimRequest> {
        let account = ensure_account_registered(&claims)?;
        let mut conn = db_pool.get()?;
        let reward =
            ForestProjectFundsAffiliateRewardRecord::find(&mut conn, investment_record_id)?
                .ok_or_else(|| Error::NotFound(PlainText("Reward not found".to_string())))?;
        let remaining_reward_amount = reward.remaining_reward_amount;
        if remaining_reward_amount.is_zero() {
            return Err(Error::BadRequest(PlainText(
                "Reward already claimed".to_string(),
            )));
        }
        let nonce = OffchainRewardee::find(
            &mut conn,
            contracts.offchain_rewards_contract_index,
            &account.to_string(),
        )?
        .map(|r| r.nonce)
        .unwrap_or(Decimal::ZERO);
        let claim = ClaimInfo {
            account:               account.to_string(),
            account_nonce:         nonce.to_u64().unwrap(),
            contract_address:      contracts.offchain_rewards_contract_index,
            reward_id:             reward.investment_record_id.as_bytes().to_vec(),
            reward_amount:         remaining_reward_amount,
            reward_token_id:       "".to_string(),
            reward_token_contract: contracts.euro_e_contract_index,
        };
        let signature = hash_and_sign(&claim, &config.agent)?;
        let signature = serde_json::to_value(signature).map_err(|_| {
            Error::InternalServer(PlainText("Failed to serialize signature".to_string()))
        })?;
        Ok(Json(ClaimRequest {
            claim,
            signer: config.agent.address().to_string(),
            signature,
        }))
    }

    #[oai(
        path = "/user/transactions/list/:page",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn user_transactions_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<UserTransaction>> {
        let mut conn = db_pool.get()?;
        let (users, page_count) =
            UserTransaction::list_by_cognito_user_id(&mut conn, &claims.sub, page, PAGE_SIZE)
                .map_err(|e| {
                    error!("Error listing user transactions: {:?}", e);
                    e
                })?;

        Ok(Json(PagedResponse {
            data: users,
            page,
            page_count,
        }))
    }

    #[oai(path = "/system_config", method = "get", tag = "ApiTags::User")]
    pub async fn system_config(
        &self,
        Data(contracts): Data<&SystemContractsConfig>,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<SystemContractsConfigApiModel> {
        let mut conn = db_pool.get()?;
        let mint_funds_contract =
            SecurityMintFundContract::find(&mut conn, contracts.mint_funds_contract_index)?;
        let trading_contract = P2PTradeContract::find(&mut conn, contracts.trading_contract_index)?;

        Ok(Json(SystemContractsConfigApiModel {
            identity_registry_contract_index: contracts.identity_registry_contract_index,
            compliance_contract_index: contracts.compliance_contract_index,
            carbon_credit_contract_index: contracts.carbon_credit_contract_index,
            carbon_credit_token_id: contracts.carbon_credit_token_id,
            euro_e_contract_index: contracts.euro_e_contract_index,
            euro_e_token_id: contracts.euro_e_token_id,
            tree_ft_contract_index: contracts.tree_ft_contract_index,
            tree_nft_contract_index: contracts.tree_nft_contract_index,
            offchain_rewards_contract_index: contracts.offchain_rewards_contract_index,
            mint_funds_contract_index: contracts.mint_funds_contract_index,
            trading_contract_index: contracts.trading_contract_index,
            yielder_contract_index: contracts.yielder_contract_index,
            mint_funds_contract,
            trading_contract,
        }))
    }
}

#[derive(Object, Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemContractsConfigApiModel {
    pub identity_registry_contract_index: Decimal,
    pub compliance_contract_index:        Decimal,
    pub carbon_credit_contract_index:     Decimal,
    pub carbon_credit_token_id:           Decimal,
    pub euro_e_contract_index:            Decimal,
    pub euro_e_token_id:                  Decimal,
    pub tree_ft_contract_index:           Decimal,
    pub tree_nft_contract_index:          Decimal,
    pub offchain_rewards_contract_index:  Decimal,
    pub mint_funds_contract_index:        Decimal,
    pub trading_contract_index:           Decimal,
    pub yielder_contract_index:           Decimal,
    pub mint_funds_contract:              Option<SecurityMintFundContract>,
    pub trading_contract:                 Option<P2PTradeContract>,
}

#[derive(Object, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClaimRequest {
    pub claim:     ClaimInfo,
    pub signer:    String,
    /// Json serialized `AccountSignatures`
    pub signature: serde_json::Value,
}

#[derive(Object, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClaimInfo {
    pub contract_address:      Decimal,
    pub account:               String,
    pub account_nonce:         u64,
    pub reward_id:             Vec<u8>,
    pub reward_token_id:       String,
    pub reward_token_contract: Decimal,
    pub reward_amount:         Decimal,
}

impl ClaimInfo {
    pub fn hash<T>(&self, hasher: T) -> std::result::Result<[u8; 32], HashError>
    where T: FnOnce(Vec<u8>) -> [u8; 32] {
        let internal = ::offchain_rewards::types::ClaimInfo {
            contract_address:      ContractAddress::new(
                self.contract_address
                    .to_u64()
                    .expect("unable to convert contract address to u64"),
                0,
            ),
            account:               self.account.parse().map_err(|_| HashError::AccountParse)?,
            account_nonce:         self.account_nonce,
            reward_id:             self.reward_id.clone(),
            reward_amount:         TokenAmountU64(
                self.reward_amount
                    .to_u64()
                    .expect("unable to convert reward amount to u64"),
            ),
            reward_token_id:       TokenIdUnit(),
            reward_token_contract: ContractAddress::new(
                self.reward_token_contract
                    .to_u64()
                    .expect("unable to convert reward token contract to u64"),
                0,
            ),
        };
        let hash = internal.hash(hasher).map_err(|_| HashError::Hash)?;
        Ok(hash)
    }
}

fn hash_and_sign(claim: &ClaimInfo, agent: &OffchainRewardsAgent) -> Result<AccountSignatures> {
    let hash = claim.hash(hasher)?;
    let signature = agent.sign(&hash);
    Ok(signature)
}

pub struct OffchainRewardsAgent(pub WalletAccount);
impl Signer for OffchainRewardsAgent {
    fn wallet(&self) -> &WalletAccount { &self.0 }
}

#[derive(Debug)]
pub enum HashError {
    ContractParse,
    AccountParse,
    MetadataUrlHexDecode,
    Hash,
}

impl From<HashError> for Error {
    fn from(val: HashError) -> Self {
        Error::InternalServer(PlainText(format!("Hash Error: {val:?}")))
    }
}

#[derive(Clone)]
pub struct OffchainRewardsConfig {
    pub agent: Arc<OffchainRewardsAgent>,
}

#[derive(Object, Serialize, Deserialize, PartialEq, Debug)]
pub struct ApiUser {
    pub user:         User,
    pub is_admin:     bool,
    pub kyc_verified: bool,
}

impl ApiUser {
    pub fn new(db_user: User, is_admin: bool, kyc_verified: bool) -> Self {
        Self {
            user: db_user,
            is_admin,
            kyc_verified,
        }
    }
}

#[derive(Serialize, Object)]
pub struct UserUpdateAccountAddressRequest {
    pub account_address:      String,
    pub affiliate_commission: Decimal,
}

impl UserUpdateAccountAddressRequest {
    pub fn account_address(&self) -> Result<AccountAddress> {
        self.account_address
            .parse()
            .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))
    }
}

#[derive(Serialize, Object)]
pub struct UserRegisterReq {
    pub desired_investment_amount: i32,
}

#[derive(Serialize, Object)]
pub struct UserRegistrationRequestApi {
    pub email:                     String,
    pub affiliate_account_address: Option<String>,
}

impl UserRegistrationRequestApi {
    pub fn affiliate_account_address(&self) -> Result<Option<AccountAddress>> {
        self.affiliate_account_address
            .as_ref()
            .map(|a| a.parse())
            .transpose()
            .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))
    }
}

async fn verify_presentation(
    concordium_client: &mut v2::Client,
    proof: Presentation,
    account_address: AccountAddress,
    network: &web3id::did::Network,
    global_context: &concordium::identity::GlobalContext,
    challenge: [u8; 32],
) -> Result<VerifyPresentationResponse> {
    let mut cred_ids = proof
        .verifiable_credential
        .iter()
        .filter_map(|c| match c.metadata().cred_metadata {
            CredentialMetadata::Account { cred_id, .. } => Some(cred_id),
            CredentialMetadata::Web3Id { .. } => None,
        })
        .collect::<Vec<_>>();
    cred_ids.dedup_by(|a, b| (*a).eq(b));
    for account_id in cred_ids {
        let account_info = concordium_client
            .get_account_info(&account_id.into(), BlockIdentifier::LastFinal)
            .await?
            .response
            .account_address
            .eq(&account_address);
        if !account_info {
            return Err(Error::BadRequest(PlainText(
                "Invalid proof credentials".to_string(),
            )));
        }
    }
    let res = concordium::identity::verify_presentation(
        *network,
        concordium_client,
        global_context,
        &proof,
        challenge,
    )
    .await
    .map_err(|_| Error::BadRequest(PlainText("Invalid proof".to_string())))?;

    Ok(res)
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct UserRegisterGetRes {
    pub id_statement: serde_json::Value,
    /// The hex of the challenge
    pub challenge:    String,
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct UserCreatePostReq {
    pub account_address:           String,
    pub desired_investment_amount: Option<i32>,
    pub proof:                     Option<serde_json::Value>,
    pub password:                  String,
    pub affiliate_commission:      Option<Decimal>,
}

impl UserCreatePostReq {
    pub fn proof(&self) -> Result<Option<concordium::identity::Presentation>> {
        let proof = self.proof.clone().map_or(Ok(None), |p| {
            serde_json::from_value(p)
                .map_err(|_| Error::BadRequest(PlainText("Invalid proof".to_string())))
                .map(Some)
        })?;
        Ok(proof)
    }
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct UserCreatePostReqAdmin {
    pub email:                     String,
    pub account_address:           String,
    pub desired_investment_amount: Option<i32>,
    pub password:                  String,
    pub first_name:                String,
    pub last_name:                 String,
    pub nationality:               String,
    pub affiliate_commission:      Option<Decimal>,
    pub affiliate_account_address: Option<String>,
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct LoginReq {
    pub email:    String,
    pub password: String,
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct LoginRes {
    pub id_token:  String,
    pub user:      ApiUser,
    pub contracts: SystemContractsConfig,
}
