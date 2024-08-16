use concordium_protocols::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use concordium_std::*;

pub enum HolderStateError {
    AmountTooLarge,
    AmountOverflow,
}
pub type HolderStateResult<T> = Result<T, HolderStateError>;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct HolderBalances<T, A, S> {
    balances: StateMap<T, A, S>,
}

impl<T: IsTokenId, A: IsTokenAmount, S: HasStateApi> HolderBalances<T, A, S> {
    pub fn new(state_builder: &mut StateBuilder<S>) -> Self {
        Self {
            balances: state_builder.new_map(),
        }
    }

    pub fn balance_of(&self, token_id: &T) -> A {
        self.balances.get(token_id).map(|a| *a).unwrap_or(A::zero())
    }

    pub fn sub(&mut self, token_id: &T, amount: &A) -> HolderStateResult<()> {
        let amount = self
            .balances
            .entry(token_id.to_owned())
            .occupied_or(HolderStateError::AmountTooLarge)?
            .try_modify(|e| {
                e.checked_sub_assign(*amount)
                    .ok_or(HolderStateError::AmountTooLarge)?;
                Ok(e.to_owned())
            })?;

        if A::zero().eq(&amount) {
            self.balances.remove(&token_id);
        }

        Ok(())
    }

    pub fn add(&mut self, token_id: &T, amount: &A) -> HolderStateResult<()> {
        self.balances
            .entry(token_id.to_owned())
            .or_insert_with(|| A::zero())
            .try_modify(|e| {
                e.checked_add_assign(*amount)
                    .ok_or(HolderStateError::AmountOverflow)
            })
    }
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// The state of a single address for all tokens.
///
/// This struct holds the operators and balances for a single address. The operators
/// are stored in a `StateSet`, and the balances are stored in a `HolderBalances`
/// struct.
pub struct HolderState<T, A, S> {
    operators: StateSet<Address, S>,
    balances:  HolderBalances<T, A, S>,
}

impl<T: IsTokenId, A: IsTokenAmount, S: HasStateApi> HolderState<T, A, S> {
    /// Creates a new `HolderState` with empty operators and balances.
    ///
    /// # Arguments
    ///
    /// * `state_builder` - A mutable reference to the state builder.
    pub fn new(state_builder: &mut StateBuilder<S>) -> Self {
        Self {
            operators: state_builder.new_set(),
            balances:  HolderBalances::new(state_builder),
        }
    }

    /// Checks if the given address is an operator for the holder.
    ///
    /// # Arguments
    ///
    /// * `operator` - The address to check.
    pub fn is_operator(&self, operator: &Address) -> bool { self.operators.contains(operator) }

    /// Adds an operator for the holder.
    ///
    /// # Arguments
    ///
    /// * `operator` - The address of the operator to add.
    pub fn add_operator(&mut self, operator: Address) { self.operators.insert(operator); }

    /// Removes an operator from the holder.
    ///
    /// # Arguments
    ///
    /// * `operator` - The address of the operator to remove.
    pub fn remove_operator(&mut self, operator: &Address) { self.operators.remove(operator); }

    /// Returns the balance of a specific token for the holder.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to get the balance for.
    pub fn balance_of(&self, token_id: &T) -> A { self.balances.balance_of(token_id) }

    /// Subtracts a specified amount from the balance of a specific token for
    /// the holder.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to subtract from.
    /// * `amount` - The amount to subtract.
    ///
    /// # Errors
    ///
    /// Returns `HolderStateError::AmountTooLarge` if the amount to subtract is
    /// larger than the current balance.
    pub fn sub(&mut self, token_id: &T, amount: &A) -> HolderStateResult<()> {
        self.balances.sub(token_id, amount)
    }

    /// Adds a specified amount to the balance of a specific token for the
    /// holder.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to add to.
    /// * `amount` - The amount to add.
    ///
    /// # Errors
    ///
    /// Returns `HolderStateError::AmountOverflow` if the amount to add and the
    /// current balance exceed the maximum value of `A`.
    pub fn add(&mut self, token_id: &T, amount: &A) -> HolderStateResult<()> {
        self.balances.add(token_id, amount)
    }
}

pub trait IHoldersState<T: IsTokenId, A: IsTokenAmount, S: HasStateApi> {
    fn holders(&self) -> &StateMap<Address, HolderState<T, A, S>, S>;
    fn holders_mut(&mut self) -> &mut StateMap<Address, HolderState<T, A, S>, S>;

    /// Checks if the given operator is an operator for the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to check.
    /// * `operator` - The operator to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the operator is an operator for the address, `false`
    /// otherwise.
    fn is_operator(&self, address: &Address, operator: &Address) -> bool {
        self.holders()
            .get(address)
            .map(|address| address.is_operator(operator))
            .unwrap_or(false)
    }

    /// Adds an operator for the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to add an operator for.
    /// * `operator` - The operator to add.
    /// * `state_builder` - A mutable reference to the state builder.
    fn add_operator(
        &mut self,
        address: Address,
        operator: Address,
        state_builder: &mut StateBuilder<S>,
    ) {
        self.holders_mut()
            .entry(address)
            .or_insert_with(|| HolderState::new(state_builder))
            .add_operator(operator);
    }

    /// Removes an operator from the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to remove an operator from.
    /// * `operator` - The operator to remove.
    fn remove_operator(&mut self, address: Address, operator: &Address) {
        self.holders_mut().entry(address).and_modify(|address| {
            address.remove_operator(operator);
        });
    }

    /// Returns the balance of the given address for the given token.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to get the balance for.
    /// * `token_id` - The ID of the token to get the balance for.
    ///
    /// # Returns
    ///
    /// Returns the balance of the given token for the given address. If the
    /// address does not exist, returns zero.
    fn balance_of(&self, address: &Address, token_id: &T) -> A {
        self.holders()
            .get(address)
            .map(|address| address.balance_of(token_id))
            .unwrap_or(A::zero())
    }

    fn ensure_has_sufficient_balance(
        &self,
        address: &Address,
        token_id: &T,
        amount: &A,
    ) -> HolderStateResult<()> {
        let balance = self.balance_of(address, token_id);
        if balance.lt(amount) {
            Err(HolderStateError::AmountTooLarge)
        } else {
            Ok(())
        }
    }

    /// Adds a specified amount to the balance of a specific token for the given
    /// address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to add the balance to.
    /// * `token_id` - The ID of the token to add the balance to.
    /// * `amount` - The amount to add.
    /// * `state_builder` - A mutable reference to the state builder.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful. If the address does
    /// not exist, it is created.
    ///
    /// # Errors
    ///
    /// Returns `HolderStateError::AmountOverflow` if the amount to add and the
    /// current balance exceed the maximum value of `A`.
    fn add_balance(
        &mut self,
        address: &Address,
        token_id: &T,
        amount: &A,
        state_builder: &mut StateBuilder<S>,
    ) -> HolderStateResult<()> {
        self.holders_mut()
            .entry(*address)
            .or_insert_with(|| HolderState::new(state_builder))
            .add(token_id, amount)
    }

    /// Subtracts a specified amount from the balance of a specific token for
    /// the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to subtract the balance from.
    /// * `token_id` - The ID of the token to subtract the balance from.
    /// * `amount` - The amount to subtract.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful.
    ///
    /// # Errors
    ///
    /// Returns `HolderStateError::AmountTooLarge` if the amount to subtract is
    /// larger than the current balance.
    fn sub_balance(
        &mut self,
        address: &Address,
        token_id: &T,
        amount: &A,
    ) -> HolderStateResult<()> {
        self.holders_mut()
            .entry(*address)
            .occupied_or(HolderStateError::AmountTooLarge)?
            .try_modify(|address| address.sub(token_id, amount))
    }

    /// Transfers a specified amount of a specific token from one address to
    /// another.
    ///
    /// # Arguments
    ///
    /// * `from` - The address to transfer from.
    /// * `to` - The address to transfer to.
    /// * `token_id` - The ID of the token to transfer.
    /// * `amount` - The amount to transfer.
    /// * `state_builder` - A mutable reference to the state builder.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful.
    ///
    /// # Errors
    ///
    /// Returns `HolderStateError::AmountTooLarge` if the amount to transfer is
    /// larger than the current balance of the `from` address.
    /// Returns `HolderStateError::AmountOverflow` if the amount to add and the
    /// current balance of the `to` address exceed the maximum value of `A`.
    fn transfer(
        &mut self,
        from: &Address,
        to: &Address,
        token_id: &T,
        amount: &A,
        state_builder: &mut StateBuilder<S>,
    ) -> HolderStateResult<()> {
        self.sub_balance(from, token_id, amount)?;
        self.add_balance(to, token_id, amount, state_builder)?;

        Ok(())
    }
}
