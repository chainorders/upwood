use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::contract_logic::{
    Cis2SecurityState, SecurityTokenState,
};
use concordium_protocols::concordium_cis2_security::{SecurityParams, TokenAmountSecurity};
use concordium_std::*;

use super::types::{AgentRole, ContractResult, TokenAmount};
use crate::error::Error;
use crate::types::TokenId;

#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
pub enum HolderState<S=StateApi> {
    Active(HolderStateActive<S>),
    Recovered(Address),
}

impl HolderState {
    pub fn new_active(state_builder: &mut StateBuilder<StateApi>) -> Self {
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

    pub fn sub_assign_unfrozen(
        &mut self,
        _: TokenId,
        amount: TokenAmount,
        forced: bool,
    ) -> Result<TokenAmount, Error> {
        match self {
            HolderState::Active(holder) => holder
                .balance
                .sub_assign_unfrozen(amount, forced)
                .map_err(Into::into),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn add_assign(
        &mut self,
        _: TokenId,
        amount: TokenAmountSecurity<TokenAmount>,
    ) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => {
                holder.balance.add_assign(amount);
                Ok(())
            }
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn add_assign_unfrozen(&mut self, _: TokenId, amount: TokenAmount) -> Result<(), Error> {
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

    pub fn freeze(&mut self, _: TokenId, amount: TokenAmount) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => holder.balance.freeze(amount).map_err(Into::into),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn frozen_balance(&self, _: TokenId) -> TokenAmount {
        match self {
            HolderState::Active(holder) => holder.balance.frozen,
            _ => TokenAmount::zero(),
        }
    }

    pub fn un_frozen_balance(&self, _: TokenId) -> TokenAmount {
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

    pub fn un_freeze(&mut self, _: TokenId, amount: TokenAmount) -> Result<(), Error> {
        match self {
            HolderState::Active(holder) => holder.balance.un_freeze(amount).map_err(Into::into),
            _ => Err(Error::RecoveredAddress),
        }
    }

    pub fn balance_total(&self, _: TokenId) -> TokenAmount {
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
pub struct State<S=StateApi> {
    pub token:     SecurityTokenState<TokenAmount>,
    pub security:  Option<SecurityParams>,
    pub addresses: StateMap<Address, HolderState<S>, S>,
}

impl Cis2SecurityState<Error, TokenId, TokenAmount> for State {
    fn mint(
        &mut self,
        state_builder: &mut StateBuilder,
        token_id: TokenId,
        owner: Address,
        amount: TokenAmountSecurity<TokenAmount>,
    ) -> ContractResult<()> {
        ensure!(amount.gt(&0.into()), Error::InvalidAmount);
        self.token.add_assign_supply(amount.total())?;
        self.addresses
            .entry(owner)
            .or_insert_with(|| HolderState::new_active(state_builder))
            .add_assign(token_id, amount)?;
        Ok(())
    }

    fn transfer(
        &mut self,
        state_builder: &mut StateBuilder,
        token_id: TokenId,
        from: Address,
        to: Address,
        amount: TokenAmount,
        forced: bool,
    ) -> ContractResult<TokenAmount> {
        ensure!(!self.token.paused, Error::InvalidTokenId);
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        self.addresses
            .entry(to)
            .or_insert_with(|| HolderState::new_active(state_builder))
            .try_modify(|holder| holder.add_assign_unfrozen(token_id, amount))?;
        let unfrozen_amount = self
            .addresses
            .entry(from)
            .occupied_or(Error::InvalidAddress)?
            .try_modify(|holder| holder.sub_assign_unfrozen(token_id, amount, forced))?;
        Ok(unfrozen_amount)
    }

    fn burn(
        &mut self,
        token_id: TokenId,
        amount: TokenAmount,
        owner: Address,
        forced: bool,
    ) -> ContractResult<TokenAmount> {
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        self.token.sub_assign_supply(amount)?;
        let unfrozen_amount = self
            .addresses
            .entry(owner)
            .occupied_or(Error::InvalidAddress)?
            .try_modify(|holder| holder.sub_assign_unfrozen(token_id, amount, forced))?;
        Ok(unfrozen_amount)
    }

    fn recover(&mut self, lost_account: Address, new_account: Address) -> ContractResult<()> {
        let lost_holder = self
            .addresses
            .insert(lost_account, HolderState::Recovered(new_account));
        let previous_new_account = match lost_holder {
            Some(HolderState::Active(lost_holder)) => self
                .addresses
                .insert(new_account, HolderState::Active(lost_holder)),
            _ => return Err(Error::RecoveredAddress),
        };
        ensure!(previous_new_account.is_none(), Error::InvalidAddress);
        Ok(())
    }
}

#[derive(DeserialWithState, Serial, Deletable)]
#[concordium(state_parameter = "S")]
pub struct HolderStateActive<S=StateApi> {
    pub operators:   StateSet<Address, S>,
    pub balance:     TokenAmountSecurity<TokenAmount>,
    pub agent_roles: StateSet<AgentRole, S>,
}

impl HolderStateActive {
    pub fn new(state_builder: &mut StateBuilder<StateApi>) -> Self {
        HolderStateActive {
            operators:   state_builder.new_set(),
            balance:     TokenAmountSecurity::default(),
            agent_roles: state_builder.new_set(),
        }
    }

    pub fn new_with_roles(state_builder: &mut StateBuilder<StateApi>, roles: &[AgentRole]) -> Self {
        let mut holder = HolderStateActive::new(state_builder);
        for role in roles {
            holder.agent_roles.insert(*role);
        }
        holder
    }
}
