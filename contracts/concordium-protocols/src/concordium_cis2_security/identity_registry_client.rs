use concordium_std::{
    Address, ContractAddress, DeserialWithState, EntrypointName, ExternStateApi, Host, Serial,
};

use super::{Identity, IDENTITY_REGISTRY_STANDARD_IDENTIFIER};
use crate::contract_client::{invoke_contract_read_only, supports, ContractClientError};

pub type IdentityRegistryClientError = ContractClientError<()>;

pub trait IdentityRegistryClient {
    fn invoke_identity_registry_get_identity(
        &self,
        contract: &ContractAddress,
        address: Address,
    ) -> Result<Identity, IdentityRegistryClientError>;

    fn invoke_identity_registry_is_verified(
        &self,
        contract: &ContractAddress,
        address: &Address,
    ) -> Result<bool, IdentityRegistryClientError>;

    fn invoke_supports_rwa_identity_registry_standard(
        &self,
        contract: &ContractAddress,
    ) -> Result<bool, IdentityRegistryClientError>;
}

impl<S> IdentityRegistryClient for Host<S>
where S: Serial+DeserialWithState<ExternStateApi>
{
    fn invoke_identity_registry_get_identity(
        &self,
        contract: &ContractAddress,
        address: Address,
    ) -> Result<Identity, IdentityRegistryClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("getIdentity"),
            &address,
        )
    }

    fn invoke_identity_registry_is_verified(
        &self,
        contract: &ContractAddress,
        address: &Address,
    ) -> Result<bool, IdentityRegistryClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("isVerified"),
            address,
        )
    }

    fn invoke_supports_rwa_identity_registry_standard(
        &self,
        contract: &ContractAddress,
    ) -> Result<bool, IdentityRegistryClientError> {
        supports(self, contract, IDENTITY_REGISTRY_STANDARD_IDENTIFIER)
    }
}
