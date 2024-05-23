use super::contract_client::{ContractClientError, IContractClient, IContractState};
use crate::compliance_types::*;
use concordium_cis2::StandardIdentifier;
use concordium_protocols::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use concordium_std::{Address, ContractAddress, EntrypointName, Host};

const COMPLIANCE_CAN_TRANSFER_ENTRYPOINT: EntrypointName =
    EntrypointName::new_unchecked("canTransfer");
const COMPLIANCE_BURNED_ENTRYPOINT: EntrypointName = EntrypointName::new_unchecked("burned");
const COMPLIANCE_MINTED_ENTRYPOINT: EntrypointName = EntrypointName::new_unchecked("minted");
const COMPLIANCE_TRANSFERRED_ENTRYPOINT: EntrypointName =
    EntrypointName::new_unchecked("transferred");
pub const COMPLIANCE_STANDARD_IDENTIFIER: StandardIdentifier =
    StandardIdentifier::new_unchecked("rwa_compliance");

/// The compliance contract.
/// The compliance contract is used to check if a transfer can be made.
pub struct ComplianceContract(pub ContractAddress);

pub type ComplianceError = ContractClientError<()>;

/// A client for the compliance contract.
/// The compliance contract is used to check if a transfer can be made.
pub trait IComplianceClient<T: IsTokenId, A: IsTokenAmount, S: IContractState>:
    IContractClient<S> {
    /// Checks if a transfer can be made.
    ///
    /// # Arguments
    ///
    /// * `host` - A reference to the host.
    /// * `token_id` - The ID of the token to transfer.
    /// * `from` - The address to transfer from.
    /// * `to` - The address to transfer to.
    /// * `amount` - The amount of tokens to transfer.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the transfer can be
    /// made.
    fn can_transfer(
        &self,
        host: &Host<S>,
        token_id: Token<T>,
        to: Address,
        amount: A,
    ) -> Result<bool, ComplianceError> {
        let res = self.invoke_contract_read_only(
            host,
            COMPLIANCE_CAN_TRANSFER_ENTRYPOINT,
            &CanTransferParam {
                token_id,
                to,
                amount,
            },
        )?;

        Ok(res)
    }

    /// Notifies the contract that tokens have been burned.
    ///
    /// # Arguments
    ///
    /// * `host` - A reference to the host.
    /// * `token_id` - The ID of the token that was burned.
    /// * `owner` - The address of the owner of the burned tokens.
    /// * `amount` - The amount of tokens that were burned.
    fn burned(
        &self,
        host: &Host<S>,
        token_id: Token<T>,
        owner: Address,
        amount: A,
    ) -> Result<(), ComplianceError> {
        self.invoke_contract_read_only(host, COMPLIANCE_BURNED_ENTRYPOINT, &BurnedParam {
            token_id,
            owner,
            amount,
        })?;

        Ok(())
    }

    /// Notifies the contract that tokens have been minted.
    ///
    /// # Arguments
    ///
    /// * `host` - A reference to the host.
    /// * `token_id` - The ID of the token that was minted.
    /// * `owner` - The address of the owner of the minted tokens.
    /// * `amount` - The amount of tokens that were minted.
    fn minted(
        &self,
        host: &Host<S>,
        token_id: Token<T>,
        owner: Address,
        amount: A,
    ) -> Result<(), ComplianceError> {
        self.invoke_contract_read_only(host, COMPLIANCE_MINTED_ENTRYPOINT, &MintedParam {
            token_id,
            owner,
            amount,
        })?;

        Ok(())
    }

    /// Notifies the contract that tokens have been transferred.
    ///
    /// # Arguments
    ///
    /// * `host` - A reference to the host.
    /// * `token_id` - The ID of the token that was transferred.
    /// * `from` - The address of the sender of the transfer.
    /// * `to` - The address of the receiver of the transfer.
    /// * `amount` - The amount of tokens that were transferred.
    fn transferred(
        &self,
        host: &Host<S>,
        token_id: Token<T>,
        from: Address,
        to: Address,
        amount: A,
    ) -> Result<(), ComplianceError> {
        self.invoke_contract_read_only(
            host,
            COMPLIANCE_TRANSFERRED_ENTRYPOINT,
            &TransferredParam {
                token_id,
                from,
                to,
                amount,
            },
        )?;

        Ok(())
    }
}

impl<S: IContractState> IContractClient<S> for ComplianceContract {
    fn contract_address(&self) -> ContractAddress { self.0 }

    fn standard_identifier(&self) -> concordium_cis2::StandardIdentifier {
        COMPLIANCE_STANDARD_IDENTIFIER
    }
}

impl<T: IsTokenId, A: IsTokenAmount, S: IContractState> IComplianceClient<T, A, S>
    for ComplianceContract
{
}
