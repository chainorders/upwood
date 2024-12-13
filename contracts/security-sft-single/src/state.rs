use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_std::ops::{Add, AddAssign, Sub, SubAssign};
use concordium_std::{
    ensure, Address, ContractAddress, Deletable, Deserial, DeserialWithState, HasStateApi,
    MetadataUrl, OccupiedEntry, Serial, Serialize, StateApi, StateBuilder, StateMap, StateRef,
    StateRefMut, StateSet,
};

use super::types::{AgentRole, TokenAmount, TokenId};
use crate::error::Error;

#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
pub enum HolderState<S> {
    Active(HolderStateActive<S>),
    Recovered(Address),
}

impl<S: HasStateApi> HolderState<S> {
    pub fn active(&self) -> Option<&HolderStateActive<S>> {
        match self {
            HolderState::Active(holder) => Some(holder),
            _ => None,
        }
    }

    pub fn active_mut(&mut self) -> Option<&mut HolderStateActive<S>> {
        match self {
            HolderState::Active(holder) => Some(holder),
            _ => None,
        }
    }

    pub fn recovered(&self) -> Option<&Address> {
        match self {
            HolderState::Recovered(address) => Some(address),
            _ => None,
        }
    }

    pub fn is_agent(&self, roles: &[AgentRole]) -> bool {
        match self {
            HolderState::Active(holder) => roles.iter().all(|r| holder.agent_roles.contains(r)),
            _ => false,
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

impl<S: HasStateApi> State<S> {
    pub fn address(&self, address: &Address) -> Option<StateRef<HolderState<S>>> {
        self.addresses.get(address)
    }

    pub fn address_or_insert_holder(
        &mut self,
        address: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> StateRefMut<HolderState<S>, S> {
        self.addresses
            .entry(*address)
            .or_insert(HolderState::Active(HolderStateActive::new(state_builder)));
        self.address_mut(address).unwrap()
    }

    pub fn address_mut(&mut self, address: &Address) -> Option<StateRefMut<HolderState<S>, S>> {
        self.addresses.get_mut(address)
    }

    pub fn add_address(
        &mut self,
        address: Address,
        state: HolderState<S>,
    ) -> Result<OccupiedEntry<'_, Address, HolderState<S>, S>, Error> {
        Ok(self
            .addresses
            .entry(address)
            .vacant_or(Error::InvalidAddress)?
            .insert(state))
    }

    pub fn sub_assign_supply(&mut self, _: &TokenId, amount: TokenAmount) -> Result<(), Error> {
        self.token.sub_assign_supply(amount)?;
        Ok(())
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

    pub fn has_operator(&self, operator: &Address) -> bool { self.operators.contains(operator) }

    pub fn add_operator(&mut self, operator: Address) { self.operators.insert(operator); }

    pub fn remove_operator(&mut self, operator: &Address) { self.operators.remove(operator); }

    pub fn clone_for_recovery(&self, state_builder: &mut StateBuilder<S>) -> Self {
        let mut new_holder = HolderStateActive::new(state_builder);
        new_holder.balance = self.balance;
        new_holder
    }
}
