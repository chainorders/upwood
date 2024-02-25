use chrono::{DateTime, Utc};
use concordium_rust_sdk::{
    common::to_bytes,
    constants::SHA256,
    id::types::IpIdentity,
    smart_contracts::common::{AccountAddress, AccountAddressParseError},
    types::{smart_contracts::InstanceInfo, Address, ContractAddress, Energy, WalletAccount},
    v2::{BlockIdentifier, QueryError},
    web3id::did::Network,
};
use log::debug;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::shared::api::ApiContractAddress;

use super::{
    db::{Db, DbChallenge},
    identity_registry_client::{Error as IdentityRegistryError, IdentityRegistryClient},
    web3_id_utils::{
        verify_presentation, CredStatement, GlobalContext, IdStatement, Presentation,
        VerifyPresentationError,
    },
};

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
impl From<VerifyPresentationError> for Error {
    fn from(_: VerifyPresentationError) -> Self { Error::BadRequest }
}
impl From<IdentityRegistryError> for Error {
    fn from(_: IdentityRegistryError) -> Self { Error::InternalServer }
}
#[derive(Object)]
pub struct GenerateChallengeRequest {
    account: String,
}

#[derive(Object)]
pub struct GenerateChallengeResponse {
    challenge:          String,
    id_statement:       Value,
    cred_statement:     Value,
    issuers:            Vec<ApiContractAddress>,
    identity_providers: Vec<u32>,
}

pub struct Api {
    pub identity_registry:  ContractAddress,
    pub agent_wallet:       WalletAccount,
    pub id_statement:       IdStatement,
    pub cred_statement:     CredStatement,
    pub db:                 Db,
    pub concordium_client:  concordium_rust_sdk::v2::Client,
    pub global_context:     GlobalContext,
    pub max_energy:         Energy,
    pub network:            Network,
    pub issuers:            Vec<ContractAddress>,
    pub identity_providers: Vec<IpIdentity>,
}

#[derive(Object)]
pub struct RegisterIdentityRequest {
    pub proof:    Value,
    pub account:  String,
    pub contract: Option<ApiContractAddress>,
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
        let challenge = self.get_or_create_db_challenge(account).await?;

        let id_statement =
            serde_json::to_value(&self.id_statement).map_err(|_| Error::InternalServer)?;
        let cred_statement =
            serde_json::to_value(&self.cred_statement).map_err(|_| Error::InternalServer)?;
        Ok(Json(GenerateChallengeResponse {
            challenge: challenge.challenge,
            id_statement,
            cred_statement,
            issuers: self
                .issuers
                .iter()
                .map(|c| ApiContractAddress::from_contract_address(*c))
                .collect(),
            identity_providers: self.identity_providers.iter().map(|i| i.0).collect(),
        }))
    }

    #[oai(path = "/verifier/registerIdentity", method = "post")]
    pub async fn register_identity(
        &self,
        request: Json<RegisterIdentityRequest>,
    ) -> Result<Json<RegisterIdentityResponse>, Error> {
        let account: AccountAddress = request.account.parse()?;
        debug!(
            "Registering identity for contract: {:?} from account: {}",
            request.contract, request.account
        );
        debug!("Register Identity Proofs: {:?}", request.proof);
        let proof: Presentation = request.proof.clone().try_into()?;
        let challenge = self
            .db
            .find_challenge(&account)
            .await?
            .map(|db_challenge| {
                let mut challenge = [0u8; SHA256];
                hex::decode_to_slice(db_challenge.challenge, &mut challenge)
                    .map_err(|_| Error::InternalServer)?;
                Result::<_, Error>::Ok(challenge)
            })
            .ok_or(Error::NotFound)??;
        debug!("Challenge: {:?}", challenge);

        let mut concordium_client = self.concordium_client.clone();
        let verification_response = verify_presentation(
            self.network,
            &mut concordium_client,
            &self.global_context,
            &proof,
            challenge,
        )
        .await?;
        debug!("Revealed Id Attributes: {:?}", verification_response.revealed_attributes);
        debug!("Credentials: {:?}", verification_response.credentials);
        let identity_address: Address = {
            if let Some(contract) = request.contract {
                let contract: ContractAddress = contract.into();
                let contract_info = self
                    .concordium_client
                    .clone()
                    .get_instance_info(contract, BlockIdentifier::LastFinal)
                    .await?;
                let contract_owner = match contract_info.response {
                    InstanceInfo::V0 {
                        owner,
                        ..
                    } => owner,
                    InstanceInfo::V1 {
                        owner,
                        ..
                    } => owner,
                };

                if contract_owner != account {
                    debug!("Contract owner: {} does not match", contract_owner.to_string());
                    return Err(Error::BadRequest);
                }

                Address::Contract(contract)
            } else {
                Address::Account(account)
            }
        };

        let txn =
            IdentityRegistryClient::new(self.concordium_client.clone(), self.identity_registry)
                .register_identity(
                    &self.agent_wallet,
                    identity_address,
                    verification_response,
                    self.max_energy,
                )
                .await?;

        debug!("Register Identity Transaction Hash: {}", txn.to_string());
        Ok(Json(RegisterIdentityResponse {
            txn_hash: txn.to_string(),
        }))
    }

    async fn get_or_create_db_challenge(
        &self,
        account: AccountAddress,
    ) -> Result<DbChallenge, Error> {
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

        Ok(challenge)
    }

    fn create_new_challenge(&self, account: AccountAddress, now: DateTime<Utc>) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(to_bytes(&self.id_statement));
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
