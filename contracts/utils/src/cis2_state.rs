use super::{
    holders_state::{HolderStateError, IHoldersState},
    tokens_state::{ITokensState, TokenStateError},
};
use crate::clients::contract_client::IContractState;
use concordium_protocols::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use concordium_std::*;

pub type Cis2Result<R> = Result<R, Cis2StateError>;

pub enum Cis2StateError {
    InvalidTokenId,
    InsufficientFunds,
    InvalidAmount,
}

pub trait ICis2State<T: IsTokenId, A: IsTokenAmount, TTokenState: Serialize + Clone, S: HasStateApi>:
    IContractState + IHoldersState<T, A, S> + ITokensState<T, TTokenState, S> {
    /// Mints a token.
    ///
    /// This function mints a new token with the specified `token_id` and
    /// `metadata_url`. It also initializes the balances of the token for
    /// the given `balances` vector. The `state_builder` is used to update
    /// the state of the contract.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to be minted.
    /// * `metadata_url` - The URL of the metadata associated with the token.
    /// * `balances` - A vector of tuples containing the address and amount of
    ///   tokens to be minted.
    /// * `state_builder` - A mutable reference to the `StateBuilder` used to
    ///   update the contract state.
    ///
    /// # Returns
    ///
    /// This function returns `Ok(())` if the token minting and balance
    /// initialization is successful. Otherwise, it returns an error of type
    /// `Cis2Result`.
    fn mint_token(
        &mut self,
        token_id: T,
        state: TTokenState,
        balances: Vec<(Address, A)>,
        state_builder: &mut StateBuilder<S>,
    ) -> Cis2Result<()> {
        self.add_token(token_id.to_owned(), state)?;
        for (address, amount) in balances {
            self.add_balance(address, &token_id, amount, state_builder)?;
        }

        Ok(())
    }
}

impl From<TokenStateError> for Cis2StateError {
    fn from(value: TokenStateError) -> Self {
        match value {
            TokenStateError::TokenAlreadyExists => Cis2StateError::InvalidTokenId,
            TokenStateError::TokenDoesNotExist => Cis2StateError::InvalidTokenId,
        }
    }
}

impl From<HolderStateError> for Cis2StateError {
    fn from(value: HolderStateError) -> Self {
        match value {
            HolderStateError::AmountTooLarge => Cis2StateError::InsufficientFunds,
            HolderStateError::AmountOverflow => Cis2StateError::InvalidAmount,
        }
    }
}
