use concordium_cis2::TokenIdUnit;
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_rwa_utils::state_implementations::agent_with_roles_state::IAgentWithRolesState;
use concordium_rwa_utils::state_implementations::cis2_security_state::{
    ICis2SecurityTokenState, ICis2SingleSecurityState,
};
use concordium_rwa_utils::state_implementations::cis2_state::{
    Cis2Result, Cis2StateError, ICis2SingleState, ICis2TokenState,
};
use concordium_rwa_utils::state_implementations::holders_security_state::{
    HolderSecurityStateResult, IHoldersSecurityState, ISecurityHolderState,
};
use concordium_rwa_utils::state_implementations::holders_state::{
    HolderStateError, IHolderState, IHoldersState,
};
use concordium_rwa_utils::state_implementations::sft_state::ITokenState;
use concordium_rwa_utils::state_implementations::sponsors_state::ISponsorsState;
use concordium_rwa_utils::state_implementations::tokens_security_state::ISecurityTokenState;
use concordium_std::ops::{Add, AddAssign, Sub, SubAssign};
use concordium_std::{
    ensure, Address, ContractAddress, Deserial, DeserialWithState, HasStateApi, MetadataUrl,
    Serial, StateApi, StateBuilder, StateMap, StateSet,
};

use super::types::{Agent, AgentRole, TokenAmount};

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S=StateApi> {
    pub paused:            bool,
    pub supply:            TokenAmount,
    pub metadata_url:      MetadataUrl,
    /// A set that stores the addresses of the agents in the contract.
    agents:                StateMap<Address, StateSet<AgentRole, S>, S>,
    /// A map that stores the state of each holder's address for each token.
    holders:               StateMap<Address, HolderState, S>,
    /// A set that stores the addresses of the sponsors of the contract.
    sponsors:              StateSet<ContractAddress, S>,
    pub identity_registry: ContractAddress,
    pub compliance:        ContractAddress,
    recovery_addresses:    StateMap<Address, Address, S>,
}

impl<S: HasStateApi> State<S> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        identity_registry: ContractAddress,
        compliance: ContractAddress,
        sponsors: Vec<ContractAddress>,
        agents: Vec<Agent>,
        metadata_url: MetadataUrl,
        state_builder: &mut StateBuilder<S>,
    ) -> Self {
        // Insert Agents
        let mut state_agents = state_builder.new_map();
        for agent in agents {
            let mut agent_roles = state_builder.new_set();
            agent.roles.iter().for_each(|r| {
                agent_roles.insert(*r);
            });
            let _ = state_agents.insert(agent.address, agent_roles);
        }

        // Insert Sponsors
        let mut state_sponsors = state_builder.new_set();
        sponsors.iter().for_each(|s| {
            let _ = state_sponsors.insert(*s);
        });

        State {
            metadata_url,
            paused: false,
            supply: 0.into(),
            agents: state_agents,
            holders: state_builder.new_map(),
            sponsors: state_sponsors,
            identity_registry,
            compliance,
            recovery_addresses: state_builder.new_map(),
        }
    }
}

impl IAgentWithRolesState<StateApi, AgentRole> for State {
    fn agents(&self) -> &StateMap<Address, StateSet<AgentRole, StateApi>, StateApi> { &self.agents }

    fn agents_mut(&mut self) -> &mut StateMap<Address, StateSet<AgentRole, StateApi>, StateApi> {
        &mut self.agents
    }
}
impl ISponsorsState<StateApi> for State {
    fn sponsors(&self) -> &StateSet<ContractAddress, StateApi> { &self.sponsors }

    fn sponsors_mut(&mut self) -> &mut StateSet<ContractAddress, StateApi> { &mut self.sponsors }
}

#[derive(Deserial, Serial)]
pub struct HolderStateBalance {
    pub frozen:    TokenAmount,
    pub un_frozen: TokenAmount,
}
impl HolderStateBalance {
    pub fn default() -> Self {
        Self {
            frozen:    TokenAmount::zero(),
            un_frozen: TokenAmount::zero(),
        }
    }

    pub fn total(&self) -> TokenAmount { self.frozen.add(self.un_frozen) }

    pub fn freeze(&mut self, amount: TokenAmount) -> Result<(), HolderStateError> {
        ensure!(
            self.un_frozen.ge(&amount),
            HolderStateError::InsufficientFunds
        );
        self.frozen = self.frozen.add(amount);
        self.un_frozen = self.un_frozen.sub(amount);

        Ok(())
    }

    pub fn un_freeze(&mut self, amount: TokenAmount) -> Result<(), HolderStateError> {
        ensure!(self.frozen.ge(&amount), HolderStateError::InsufficientFunds);
        self.frozen = self.frozen.sub(amount);
        self.un_frozen = self.un_frozen.add(amount);

        Ok(())
    }

    pub fn add(&mut self, amount: TokenAmount) { self.un_frozen = self.un_frozen.add(amount); }

    pub fn sub(&mut self, amount: TokenAmount) -> Result<(), HolderStateError> {
        ensure!(
            self.un_frozen.ge(&amount),
            HolderStateError::InsufficientFunds
        );
        self.un_frozen = self.un_frozen.sub(amount);
        Ok(())
    }

    pub fn sub_forced(&mut self, amount: TokenAmount) -> Result<TokenAmount, HolderStateError> {
        ensure!(
            self.total().ge(&amount),
            HolderStateError::InsufficientFunds
        );
        self.un_frozen = self.un_frozen.sub(self.un_frozen.min(amount));
        let un_freeze = amount.sub(self.un_frozen);
        self.frozen = self.frozen.sub(un_freeze);

        Ok(un_freeze)
    }
}

#[derive(DeserialWithState, Serial)]
#[concordium(state_parameter = "S")]
pub struct HolderState<S=StateApi> {
    pub operators: StateSet<Address, S>,
    pub balances:  HolderStateBalance,
}

impl IHolderState<TokenIdUnit, TokenAmount, StateApi> for HolderState {
    fn is_operator(&self, operator: &Address) -> bool { self.operators.contains(operator) }

    fn add_operator(&mut self, operator: Address) { self.operators.insert(operator); }

    fn remove_operator(&mut self, operator: &Address) { self.operators.remove(operator); }

    fn balance_of(&self, _: &TokenIdUnit) -> TokenAmount { self.balances.total() }

    fn add_balance(&mut self, _: &TokenIdUnit, amount: TokenAmount) { self.balances.add(amount); }

    fn sub_balance(
        &mut self,
        _: &TokenIdUnit,
        amount: TokenAmount,
    ) -> Result<(), HolderStateError> {
        self.balances.sub(amount)?;
        Ok(())
    }

    fn new(state_builder: &mut StateBuilder<StateApi>) -> Self {
        HolderState {
            operators: state_builder.new_set(),
            balances:  HolderStateBalance::default(),
        }
    }
}
impl ISecurityHolderState<TokenIdUnit, TokenAmount, StateApi> for HolderState {
    fn freeze(&mut self, _: &TokenIdUnit, amount: TokenAmount) -> HolderSecurityStateResult<()> {
        self.balances.freeze(amount)?;
        Ok(())
    }

    fn un_freeze(&mut self, _: &TokenIdUnit, amount: TokenAmount) -> HolderSecurityStateResult<()> {
        self.balances.un_freeze(amount)?;
        Ok(())
    }

    fn balance_of_frozen(&self, _: &TokenIdUnit) -> TokenAmount { self.balances.frozen }

    fn balance_of_un_frozen(&self, _: &TokenIdUnit) -> TokenAmount { self.balances.un_frozen }
}

impl IHoldersState<TokenIdUnit, TokenAmount, HolderState, StateApi> for State {
    fn holders(&self) -> &StateMap<Address, HolderState, StateApi> { &self.holders }

    fn holders_mut(&mut self) -> &mut StateMap<Address, HolderState, StateApi> { &mut self.holders }
}
impl IHoldersSecurityState<TokenIdUnit, TokenAmount, HolderState, StateApi> for State {
    fn recovery_addresses(&self) -> &StateMap<Address, Address, StateApi> {
        &self.recovery_addresses
    }

    fn recovery_addresses_mut(&mut self) -> &mut StateMap<Address, Address, StateApi> {
        &mut self.recovery_addresses
    }
}
impl ICis2SingleState<TokenAmount, HolderState, StateApi> for State {}
impl ICis2SingleSecurityState<TokenAmount, HolderState, StateApi> for State {}
impl ITokenState<StateApi> for State {
    fn metadata_url(&self) -> &MetadataUrl { &self.metadata_url }
}
impl ICis2TokenState<TokenAmount, StateApi> for State {
    fn inc_supply(&mut self, amount: TokenAmount) { self.supply.add_assign(amount) }

    fn dec_supply(&mut self, amount: TokenAmount) -> Cis2Result<()> {
        ensure!(amount.ge(&self.supply), Cis2StateError::InsufficientFunds);
        self.supply.sub_assign(amount);
        Ok(())
    }

    fn supply(&self) -> TokenAmount { self.supply }
}
impl ISecurityTokenState<StateApi> for State {
    fn paused(&self) -> bool { self.paused }

    fn pause(&mut self) { self.paused = true; }

    fn un_pause(&mut self) { self.paused = false; }
}
impl ICis2SecurityTokenState<TokenAmount, StateApi> for State {}
