use concordium_cis2::TokenIdVec;
use concordium_protocols::concordium_cis2_security::TokenUId;
use concordium_std::{
    Address, Deletable, DeserialWithState, HasStateApi, MetadataUrl, Serial, StateApi,
    StateBuilder, StateMap, StateRef, StateRefMut, StateSet,
};

use super::types::TokenId;
use crate::error::Error;

#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
pub enum AddressState<S> {
    Agent,
    Holder(HolderState<S>),
}

impl<S> AddressState<S> {
    #[inline]
    pub fn agent(&self) -> Option<()> {
        match self {
            AddressState::Agent => Some(()),
            _ => None,
        }
    }

    #[inline]
    pub fn holder(&self) -> Option<&HolderState<S>> {
        match self {
            AddressState::Holder(holder) => Some(holder),
            _ => None,
        }
    }

    #[inline]
    pub fn holder_mut(&mut self) -> Option<&mut HolderState<S>> {
        match self {
            AddressState::Holder(holder) => Some(holder),
            _ => None,
        }
    }
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S=StateApi> {
    pub reward_token:  TokenUId<TokenIdVec>,
    pub curr_token_id: TokenId,
    pub tokens:        StateMap<TokenId, MetadataUrl, S>,
    pub addresses:     StateMap<Address, AddressState<S>, S>,
}

impl<S: HasStateApi> State<S> {
    #[inline]
    pub fn address(&self, address: &Address) -> Option<StateRef<AddressState<S>>> {
        self.addresses.get(address)
    }

    #[inline]
    pub fn address_or_insert_holder(
        &mut self,
        address: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> StateRefMut<AddressState<S>, S> {
        self.addresses
            .entry(*address)
            .or_insert(AddressState::Holder(HolderState::new(state_builder)));
        self.address_mut(address).unwrap()
    }

    #[inline]
    pub fn address_mut(&mut self, address: &Address) -> Option<StateRefMut<AddressState<S>, S>> {
        self.addresses.get_mut(address)
    }

    pub fn add_address(
        &mut self,
        address: Address,
        state: AddressState<S>,
    ) -> Result<StateRefMut<AddressState<S>, S>, Error> {
        self.addresses
            .entry(address)
            .vacant_or(Error::InvalidAddress)?
            .insert(state);
        let address = self.address_mut(&address).ok_or(Error::InvalidAddress)?;
        Ok(address)
    }

    pub fn remove_and_get_address(&mut self, address: &Address) -> Result<AddressState<S>, Error> {
        let address = self
            .addresses
            .remove_and_get(address)
            .ok_or(Error::InvalidAddress)?;
        Ok(address)
    }

    #[inline]
    pub fn token(&self, token_id: &TokenId) -> Option<StateRef<MetadataUrl>> {
        self.tokens.get(token_id)
    }

    #[inline]
    pub fn token_mut(&mut self, token_id: &TokenId) -> Option<StateRefMut<MetadataUrl, S>> {
        self.tokens.get_mut(token_id)
    }

    #[inline]
    pub fn add_token(&mut self, token_id: TokenId, token_state: MetadataUrl) -> Result<(), Error> {
        self.tokens
            .entry(token_id)
            .vacant_or(Error::InvalidTokenId)?
            .insert(token_state);
        Ok(())
    }
}

#[derive(DeserialWithState, Serial, Deletable)]
#[concordium(state_parameter = "S")]
pub struct HolderState<S=StateApi> {
    pub operators: StateSet<Address, S>,
    pub balances:  StateSet<TokenId, S>,
}
impl<S: HasStateApi> HolderState<S> {
    #[inline]
    pub fn new(state_builder: &mut StateBuilder<S>) -> Self {
        HolderState {
            operators: state_builder.new_set(),
            balances:  state_builder.new_set(),
        }
    }

    #[inline]
    pub fn balance(&self, token_id: &TokenId) -> bool { self.balances.contains(token_id) }

    #[inline]
    pub fn add_balance(&mut self, token_id: TokenId) { self.balances.insert(token_id); }

    #[inline]
    pub fn remove_balance(&mut self, token_id: &TokenId) { self.balances.remove(token_id); }

    #[inline]
    pub fn has_operator(&self, operator: &Address) -> bool { self.operators.contains(operator) }

    #[inline]
    pub fn add_operator(&mut self, operator: Address) { self.operators.insert(operator); }

    #[inline]
    pub fn remove_operator(&mut self, operator: &Address) { self.operators.remove(operator); }
}
