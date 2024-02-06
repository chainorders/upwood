use std::str::FromStr;

use chrono::{DateTime, Utc};
use concordium_rust_sdk::{
    common::{to_bytes, types::TransactionTime, Versioned},
    id::{
        constants::{ArCurve, AttributeKind},
        id_proof_types::{AtomicProof, AtomicStatement, Proof, ProofVersion, Statement},
        types::{AccountCredentialWithoutProofs, AttributeTag, GlobalContext},
    },
    smart_contracts::common::{AccountAddress, AccountAddressParseError, Amount, Serial},
    types::{
        smart_contracts::{OwnedParameter, OwnedReceiveName},
        transactions::{send, UpdateContractPayload},
        ContractAddress, CredentialRegistrationID, Energy, WalletAccount,
    },
    v2::{BlockIdentifier, QueryError},
};
use concordium_rwa_identity_registry::{identities::RegisterIdentityParams, types::Identity};
use concordium_rwa_utils::common_types::IdentityAttribute;
use log::{debug, error};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::db::{Db, DbChallenge};

#[derive(Debug, ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 500)]
    InternalServer,
    #[oai(status = 404)]
    NotFound,
}
impl From<mongodb::error::Error> for Error {
    fn from(_: mongodb::error::Error) -> Self { Self::InternalServer }
}
impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self { Self::InternalServer }
}
impl From<AccountAddressParseError> for Error {
    fn from(_: AccountAddressParseError) -> Self { Self::BadRequest }
}
impl From<bson::ser::Error> for Error {
    fn from(_: bson::ser::Error) -> Self { Self::BadRequest }
}
impl From<QueryError> for Error {
    fn from(_: QueryError) -> Self { Self::InternalServer }
}

#[derive(Object)]
pub struct GenerateChallengeRequest {
    account: String,
}

#[derive(Object)]
pub struct GenerateChallengeResponse {
    challenge: String,
    statement: Value,
}

pub type GlobalContextType = GlobalContext<ArCurve>;
pub type ProofType = Proof<ArCurve, AttributeKind>;
pub type StatementType = Statement<ArCurve, AttributeKind>;
pub struct Api {
    pub identity_registry: ContractAddress,
    pub register_identity_receive_name: OwnedReceiveName,
    pub agent_wallet: WalletAccount,
    pub statement: StatementType,
    pub db: Db,
    pub concordium_client: concordium_rust_sdk::v2::Client,
    pub global_context: GlobalContextType,
    pub max_energy: Energy,
}

#[derive(Object)]
pub struct RegisterIdentityRequest {
    pub proof:   ApiProofWithContext,
    pub account: String,
}

#[derive(Debug)]
pub struct ProofWithContext {
    pub credential: CredentialRegistrationID,
    pub proof:      Versioned<ProofType>,
}

#[derive(Object)]
pub struct ApiProofWithContext {
    pub credential: String, //CredentialRegistrationID,
    pub proof:      String, //Versioned<Proof<ArCurve, AttributeKind>>,
}

impl ApiProofWithContext {
    pub fn parse(&self) -> anyhow::Result<ProofWithContext> {
        let credential = CredentialRegistrationID::from_str(&self.credential)?;
        debug!("Credential: {:?}", credential);
        let proof: Versioned<Proof<ArCurve, AttributeKind>> =
            serde_json::from_value(serde_json::from_str(&self.proof)?)?;
        debug!("Proof: {:?}", proof);
        Ok(ProofWithContext {
            credential,
            proof,
        })
    }
}

#[derive(Object)]
pub struct RegisterIdentityResponse {
    txn_hash: String,
}

#[OpenApi]
impl Api {
    #[oai(path = "/verifier/generateChallenge", method = "post")]
    pub async fn generate_challenge(
        &self,
        request: Json<GenerateChallengeRequest>,
    ) -> Result<Json<GenerateChallengeResponse>, Error> {
        let account: AccountAddress = request.account.parse()?;
        debug!("Generating challenge for account: {}", account.to_string());
        let challenge = self.db.find_challenge(&account).await?;
        debug!("Challenge: {:?}", challenge);
        let challenge = match challenge {
            Some(challenge) => challenge,
            None => {
                let now = Utc::now();
                let challenge = hex::encode(self.create_new_challenge(account, now));
                let challenge = DbChallenge {
                    challenge,
                    address: account,
                    created_at: now,
                };
                self.db.insert_challenge(challenge.clone()).await?;
                challenge
            }
        };

        let statement = serde_json::to_value(&self.statement).map_err(|_| Error::InternalServer)?;
        // let statement =
        //     serde_json::to_string(&self.statement).map_err(|_|
        // Error::InternalServerError)?;
        debug!("Statement: {:?}", statement);
        Ok(Json(GenerateChallengeResponse {
            challenge: challenge.challenge,
            statement,
        }))
    }

    #[oai(path = "/verifier/registerIdentity", method = "post")]
    pub async fn register_identity(
        &self,
        request: Json<RegisterIdentityRequest>,
    ) -> Result<Json<RegisterIdentityResponse>, Error> {
        let account: AccountAddress = request.account.parse()?;
        debug!("Registering identity for account: {}", request.account);
        let proof = request.proof.parse()?;
        debug!("Register Identity Proofs: {:?}", proof);
        let challenge = self.db.find_challenge(&account).await?;
        debug!("Challenge: {:?}", challenge);
        let challenge = match challenge {
            Some(challenge) => {
                hex::decode(challenge.challenge).map_err(|_| Error::InternalServer)?
            }
            None => return Err(Error::NotFound),
        };
        let mut concordium_client = self.concordium_client.clone();
        let acc_info = concordium_client
            .get_account_info(&account.into(), BlockIdentifier::LastFinal)
            .await
            .map_err(|e| {
                error!("Error getting account info: {:?}", e);
                Error::InternalServer
            })?;
        debug!("Retrieved Account Info");

        let commitments = acc_info
            .response
            .account_credentials
            .iter()
            .find_map(|(_, credential)| {
                if to_bytes(credential.value.cred_id()).eq(&to_bytes(&proof.credential)) {
                    match &credential.value {
                        AccountCredentialWithoutProofs::Initial {
                            icdv: _,
                            ..
                        } => None,
                        AccountCredentialWithoutProofs::Normal {
                            commitments,
                            ..
                        } => Some(commitments),
                    }
                } else {
                    None
                }
            })
            .ok_or(Error::InternalServer)?;
        let is_verified = self.statement.verify(
            ProofVersion::Version1,
            &challenge,
            &self.global_context,
            proof.credential.as_ref(),
            commitments,
            &proof.proof.value,
        );
        debug!("Is Verified: {:?}", is_verified);
        if !is_verified {
            return Err(Error::BadRequest);
        }

        let revealed_attributes: Vec<(AttributeTag, String)> = self
            .statement
            .statements
            .iter()
            .zip(proof.proof.value.proofs)
            .filter_map(|(s, p)| {
                if let (
                    AtomicStatement::RevealAttribute {
                        statement,
                    },
                    AtomicProof::RevealAttribute {
                        attribute,
                        proof: _,
                    },
                ) = (s, p)
                {
                    Some((statement.attribute_tag, attribute.0))
                } else {
                    None
                }
            })
            .collect();

        debug!("Revealed Attributes: {:?}", revealed_attributes);
        let register_identity_payload = RegisterIdentityParams {
            address:  concordium_rust_sdk::types::Address::Account(account),
            identity: Identity {
                attributes:  revealed_attributes
                    .iter()
                    .map::<Result<IdentityAttribute, _>, _>(|(tag, value)| {
                        Ok(IdentityAttribute {
                            tag:   tag.0,
                            value: value.to_string(),
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?,
                credentials: Vec::new(),
            },
        };

        let agent_account_info = concordium_client
            .get_account_info(&self.agent_wallet.address.into(), BlockIdentifier::LastFinal)
            .await
            .map_err(|e| {
                error!("Error getting agent account info: {:?}", e);
                Error::InternalServer
            })?;
        debug!("Retrieved Agent Account Info");

        let mut register_identity_payload_bytes: Vec<u8> = Vec::new();
        register_identity_payload
            .serial(&mut register_identity_payload_bytes)
            .map_err(|_| Error::InternalServer)?;
        let expiry: TransactionTime =
            TransactionTime::from_seconds((Utc::now().timestamp() + 300) as u64);
        let txn = send::update_contract(
            &self.agent_wallet.keys,
            self.agent_wallet.address,
            agent_account_info.response.account_nonce,
            expiry,
            UpdateContractPayload {
                amount:       Amount::from_ccd(0u64),
                address:      self.identity_registry,
                receive_name: self.register_identity_receive_name.clone(),
                message:      OwnedParameter::new_unchecked(register_identity_payload_bytes),
            },
            self.max_energy,
        );
        let txn = concordium_client.send_account_transaction(txn).await.map_err(|e| {
            error!("Error sending transaction: {:?}", e);
            Error::InternalServer
        })?;
        debug!("Register Identity Transaction Hash: {}", txn.to_string());
        Ok(Json(RegisterIdentityResponse {
            txn_hash: txn.to_string(),
        }))
    }

    fn create_new_challenge(&self, account: AccountAddress, now: DateTime<Utc>) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(to_bytes(&self.statement));
        hasher.update(AsRef::<[u8; 32]>::as_ref(&account)); // Add type annotation to specify the implementation of AsRef to use
        hasher.update(self.identity_registry.index.to_be_bytes());
        hasher.update(self.identity_registry.subindex.to_be_bytes());
        hasher.update(now.to_rfc3339().as_bytes());

        let result = hasher.finalize();
        let mut challenge = [0; 32];
        challenge.copy_from_slice(&result);
        challenge
    }
}
