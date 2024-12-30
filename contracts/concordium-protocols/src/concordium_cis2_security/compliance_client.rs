use concordium_std::{
    ContractAddress, DeserialWithState, EntrypointName, ExternStateApi, Host, Serial,
};

use super::{
    BurnedParam, CanTransferParam, MintedParam, TransferredParam, COMPLIANCE_STANDARD_IDENTIFIER,
};
use crate::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use crate::contract_client::{
    invoke_contract, invoke_contract_read_only, supports, ContractClientError,
};

pub type ComplianceError = ContractClientError<()>;

pub trait ComplianceClient<T: IsTokenId, A: IsTokenAmount> {
    fn invoke_compiliance_can_transfer(
        &self,
        address: &ContractAddress,
        params: &CanTransferParam<T, A>,
    ) -> Result<bool, ContractClientError<()>>;

    fn invoke_compiliance_transferred(
        &mut self,
        address: &ContractAddress,
        params: &TransferredParam<T, A>,
    ) -> Result<(), ContractClientError<()>>;

    fn invoke_compiliance_burned(
        &mut self,
        address: &ContractAddress,
        params: &BurnedParam<T, A>,
    ) -> Result<(), ContractClientError<()>>;

    fn invoke_compiliance_minted(
        &mut self,
        address: &ContractAddress,
        params: &MintedParam<T, A>,
    ) -> Result<(), ContractClientError<()>>;

    fn invoke_supports_rwa_compliance_standard(
        &self,
        address: &ContractAddress,
    ) -> Result<bool, ContractClientError<()>>;
}

impl<S, T, A> ComplianceClient<T, A> for Host<S>
where
    T: IsTokenId,
    A: IsTokenAmount,
    S: Serial+DeserialWithState<ExternStateApi>,
{
    #[inline]
    fn invoke_compiliance_can_transfer(
        &self,
        address: &ContractAddress,
        params: &CanTransferParam<T, A>,
    ) -> Result<bool, ContractClientError<()>> {
        invoke_contract_read_only(
            self,
            address,
            EntrypointName::new_unchecked("canTransfer"),
            params,
        )
    }

    #[inline]
    fn invoke_compiliance_transferred(
        &mut self,
        address: &ContractAddress,
        params: &TransferredParam<T, A>,
    ) -> Result<(), ContractClientError<()>> {
        invoke_contract(
            self,
            address,
            EntrypointName::new_unchecked("transferred"),
            params,
        )
    }

    #[inline]
    fn invoke_compiliance_burned(
        &mut self,
        address: &ContractAddress,
        params: &BurnedParam<T, A>,
    ) -> Result<(), ContractClientError<()>> {
        invoke_contract(
            self,
            address,
            EntrypointName::new_unchecked("burned"),
            params,
        )
    }

    #[inline]
    fn invoke_compiliance_minted(
        &mut self,
        address: &ContractAddress,
        params: &MintedParam<T, A>,
    ) -> Result<(), ContractClientError<()>> {
        invoke_contract(
            self,
            address,
            EntrypointName::new_unchecked("minted"),
            params,
        )
    }

    #[inline]
    fn invoke_supports_rwa_compliance_standard(
        &self,
        address: &ContractAddress,
    ) -> Result<bool, ContractClientError<()>> {
        supports(self, address, COMPLIANCE_STANDARD_IDENTIFIER)
    }
}
