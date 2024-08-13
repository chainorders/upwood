use concordium_cis2::StandardIdentifier;
use concordium_protocols::concordium_cis2_security::Identity;
use concordium_std::{Address, ContractAddress, EntrypointName, Host};

use super::contract_client::{ContractClientError, IContractClient, IContractState};

pub const IDENTITY_REGISTRY_STANDARD_IDENTIFIER: StandardIdentifier =
    StandardIdentifier::new_unchecked("rwa_identity_registry");
const IDENTITY_REGISTRY_IS_VERIFIED_ENTRYPOINT_NAME: EntrypointName =
    EntrypointName::new_unchecked("isVerified");
const IDENTITY_REGISTRY_IS_SAME_ENTRYPOINT_NAME: EntrypointName =
    EntrypointName::new_unchecked("isSame");
const IDENTITY_REGISTRY_GET_IDENTITY_ENTRYPOINT_NAME: EntrypointName =
    EntrypointName::new_unchecked("getIdentity");

/// The identity registry contract.
/// The identity registry contract is used to check if an address is verified
pub struct IdentityRegistryContract(pub ContractAddress);
impl<S: IContractState> IContractClient<S> for IdentityRegistryContract {
    fn contract_address(&self) -> ContractAddress { self.0 }

    fn standard_identifier(&self) -> StandardIdentifier { IDENTITY_REGISTRY_STANDARD_IDENTIFIER }
}

pub type IdentityRegistryClientError = ContractClientError<()>;

/// A client for the identity registry contract.
pub trait IdentityRegistryClient<S: IContractState>: IContractClient<S> {
    /// Checks if the given address is verified.
    ///
    /// # Arguments
    ///
    /// * `host` - A reference to the host.
    /// * `address` - The address to check.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the address is
    /// verified.
    fn is_verified(
        &self,
        host: &Host<S>,
        address: &Address,
    ) -> Result<bool, IdentityRegistryClientError> {
        let res = self.invoke_contract_read_only(
            host,
            IDENTITY_REGISTRY_IS_VERIFIED_ENTRYPOINT_NAME,
            address,
        )?;

        Ok(res)
    }

    /// Checks if the two given addresses are the same.
    ///
    /// # Arguments
    ///
    /// * `host` - A reference to the host.
    /// * `address1` - The first address to compare.
    /// * `address2` - The second address to compare.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the two addresses are
    /// the same.
    fn is_same(
        &self,
        host: &Host<S>,
        address1: &Address,
        address2: &Address,
    ) -> Result<bool, IdentityRegistryClientError> {
        let res = self.invoke_contract_read_only(
            host,
            IDENTITY_REGISTRY_IS_SAME_ENTRYPOINT_NAME,
            &(address1, address2),
        )?;

        Ok(res)
    }

    /// Gets the identity of the given address.
    fn get_identity(
        &self,
        host: &Host<S>,
        address: Address,
    ) -> Result<Identity, IdentityRegistryClientError> {
        let res = self.invoke_contract_read_only(
            host,
            IDENTITY_REGISTRY_GET_IDENTITY_ENTRYPOINT_NAME,
            &address,
        )?;

        Ok(res)
    }
}

impl<S: IContractState> IdentityRegistryClient<S> for IdentityRegistryContract {}
