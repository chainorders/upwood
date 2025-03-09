use aws::cognito::{
    account_address_attribute, affiliate_account_address_attribute, email_attribute,
    email_verified_attribute, first_name_attribute, last_name_attribute, nationality_attribute,
};
use chrono::Utc;
use concordium::account::Signer;
use concordium::identity::{Presentation, VerifyPresentationResponse};
use concordium_cis2::{TokenAmountU64, TokenIdUnit};
use concordium_rust_sdk::base::contracts_common::AccountSignatures;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::v2::BlockIdentifier;
use concordium_rust_sdk::web3id::CredentialMetadata;
use concordium_rust_sdk::{v2, web3id};
use diesel::Connection;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Attachment, AttachmentType, Json, PlainText};
use poem_openapi::{Object, OpenApi};
use serde::Serialize;
use shared::api::PagedResponse;
use shared::db::identity_registry::Identity;
use shared::db::offchain_rewards::OffchainRewardee;
use shared::db::security_mint_fund::SecurityMintFundContract;
use shared::db_app::forest_project::Notification;
use shared::db_app::forest_project_crypto::{
    ForestProjectFundsAffiliateRewardRecord, ForestProjectFundsInvestmentRecord, TokenMetadata,
};
use shared::db_app::portfolio::UserTransaction;
use shared::db_app::users::{
    Company, CompanyInvitation, User, UserKYCModel, UserRegistrationRequest, UserTokenHolder,
};
use uuid::Uuid;

use crate::api::*;
use crate::utils::aws::cognito::company_id_attribute;
use crate::utils::aws::ses::Emailer;
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
        path = "/admin/registration-request/list",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn admin_registration_request_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<UserRegistrationRequest>> {
        ensure_is_admin(&claims)?;
        let page = page.unwrap_or(0);
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let mut conn = db_pool.get()?;
        let (requests, page_count) = UserRegistrationRequest::list(&mut conn, page, page_size)?;
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
            admin_create_temp_user(
                user_pool,
                &request.email,
                request.affiliate_account_address.as_deref(),
            )
            .await?;
        }

        UserRegistrationRequest::delete(&mut conn, id)?;
        Ok(())
    }

    /// Registers a user in the Cognito user pool and in the database.
    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/user/register", method = "post", tag = "ApiTags::User")]
    pub async fn post_user_register(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        Data(global_context): Data<&concordium::identity::GlobalContext>,
        Data(network): Data<&web3id::did::Network>,
        Data(concordium_client): Data<&v2::Client>,
        Data(affiliate_commission): Data<&AffiliateCommission>,
        Data(contracts): Data<&SystemContractsConfig>,
        Json(req): Json<UserCreatePostReq>,
    ) -> JsonResult<UserKYCModel> {
        let mut conn = db_pool.get()?;
        let verification_res = {
            let proof = req
                .proof()?
                .ok_or_else(|| Error::BadRequest(PlainText("Proof not provided".to_string())))?;
            let account_address = req
                .account_address
                .parse()
                .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))?;
            let challenge = concordium::identity::generate_challenge(&req.email);

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
        let cognito_user = user_pool
            .confirm_user(&req.email, &req.temp_password, &req.password, vec![
                email_verified_attribute(true),
                account_address_attribute(&req.account_address),
                first_name_attribute(&verification_res.first_name),
                last_name_attribute(&verification_res.last_name),
                nationality_attribute(&verification_res.nationality),
            ])
            .await
            .map_err(|e| {
                error!("Error confirming in user pool: {:?}", e);
                Error::InternalServer(PlainText(
                    "Failed to confirming user in user pool".to_string(),
                ))
            })?;
        let cognito_user_id = cognito_user
            .user_attributes()
            .iter()
            .find(|a| a.name == "sub")
            .expect("Cognito user ID not found")
            .value
            .clone()
            .expect("Cognito user ID not found");
        let affiliate_account = cognito_user
            .user_attributes()
            .iter()
            .find(|a| a.name == "custom:affiliate_con_accnt")
            .and_then(|a| a.value.clone());
        let user = User {
            account_address: req.account_address.to_string(),
            cognito_user_id,
            email: req.email.clone(),
            first_name: verification_res.first_name,
            last_name: verification_res.last_name,
            nationality: verification_res.nationality,
            affiliate_commission: affiliate_commission.commission,
            desired_investment_amount: req.desired_investment_amount,
            affiliate_account_address: affiliate_account.clone(),
            company_id: None,
        }
        .upsert(&mut conn)?;
        let kyc_verified = Identity::exists(
            &mut conn,
            contracts.identity_registry_contract_index,
            &user.account_address,
        )?;
        Ok(Json(UserKYCModel {
            account_address: user.account_address,
            cognito_user_id: user.cognito_user_id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            nationality: user.nationality,
            affiliate_account_address: user.affiliate_account_address,
            affiliate_commission: user.affiliate_commission,
            desired_investment_amount: user.desired_investment_amount,
            kyc_verified,
        }))
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
    ) -> JsonResult<UserKYCModel> {
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
            company_id:                None,
        }
        .upsert(&mut conn)?;
        let kyc_verified = Identity::exists(
            &mut conn,
            contracts.identity_registry_contract_index,
            &user.account_address,
        )?;

        Ok(Json(UserKYCModel::new(user, kyc_verified)))
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
    ) -> JsonResult<UserKYCModel> {
        let mut conn = db_pool.get()?;
        let user = UserKYCModel::find(
            &mut conn,
            contracts.identity_registry_contract_index,
            &claims.sub,
        )?;
        let user = match user {
            Some(user) => user,
            None => {
                let account_address = claims.account.unwrap_or_default();
                let kyc_verified = Identity::exists(
                    &mut conn,
                    contracts.identity_registry_contract_index,
                    &account_address,
                )
                .map_err(|_| {
                    Error::InternalServer(PlainText("Failed to check KYC status".to_string()))
                })?;

                UserKYCModel {
                    account_address,
                    cognito_user_id: claims.sub.clone(),
                    email: claims.email,
                    first_name: claims.first_name.unwrap_or_default(),
                    last_name: claims.last_name.unwrap_or_default(),
                    nationality: claims.nationality.unwrap_or_default(),
                    affiliate_commission: Decimal::ZERO,
                    desired_investment_amount: None,
                    affiliate_account_address: claims.affiliate_account.clone(),
                    kyc_verified,
                }
            }
        };

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
    #[oai(path = "/admin/user/list", method = "get", tag = "ApiTags::User")]
    pub async fn admin_user_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<UserKYCModel>> {
        ensure_is_admin(&claims)?;
        let page = page.unwrap_or(0);
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let conn = &mut db_pool.get()?;
        let (users, page_count) = UserKYCModel::list(
            conn,
            contracts.identity_registry_contract_index,
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: users,
            page,
            page_count,
        }))
    }

    #[oai(path = "/admin/holder/:token_id/:contract/list", method = "get", tag = "ApiTags::User")]
    pub async fn admin_holder_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(token_id): Path<Decimal>,
        Path(contract): Path<Decimal>,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<UserTokenHolder>> {
        ensure_is_admin(&claims)?;
        let page = page.unwrap_or(0);
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let conn = &mut db_pool.get()?;
        let (users, page_count) = UserTokenHolder::list(
            conn,
            token_id,
            contract,
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: users,
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
        path = "/user/affiliate/rewards/list",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn user_affiliate_rewards_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<ForestProjectFundsAffiliateRewardRecord>> {
        let page = page.unwrap_or(0);
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
        path = "/user/transactions/list",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn user_transactions_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<UserTransaction>> {
        let page = page.unwrap_or(0);
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

    #[oai(
        path = "/user/transactions/list/download",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn user_transactions_list_download(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> Result<Attachment<Vec<u8>>> {
        let mut conn = db_pool.get()?;
        let (transactions, _) =
            UserTransaction::list_by_cognito_user_id(&mut conn, &claims.sub, 0, i64::MAX).map_err(
                |e| {
                    error!("Error listing user transactions: {:?}", e);
                    e
                },
            )?;

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record(["Transaction Hash", "Type", "Sender", "Amount"])
            .map_err(|e| {
                error!("Failed to write csv header: {}", e);
                Error::InternalServer(PlainText(format!("Failed to write csv header: {}", e)))
            })?;

        for transaction in transactions {
            wtr.write_record(&[
                transaction.transaction_hash.to_string(),
                transaction.transaction_type.to_string(),
                transaction.account_address.to_string(),
                transaction.currency_amount.to_string(),
            ])
            .map_err(|e| {
                error!("Failed to write csv record: {}", e);
                Error::InternalServer(PlainText(format!("Failed to write csv record: {}", e)))
            })?;
        }

        let data = wtr.into_inner().map_err(|e| {
            error!("Failed to write csv: {}", e);
            Error::InternalServer(PlainText(format!("Failed to write csv: {}", e)))
        })?;

        Ok(Attachment::new(data)
            .attachment_type(AttachmentType::Attachment)
            .filename(
                format!("user_transactions_{}.csv", chrono::Utc::now().timestamp()).to_string(),
            ))
    }

    #[oai(
        path = "/user/affiliate/rewards/list/download",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn user_affiliate_rewards_list_download(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> Result<Attachment<Vec<u8>>> {
        let mut conn = db_pool.get()?;
        let (rewards, _) =
            ForestProjectFundsAffiliateRewardRecord::list(&mut conn, &claims.sub, 0, i64::MAX)
                .map_err(|e| {
                    error!("Error listing user affiliate rewards: {:?}", e);
                    e
                })?;

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "Investment Record ID",
            "Investor Account Address",
            "Reward Amount",
            "Affiliate Commission",
            "Currency Amount",
            "Remaining Reward Amount",
        ])
        .map_err(|e| {
            error!("Failed to write csv header: {}", e);
            Error::InternalServer(PlainText(format!("Failed to write csv header: {}", e)))
        })?;

        for reward in rewards {
            wtr.write_record(&[
                reward.investment_record_id.to_string(),
                reward.investor_account_address.to_string(),
                reward.reward_amount.to_string(),
                reward.affiliate_commission.to_string(),
                reward.currency_amount.to_string(),
                reward.remaining_reward_amount.to_string(),
            ])
            .map_err(|e| {
                error!("Failed to write csv record: {}", e);
                Error::InternalServer(PlainText(format!("Failed to write csv record: {}", e)))
            })?;
        }

        let data = wtr.into_inner().map_err(|e| {
            error!("Failed to write csv: {}", e);
            Error::InternalServer(PlainText(format!("Failed to write csv: {}", e)))
        })?;

        Ok(Attachment::new(data)
            .attachment_type(AttachmentType::Attachment)
            .filename(
                format!(
                    "user_affiliate_rewards_{}.csv",
                    chrono::Utc::now().timestamp()
                )
                .to_string(),
            ))
    }

    #[oai(path = "/system_config", method = "get", tag = "ApiTags::User")]
    pub async fn system_config(
        &self,
        Data(contracts): Data<&SystemContractsConfig>,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<SystemContractsConfigApiModel> {
        let conn = &mut db_pool.get()?;
        let mint_funds_contract =
            SecurityMintFundContract::find(conn, contracts.mint_funds_contract_index)?.unwrap_or(
                SecurityMintFundContract {
                    contract_address:                contracts.mint_funds_contract_index,
                    create_time:                     Utc::now().naive_utc(),
                    currency_token_id:               contracts.euro_e_token_id,
                    currency_token_contract_address: contracts.euro_e_contract_index,
                },
            );
        let trading_contract = P2PTradeContract::find(conn, contracts.trading_contract_index)?
            .unwrap_or(P2PTradeContract {
                contract_address:                contracts.trading_contract_index,
                create_time:                     Utc::now().naive_utc(),
                currency_token_id:               contracts.euro_e_token_id,
                currency_token_contract_address: contracts.euro_e_contract_index,
                total_sell_currency_amount:      Decimal::ZERO,
            });

        let euro_e_metdata = TokenMetadata::find(
            conn,
            contracts.euro_e_contract_index,
            contracts.euro_e_token_id,
        )?
        .unwrap_or(TokenMetadata {
            contract_address: contracts.euro_e_contract_index,
            token_id:         contracts.euro_e_token_id,
            decimals:         Some(6),
            symbol:           Some("€".to_string()),
        });

        let carbon_credit_metadata = TokenMetadata::find(
            conn,
            contracts.carbon_credit_contract_index,
            contracts.carbon_credit_token_id,
        )?
        .unwrap_or(TokenMetadata {
            contract_address: contracts.carbon_credit_contract_index,
            token_id:         contracts.carbon_credit_token_id,
            decimals:         Some(0),
            symbol:           Some("CC".to_string()),
        });

        let tree_ft_metadata = TokenMetadata::find(
            conn,
            contracts.tree_ft_contract_index,
            contracts.tree_nft_contract_index,
        )?
        .unwrap_or(TokenMetadata {
            contract_address: contracts.tree_ft_contract_index,
            token_id:         contracts.tree_nft_contract_index,
            decimals:         Some(0),
            symbol:           Some("ETrees".to_string()),
        });

        Ok(Json(SystemContractsConfigApiModel {
            identity_registry_contract_index: contracts.identity_registry_contract_index,
            compliance_contract_index: contracts.compliance_contract_index,
            carbon_credit_contract_index: contracts.carbon_credit_contract_index,
            carbon_credit_token_id: contracts.carbon_credit_token_id,
            carbon_credit_metadata,
            euro_e_contract_index: contracts.euro_e_contract_index,
            euro_e_token_id: contracts.euro_e_token_id,
            euro_e_metadata: euro_e_metdata,
            tree_ft_contract_index: contracts.tree_ft_contract_index,
            tree_ft_metadata,
            tree_nft_contract_index: contracts.tree_nft_contract_index,
            offchain_rewards_contract_index: contracts.offchain_rewards_contract_index,
            mint_funds_contract_index: contracts.mint_funds_contract_index,
            trading_contract_index: contracts.trading_contract_index,
            yielder_contract_index: contracts.yielder_contract_index,
            mint_funds_contract,
            trading_contract,
        }))
    }

    #[oai(path = "/user/notifications", method = "post", tag = "ApiTags::User")]
    pub async fn user_notifications_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(project_id): Json<Uuid>,
    ) -> JsonResult<Notification> {
        let conn = &mut db_pool.get()?;
        let now = Utc::now().naive_utc();
        let notification = Notification::find(conn, project_id, &claims.sub)?;
        let notification = match notification {
            Some(notification) => notification,
            None => Notification {
                cognito_user_id: claims.sub.clone(),
                id: Uuid::new_v4(),
                project_id,
                created_at: now,
                updated_at: now,
            }
            .insert(conn)
            .map_err(|e| {
                error!("Error creating notification: {:?}", e);
                Error::InternalServer(PlainText("Failed to create notification".to_string()))
            })?,
        };
        Ok(Json(notification))
    }

    #[oai(path = "/company", method = "get", tag = "ApiTags::User")]
    pub async fn user_company_get(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<Option<Company>> {
        let conn = &mut db_pool.get()?;
        let company_id = match claims.company_id {
            Some(company_id) => company_id,
            None => return Ok(Json(None)),
        };
        let company = Company::find(conn, company_id)?;
        Ok(Json(company))
    }

    #[oai(path = "/company", method = "post", tag = "ApiTags::User")]
    pub async fn user_company_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Json(req): Json<UserCompanyCreateUpdateReq>,
    ) -> JsonResult<Company> {
        let conn = &mut db_pool.get()?;
        let company = conn.transaction(|conn| {
            let company = Company {
                id:                   Uuid::new_v4(),
                name:                 req.name.clone(),
                registration_address: Some(req.registration_address.clone()),
                vat_no:               Some(req.vat_no.clone()),
                country:              Some(req.country.clone()),
                profile_picture_url:  Some(req.profile_picture_url.clone()),
                created_at:           Utc::now().naive_utc(),
                updated_at:           Utc::now().naive_utc(),
            }
            .insert(conn)
            .map_err(|e| {
                error!("Error creating company: {:?}", e);
                Error::InternalServer(PlainText("Failed to create company".to_string()))
            })?;
            User::update_company_id(conn, &claims.sub, Some(company.id)).map_err(|e| {
                error!("Error updating user: {:?}", e);
                Error::InternalServer(PlainText("Failed to update user".to_string()))
            })?;
            Result::Ok(company)
        })?;
        user_pool
            .admin_update_user_attributes(&claims.sub, vec![company_id_attribute(Some(
                &company.id,
            ))])
            .await
            .map_err(|e| {
                error!("Error updating user in user pool: {:?}", e);
                Error::InternalServer(PlainText("Failed to update user in user pool".to_string()))
            })?;
        Ok(Json(company))
    }

    #[oai(path = "/company", method = "put", tag = "ApiTags::User")]
    pub async fn user_company_update(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(req): Json<UserCompanyCreateUpdateReq>,
    ) -> JsonResult<Company> {
        let conn = &mut db_pool.get()?;
        let company = Company::find(conn, claims.company_id.unwrap_or_default())?
            .ok_or_else(|| Error::NotFound(PlainText("Company not found".to_string())))?;
        let company = Company {
            id:                   company.id,
            name:                 req.name.clone(),
            registration_address: Some(req.registration_address.clone()),
            vat_no:               Some(req.vat_no.clone()),
            country:              Some(req.country.clone()),
            profile_picture_url:  Some(req.profile_picture_url.clone()),
            created_at:           company.created_at,
            updated_at:           Utc::now().naive_utc(),
        }
        .update(conn)
        .map_err(|e| {
            error!("Error updating company: {:?}", e);
            Error::InternalServer(PlainText("Failed to update company".to_string()))
        })?;
        Ok(Json(company))
    }

    #[oai(path = "/company/invitation", method = "post", tag = "ApiTags::User")]
    pub async fn user_company_invitation_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(emailer): Data<&Emailer>,
        Data(db_pool): Data<&DbPool>,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Json(req): Json<UserCompanyInvitationCreateReq>,
    ) -> JsonResult<CompanyInvitation> {
        let company_id = claims.company_id.ok_or_else(|| {
            Error::BadRequest(PlainText(
                "User is not associated with a company".to_string(),
            ))
        })?;
        let conn = &mut db_pool.get()?;
        let company = Company::find(conn, company_id)?
            .ok_or_else(|| Error::BadRequest(PlainText("Company not found".to_string())))?;
        if CompanyInvitation::find_by_email(conn, company_id, &req.email)?.is_some() {
            return Err(Error::BadRequest(PlainText(
                "Invitation already exists".to_string(),
            )));
        }
        let user = user_pool
            .find_user_by_email(&req.email)
            .await
            .map_err(|e| {
                error!("Error finding user in user pool: {:?}", e);
                Error::InternalServer(PlainText("Failed to find user in user pool".to_string()))
            })?;
        if user.is_none() {
            admin_create_temp_user(user_pool, &req.email, None).await?;
        }
        let invitation_id = Uuid::new_v4();
        emailer
            .send_company_invitation_email(&invitation_id, &req.email, &claims.email, &company)
            .await
            .map_err(|e| {
                error!("Error sending company invitation email: {:?}", e);
                Error::InternalServer(PlainText(
                    "Failed to send company invitation email".to_string(),
                ))
            })?;
        let invitation = CompanyInvitation {
            id: invitation_id,
            company_id,
            email: req.email.clone(),
            created_at: Utc::now().naive_utc(),
            created_by: claims.sub.to_string(),
        }
        .insert(conn)
        .map_err(|e| {
            error!("Error creating company invitation: {:?}", e);
            Error::InternalServer(PlainText("Failed to create company invitation".to_string()))
        })?;
        JsonResult::Ok(Json(invitation))
    }

    #[oai(path = "/company/invitation", method = "put", tag = "ApiTags::User")]
    pub async fn user_company_invitation_update(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Query(invitation_id): Query<Uuid>,
        Query(accepted): Query<bool>,
    ) -> NoResResult {
        let conn = &mut db_pool.get()?;
        let invitation = CompanyInvitation::find(conn, invitation_id)?
            .ok_or_else(|| Error::BadRequest(PlainText("Invitation not found".to_string())))?;
        if invitation.email != claims.email {
            return Err(Error::UnAuthorized(PlainText("Invalid email".to_string())));
        }
        let company = Company::find(conn, invitation.company_id)?
            .ok_or_else(|| Error::BadRequest(PlainText("Company not found".to_string())))?;
        if accepted {
            user_pool
                .admin_update_user_attributes(&claims.sub, vec![company_id_attribute(Some(
                    &company.id,
                ))])
                .await
                .map_err(|e| {
                    error!("Error updating user in user pool: {:?}", e);
                    Error::InternalServer(PlainText(
                        "Failed to update user in user pool".to_string(),
                    ))
                })?;
            User::update_company_id(conn, &claims.sub, Some(company.id)).map_err(|e| {
                error!("Error updating user: {:?}", e);
                Error::InternalServer(PlainText("Failed to update user".to_string()))
            })?;
        }
        CompanyInvitation::delete(conn, invitation_id).map_err(|e| {
            error!("Error deleting company invitation: {:?}", e);
            Error::InternalServer(PlainText("Failed to delete company invitation".to_string()))
        })?;
        Ok(())
    }

    #[oai(path = "/company/invitation", method = "delete", tag = "ApiTags::User")]
    pub async fn user_company_invitation_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(invitation_id): Query<Uuid>,
    ) -> NoResResult {
        let conn = &mut db_pool.get()?;
        let invitation = CompanyInvitation::find(conn, invitation_id)?
            .ok_or_else(|| Error::BadRequest(PlainText("Invitation not found".to_string())))?;
        if claims.company_id != Some(invitation.company_id) {
            return Err(Error::UnAuthorized(PlainText(
                "Invalid company".to_string(),
            )));
        }
        CompanyInvitation::delete(conn, invitation_id).map_err(|e| {
            error!("Error deleting company invitation: {:?}", e);
            Error::InternalServer(PlainText("Failed to delete company invitation".to_string()))
        })?;
        Ok(())
    }

    #[oai(
        path = "/company/invitation/list",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn user_company_invitation_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<PagedResponse<CompanyInvitation>> {
        let conn = &mut db_pool.get()?;
        let company_id = claims.company_id.ok_or_else(|| {
            Error::BadRequest(PlainText(
                "User is not associated with a company".to_string(),
            ))
        })?;
        let (invitations, page_count) =
            CompanyInvitation::list_by_company_id(conn, company_id, 0, PAGE_SIZE)?;
        Ok(Json(PagedResponse {
            data: invitations,
            page: 0,
            page_count,
        }))
    }

    #[oai(path = "/company/members/list", method = "get", tag = "ApiTags::User")]
    pub async fn user_company_users_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<PagedResponse<UserKYCModel>> {
        let conn = &mut db_pool.get()?;
        let company_id = claims.company_id.ok_or_else(|| {
            Error::BadRequest(PlainText(
                "User is not associated with a company".to_string(),
            ))
        })?;
        let (users, page_count) = UserKYCModel::list_by_company_id(
            conn,
            contracts.identity_registry_contract_index,
            company_id,
            0,
            PAGE_SIZE,
        )?;
        Ok(Json(PagedResponse {
            data: users,
            page: 0,
            page_count,
        }))
    }

    #[oai(path = "/company/members", method = "delete", tag = "ApiTags::User")]
    pub async fn user_company_users_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Query(user_id): Query<String>,
    ) -> NoResResult {
        let conn = &mut db_pool.get()?;
        let company_id = claims.company_id.ok_or_else(|| {
            Error::BadRequest(PlainText(
                "User is not associated with a company".to_string(),
            ))
        })?;
        let user = User::find(conn, &user_id)?
            .ok_or_else(|| Error::BadRequest(PlainText("User not found".to_string())))?;
        if user.company_id != Some(company_id) {
            return Err(Error::BadRequest(PlainText(
                "User not in company".to_string(),
            )));
        }
        user_pool
            .admin_update_user_attributes(&user.cognito_user_id, vec![company_id_attribute(None)])
            .await
            .map_err(|e| {
                error!("Error updating user in user pool: {:?}", e);
                Error::InternalServer(PlainText("Failed to update user in user pool".to_string()))
            })?;
        User::update_company_id(conn, &user_id, None).map_err(|e| {
            error!("Error updating user: {:?}", e);
            Error::InternalServer(PlainText("Failed to update user".to_string()))
        })?;
        Ok(())
    }
}

#[derive(Object, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserCompanyCreateUpdateReq {
    pub name:                 String,
    pub registration_address: String,
    pub vat_no:               String,
    pub country:              String,
    pub profile_picture_url:  String,
}

#[derive(Object, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserCompanyInvitationCreateReq {
    pub email: String,
}

#[derive(Object, Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemContractsConfigApiModel {
    pub identity_registry_contract_index: Decimal,
    pub compliance_contract_index:        Decimal,
    pub carbon_credit_contract_index:     Decimal,
    pub carbon_credit_token_id:           Decimal,
    pub carbon_credit_metadata:           TokenMetadata,
    pub euro_e_contract_index:            Decimal,
    pub euro_e_token_id:                  Decimal,
    pub euro_e_metadata:                  TokenMetadata,
    pub tree_ft_contract_index:           Decimal,
    pub tree_ft_metadata:                 TokenMetadata,
    pub tree_nft_contract_index:          Decimal,
    pub offchain_rewards_contract_index:  Decimal,
    pub mint_funds_contract_index:        Decimal,
    pub trading_contract_index:           Decimal,
    pub yielder_contract_index:           Decimal,
    pub mint_funds_contract:              SecurityMintFundContract,
    pub trading_contract:                 P2PTradeContract,
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
pub struct UserCreatePostReq {
    pub email:                     String,
    pub temp_password:             String,
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

/// Adds the user to the Cognito user pool and triggers an invitation email to the user.
async fn admin_create_temp_user(
    user_pool: &aws::cognito::UserPool,
    email: &str,
    affiliate_account_address: Option<&str>,
) -> Result<()> {
    let mut attrs = vec![email_attribute(email), email_verified_attribute(false)];
    if let Some(affiliate_account_address) = affiliate_account_address {
        attrs.push(affiliate_account_address_attribute(
            affiliate_account_address,
        ));
    }
    user_pool
        .admin_create_temp_user(email, attrs)
        .await
        .map_err(|e| {
            error!("Error creating user in user pool: {:?}", e);
            e
        })?;
    Ok(())
}
