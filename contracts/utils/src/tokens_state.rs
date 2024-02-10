use concordium_cis2::*;
use concordium_std::{ops, *};

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct TokensState<T, TTokenState, S> {
    tokens: StateMap<T, TTokenState, S>,
}

/// Trait representing a token amount.
///
/// This trait is used to define the behavior of token amounts.
pub trait IsTokenAmount:
    concordium_cis2::IsTokenAmount
    + PartialOrd
    + ops::SubAssign
    + Copy
    + ops::AddAssign
    + ops::Sub<Output = Self> {
    /// Returns the zero value of the token amount.
    fn zero() -> Self;

    /// Returns the maximum value of the token amount.
    /// This should return `1` for NFTs.
    fn max_value() -> Self;

    /// Subtracts the given amount from self. Returns None if the amount is too
    /// large.
    ///
    /// # Arguments
    ///
    /// * `other` - The amount to subtract.
    ///
    /// # Returns
    ///
    /// Returns `Some(())` if the subtraction was successful, `None` otherwise.
    fn checked_sub_assign(&mut self, other: Self) -> Option<()> {
        if other.le(self) {
            self.sub_assign(other);
            Some(())
        } else {
            None
        }
    }

    /// Adds the given amount to self. Returns None if the amount is too large.
    ///
    /// # Arguments
    ///
    /// * `other` - The amount to add.
    ///
    /// # Returns
    ///
    /// Returns `Some(())` if the addition was successful, `None` otherwise.
    fn checked_add_assign(&mut self, other: Self) -> Option<()> {
        if other.le(&Self::max_value().sub(*self)) {
            self.add_assign(other);
            Some(())
        } else {
            None
        }
    }

    /// Returns true if the amount is zero.
    fn is_zero(&self) -> bool { self.eq(&Self::zero()) }
}

pub enum TokenStateError {
    TokenAlreadyExists,
    TokenDoesNotExist,
}

pub type TokenStateResult<T> = Result<T, TokenStateError>;

pub trait ITokensState<T: IsTokenId, TTokenState: Serialize + Clone, S: HasStateApi> {
    fn tokens(&self) -> &StateMap<T, TTokenState, S>;
    fn tokens_mut(&mut self) -> &mut StateMap<T, TTokenState, S>;

    /// Checks if the token with the given ID exists.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to check.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the token exists,
    /// `Err(TokenStateError::TokenDoesNotExist)` otherwise.
    fn ensure_token_exists(&self, token_id: &T) -> TokenStateResult<()> {
        self.tokens().get(token_id).ok_or(TokenStateError::TokenDoesNotExist)?;
        Ok(())
    }

    fn token(&self, token_id: &T) -> TokenStateResult<TTokenState> {
        self.tokens()
            .get(token_id)
            .map(|token| token.clone())
            .ok_or(TokenStateError::TokenDoesNotExist)
    }

    /// Adds a token with the given ID and metadata URL to the state.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to add.
    /// * `metadata_url` - The metadata URL of the token to add.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the token was added successfully,
    /// `Err(TokenStateError::TokenAlreadyExists)` if the token already exists.
    fn add_token(&mut self, token_id: T, state: TTokenState) -> TokenStateResult<()> {
        self.tokens_mut()
            .entry(token_id)
            .vacant_or(TokenStateError::TokenAlreadyExists)?
            .insert(state);

        Ok(())
    }

    fn add_or_replace_token(&mut self, token_id: T, state: TTokenState) -> Option<TTokenState> {
        self.tokens_mut().insert(token_id, state)
    }
}
