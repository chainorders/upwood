use concordium_cis2::*;
use concordium_std::{ops, *};

/// `TokenState` is a struct that holds the state of a token.
#[derive(Serial, Deserial)]
pub struct TokenState {
    /// The metadata URL of the token.
    metadata_url: MetadataUrl,
}

impl TokenState {
    /// Creates a new `TokenState` with the given metadata URL.
    ///
    /// # Arguments
    ///
    /// * `metadata_url` - The metadata URL of the token.
    ///
    /// # Returns
    ///
    /// Returns a new `TokenState` with the given metadata URL.
    pub fn new(metadata_url: MetadataUrl) -> Self {
        Self {
            metadata_url,
        }
    }

    /// Returns the metadata URL of the token.
    ///
    /// # Returns
    ///
    /// Returns a reference to the metadata URL of the token.
    pub fn metadata_url(&self) -> &MetadataUrl { &self.metadata_url }
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct TokensState<T, S> {
    tokens: StateMap<T, TokenState, S>,
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
}

pub enum TokenStateError {
    TokenAlreadyExists,
    TokenDoesNotExist,
}

pub type TokenStateResult<T> = Result<T, TokenStateError>;

pub trait ITokensState<T: IsTokenId, S: HasStateApi> {
    fn tokens(&self) -> &StateMap<T, TokenState, S>;
    fn tokens_mut(&mut self) -> &mut StateMap<T, TokenState, S>;

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

    /// Returns the metadata URL of the token with the given ID.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to get the metadata URL for.
    ///
    /// # Returns
    ///
    /// Returns `Ok(MetadataUrl)` if the token exists,
    /// `Err(TokenStateError::TokenDoesNotExist)` otherwise.
    fn token_metadata_url(&self, token_id: &T) -> TokenStateResult<MetadataUrl> {
        self.tokens()
            .get(token_id)
            .map(|token| token.metadata_url().to_owned())
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
    fn add_token(&mut self, token_id: T, metadata_url: MetadataUrl) -> TokenStateResult<()> {
        self.tokens_mut()
            .entry(token_id)
            .vacant_or(TokenStateError::TokenAlreadyExists)?
            .insert(TokenState::new(metadata_url));

        Ok(())
    }
}
