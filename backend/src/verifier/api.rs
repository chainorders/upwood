use super::{
    db::{DbChallenge, VerifierDb},
    identity_registry_client::{Error as IdentityRegistryError, IdentityRegistryClient},
    web3_id_utils::{
        verify_presentation, CredStatement, GlobalContext, IdStatement, Presentation,
        VerifyPresentationError,
    },
};
use crate::shared::api::ApiContractAddress;
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
use poem::web::Data;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, ApiResponse)]
pub enum VerifierApiError {
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 500)]
    InternalServer,
    #[oai(status = 404)]
    NotFound,
}
impl From<mongodb::error::Error> for VerifierApiError {
    fn from(_: mongodb::error::Error) -> Self { Self::InternalServer }
}
impl From<anyhow::Error> for VerifierApiError {
    fn from(_: anyhow::Error) -> Self { Self::InternalServer }
}
impl From<AccountAddressParseError> for VerifierApiError {
    fn from(_: AccountAddressParseError) -> Self { Self::BadRequest }
}
impl From<bson::ser::Error> for VerifierApiError {
    fn from(_: bson::ser::Error) -> Self { Self::BadRequest }
}
impl From<QueryError> for VerifierApiError {
    fn from(_: QueryError) -> Self { Self::InternalServer }
}
impl From<VerifyPresentationError> for VerifierApiError {
    fn from(_: VerifyPresentationError) -> Self { VerifierApiError::BadRequest }
}
impl From<IdentityRegistryError> for VerifierApiError {
    fn from(_: IdentityRegistryError) -> Self { VerifierApiError::InternalServer }
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

/// The API for the verifier.
pub struct VerifierApi {
    pub agent_wallet: WalletAccount,
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
impl VerifierApi {
    /// Generate a challenge for the user to generate Identity and Credential
    /// proofs.
    ///
    /// # Arguments
    ///
    /// * `request` - The account for which the challenge is to be generated.
    ///
    /// # Returns
    ///
    /// * A challenge for the user to generate Identity and Credential proofs.
    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/verifier/generateChallenge", method = "post")]
    pub async fn generate_challenge(
        &self,
        Data(db): Data<&VerifierDb>,
        Data(identity_providers): Data<&Vec<IpIdentity>>,
        Data(issuers): Data<&Vec<ContractAddress>>,
        Data(id_statement): Data<&IdStatement>,
        Data(cred_statement): Data<&CredStatement>,
        Data(identity_registry): Data<&ContractAddress>,
        request: Json<GenerateChallengeRequest>,
    ) -> Result<Json<GenerateChallengeResponse>, VerifierApiError> {
        // Generate challenge for the specified account
        let account: AccountAddress = request.account.parse()?;
        debug!("Generating challenge for account: {}", account.to_string());
        let challenge =
            get_or_create_db_challenge(db, id_statement, identity_registry, account).await?;

        // Convert id_statement and cred_statement to JSON values
        let id_statement =
            serde_json::to_value(id_statement).map_err(|_| VerifierApiError::InternalServer)?;
        let cred_statement =
            serde_json::to_value(cred_statement).map_err(|_| VerifierApiError::InternalServer)?;

        // Return the challenge along with other response data
        Ok(Json(GenerateChallengeResponse {
            challenge: challenge.challenge,
            id_statement,
            cred_statement,
            issuers: issuers
                .iter()
                .map(|c| ApiContractAddress::from_contract_address(*c))
                .collect(),
            identity_providers: identity_providers.iter().map(|i| i.0).collect(),
        }))
    }

    /// Register an identity.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to register an identity.
    ///
    /// # Returns
    ///
    /// * A transaction hash of the transaction that registered the identity.
    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/verifier/registerIdentity", method = "post")]
    pub async fn register_identity(
        &self,
        Data(db): Data<&VerifierDb>,
        Data(global_context): Data<&GlobalContext>,
        Data(concordium_client): Data<&concordium_rust_sdk::v2::Client>,
        Data(identity_registry): Data<&ContractAddress>,
        Data(network): Data<&Network>,
        Data(max_energy): Data<&Energy>,
        Json(request): Json<RegisterIdentityRequest>,
    ) -> Result<Json<RegisterIdentityResponse>, VerifierApiError> {
        // Parse the account from the request
        let account: AccountAddress = request.account.parse()?;
        debug!(
            "Registering identity for contract: {:?} from account: {}",
            request.contract, request.account
        );
        debug!("Register Identity Proofs: {:?}", request.proof);

        // Convert the proof to Presentation type
        let proof: Presentation = request.proof.clone().try_into()?;
        let challenge = db
            .find_challenge(&account)
            .await?
            .map(|db_challenge| {
                let mut challenge = [0u8; SHA256];
                hex::decode_to_slice(db_challenge.challenge, &mut challenge)
                    .map_err(|_| VerifierApiError::InternalServer)?;
                Result::<_, VerifierApiError>::Ok(challenge)
            })
            .ok_or(VerifierApiError::NotFound)??;
        debug!("Challenge: {:?}", challenge);

        // Verify the presentation and get the verification response
        let mut concordium_client = concordium_client.clone();
        let verification_response = verify_presentation(
            *network,
            &mut concordium_client,
            global_context,
            &proof,
            challenge,
        )
        .await?;
        debug!("Revealed Id Attributes: {:?}", verification_response.revealed_attributes);
        debug!("Credentials: {:?}", verification_response.credentials);

        // Determine the identity address based on the contract and account
        let identity_address: Address = {
            if let Some(contract) = request.contract {
                let contract: ContractAddress = contract.into();
                let contract_info = concordium_client
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
                    // Enhancement: Return a more specific error. Include debug message as error
                    // message
                    return Err(VerifierApiError::BadRequest);
                }

                Address::Contract(contract)
            } else {
                Address::Account(account)
            }
        };

        // Register the identity using the IdentityRegistryClient
        let txn = IdentityRegistryClient::new(concordium_client, identity_registry.to_owned())
            .register_identity(
                &self.agent_wallet,
                identity_address,
                verification_response,
                *max_energy,
            )
            .await?;

        debug!("Register Identity Transaction Hash: {}", txn.to_string());
        Ok(Json(RegisterIdentityResponse {
            txn_hash: txn.to_string(),
        }))
    }
}

async fn get_or_create_db_challenge(
    db: &VerifierDb,
    id_statement: &IdStatement,
    identity_registry: &ContractAddress,
    account: AccountAddress,
) -> Result<DbChallenge, VerifierApiError> {
    // Get the challenge from the database or create a new one
    let challenge = db.find_challenge(&account).await?;
    debug!("Challenge: {:?}", challenge);
    let challenge = match challenge {
        Some(challenge) => challenge,
        None => {
            let now = Utc::now();
            let challenge =
                hex::encode(create_new_challenge(account, now, id_statement, identity_registry));
            let challenge = DbChallenge {
                challenge,
                address: account,
                created_at: now,
            };
            db.insert_challenge(challenge.clone()).await?;
            challenge
        }
    };

    Ok(challenge)
}

fn create_new_challenge(
    account: AccountAddress,
    now: DateTime<Utc>,
    id_statement: &IdStatement,
    identity_registry: &ContractAddress,
) -> [u8; 32] {
    // Create a new challenge based on various inputs
    let mut hasher = Sha256::new();
    hasher.update(to_bytes(id_statement));
    hasher.update(AsRef::<[u8; 32]>::as_ref(&account)); // Add type annotation to specify the implementation of AsRef to use
    hasher.update(identity_registry.index.to_be_bytes());
    hasher.update(identity_registry.subindex.to_be_bytes());
    hasher.update(now.to_rfc3339().as_bytes());

    let result = hasher.finalize();
    let mut challenge = [0; 32];
    challenge.copy_from_slice(&result);
    challenge
}
