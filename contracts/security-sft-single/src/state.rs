use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_std::ops::{Add, AddAssign, Sub, SubAssign};
use concordium_std::{
    ensure, Address, ContractAddress, Deletable, Deserial, DeserialWithState, HasStateApi,
    MetadataUrl, Serial, Serialize, StateApi, StateBuilder, StateMap, StateSet,
};

use super::types::{AgentRole, TokenAmount};
use crate::error::Error;

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
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S=StateApi> {
    pub token:             SecurityTokenState,
    pub identity_registry: Option<ContractAddress>,
    pub compliance:        Option<ContractAddress>,
    pub addresses:         StateMap<Address, HolderState<S>, S>,
    pub sponsor:           Option<ContractAddress>,
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

    pub fn metadata_url(&self) -> &MetadataUrl { &self.metadata_url }
}

#[derive(Deserial, Serial, Clone, Copy)]
pub struct HolderStateSecurityBalance {
    pub frozen:    TokenAmount,
    pub un_frozen: TokenAmount,
}

impl Default for HolderStateSecurityBalance {
    fn default() -> Self {
        Self {
            frozen:    TokenAmount::zero(),
            un_frozen: TokenAmount::zero(),
        }
    }
}

impl HolderStateSecurityBalance {
    pub fn total(&self) -> TokenAmount { self.frozen.add(self.un_frozen) }

    pub fn sub_assign_unfrozen(&mut self, amount: TokenAmount) -> Result<TokenAmount, Error> {
        ensure!(self.un_frozen.ge(&amount), Error::InsufficientFunds);
        self.un_frozen.sub_assign(amount);
        Ok(self.un_frozen)
    }

    pub fn add_assign_unfrozen(&mut self, amount: TokenAmount) {
        self.un_frozen.add_assign(amount);
    }

    pub fn freeze(&mut self, amount: TokenAmount) -> Result<(), Error> {
        ensure!(self.un_frozen.ge(&amount), Error::InsufficientFunds);
        self.frozen.add_assign(amount);
        self.un_frozen.sub_assign(amount);

        Ok(())
    }

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

#[derive(DeserialWithState, Serial, Deletable)]
#[concordium(state_parameter = "S")]
pub struct HolderStateActive<S=StateApi> {
    pub operators:   StateSet<Address, S>,
    pub balance:     HolderStateSecurityBalance,
    pub agent_roles: StateSet<AgentRole, S>,
}

impl<S: HasStateApi> HolderStateActive<S> {
    pub fn new(state_builder: &mut StateBuilder<S>) -> Self {
        HolderStateActive {
            operators:   state_builder.new_set(),
            balance:     Default::default(),
            agent_roles: state_builder.new_set(),
        }
    }

    pub fn new_with_roles(state_builder: &mut StateBuilder<S>, roles: &[AgentRole]) -> Self {
        let mut holder = HolderStateActive::new(state_builder);
        for role in roles {
            holder.agent_roles.insert(*role);
        }
        holder
    }
}
