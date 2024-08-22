use concordium_cis2::TokenIdUnit;
use concordium_protocols::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use concordium_std::{Address, HasStateApi, StateBuilder};

use super::cis2_state::{Cis2StateError, ICis2SingleState, ICis2State, ICis2TokenState};
use super::holders_security_state::{
    HolderSecurityStateError, IHoldersSecurityState, ISecurityHolderState,
};
use super::holders_state::IHolderState;
use super::tokens_security_state::{ISecurityTokenState, ITokensSecurityState, TokenSecurityError};

pub trait ICis2SecurityTokenState<A, S: HasStateApi>:
    ISecurityTokenState<S>+ICis2TokenState<A, S> {
}

/// Trait representing the security state of the Cis2 contract.
/// It combines the functionality of `ITokensSecurityState` and
/// `IHoldersSecurityState` traits.
pub trait ICis2SecurityState<
    T: IsTokenId,
    A: IsTokenAmount,
    TTokenState: ICis2SecurityTokenState<A, S>,
    THolderState: IHolderState<T, A, S>+ISecurityHolderState<T, A, S>,
    S: HasStateApi,
>: ITokensSecurityState<T, TTokenState, S>+IHoldersSecurityState<T, A, THolderState, S>+ICis2State<T, A, TTokenState, THolderState, S>
{
    fn mint(
        &mut self,
        token_id: &T,
        amount: A,
        to: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> Result<(), Cis2SecurityStateError> {
        self.ensure_not_paused(token_id)?;
        self.ensure_not_recovered(to)?;
        ICis2State::mint(self, token_id, amount, to, state_builder)?;
        Ok(())
    }

    fn burn(
        &mut self,
        token_id: &T,
        amount: A,
        from: &Address,
    ) -> Result<(), Cis2SecurityStateError> {
        self.ensure_not_paused(token_id)?;
        ICis2State::burn(self, token_id, amount, from)?;
        Ok(())
    }

    fn forced_burn(
        &mut self,
        token_id: &T,
        amount: A,
        from: &Address,
    ) -> Result<A, Cis2SecurityStateError> {
        let un_frozen_balance = self.unfreeze_to_match(from, token_id, amount)?;
        ICis2SecurityState::burn(self, token_id, amount, from)?;
        Ok(un_frozen_balance)
    }

    fn transfer(
        &mut self,
        from: &Address,
        to: &Address,
        token_id: &T,
        amount: A,
        forced: bool,
        state_builder: &mut StateBuilder<S>,
    ) -> Result<A, Cis2SecurityStateError> {
        self.ensure_not_paused(token_id)?;
        self.ensure_not_recovered(to)?;
        let un_frozen_balance = match forced {
            true => self.unfreeze_to_match(from, token_id, amount)?,
            false => A::zero(),
        };
        ICis2State::transfer(self, from, to, token_id, amount, state_builder)?;
        Ok(un_frozen_balance)
    }

    fn unfreeze_to_match(
        &mut self,
        from: &Address,
        token_id: &T,
        amount: A,
    ) -> Result<A, Cis2SecurityStateError> {
        let un_frozen_balance = self.balance_of_unfrozen(from, token_id);
        let un_frozen_balance = amount.sub(un_frozen_balance.min(amount));
        self.un_freeze(from, token_id, un_frozen_balance)?;
        Ok(un_frozen_balance)
    }
}

pub enum Cis2SecurityStateError {
    InvalidTokenId,
    InsufficientFunds,
    InvalidAmount,
    InvalidAddress,
    PausedToken,
}
impl From<TokenSecurityError> for Cis2SecurityStateError {
    fn from(value: TokenSecurityError) -> Self {
        match value {
            TokenSecurityError::PausedToken => Cis2SecurityStateError::PausedToken,
            TokenSecurityError::InvalidTokenId => Cis2SecurityStateError::InvalidTokenId,
        }
    }
}
impl From<HolderSecurityStateError> for Cis2SecurityStateError {
    fn from(value: HolderSecurityStateError) -> Self {
        match value {
            HolderSecurityStateError::InsufficientFunds => {
                Cis2SecurityStateError::InsufficientFunds
            }
            HolderSecurityStateError::AddressAlreadyRecovered => {
                Cis2SecurityStateError::InvalidAddress
            }
            HolderSecurityStateError::InvalidRecoveryAddress => {
                Cis2SecurityStateError::InvalidAddress
            }
        }
    }
}
impl From<Cis2StateError> for Cis2SecurityStateError {
    fn from(value: Cis2StateError) -> Self {
        match value {
            Cis2StateError::InvalidTokenId => Cis2SecurityStateError::InvalidTokenId,
            Cis2StateError::InsufficientFunds => Cis2SecurityStateError::InsufficientFunds,
            Cis2StateError::InvalidAmount => Cis2SecurityStateError::InvalidAmount,
        }
    }
}

pub trait ICis2SingleSecurityState<
    A: IsTokenAmount,
    THolderState: ISecurityHolderState<TokenIdUnit, A, S>,
    S: HasStateApi,
>: ICis2SecurityTokenState<A, S>+IHoldersSecurityState<TokenIdUnit, A, THolderState, S>+ICis2SingleState<A, THolderState, S>
{
    fn mint(
        &mut self,
        amount: A,
        to: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> Result<(), Cis2SecurityStateError> {
        self.ensure_not_paused()?;
        self.ensure_not_recovered(to)?;
        ICis2SingleState::mint(self, amount, to, state_builder)?;
        Ok(())
    }

    fn burn(&mut self, amount: A, from: &Address) -> Result<(), Cis2SecurityStateError> {
        self.ensure_not_paused()?;
        ICis2SingleState::burn(self, amount, from)?;
        Ok(())
    }

    fn forced_burn(&mut self, amount: A, from: &Address) -> Result<A, Cis2SecurityStateError> {
        let un_frozen_balance = self.unfreeze_to_match(from, amount)?;
        ICis2SingleState::burn(self, amount, from)?;
        Ok(un_frozen_balance)
    }

    fn transfer(
        &mut self,
        from: &Address,
        to: &Address,
        amount: A,
        forced: bool,
        state_builder: &mut StateBuilder<S>,
    ) -> Result<A, Cis2SecurityStateError> {
        self.ensure_not_paused()?;
        self.ensure_not_recovered(to)?;
        let un_frozen_balance = match forced {
            true => self.unfreeze_to_match(from, amount)?,
            false => A::zero(),
        };
        ICis2SingleState::transfer(self, from, to, amount, state_builder)?;
        Ok(un_frozen_balance)
    }

    #[inline]
    fn unfreeze_to_match(
        &mut self,
        from: &Address,
        amount: A,
    ) -> Result<A, Cis2SecurityStateError> {
        let un_frozen_balance = self.balance_of_unfrozen(from, &TokenIdUnit());
        let un_frozen_balance = amount.sub(un_frozen_balance.min(amount));
        self.un_freeze(from, &TokenIdUnit(), un_frozen_balance)?;
        Ok(un_frozen_balance)
    }
}
