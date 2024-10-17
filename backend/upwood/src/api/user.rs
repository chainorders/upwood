use chrono::Utc;
use concordium::identity::Presentation;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::Address;
use concordium_rust_sdk::v2::BlockIdentifier;
use concordium_rust_sdk::web3id::CredentialMetadata;
use concordium_rust_sdk::{v2, web3id};
use diesel::Connection;
use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use serde::Serialize;
use shared::db::DbPool;
use tracing::info;

use crate::api::*;
use crate::db;
use crate::utils::*;

#[derive(Object, Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
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

impl User {
    pub fn new(db_user: &db::users::User, is_admin: bool, kyc_verified: bool) -> Self {
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

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/users/self", method = "get")]
    pub async fn user_self(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(identity_registry): Data<&IdentityRegistry>,
        BearerAuthorization(claims): BearerAuthorization,
    ) -> JsonResult<User> {
        let mut conn = db_pool.get()?;
        let user = db::users::find_user_by_cognito_user_id(&mut conn, &claims.sub)?
            .ok_or_else(|| Error::NotFound(PlainText("User not found".to_string())))?;
        let user = User::new(
            &user,
            claims.is_admin(),
            user.account_address()
                .map(|a| identity_registry.is_registered(&mut conn, &Address::Account(a)))
                .unwrap_or(Ok(false))?,
        );
        Ok(Json(user))
    }

    /// Calls `AdminCreateUser` on Cognito to create a new user.
    /// This user is not verified/registered still.
    #[oai(path = "/users/register/invitation", method = "post")]
    pub async fn register_invitation_send(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Json(req): Json<UserRegistrationInvitationSendReq>,
    ) -> JsonResult<String> {
        let user_id = user_pool.find_user_by_email(&req.email).await?;
        let user_id = match user_id {
            Some(user_id) => user_id,
            None => user_pool.admin_create_user(&req.email).await?,
        };

        Ok(Json(user_id))
    }

    #[oai(path = "/users/register", method = "post")]
    pub async fn user_register(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        Data(identity_registry): Data<&IdentityRegistry>,
        BearerAuthorization(claims): BearerAuthorization,
        Json(req): Json<UserRegisterReq>,
    ) -> JsonResult<User> {
        if !claims.email_verified() {
            user_pool.admin_set_email_verified(&claims.sub).await?;
        }

        let mut conn = db_pool.get()?;
        let user = db::users::insert(&mut conn, &db::users::User {
            email:                     claims.email.to_owned(),
            cognito_user_id:           claims.sub.to_owned(),
            account_address:           None,
            desired_investment_amount: Some(req.desired_investment_amount),
        })?;
        let user = User::new(
            &user,
            claims.is_admin(),
            user.account_address()
                .map(|a| identity_registry.is_registered(&mut conn, &Address::Account(a)))
                .unwrap_or(Ok(false))?,
        );
        Ok(Json(user))
    }

    #[oai(path = "/users/account_address/generate_challenge", method = "post")]
    pub async fn challenge_create(
        &self,
        Data(id_statement): Data<&concordium::identity::IdStatement>,
        Data(db_pool): Data<&DbPool>,
        Data(config): Data<&UserChallengeConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Json(req): Json<CreateChallengeRequest>,
    ) -> JsonResult<CreateChallengeResponse> {
        let id_statement = serde_json::to_value(id_statement).map_err(|_| {
            Error::InternalServer(PlainText("Failed to serialize id statement".to_string()))
        })?;
        let challenge = db_pool.get()?.transaction(|conn| {
            let challenge = match db::user_challenges::find_by_user_id(
                conn,
                &claims.sub,
                Utc::now(),
                config.challenge_expiry_duration,
            )? {
                Some(challenge) => challenge,
                None => {
                    let challenge = concordium::identity::generate_challenge(&claims.sub);
                    let challenge = db::user_challenges::UserChallengeInsert::new(
                        claims.sub,
                        challenge,
                        req.account_address()?,
                    );
                    db::user_challenges::insert(conn, &challenge)?
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
    #[oai(path = "/users/account_address", method = "put")]
    pub async fn update_account_address(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(concordium_client): Data<&v2::Client>,
        Data(db_pool): Data<&DbPool>,
        Data(network): Data<&web3id::did::Network>,
        Data(global_context): Data<&concordium::identity::GlobalContext>,
        Data(config): Data<&UserChallengeConfig>,
        Json(request): Json<UpdateAccountAddressReq>,
    ) -> NoResResult {
        let mut concordium_client = concordium_client.clone();
        let proof = request.proof()?;
        let mut conn = db_pool.get()?;
        let account_address = {
            let db_challenge = db::user_challenges::find_by_user_id(
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
        conn.transaction(|conn| {
            db::users::update_account_address(conn, &claims.sub, &account_address)?;
            db::user_challenges::delete_by_user_id(conn, &claims.sub)?;
            Ok::<_, Error>(())
        })
    }

    #[oai(path = "/users", method = "delete")]
    pub async fn user_delete(
        &self,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Json(req): Json<UserDeleteReq>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let user_id = user_pool.find_user_by_email(&req.email).await?;
        let mut conn = db_pool.get()?;
        if let Some(user_id) = user_id {
            user_pool.admin_delete_user(&user_id).await?;
            info!("Deleted user from cognito: {}", user_id);
            if db::users::delete_by_cognito_user_id(&mut conn, &user_id)?.ge(&1) {
                info!("Deleted user from db: {}", user_id);
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Object)]
pub struct UserDeleteReq {
    pub email: String,
}

#[derive(Serialize, Object)]
pub struct UserRegisterReq {
    pub desired_investment_amount: i32,
}

#[derive(Serialize, Object)]
pub struct UserRegistrationInvitationSendReq {
    pub email: String,
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
    pub proof: serde_json::Value,
}
impl UpdateAccountAddressReq {
    pub fn proof(&self) -> Result<concordium::identity::Presentation> {
        let proof = serde_json::from_value(self.proof.clone())
            .map_err(|_| Error::BadRequest(PlainText("Invalid proof".to_string())))?;
        Ok(proof)
    }
}
#[derive(Object)]
pub struct UpdateAccountAddressRes {}

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
