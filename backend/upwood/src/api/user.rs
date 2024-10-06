use chrono::Utc;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::v2::{AccountIdentifier, BlockIdentifier};
use concordium_rust_sdk::web3id::CredentialMetadata;
use concordium_rust_sdk::{v2, web3id};
use diesel::Connection;
use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use shared::db::DbPool;

use crate::api::*;
use crate::db;
use crate::utils::*;

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/users/challenge", method = "post")]
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
    #[oai(path = "/users/update_account_address", method = "put")]
    pub async fn update_account_address(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(concordium_client): Data<&v2::Client>,
        Data(db_pool): Data<&DbPool>,
        Data(network): Data<&web3id::did::Network>,
        Data(global_context): Data<&concordium::identity::GlobalContext>,
        Data(user_pool): Data<&aws::cognito::UserPool>,
        Data(config): Data<&UserChallengeConfig>,
        Json(request): Json<UpdateAccountAddressReq>,
    ) -> NoResResult {
        let mut concordium_client = concordium_client.clone();
        let mut conn = db_pool.get()?;
        let db_challenge = db::user_challenges::find_by_user_id(
            &mut conn,
            &claims.sub,
            Utc::now(),
            config.challenge_expiry_duration,
        )?
        .ok_or(Error::NotFound(PlainText(
            "Challenge not found".to_string(),
        )))?;
        let proof = request.proof()?;
        let account_ids = proof
            .verifiable_credential
            .iter()
            .filter_map(|c| match c.metadata().cred_metadata {
                CredentialMetadata::Account { cred_id, .. } => {
                    let account_id: AccountIdentifier = cred_id.into();
                    Some(account_id)
                }
                CredentialMetadata::Web3Id { .. } => None,
            })
            .collect::<Vec<_>>();
        for account_id in account_ids {
            let account_info = concordium_client
                .get_account_info(&account_id, BlockIdentifier::LastFinal)
                .await?
                .response
                .account_address
                .eq(&db_challenge.account_address());
            if !account_info {
                return Err(Error::BadRequest(PlainText(
                    "Invalid proof credentials".to_string(),
                )));
            }
        }
        concordium::identity::verify_presentation(
            *network,
            &mut concordium_client,
            global_context,
            &proof,
            db_challenge.challenge(),
        )
        .await
        .map_err(|_| Error::BadRequest(PlainText("Invalid proof".to_string())))?;
        user_pool
            .update_account_address(&claims.sub, &db_challenge.account_address)
            .await?;
        db::user_challenges::delete_by_user_id(&mut conn, &claims.sub)?;
        Ok(())
    }
}


#[derive(Clone)]
pub struct UserChallengeConfig {
    pub challenge_expiry_duration: chrono::Duration,
}

#[derive(Object)]
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

#[derive(Object)]
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
