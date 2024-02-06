use concordium_cis2::StandardIdentifier;
use concordium_std::*;

use super::contract_client::*;

const CIS4_SUPPORTS_IDENTIFIER: StandardIdentifier = StandardIdentifier::new_unchecked("CIS-4");
const CIS4_CREDENTIAL_STATUS_ENTRYPOINT: EntrypointName =
    EntrypointName::new_unchecked("credentialStatus");
const CIS4_ISSUER_ENTRYPOINT: EntrypointName = EntrypointName::new_unchecked("issuer");

/// The [credential holder id](https://proposals.concordium.software/CIS/cis-4.html#credentialholderid).
pub type CredentialHolderId = PublicKeyEd25519;

/// The [status of a credential](https://proposals.concordium.software/CIS/cis-4.html#credentialstatus).
#[derive(Serial, SchemaType, Deserial, PartialEq, Eq)]
pub enum CredentialStatus {
    Active,
    Revoked,
    Expired,
    NotActivated,
}

pub type Cis4ClientError = ContractClientError<()>;
pub struct Cis4ContractAddress(pub ContractAddress);

impl<State: IContractState> IContractClient<State> for Cis4ContractAddress {
    fn contract_address(&self) -> ContractAddress { self.0 }

    fn standard_identifier(&self) -> concordium_cis2::StandardIdentifier {
        CIS4_SUPPORTS_IDENTIFIER
    }
}

impl<State: IContractState> Cis4Client<State> for Cis4ContractAddress {}

pub trait Cis4Client<State: IContractState>: IContractClient<State> {
    fn credential_status(
        &self,
        host: &Host<State>,
        credential_holder_id: CredentialHolderId,
    ) -> Result<CredentialStatus, Cis4ClientError> {
        self.invoke_contract_read_only(
            host,
            CIS4_CREDENTIAL_STATUS_ENTRYPOINT,
            &Parameter::new_unchecked(&credential_holder_id.0),
        )
    }

    fn issuer(&self, host: &Host<State>) -> Result<PublicKeyEd25519, Cis4ClientError> {
        self.invoke_contract_read_only(host, CIS4_ISSUER_ENTRYPOINT, &Parameter::empty())
    }
}
