use chrono::Utc;
use concordium_rust_sdk::{
    base::{
        contracts_common::NewReceiveNameError,
        hashes::TransactionHash,
        smart_contracts::{ExceedsParameterSize, OwnedContractName},
        transactions::send::GivenEnergy,
    },
    common::types::{Amount, TransactionTime},
    contract_client::{ContractClient, ContractTransactionMetadata},
    types::{AccountInfo, ContractAddress, Energy, WalletAccount},
    v2::{self, BlockIdentifier},
};
use concordium_rwa_sponsor::types::PermitParam;

const SPONSOR_CONTRACT_NAME: &str = "init_rwa_sponsor";

pub enum Error {
    AccountInfoQuery(v2::QueryError),
    Rpc(v2::RPCError),
    ExceedsParameterSize(ExceedsParameterSize),
    ReceiveName(NewReceiveNameError),
}

impl From<NewReceiveNameError> for Error {
    fn from(e: NewReceiveNameError) -> Self { Error::ReceiveName(e) }
}

impl From<v2::RPCError> for Error {
    fn from(e: v2::RPCError) -> Self { Error::Rpc(e) }
}

impl From<ExceedsParameterSize> for Error {
    fn from(e: ExceedsParameterSize) -> Self { Error::ExceedsParameterSize(e) }
}

pub struct SponsorClient {
    pub client: ContractClient<Self>,
}

impl SponsorClient {
    pub fn new(concordium_client: v2::Client, sponsor_contract: ContractAddress) -> Self {
        let client = ContractClient::new(
            concordium_client,
            sponsor_contract,
            OwnedContractName::new_unchecked(SPONSOR_CONTRACT_NAME.to_owned()),
        );
        Self {
            client,
        }
    }

    pub async fn permit(
        &mut self,
        sponsor: &WalletAccount,
        energy: Energy,
        param: PermitParam,
    ) -> Result<TransactionHash, Error> {
        let AccountInfo {
            account_nonce,
            ..
        } = self
            .client
            .client
            .get_account_info(&sponsor.address.into(), BlockIdentifier::LastFinal)
            .await
            .map_err(Error::AccountInfoQuery)?
            .response;

        let txn = self
            .client
            .update::<_, Error>(
                &sponsor.keys,
                &ContractTransactionMetadata {
                    nonce:          account_nonce,
                    expiry:         default_expiry_time(),
                    sender_address: sponsor.address,
                    amount:         Amount::zero(),
                    energy:         GivenEnergy::Absolute(energy),
                },
                "registerIdentity",
                &param,
            )
            .await?;

        Ok(txn)
    }
}

fn default_expiry_time() -> TransactionTime {
    TransactionTime::from_seconds((Utc::now().timestamp() + 300) as u64)
}
