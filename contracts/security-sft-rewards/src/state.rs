use concordium_cis2::TokenIdU32;
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_rwa_utils::state_implementations::agent_with_roles_state::IAgentWithRolesState;
use concordium_rwa_utils::state_implementations::cis2_security_state::ICis2SecurityState;
use concordium_rwa_utils::state_implementations::cis2_state::{ICis2State, ICis2TokenState};
use concordium_rwa_utils::state_implementations::holders_security_state::{
    HolderSecurityStateError, HolderSecurityStateResult, IHoldersSecurityState,
    ISecurityHolderState,
};
use concordium_rwa_utils::state_implementations::holders_state::{IHolderState, IHoldersState};
use concordium_rwa_utils::state_implementations::sponsors_state::ISponsorsState;
use concordium_rwa_utils::state_implementations::tokens_security_state::{
    ISecurityTokenState, ITokensSecurityState,
};
use concordium_rwa_utils::state_implementations::tokens_state::ITokensState;
use concordium_std::ops::{Add, Sub};
use concordium_std::{
    Address, ContractAddress, Deserial, DeserialWithState, HasStateApi, MetadataUrl, Serial,
    Serialize, StateApi, StateBuilder, StateMap, StateSet,
};

use super::types::{Agent, AgentRole, TokenAmount, TokenId};

#[derive(Serialize, Clone)]
pub struct TokenState {
    pub metadata_url: MetadataUrl,
    pub paused:       bool,
    pub supply:       TokenAmount,
}
impl ISecurityTokenState for TokenState {
    fn paused(&self) -> bool { self.paused }

    fn set_paused(&mut self, is_paused: bool) { self.paused = is_paused; }
}
impl ICis2TokenState<TokenAmount> for TokenState {
    fn inc_supply(&mut self, amount: &TokenAmount) { self.supply = self.supply.add(*amount); }

    fn dec_supply(&mut self, amount: &TokenAmount) { self.supply = self.supply.sub(*amount); }
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

    pub fn freeze(&mut self, amount: &TokenAmount) {
        self.frozen = self.frozen.add(*amount);
        self.un_frozen = self.un_frozen.sub(*amount);
    }

    pub fn un_freeze(&mut self, amount: &TokenAmount) {
        self.frozen = self.frozen.sub(*amount);
        self.un_frozen = self.un_frozen.add(*amount);
    }

    pub fn add(&mut self, amount: TokenAmount) { self.un_frozen = self.un_frozen.add(amount); }

    pub fn sub(&mut self, amount: TokenAmount) { self.un_frozen = self.un_frozen.sub(amount); }

    pub fn sub_forced(&mut self, amount: TokenAmount) -> TokenAmount {
        self.un_frozen = self.un_frozen.sub(self.un_frozen.min(amount));
        let un_freeze = amount.sub(self.un_frozen);
        self.frozen = self.frozen.sub(un_freeze);

        un_freeze
    }
}

#[derive(DeserialWithState, Serial)]
#[concordium(state_parameter = "S")]
pub struct HolderState<S=StateApi> {
    pub operators: StateSet<Address, S>,
    pub balances:  StateMap<TokenId, HolderStateBalance, S>,
}
impl IHolderState<TokenId, TokenAmount, StateApi> for HolderState {
    fn is_operator(&self, operator: &Address) -> bool { self.operators.contains(operator) }

    fn add_operator(&mut self, operator: Address) { self.operators.insert(operator); }

    fn remove_operator(&mut self, operator: &Address) { self.operators.remove(operator); }

    fn balance_of(&self, token_id: &TokenId) -> TokenAmount {
        self.balances
            .get(token_id)
            .map(|a| a.total())
            .unwrap_or(TokenAmount::zero())
    }

    fn add_balance(&mut self, token_id: &TokenId, amount: &TokenAmount) {
        self.balances
            .entry(*token_id)
            .or_insert(HolderStateBalance::default())
            .modify(|a| a.add(*amount));
    }

    fn sub_balance(&mut self, token_id: &TokenId, amount: &TokenAmount) {
        self.balances
            .entry(*token_id)
            .or_insert(HolderStateBalance::default())
            .modify(|a| a.sub(*amount));
    }

    fn new(state_builder: &mut StateBuilder<StateApi>) -> Self {
        HolderState {
            operators: state_builder.new_set(),
            balances:  state_builder.new_map(),
        }
    }
}
impl ISecurityHolderState<TokenId, TokenAmount, StateApi> for HolderState {
    fn freeze(
        &mut self,
        token_id: &TokenId,
        amount: &TokenAmount,
    ) -> HolderSecurityStateResult<()> {
        self.balances
            .entry(*token_id)
            .occupied_or(HolderSecurityStateError::AmountTooLarge)?
            .modify(|e| {
                e.freeze(amount);
            });
        Ok(())
    }

    fn un_freeze(
        &mut self,
        token_id: &TokenId,
        amount: &TokenAmount,
    ) -> HolderSecurityStateResult<()> {
        self.balances
            .entry(*token_id)
            .occupied_or(HolderSecurityStateError::AmountTooLarge)?
            .modify(|e| e.un_freeze(amount));
        Ok(())
    }

    fn balance_of_frozen(&self, token_id: &TokenId) -> TokenAmount {
        self.balances
            .get(token_id)
            .map(|a| a.frozen)
            .unwrap_or(TokenAmount::zero())
    }

    fn balance_of_un_frozen(&self, token_id: &TokenId) -> TokenAmount {
        self.balances
            .get(token_id)
            .map(|a| a.un_frozen)
            .unwrap_or(TokenAmount::zero())
    }
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S=StateApi> {
    /// A set that stores the addresses of the agents in the contract.
    agents:             StateMap<Address, StateSet<AgentRole, S>, S>,
    /// A map that stores the state of each token in the contract.
    tokens:             StateMap<TokenId, TokenState, S>,
    /// A map that stores the state of each holder's address for each token.
    holders:            StateMap<Address, HolderState, S>,
    /// A set that stores the addresses of the sponsors of the contract.
    sponsors:           StateSet<ContractAddress, S>,
    identity_registry:  ContractAddress,
    compliance:         ContractAddress,
    recovery_addresses: StateMap<Address, Address, S>,
}

impl<S: HasStateApi> State<S> {
    pub fn new(
        identity_registry: ContractAddress,
        compliance: ContractAddress,
        sponsors: Vec<ContractAddress>,
        agents: Vec<Agent>,
        metadata_url: MetadataUrl,
        state_builder: &mut StateBuilder<S>,
    ) -> Self {
        let mut state_tokens = state_builder.new_map();
        let _ = state_tokens.insert(TokenIdU32(0), TokenState {
            metadata_url,
            paused: false,
            supply: TokenAmount::zero(),
        });

        let mut state_agents = state_builder.new_map();
        for agent in agents {
            let mut agent_roles = state_builder.new_set();
            agent.roles.iter().for_each(|r| {
                agent_roles.insert(*r);
            });
            let _ = state_agents.insert(agent.address, agent_roles);
        }

        let mut state_sponsors = state_builder.new_set();
        sponsors.iter().for_each(|s| {
            let _ = state_sponsors.insert(*s);
        });

        State {
            agents: state_agents,
            tokens: state_tokens,
            holders: state_builder.new_map(),
            sponsors: state_sponsors,
            identity_registry,
            compliance,
            recovery_addresses: state_builder.new_map(),
        }
    }
}

impl<S: HasStateApi> IAgentWithRolesState<S, AgentRole> for State<S> {
    fn agents(&self) -> &StateMap<Address, StateSet<AgentRole, S>, S> { &self.agents }

    fn agents_mut(&mut self) -> &mut StateMap<Address, StateSet<AgentRole, S>, S> {
        &mut self.agents
    }
}

impl ITokensState<TokenId, TokenState, StateApi> for State {
    fn tokens(&self) -> &StateMap<TokenId, TokenState, StateApi> { &self.tokens }

    fn tokens_mut(&mut self) -> &mut StateMap<TokenId, TokenState, StateApi> { &mut self.tokens }
}

impl IHoldersState<TokenId, TokenAmount, HolderState, StateApi> for State {
    fn holders(&self) -> &StateMap<Address, HolderState, StateApi> { &self.holders }

    fn holders_mut(&mut self) -> &mut StateMap<Address, HolderState, StateApi> { &mut self.holders }
}

impl ITokensSecurityState<TokenId, TokenState, StateApi> for State {}

impl IHoldersSecurityState<TokenId, TokenAmount, HolderState, StateApi> for State {
    fn recovery_addresses(&self) -> &StateMap<Address, Address, StateApi> {
        &self.recovery_addresses
    }

    fn recovery_addresses_mut(&mut self) -> &mut StateMap<Address, Address, StateApi> {
        &mut self.recovery_addresses
    }
}

impl ISponsorsState<StateApi> for State {
    fn sponsors(&self) -> &StateSet<ContractAddress, StateApi> { &self.sponsors }

    fn sponsors_mut(&mut self) -> &mut StateSet<ContractAddress, StateApi> { &mut self.sponsors }
}

impl ICis2State<TokenId, TokenAmount, TokenState, HolderState, StateApi> for State {}
impl ICis2SecurityState<TokenId, TokenAmount, TokenState, HolderState, StateApi> for State {
    fn set_compliance(&mut self, compliance: ContractAddress) { self.compliance = compliance; }

    fn set_identity_registry(&mut self, identity_registry: ContractAddress) {
        self.identity_registry = identity_registry;
    }

    fn identity_registry(&self) -> ContractAddress { self.identity_registry }

    fn compliance(&self) -> ContractAddress { self.compliance }
}
