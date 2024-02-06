use concordium_cis2::IsTokenId;
use concordium_std::*;

use super::tokens_state::ITokensState;

/// The `TokenSecurityState` struct represents the security state of a token.
#[derive(Serial, Deserial)]
pub struct TokenSecurityState {
    is_paused: bool,
}

impl TokenSecurityState {
    /// Creates a new instance of `TokenSecurityState` with the `is_paused`
    /// field set to `false`.
    pub fn new() -> Self {
        Self {
            is_paused: false,
        }
    }
}

impl Default for TokenSecurityState {
    fn default() -> Self { Self::new() }
}

pub enum TokenSecurityError {
    /// The token is paused.
    PausedToken,
}

/// The `ITokensSecurityState` trait defines the interface for managing the
/// security state of tokens.
pub trait ITokensSecurityState<T: IsTokenId, S: HasStateApi>: ITokensState<T, S> {
    /// Returns a reference to the map of security states for tokens.
    fn security_tokens(&self) -> &StateMap<T, TokenSecurityState, S>;

    /// Returns a mutable reference to the map of security states for tokens.
    fn security_tokens_mut(&mut self) -> &mut StateMap<T, TokenSecurityState, S>;

    /// Checks if the token with the given ID is paused.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the token is paused, `false` otherwise. If the token
    /// does not exist, returns `false`.
    fn is_paused(&self, token_id: &T) -> bool {
        self.security_tokens().get(token_id).map(|token| token.is_paused).unwrap_or(false)
    }

    /// Ensures that the given token is not paused.
    ///
    /// # Errors
    ///
    /// Returns a `TokenSecurityError::TokenPaused` error if the token is
    /// paused.
    fn ensure_not_paused(&self, token_id: &T) -> Result<(), TokenSecurityError> {
        if self.is_paused(token_id) {
            Err(TokenSecurityError::PausedToken)
        } else {
            Ok(())
        }
    }

    /// Pauses the token with the given ID.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to pause.
    ///
    /// If the token does not exist, it is created and then paused.
    fn pause(&mut self, token_id: T) {
        self.security_tokens_mut()
            .entry(token_id)
            .or_insert_with(TokenSecurityState::new)
            .modify(|token| token.is_paused = true)
    }

    /// Unpauses the token with the given ID.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the token to unpause.
    ///
    /// If the token does not exist, nothing happens.
    fn un_pause(&mut self, token_id: T) {
        self.security_tokens_mut().entry(token_id).and_modify(|token| token.is_paused = false);
    }
}
