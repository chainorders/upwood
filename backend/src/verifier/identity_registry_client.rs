use chrono::Utc;
use concordium_rust_sdk::{
    base::{
        contracts_common::{NewReceiveNameError, ParseError, PublicKeyEd25519},
        hashes::TransactionHash,
        smart_contracts::{ExceedsParameterSize, OwnedContractName, OwnedParameter},
        transactions::send::GivenEnergy,
    },
    common::types::{Amount, TransactionTime},
    contract_client::{ContractClient, ContractTransactionMetadata},
    types::{AccountInfo, Address, ContractAddress, Energy, RejectReason, WalletAccount},
    v2::{self, BlockIdentifier},
};
use concordium_rwa_identity_registry::{identities::RegisterIdentityParams, types::Identity};
use concordium_rwa_utils::common_types::{IdentityAttribute, IdentityCredential};

use super::web3_id_utils::VerifyPresentationResponse;

pub enum Error {
    ParamsSerialization,
    AccountInfoQuery(v2::QueryError),
    TransactionSend(v2::RPCError),
    InstanceInvokeQuery(v2::QueryError),
    ReceiveName(NewReceiveNameError),
    ClientQuery(v2::QueryError),
    InvokeInstance(RejectReason),
    Parse(ParseError),
    Rpc(v2::RPCError),
    ExceedsParameterSize(ExceedsParameterSize),
}

impl From<v2::QueryError> for Error {
    fn from(e: v2::QueryError) -> Self { Error::ClientQuery(e) }
}

impl From<NewReceiveNameError> for Error {
    fn from(e: NewReceiveNameError) -> Self { Error::ReceiveName(e) }
}

impl From<RejectReason> for Error {
    fn from(e: RejectReason) -> Self { Error::InvokeInstance(e) }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self { Error::Parse(e) }
}

impl From<v2::RPCError> for Error {
    fn from(e: v2::RPCError) -> Self { Error::Rpc(e) }
}

impl From<ExceedsParameterSize> for Error {
    fn from(e: ExceedsParameterSize) -> Self { Error::ExceedsParameterSize(e) }
}

pub struct IdentityRegistryClient {
    pub client: ContractClient<Self>,
}

impl IdentityRegistryClient {
    pub fn new(concordium_client: v2::Client, identity_registry: ContractAddress) -> Self {
        let client = ContractClient::new(
            concordium_client,
            identity_registry,
            OwnedContractName::new_unchecked("init_rwa_identity_registry".to_owned()),
        );
        Self {
            client,
        }
    }

    pub async fn issuers(&mut self) -> Result<Vec<ContractAddress>, Error> {
        let res = self
            .client
            .view_raw::<Vec<ContractAddress>, Error>(
                "issuers",
                OwnedParameter::empty(),
                BlockIdentifier::LastFinal,
            )
            .await?;

        Ok(res)
    }

    pub async fn register_identity(
        &mut self,
        agent: &WalletAccount,
        address: Address,
        verification_response: VerifyPresentationResponse,
        energy: Energy,
    ) -> Result<TransactionHash, Error> {
        let AccountInfo {
            account_nonce,
            ..
        } = self
            .client
            .client
            .get_account_info(&agent.address.into(), BlockIdentifier::LastFinal)
            .await
            .map_err(Error::AccountInfoQuery)?
            .response;
        let register_identity_payload = RegisterIdentityParams {
            address,
            identity: Identity {
                attributes:  verification_response
                    .revealed_attributes
                    .iter()
                    .map::<Result<IdentityAttribute, _>, _>(|(tag, value)| {
                        Ok(IdentityAttribute {
                            tag:   tag.0,
                            value: value.to_string(),
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?,
                credentials: verification_response
                    .credentials
                    .iter()
                    .map(|(contract_address, key)| IdentityCredential {
                        issuer: contract_address.to_owned(),
                        key:    PublicKeyEd25519(key.public_key.to_bytes()),
                    })
                    .collect(),
            },
        };
        let txn = self
            .client
            .update::<_, Error>(
                &agent.keys,
                &ContractTransactionMetadata {
                    nonce:          account_nonce,
                    expiry:         default_expiry_time(),
                    sender_address: agent.address,
                    amount:         Amount::zero(),
                    energy:         GivenEnergy::Absolute(energy),
                },
                "registerIdentity",
                &register_identity_payload,
            )
            .await?;
        Ok(txn)
    }
}

fn default_expiry_time() -> TransactionTime {
    TransactionTime::from_seconds((Utc::now().timestamp() + 300) as u64)
}
