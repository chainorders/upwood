use super::holders_state::{HolderBalances, HolderStateError, IHoldersState};
use concordium_protocols::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use concordium_std::*;

/// Represents the different types of errors that can occur in the holder's
/// security state.
pub enum HolderSecurityStateError {
    /// Triggered when the amount is too large for the operation.
    AmountTooLarge,
    /// Triggered when the amount causes an overflow.
    AmountOverflow,
    /// Triggered when the address has already been recovered.
    AddressAlreadyRecovered,
    /// Triggered when the recovery address provided is invalid.
    InvalidRecoveryAddress,
}

impl From<HolderStateError> for HolderSecurityStateError {
    fn from(e: HolderStateError) -> Self {
        match e {
            HolderStateError::AmountTooLarge => HolderSecurityStateError::AmountTooLarge,
            HolderStateError::AmountOverflow => HolderSecurityStateError::AmountOverflow,
        }
    }
}

pub type HolderSecurityStateResult<T> = Result<T, HolderSecurityStateError>;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct HoldersSecurityState<T, A, S> {
    recovery_addresses: StateMap<Address, Address, S>,
    identity_registry:  ContractAddress,
    compliance:         ContractAddress,
    frozen_balances:    StateMap<Address, HolderBalances<T, A, S>, S>,
}

impl<T, A, S: HasStateApi> HoldersSecurityState<T, A, S> {
    pub fn new(
        identity_registry: ContractAddress,
        compliance: ContractAddress,
        state_builder: &mut StateBuilder<S>,
    ) -> Self {
        Self {
            identity_registry,
            compliance,
            recovery_addresses: state_builder.new_map(),
            frozen_balances: state_builder.new_map(),
        }
    }
}

pub trait IHoldersSecurityState<T: IsTokenId, A: IsTokenAmount, S: HasStateApi>:
    IHoldersState<T, A, S> {
    /// Returns a reference to the holder's security state.
    fn state(&self) -> &HoldersSecurityState<T, A, S>;

    /// Returns a mutable reference to the holder's security state.
    fn state_mut(&mut self) -> &mut HoldersSecurityState<T, A, S>;

    /// Sets the compliance contract address.
    fn set_compliance(&mut self, compliance: ContractAddress) {
        self.state_mut().compliance = compliance;
    }

    /// Sets the identity registry contract address.
    fn set_identity_registry(&mut self, identity_registry: ContractAddress) {
        self.state_mut().identity_registry = identity_registry;
    }

    /// Returns the identity registry contract address.
    fn identity_registry(&self) -> ContractAddress { self.state().identity_registry }

    /// Returns the compliance contract address.
    fn compliance(&self) -> ContractAddress { self.state().compliance }

    /// Returns the recovery address for the given address if it exists.
    /// Retrieves the recovery address associated with the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address for which to retrieve the recovery address.
    ///
    /// # Returns
    ///
    /// An `Option<Address>` representing the recovery address, or `None` if no
    /// recovery address is found.
    fn get_recovery_address(&self, address: &Address) -> Option<Address> {
        self.state().recovery_addresses.get(address).map(|a| *a)
    }

    /// Ensures that the given address has not been recovered.
    ///
    /// # Errors
    ///
    /// Returns a `RecoveryError::AddressAlreadyRecovered` error if the address
    /// has already been recovered.
    fn ensure_not_recovered(&self, address: &Address) -> HolderSecurityStateResult<()> {
        ensure!(
            self.get_recovery_address(address).is_none(),
            HolderSecurityStateError::AddressAlreadyRecovered
        );

        Ok(())
    }

    /// Sets the recovery address for the given address.
    /// Sets the recovery address for a given address in the holders security
    /// state.
    ///
    /// # Arguments
    ///
    /// * `address` - The address for which the recovery address is being set.
    /// * `recovery_address` - The recovery address to be set.
    fn set_recovery_address(&mut self, address: Address, recovery_address: Address) {
        let _ = self.state_mut().recovery_addresses.insert(address, recovery_address);
    }

    /// Removes the recovery address of the given address and sets it to the new
    /// address. Also transfers any frozen balances from the old address to
    /// the new address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to remove the recovery address from.
    /// * `new_address` - The new address to set as the recovery address.
    ///
    /// # Errors
    ///
    /// Returns `RecoveryError::AddressAlreadyRecovered` if the address has
    /// already been recovered.
    /// Returns `RecoveryError::InvalidRecoveryAddress` if the new address
    /// already has state / frozen balances / already been recovered.
    fn recover(&mut self, address: Address, new_address: Address) -> HolderSecurityStateResult<()> {
        // The input address should not already have a recovery address.
        self.ensure_not_recovered(&address)?;
        // The new address should not already have a recovery address.
        self.ensure_not_recovered(&new_address)
            .map_err(|_| HolderSecurityStateError::InvalidRecoveryAddress)?;
        // replace holders state
        self.holders_mut()
            .remove_and_get(&address)
            .map(|old_address_state| self.holders_mut().insert(new_address, old_address_state))
            .map(|new_address_state| match new_address_state {
                // new address should not have State
                Some(_) => Err(HolderSecurityStateError::InvalidRecoveryAddress),
                None => Ok(()),
            })
            .transpose()?;
        // replace frozen balances
        self.state_mut()
            .frozen_balances
            .remove_and_get(&address)
            .map(|frozen_balances| {
                self.state_mut().frozen_balances.insert(new_address, frozen_balances)
            })
            .map(|new_address_state| match new_address_state {
                // new address should not have State
                Some(_) => Err(HolderSecurityStateError::InvalidRecoveryAddress),
                None => Ok(()),
            })
            .transpose()?;
        // set recovery address
        self.set_recovery_address(address, new_address);
        Ok(())
    }

    /// Adds a specified amount to the frozen balance of a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to freeze the balance for.
    /// * `token_id` - The ID of the token to freeze.
    /// * `amount` - The amount to freeze.
    /// * `state_builder` - A mutable reference to the state builder.
    fn freeze(
        &mut self,
        address: Address,
        token_id: &T,
        amount: A,
        state_builder: &mut StateBuilder<S>,
    ) -> HolderSecurityStateResult<()> {
        self.state_mut()
            .frozen_balances
            .entry(address)
            .or_insert_with(|| HolderBalances::new(state_builder))
            .add(token_id, amount)
            .map_err(|e| e.into())
    }

    /// Subtracts a specified amount from the frozen balance of a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to unfreeze the balance for.
    /// * `token_id` - The ID of the token to unfreeze.
    /// * `amount` - The amount to unfreeze.
    fn un_freeze(
        &mut self,
        address: Address,
        token_id: T,
        amount: A,
    ) -> HolderSecurityStateResult<()> {
        self.state_mut()
            .frozen_balances
            .entry(address)
            .occupied_or(HolderSecurityStateError::AmountTooLarge)?
            .sub(token_id, amount)
            .map_err(|e| e.into())
    }

    /// Returns the frozen balance of a given address for a specific token.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to get the frozen balance for.
    /// * `token_id` - The ID of the token to get the frozen balance for.
    fn balance_of_frozen(&self, address: &Address, token_id: &T) -> A {
        self.state()
            .frozen_balances
            .get(address)
            .map(|address| address.balance_of(token_id))
            .unwrap_or(A::zero())
    }

    /// Returns the unfrozen balance of the given token for the given addresses.
    ///
    /// # Parameters
    ///
    /// * `address`: The address to check.
    /// * `token_id`: The token ID to check.
    ///
    /// # Returns
    ///
    /// Returns `TokenAmount` containing the unfrozen balance.
    fn balance_of_unfrozen(&self, address: &Address, token_id: &T) -> A {
        let balance = self.balance_of(address, token_id);
        let frozen_balance = self.balance_of_frozen(address, token_id);
        balance.sub(frozen_balance)
    }

    /// Ensures that the holder has a sufficient unfrozen balance of the
    /// specified token.
    ///
    /// # Errors
    ///
    /// Returns a `HolderStateError::AmountTooLarge` error if the holder's
    /// unfrozen balance of the token is less than the specified amount.
    fn ensure_has_sufficient_unfrozen_balance(
        &self,
        address: &Address,
        token_id: &T,
        amount: &A,
    ) -> Result<(), HolderStateError> {
        let balance = self.balance_of_unfrozen(address, token_id);
        if balance.lt(amount) {
            Err(HolderStateError::AmountTooLarge)
        } else {
            Ok(())
        }
    }

    /// Adjusts the frozen balance of the specified token for the given address.
    ///
    /// This function calculates the difference between the frozen balance and
    /// the actual balance of the token. If the frozen balance is greater
    /// than the actual balance, it unfreezes the difference.
    ///
    /// # Returns
    ///
    /// Returns the amount that was unfrozen.
    ///
    /// # Errors
    ///
    /// Returns an error if the unfreeze operation fails.
    fn adjust_frozen_balance(
        &mut self,
        address: Address,
        token_id: T,
    ) -> HolderSecurityStateResult<A> {
        let frozen_balance = self.balance_of_frozen(&address, &token_id);
        let balance = self.balance_of(&address, &token_id);

        if frozen_balance.le(&balance) {
            return Ok(A::zero());
        }

        let in_compliant_amount = frozen_balance.sub(balance);
        self.un_freeze(address, token_id, in_compliant_amount)?;

        Ok(in_compliant_amount)
    }
}
