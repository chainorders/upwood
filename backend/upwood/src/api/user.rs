use std::collections::BTreeSet;

use chrono::Utc;
use concordium::account::Signer;
use concordium::identity::Presentation;
use concordium_cis2::{TokenAmountU64, TokenIdUnit};
use concordium_rust_sdk::base::contracts_common::AccountSignatures;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::Address;
use concordium_rust_sdk::v2::BlockIdentifier;
use concordium_rust_sdk::web3id::CredentialMetadata;
use concordium_rust_sdk::{v2, web3id};
use diesel::Connection;
use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use serde::Serialize;
use shared::api::PagedResponse;
use shared::db::identity_registry::Identity;
use shared::db::offchain_rewards::OffchainRewardee;
use shared::db_app::forest_project::UserTransaction;
use shared::db_app::user_challenges::UserChallenge;
use shared::db_app::users::{AffiliateReward, User, UserAffiliate};
use tracing::info;

use crate::api::*;
use crate::utils::*;

#[derive(Clone, Copy)]
pub struct UserApi;

#[OpenApi]
impl UserApi {
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
    #[oai(path = "/users", method = "get", tag = "ApiTags::User")]
    pub async fn user_self(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
    ) -> JsonResult<ApiUser> {
        let mut conn = db_pool.get()?;
        let user = User::find(&mut conn, &claims.sub)?
            .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
        let is_kyc_verified = claims
            .account()
            .map(|a| {
                Identity::exists(
                    &mut conn,
                    contracts.identity_registry_contract_index,
                    &a.into(),
                )
            })
            .unwrap_or(Ok(false))?;

        let user = ApiUser::new(&user, claims.is_admin(), is_kyc_verified);
        Ok(Json(user))
    }

    /// Sends a registration invitation for a new user.
    ///
    /// This function first checks if the user already exists in the Cognito user pool. If the user exists and their email is already verified, an error is returned.
    /// Otherwise, the function either resets the password for the existing user or creates a new user in the Cognito user pool.
    ///
    /// If the request includes an affiliate account address, the function also inserts the affiliation information into the database.
    ///
    /// # Arguments
    /// * `user_pool` - A reference to the Cognito user pool.
    /// * `db_pool` - A reference to the database connection pool.
    /// * `req` - The request containing the email and optional affiliate account address.
    ///
    /// # Returns
    /// The user ID of the user for whom the registration invitation was sent.
    #[oai(path = "/users/invitation", method = "post", tag = "ApiTags::User")]
    pub async fn register_invitation_send(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        Json(req): Json<UserRegistrationInvitationSendReq>,
    ) -> JsonResult<String> {
        let user = user_pool.find_user_by_email(&req.email).await?;
        let user_id = if let Some(user) = &user {
            for attr in user.attributes() {
                if attr.name().eq("email_verified") && attr.value().eq(&Some("true")) {
                    return Err(Error::BadRequest(PlainText(
                        "User already verified".to_owned(),
                    )));
                }
            }
            let user_id = user
                .username()
                .expect("User exists in pool without username")
                .to_owned();
            user_pool.reset_password(&user_id).await?;
            user_id
        } else {
            let user = user_pool.create_user(&req.email).await?;
            let user_id = user
                .username()
                .expect("User created without username")
                .to_owned();
            user_id
        };

        if let Some(account) = req.affiliate_account_address()? {
            let mut conn = db_pool.get()?;
            UserAffiliate::insert(&mut conn, &user_id, &account)?;
        }

        Ok(Json(user_id.to_owned()))
    }

    /// Inserts a new user into the database and Cognito user pool.
    ///
    /// If the user's email is not yet verified, this function will set the email as verified.
    ///
    /// The function will upsert the user information in the database, including the Cognito user ID, email, and desired investment amount.
    ///
    /// If the user has an associated account address, the function will check if the identity for that address exists in the identity registry.
    ///
    /// # Arguments
    /// * `user_pool` - A reference to the Cognito user pool.
    /// * `db_pool` - A reference to the database connection pool.
    /// * `identity_registry` - A reference to the identity registry.
    /// * `claims` - The bearer authorization claims for the current user.
    /// * `req` - The user registration request containing the desired investment amount.
    ///
    /// # Returns
    /// The newly created or updated user.
    #[oai(path = "/users", method = "post", tag = "ApiTags::User")]
    pub async fn user_insert(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(default_affiliate_commission): Data<&AffiliateCommission>,
        Json(req): Json<UserRegisterReq>,
    ) -> JsonResult<ApiUser> {
        if !claims.email_verified() {
            user_pool.set_email_verified(&claims.sub).await?;
        }

        let mut conn = db_pool.get()?;
        let user = User {
            email:                     claims.email.to_owned(),
            cognito_user_id:           claims.sub.to_owned(),
            account_address:           None,
            desired_investment_amount: Some(req.desired_investment_amount),
            affiliate_commission:      default_affiliate_commission.commission,
        }
        .upsert(&mut conn)?;
        let user = ApiUser::new(
            &user,
            claims.is_admin(),
            user.account_address()
                .map(|a| {
                    Identity::exists(
                        &mut conn,
                        contracts.identity_registry_contract_index,
                        &a.into(),
                    )
                })
                .unwrap_or(Ok(false))?,
        );
        Ok(Json(user))
    }

    /// Generates a new challenge for the user to verify their account address.
    ///
    /// This function first checks if the user has an existing valid challenge. If not, it generates a new challenge and stores it in the database.
    ///
    /// The function returns the challenge and the serialized identity statement for the user's account address.
    ///
    /// # Arguments
    /// * `id_statement` - A reference to the user's identity statement.
    /// * `db_pool` - A reference to the database connection pool.
    /// * `config` - A reference to the user challenge configuration.
    /// * `claims` - The bearer authorization claims for the current user.
    ///
    /// # Returns
    /// A `CreateChallengeResponse` containing the challenge and the serialized identity statement.
    #[oai(
        path = "/users/account_address/generate_challenge",
        method = "post",
        tag = "ApiTags::User"
    )]
    pub async fn challenge_create(
        &self,
        Data(id_statement): Data<&concordium::identity::IdStatement>,
        Data(db_pool): Data<&DbPool>,
        Data(config): Data<&UserChallengeConfig>,
        BearerAuthorization(claims): BearerAuthorization,
    ) -> JsonResult<CreateChallengeResponse> {
        let account_address = ensure_account_registered(&claims)?;
        let id_statement = serde_json::to_value(id_statement).map_err(|_| {
            Error::InternalServer(PlainText("Failed to serialize id statement".to_string()))
        })?;
        let challenge = db_pool.get()?.transaction(|conn| {
            let challenge = match UserChallenge::find_by_user_id(
                conn,
                &claims.sub,
                Utc::now(),
                config.challenge_expiry_duration,
            )? {
                Some(challenge) => challenge,
                None => {
                    let challenge = concordium::identity::generate_challenge(&claims.sub);
                    UserChallenge::new(claims.sub, challenge, account_address, Utc::now())
                        .insert(conn)?
                }
            };

            Ok::<_, Error>(challenge.challenge())
        })?;

        Ok(Json(CreateChallengeResponse {
            challenge: hex::encode(challenge),
            id_statement,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    /// Updates the account address for the current user.
    ///
    /// # Arguments
    /// * `claims` - The bearer authorization claims for the current user.
    /// * `concordium_client` - A reference to the Concordium client.
    /// * `db_pool` - A reference to the database connection pool.
    /// * `user_pool` - A reference to the Cognito user pool.
    /// * `network` - A reference to the DID network.
    /// * `global_context` - A reference to the Concordium global context.
    /// * `config` - A reference to the user challenge configuration.
    /// * `request` - The request containing the proof to verify the account address update.
    ///
    /// # Returns
    /// A `NoResResult` indicating the success or failure of the operation.
    #[oai(path = "/users/account_address", method = "put", tag = "ApiTags::User")]
    pub async fn update_account_address(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(concordium_client): Data<&v2::Client>,
        Data(db_pool): Data<&DbPool>,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(network): Data<&web3id::did::Network>,
        Data(global_context): Data<&concordium::identity::GlobalContext>,
        Data(config): Data<&UserChallengeConfig>,
        Json(request): Json<UpdateAccountAddressReq>,
    ) -> NoResResult {
        let mut concordium_client = concordium_client.clone();
        let proof = request.proof()?;
        let mut conn = db_pool.get()?;
        let account_address = {
            let db_challenge = UserChallenge::find_by_user_id(
                &mut conn,
                &claims.sub,
                Utc::now(),
                config.challenge_expiry_duration,
            )?
            .ok_or(Error::NotFound(PlainText(
                "Challenge not found".to_string(),
            )))?;
            let account_address = db_challenge.account_address();
            let challenge = db_challenge.challenge();
            verify_presentation(
                &mut concordium_client,
                proof,
                account_address,
                network,
                global_context,
                challenge,
            )
            .await?;
            account_address
        };
        user_pool
            .update_account_address(&claims.sub, &account_address)
            .await?;
        conn.transaction(|conn| {
            let mut user = User::find(conn, &claims.sub)?
                .ok_or(Error::NotFound(PlainText("User not found".to_string())))?;
            user.account_address = Some(account_address.to_string());
            user.upsert(conn)?;
            UserChallenge::delete_by_user_id(conn, &claims.sub)?;
            Ok::<_, Error>(())
        })
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
        path = "/admin/users/:cognito_user_id",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn get_by_cognito_user_id(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(cognito_user_id): Path<uuid::Uuid>,
    ) -> JsonResult<AdminUser> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let user = User::find(&mut conn, &cognito_user_id.to_string())?
            .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
        let user = AdminUser::new(
            &user,
            user.account_address()
                .map(|a| {
                    Identity::exists(
                        &mut conn,
                        contracts.identity_registry_contract_index,
                        &Address::Account(a),
                    )
                })
                .unwrap_or(Ok(false))?,
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
        path = "/admin/users/account_address/:account_address",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn get_by_account_address(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(account_address): Path<String>,
    ) -> JsonResult<AdminUser> {
        ensure_is_admin(&claims)?;
        let account_address = account_address
            .parse()
            .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))?;
        let mut conn = db_pool.get()?;
        let user = User::find_by_account_address(&mut conn, &account_address)?
            .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
        let user = AdminUser::new(
            &user,
            Identity::exists(
                &mut conn,
                contracts.identity_registry_contract_index,
                &Address::Account(account_address),
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
    #[oai(
        path = "/admin/users/list/:page",
        method = "get",
        tag = "ApiTags::User"
    )]
    pub async fn list(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<AdminUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (users, page_count) = User::list(&mut conn, page, PAGE_SIZE)?;
        let addresses = users
            .iter()
            .map(|user| user.account_address())
            .filter_map(|a| a.map(Address::Account))
            .collect::<Vec<_>>();
        let registered = Identity::exists_batch(
            &mut conn,
            contracts.identity_registry_contract_index,
            &addresses,
        )?
        .into_iter()
        .collect::<BTreeSet<_>>();
        let data = users
            .iter()
            .map(|u| {
                AdminUser::new(
                    u,
                    u.account_address()
                        .map(|a| registered.contains(&Address::Account(a)))
                        .unwrap_or(false),
                )
            })
            .collect();
        Ok(Json(PagedResponse {
            data,
            page,
            page_count,
        }))
    }

    /// Delete a user by their Cognito user ID.
    ///
    /// This endpoint is only accessible to admin users.
    ///
    /// # Arguments
    /// - `user_pool`: A reference to the Cognito user pool.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `claims`: The authorization claims of the requesting user.
    /// - `cognito_user_id`: The Cognito user ID of the user to delete.
    ///
    /// # Returns
    /// A JSON response indicating the success of the deletion.
    #[oai(
        path = "/admin/users/:cognito_user_id",
        method = "delete",
        tag = "ApiTags::User"
    )]
    pub async fn delete(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(cognito_user_id): Path<uuid::Uuid>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let cognito_user_id = cognito_user_id.to_string();
        let mut conn = db_pool.get()?;
        user_pool.delete_user(&cognito_user_id).await?;
        info!("Deleted user from cognito: {}", cognito_user_id);
        if User::delete(&mut conn, &cognito_user_id)?.ge(&1) {
            info!("Deleted user from db: {}", cognito_user_id);
        }

        Ok(())
    }

    /// Update the Concordium account address for a user.
    ///
    /// This endpoint is only accessible to admin users.
    ///
    /// # Arguments
    /// - `user_pool`: A reference to the Cognito user pool.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `claims`: The authorization claims of the requesting user.
    /// - `cognito_user_id`: The Cognito user ID of the user to update.
    /// - `request`: The request body containing the new account address.
    ///
    /// # Returns
    /// A successful response indicating the account address was updated.
    #[oai(
        path = "/admin/users/:cognito_user_id/account_address",
        method = "put",
        tag = "ApiTags::User"
    )]
    pub async fn admin_update_account_address(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(cognito_user_id): Path<uuid::Uuid>,
        Json(request): Json<UserUpdateAccountAddressRequest>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let cognito_user_id = cognito_user_id.to_string();
        let mut conn = db_pool.get()?;
        User::find(&mut conn, &cognito_user_id)?
            .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
        let account_address = request.account_address()?;
        user_pool
            .update_account_address(&cognito_user_id, &account_address)
            .await?;
        {
            let mut user = User::find(&mut conn, &cognito_user_id)?
                .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
            user.account_address = Some(account_address.to_string());
            user.affiliate_commission = request.affiliate_commission;
            user.upsert(&mut conn)?;
        }
        Ok(())
    }

    #[oai(
        path = "/user/txn_history/list/:page",
        method = "get",
        tag = "ApiTags::Wallet"
    )]
    pub async fn txn_history_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<UserTransaction>> {
        let mut conn = db_pool.get()?;
        let (users, page_count) = UserTransaction::list(&mut conn, &claims.sub, page, PAGE_SIZE)?;

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
    ) -> JsonResult<PagedResponse<AffiliateReward>> {
        let account = ensure_account_registered(&claims)?;
        let mut conn = db_pool.get()?;
        let (users, page_count) =
            AffiliateReward::list_by_affiliate(&mut conn, &account.to_string(), page, PAGE_SIZE)?;

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
        let reward = AffiliateReward::find(&mut conn, &investment_record_id)?
            .ok_or_else(|| Error::NotFound(PlainText("Reward not found".to_string())))?;
        let remaining_reward_amount = reward
            .remaining_reward_amount
            .unwrap_or(reward.reward_amount);
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

    #[oai(path = "/system_config", method = "get", tag = "ApiTags::User")]
    pub async fn system_config(
        &self,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<SystemContractsConfig> {
        Ok(Json(contracts.clone()))
    }
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
    /// The email address of the user
    /// This information is provided by the user during the signup process
    pub email:                     String,
    /// The concordium account address of the user
    /// This information is updated by the user by providing concordium identity proofs
    pub account_address:           Option<String>,
    /// The amount of money that the user wants to invest
    /// This information is supposed to be updated by the user
    pub desired_investment_amount: Option<i32>,
    /// Does user belong to the `admin` group in Cognito?
    /// This information is parsed from the identity token
    pub is_admin:                  bool,
    /// The Cognito user id
    /// This information is parsed from the identity token
    pub cognito_user_id:           String,
    /// Has the user completed the KYC process?
    /// This information is parsed from the Identity Registry contract
    /// If the user's account_address is not set, then the user has not completed the KYC process
    pub kyc_verified:              bool,
}

impl ApiUser {
    pub fn new(db_user: &User, is_admin: bool, kyc_verified: bool) -> Self {
        Self {
            email: db_user.email.clone(),
            account_address: db_user.account_address.clone(),
            desired_investment_amount: db_user.desired_investment_amount,
            is_admin,
            cognito_user_id: db_user.cognito_user_id.clone(),
            kyc_verified,
        }
    }
}

/// This is the user being returned by the Users Admin Api.
/// This dosent have the field is_admin.
#[derive(Object, Serialize, Deserialize, PartialEq, Debug)]
pub struct AdminUser {
    /// The email address of the user
    /// This information is provided by the user during the signup process
    pub email:                     String,
    /// The concordium account address of the user
    /// This information is updated by the user by providing concordium identity proofs
    pub account_address:           Option<String>,
    /// The amount of money that the user wants to invest
    /// This information is supposed to be updated by the user
    pub desired_investment_amount: Option<i32>,
    /// The Cognito user id
    /// This information is parsed from the identity token
    pub cognito_user_id:           String,
    /// Has the user completed the KYC process?
    /// This information is parsed from the Identity Registry contract
    /// If the user's account_address is not set, then the user has not completed the KYC process
    pub kyc_verified:              bool,
}

impl AdminUser {
    pub fn new(db_user: &User, kyc_verified: bool) -> Self {
        Self {
            email: db_user.email.clone(),
            account_address: db_user.account_address.clone(),
            desired_investment_amount: db_user.desired_investment_amount,
            cognito_user_id: db_user.cognito_user_id.clone(),
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
pub struct UserRegistrationInvitationSendReq {
    pub email:                     String,
    pub affiliate_account_address: Option<String>,
}

impl UserRegistrationInvitationSendReq {
    pub fn affiliate_account_address(&self) -> Result<Option<AccountAddress>> {
        self.affiliate_account_address
            .as_ref()
            .map(|a| a.parse())
            .transpose()
            .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))
    }
}

#[derive(Clone)]
pub struct UserChallengeConfig {
    pub challenge_expiry_duration: chrono::Duration,
}

#[derive(Serialize, Object)]
pub struct CreateChallengeRequest {
    pub account_address: String,
}

impl CreateChallengeRequest {
    pub fn account_address(&self) -> Result<AccountAddress> {
        self.account_address
            .parse()
            .map_err(|_| Error::BadRequest(PlainText("Invalid account address".to_string())))
    }
}

#[derive(Deserialize, Object)]
pub struct CreateChallengeResponse {
    pub id_statement: serde_json::Value,
    /// The hex of the challenge
    pub challenge:    String,
}

#[derive(Object)]
pub struct UpdateAccountAddressReq {
    pub proof:                serde_json::Value,
    pub affiliate_commission: Decimal,
}
impl UpdateAccountAddressReq {
    pub fn proof(&self) -> Result<concordium::identity::Presentation> {
        let proof = serde_json::from_value(self.proof.clone())
            .map_err(|_| Error::BadRequest(PlainText("Invalid proof".to_string())))?;
        Ok(proof)
    }
}

async fn verify_presentation(
    concordium_client: &mut v2::Client,
    proof: Presentation,
    account_address: AccountAddress,
    network: &web3id::did::Network,
    global_context: &concordium::identity::GlobalContext,
    challenge: [u8; 32],
) -> Result<()> {
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
    concordium::identity::verify_presentation(
        *network,
        concordium_client,
        global_context,
        &proof,
        challenge,
    )
    .await
    .map_err(|_| Error::BadRequest(PlainText("Invalid proof".to_string())))?;

    Ok(())
}
