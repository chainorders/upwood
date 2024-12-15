use concordium_cis2::{IsTokenId, TokenAmountU64, TokenIdVec};
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::rate::Rate;
use concordium_std::ops::{Add, AddAssign, Sub, SubAssign};
use concordium_std::{
    ensure, Address, ContractAddress, Deletable, Deserial, DeserialWithState, HasStateApi,
    MetadataUrl, Serial, Serialize, StateApi, StateBuilder, StateMap, StateRef, StateRefMut,
    StateSet,
};

use super::types::{AgentRole, TokenAmount, TokenId};
use crate::error::Error;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S=StateApi> {
    pub token:             SecurityTokenState,
    pub reward_tokens:     StateMap<TokenId, RewardTokenState, S>,
    pub identity_registry: ContractAddress,
    pub compliance:        ContractAddress,
    pub addresses:         StateMap<Address, HolderState<S>, S>,
    pub rewards_ids_range: (TokenId, TokenId),
    pub sponsor:           Option<ContractAddress>,
}

impl<S: HasStateApi> State<S> {
    /// All the Addresses in the state. Type of address can be seen from `AddressState`
    pub fn address(&self, address: &Address) -> Option<StateRef<HolderState<S>>> {
        self.addresses.get(address)
    }

    pub fn address_mut(&mut self, address: &Address) -> Option<StateRefMut<HolderState<S>, S>> {
        self.addresses.get_mut(address)
    }
}

#[derive(Serialize, Clone)]
pub struct RewardDeposited<TDeposit: IsTokenId, ADeposit: IsTokenAmount> {
    pub token_id:       TDeposit,
    pub token_contract: ContractAddress,
    pub token_amount:   ADeposit,
    pub rate:           Rate,
}

#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
pub enum HolderState<S> {
    Active(HolderStateActive<S>),
    Recovered(Address),
}

impl<S: HasStateApi> HolderState<S> {
    pub fn new_active(state_builder: &mut StateBuilder<S>) -> Self {
        HolderState::Active(HolderStateActive::new(state_builder))
    }

    pub fn is_agent(&self, roles: &[AgentRole]) -> bool {
        match self {
            HolderState::Active(holder) => roles.iter().all(|r| holder.agent_roles.contains(r)),
            _ => false,
        }
    }

    pub fn has_operator(&self, operator: &Address) -> bool {
        match self {
            HolderState::Active(holder) => holder.operators.contains(operator),
            _ => false,
        }
    }

    pub fn sub_assign_unfrozen(&mut self, amount: TokenAmount) -> Result<TokenAmount, Error> {
        match self {
            HolderState::Active(holder) => holder.balance.sub_assign_unfrozen(amount),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn add_assign_unfrozen(&mut self, amount: TokenAmount) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => {
                holder.balance.add_assign_unfrozen(amount);
                Ok(())
            }
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn add_operator(&mut self, operator: Address) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => {
                holder.operators.insert(operator);
                Ok(())
            }
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn remove_operator(&mut self, operator: &Address) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => {
                holder.operators.remove(operator);
                Ok(())
            }
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn freeze(&mut self, amount: TokenAmount) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => holder.balance.freeze(amount),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn frozen_balance(&self) -> TokenAmount {
        match self {
            HolderState::Active(holder) => holder.balance.frozen,
            _ => TokenAmount::zero(),
        }
    }

    pub fn un_frozen_balance(&self) -> TokenAmount {
        match self {
            HolderState::Active(holder) => holder.balance.un_frozen,
            _ => TokenAmount::zero(),
        }
    }

    pub fn clear_agent_roles(&mut self) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => {
                holder.agent_roles.clear();
                Ok(())
            }
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn set_agent_roles(&mut self, roles: &[AgentRole]) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => {
                holder.agent_roles.clear();
                for role in roles {
                    holder.agent_roles.insert(*role);
                }
                Ok(())
            }
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn un_freeze(&mut self, amount: TokenAmount) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => holder.balance.un_freeze(amount),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn un_freeze_balance_to_match(
        &mut self,
        amount: TokenAmount,
    ) -> Result<TokenAmount, Error> {
        match self {
            HolderState::Active(holder) => holder.balance.un_freeze_balance_to_match(amount),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn balance_total(&self) -> TokenAmount {
        match self {
            HolderState::Active(holder) => holder.balance.total(),
            _ => TokenAmount::zero(),
        }
    }

    pub fn recovery_address(&self) -> Option<Address> {
        match self {
            HolderState::Recovered(address) => Some(*address),
            _ => None,
        }
    }

    pub fn reward_balances(
        &self,
    ) -> Result<&StateMap<TokenId, HolderStateRewardBalance, S>, Error> {
        match self {
            HolderState::Active(holder) => Ok(&holder.reward_balances),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn reward_balances_mut(
        &mut self,
    ) -> Result<&mut StateMap<TokenId, HolderStateRewardBalance, S>, Error> {
        match self {
            HolderState::Active(holder) => Ok(&mut holder.reward_balances),
            _ => Err(Error::RecoveredAddress),
        }
    }
}

#[derive(DeserialWithState, Serial, Deletable)]
#[concordium(state_parameter = "S")]
pub struct HolderStateActive<S=StateApi> {
    pub operators:       StateSet<Address, S>,
    pub balance:         HolderStateSecurityBalance,
    pub reward_balances: StateMap<TokenId, HolderStateRewardBalance, S>,
    pub agent_roles:     StateSet<AgentRole, S>,
}

impl<S: HasStateApi> HolderStateActive<S> {
    pub fn new(state_builder: &mut StateBuilder<S>) -> Self {
        HolderStateActive {
            operators:       state_builder.new_set(),
            balance:         HolderStateSecurityBalance::default(),
            reward_balances: state_builder.new_map(),
            agent_roles:     state_builder.new_set(),
        }
    }

    pub fn new_with_roles(state_builder: &mut StateBuilder<S>, roles: Vec<AgentRole>) -> Self {
        let mut holder = HolderStateActive::new(state_builder);
        for role in roles {
            holder.agent_roles.insert(role);
        }
        holder
    }
}

#[derive(Serialize, Clone)]
pub struct SecurityTokenState {
    pub metadata_url: MetadataUrl,
    pub paused:       bool,
    pub supply:       TokenAmount,
}

impl SecurityTokenState {
    pub fn pause(&mut self) { self.paused = true; }

    pub fn un_pause(&mut self) { self.paused = false; }

    pub fn sub_assign_supply(&mut self, amount: TokenAmount) -> Result<TokenAmount, Error> {
        ensure!(!self.paused, Error::PausedToken);
        ensure!(self.supply.ge(&amount), Error::InsufficientFunds);
        self.supply.sub_assign(amount);
        Ok(self.supply)
    }

    pub fn add_assign_supply(&mut self, amount: TokenAmount) -> Result<(), Error> {
        ensure!(!self.paused, Error::PausedToken);
        self.supply.add_assign(amount);
        Ok(())
    }
}

#[derive(Serialize, Clone)]
pub struct RewardTokenState {
    pub reward:       Option<RewardDeposited<TokenIdVec, TokenAmountU64>>,
    pub metadata_url: MetadataUrl,
    pub supply:       TokenAmountSigned,
}

impl RewardTokenState {
    pub fn sub_assign_supply_signed(&mut self, amount: TokenAmount) -> TokenAmountSigned {
        self.supply.sub_assign(amount);
        self.supply
    }

    pub fn add_assign_supply(&mut self, amount: TokenAmount) { self.supply.add_assign(amount); }

    pub fn attach_reward(
        &mut self,
        metadata_url: MetadataUrl,
        reward: RewardDeposited<TokenIdVec, TokenAmountU64>,
    ) {
        self.reward = Some(reward);
        self.metadata_url = metadata_url;
    }
}

#[derive(Deserial, Serial, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TokenAmountSigned {
    pub amount:      TokenAmount,
    /// If the amount is negative, this will be true.
    pub is_negative: bool,
}

impl TokenAmountSigned {
    pub fn zero() -> Self {
        Self {
            amount:      TokenAmount::zero(),
            is_negative: false,
        }
    }

    pub fn as_amount(&self) -> TokenAmount {
        if self.is_negative {
            TokenAmount::zero()
        } else {
            self.amount
        }
    }

    pub fn ge(&self, other: &TokenAmount) -> bool {
        if self.is_negative {
            false
        } else {
            self.amount.ge(other)
        }
    }

    pub fn sub_assign(&mut self, other: TokenAmount) -> TokenAmount {
        match (self.is_negative, self.amount.ge(&other)) {
            (false, true) => {
                self.amount.sub_assign(other);
                TokenAmount::zero()
            }
            (false, false) => {
                self.amount = other.sub(self.amount);
                self.is_negative = true;
                self.amount
            }
            (true, _) => {
                self.amount = other.add(self.amount);
                other
            }
        }
    }

    pub fn add_assign(&mut self, other: TokenAmount) -> TokenAmount {
        match (self.is_negative, self.amount.ge(&other)) {
            (false, _) => {
                self.amount.add_assign(other);
                TokenAmount::zero()
            }
            (true, true) => {
                self.amount = self.amount.sub(other);
                other
            }
            (true, false) => {
                self.amount = other.sub(self.amount);
                self.is_negative = false;
                other.sub(self.amount)
            }
        }
    }
}

#[derive(Deserial, Serial, Clone, Copy)]
pub struct HolderStateSecurityBalance {
    pub frozen:    TokenAmount,
    pub un_frozen: TokenAmount,
}

impl HolderStateSecurityBalance {
    pub fn default() -> Self {
        Self {
            frozen:    TokenAmount::zero(),
            un_frozen: TokenAmount::zero(),
        }
    }

    pub fn total(&self) -> TokenAmount { self.frozen.add(self.un_frozen) }

    pub fn add_assign_unfrozen(&mut self, amount: TokenAmount) { self.un_frozen.add_assign(amount) }

    pub fn sub_assign_unfrozen(&mut self, amount: TokenAmount) -> Result<TokenAmount, Error> {
        ensure!(self.un_frozen.ge(&amount), Error::InsufficientFunds);
        self.un_frozen.sub_assign(amount);
        Ok(self.un_frozen)
    }

    #[inline]
    pub fn freeze(&mut self, amount: TokenAmount) -> Result<(), Error> {
        ensure!(self.un_frozen.ge(&amount), Error::InsufficientFunds);
        self.frozen.add_assign(amount);
        self.un_frozen.sub_assign(amount);

        Ok(())
    }

    #[inline]
    pub fn un_freeze(&mut self, amount: TokenAmount) -> Result<(), Error> {
        ensure!(self.frozen.ge(&amount), Error::InsufficientFunds);
        self.frozen.sub_assign(amount);
        self.un_frozen.add_assign(amount);

        Ok(())
    }

    pub fn un_freeze_balance_to_match(
        &mut self,
        amount: TokenAmount,
    ) -> Result<TokenAmount, Error> {
        if self.un_frozen.ge(&amount) {
            return Ok(TokenAmount::zero());
        }

        let to_un_freeze = amount.sub(self.un_frozen);
        if to_un_freeze.gt(&self.frozen) {
            return Err(Error::InsufficientFunds);
        }

        self.frozen.sub_assign(to_un_freeze);
        self.un_frozen.add_assign(to_un_freeze);
        Ok(to_un_freeze)
    }
}

#[derive(Deserial, Serial, Clone)]
pub struct HolderStateRewardBalance {
    pub un_frozen: TokenAmountSigned,
}

impl Default for HolderStateRewardBalance {
    fn default() -> Self {
        Self {
            un_frozen: TokenAmountSigned::zero(),
        }
    }
}

impl HolderStateRewardBalance {
    pub fn add_assign_unfrozen(&mut self, amount: TokenAmount) -> TokenAmount {
        self.un_frozen.add_assign(amount)
    }

    pub fn sub_assign_unfrozen(
        &mut self,
        amount: TokenAmount,
        allow_overflow: bool,
    ) -> Result<TokenAmount, Error> {
        if !allow_overflow {
            ensure!(self.un_frozen.ge(&amount), Error::InsufficientFunds);
        }
        self.un_frozen.sub_assign(amount);
        Ok(self.un_frozen.as_amount())
    }
}

#[cfg(test)]
mod tests {
    use super::TokenAmountSigned;

    #[test]
    pub fn token_amount_signed_tests() {
        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: false,
        };
        let carry = amount.add_assign(5.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      15.into(),
            is_negative: false,
        });
        assert_eq!(carry, 0.into());

        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: false,
        };
        let carry = amount.sub_assign(5.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      5.into(),
            is_negative: false,
        });
        assert_eq!(carry, 0.into());

        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: false,
        };
        let carry = amount.sub_assign(15.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      5.into(),
            is_negative: true,
        });
        assert_eq!(carry, 5.into());

        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: true,
        };
        let carry = amount.sub_assign(5.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      15.into(),
            is_negative: true,
        });
        assert_eq!(carry, 5.into());

        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: true,
        };
        let carry = amount.add_assign(6.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      4.into(),
            is_negative: true,
        });
        assert_eq!(carry, 6.into());

        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: true,
        };
        let carry = amount.add_assign(15.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      5.into(),
            is_negative: false,
        });
        assert_eq!(carry, 10.into());

        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: false,
        };
        let carry = amount.sub_assign(10.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      0.into(),
            is_negative: false,
        });
        assert_eq!(carry, 0.into());

        let mut amount = TokenAmountSigned {
            amount:      10.into(),
            is_negative: true,
        };
        let carry = amount.add_assign(10.into());
        assert_eq!(amount, TokenAmountSigned {
            amount:      0.into(),
            is_negative: true,
        });
        assert_eq!(carry, 10.into());
    }
}
