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

/// Represents the possible errors that can occur in the SponsorClient.
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

/// Represents a client for interacting with the Sponsor contract.
pub struct SponsorClient {
    pub client: ContractClient<Self>,
}

impl SponsorClient {
    /// Creates a new instance of SponsorClient.
    ///
    /// # Arguments
    ///
    /// * `concordium_client` - The Concordium client used for interacting with the blockchain.
    /// * `sponsor_contract` - The address of the Sponsor contract.
    ///
    /// # Returns
    ///
    /// A new instance of SponsorClient.
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

    /// Sends a permit transaction to the Sponsor contract.
    ///
    /// # Arguments
    ///
    /// * `sponsor` - The wallet account of the sponsor.
    /// * `energy` - The amount of energy to be used for the transaction.
    /// * `param` - The permit parameters.
    ///
    /// # Returns
    ///
    /// The transaction hash of the permit transaction.
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem querying the account info, sending the transaction,
    /// or if the parameter size exceeds the limit.
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
                "permit",
                &param,
            )
            .await?;

        Ok(txn)
    }
}

/// Returns the default expiry time for a transaction.
fn default_expiry_time() -> TransactionTime {
    TransactionTime::from_seconds((Utc::now().timestamp() + 300) as u64)
}
