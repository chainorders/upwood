use concordium_std::{bail, ensure, Deletable, HasStateApi, Serialize, StateMap};

use crate::tokens_state::IsTokenAmount;

pub enum DepositedStateError {
    TokenNotFound,
    InsufficientDeposits,
    InsufficientLocked,
}

#[derive(Serialize, Clone)]
pub struct DepositedTokenState<A> {
    pub amount:        A,
    pub locked_amount: A,
}

pub trait IDepositedTokensState<T: Serialize + Clone, A: IsTokenAmount, S: HasStateApi> {
    fn tokens(&self) -> &StateMap<T, DepositedTokenState<A>, S>;
    fn tokens_mut(&mut self) -> &mut StateMap<T, DepositedTokenState<A>, S>;

    fn is_deposited(&self, token_id: &T) -> bool { self.tokens().get(token_id).is_some() }

    fn balance_of_deposited(&self, token_id: &T) -> A {
        self.tokens().get(token_id).map(|info| info.amount).unwrap_or(A::zero())
    }

    fn balance_of_locked(&self, token_id: &T) -> A {
        self.tokens().get(token_id).map(|info| info.locked_amount).unwrap_or(A::zero())
    }

    fn balance_of_unlocked(&self, token_id: &T) -> A {
        self.balance_of_deposited(token_id).sub(self.balance_of_locked(token_id))
    }

    /// Adds an entry for the token with the given ID if it does not exist and
    /// sets the balance to the given amount.
    fn inc_deposits(&mut self, token_id: T, amount: A) {
        self.tokens_mut()
            .entry(token_id)
            .and_modify(|into| {
                into.amount.add_assign(amount);
            })
            .or_insert(DepositedTokenState {
                amount,
                locked_amount: A::zero(),
            });
    }

    fn set_locked_deposits(&mut self, token_id: &T, amount: A) -> Result<(), DepositedStateError> {
        let token = self.tokens_mut().get_mut(token_id);
        match token {
            Some(mut token) => {
                ensure!(token.amount.ge(&amount), DepositedStateError::InsufficientDeposits);
                token.locked_amount = amount;
            }
            None => bail!(DepositedStateError::TokenNotFound),
        };

        Ok(())
    }

    fn inc_locked_deposits(&mut self, token_id: &T, amount: A) -> Result<(), DepositedStateError> {
        let token = self.tokens_mut().get_mut(token_id);
        match token {
            Some(mut token) => {
                token.locked_amount.add_assign(amount);
                ensure!(token.amount.ge(&token.locked_amount), DepositedStateError::InsufficientDeposits);
            }
            None => bail!(DepositedStateError::TokenNotFound),
        };

        Ok(())
    }

    /// Decreases the balance of the token with the given ID and removes it if
    /// the balance becomes zero.
    /// Returns `true` if the token was removed.
    /// Returns `false` if the token was not removed.
    fn dec_deposits(&mut self, token_id: &T, amount: A) -> Result<bool, DepositedStateError> {
        let token = self.tokens_mut().get_mut(token_id);
        let token = match token {
            Some(mut token) => {
                token.amount.sub_assign(amount);
                if token.amount.lt(&token.locked_amount) {
                    bail!(DepositedStateError::InsufficientDeposits)
                }

                token
            }
            None => bail!(DepositedStateError::TokenNotFound),
        };

        if token.amount.is_zero() {
            token.clone().delete();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn dec_locked_deposits(&mut self, token_id: &T, amount: A) -> Result<A, DepositedStateError> {
        let token = self.tokens_mut().get_mut(token_id);
        match token {
            Some(mut token) => {
                ensure!(token.locked_amount.ge(&amount), DepositedStateError::InsufficientLocked);
                token.locked_amount.sub_assign(amount);
                Ok(token.locked_amount)
            }
            None => bail!(DepositedStateError::TokenNotFound),
        }
    }

    fn burn_locked_deposits(&mut self, token_id: &T, amount: A) -> Result<A, DepositedStateError> {
        let token = self.tokens_mut().get_mut(token_id);
        match token {
            Some(mut token) => {
                ensure!(token.locked_amount.ge(&amount), DepositedStateError::InsufficientLocked);
                token.locked_amount.sub_assign(amount);
                token.amount.sub_assign(amount);
                Ok(token.locked_amount)
            }
            None => bail!(DepositedStateError::TokenNotFound),
        }
    }
}
