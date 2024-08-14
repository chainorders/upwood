use concordium_cis2::TokenIdU32;
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_rwa_utils::state_implementations::agent_with_roles_state::IAgentWithRolesState;
use concordium_rwa_utils::state_implementations::holders_security_state::{
    HoldersSecurityState, IHoldersSecurityState,
};
use concordium_rwa_utils::state_implementations::holders_state::{HolderState, IHoldersState};
use concordium_rwa_utils::state_implementations::sponsors_state::ISponsorsState;
use concordium_rwa_utils::state_implementations::tokens_security_state::{
    ITokensSecurityState, TokenSecurityState,
};
use concordium_rwa_utils::state_implementations::tokens_state::ITokensState;
use concordium_std::{
    Address, ContractAddress, DeserialWithState, HasStateApi, MetadataUrl, Serial, Serialize,
    StateApi, StateBuilder, StateMap, StateSet,
};

use super::types::{Agent, AgentRole, TokenAmount, TokenId};

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S=StateApi> {
    /// A set that stores the addresses of the agents in the contract.
    agents:           StateMap<Address, StateSet<AgentRole, S>, S>,
    /// A map that stores the state of each token in the contract.
    tokens:           StateMap<TokenId, TokenState, S>,
    /// A map that stores the state of each holder's address for each token.
    holders:          StateMap<Address, HolderState<TokenId, TokenAmount, S>, S>,
    /// A map that stores the security state of each token in the contract.
    tokens_security:  StateMap<TokenId, TokenSecurityState, S>,
    /// The security state of each holder's address for each token.
    holders_security: HoldersSecurityState<TokenId, TokenAmount, S>,
    /// A set that stores the addresses of the sponsors of the contract.
    sponsors:         StateSet<ContractAddress, S>,
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
            agents:           state_agents,
            tokens:           state_tokens,
            tokens_security:  state_builder.new_map(),
            holders_security: HoldersSecurityState::new(
                identity_registry,
                compliance,
                state_builder,
            ),
            holders:          state_builder.new_map(),
            sponsors:         state_sponsors,
        }
    }
}

impl<S: HasStateApi> IAgentWithRolesState<S, AgentRole> for State<S> {
    fn agents(&self) -> &StateMap<Address, StateSet<AgentRole, S>, S> { &self.agents }

    fn agents_mut(&mut self) -> &mut StateMap<Address, StateSet<AgentRole, S>, S> {
        &mut self.agents
    }
}

#[derive(Serialize, Clone)]
pub struct TokenState {
    pub metadata_url: MetadataUrl,
    pub supply:       TokenAmount,
}

impl ITokensState<TokenId, TokenState, StateApi> for State {
    fn tokens(&self) -> &StateMap<TokenId, TokenState, StateApi> { &self.tokens }

    fn tokens_mut(&mut self) -> &mut StateMap<TokenId, TokenState, StateApi> { &mut self.tokens }
}

pub type HolderStateT = HolderState<TokenId, TokenAmount, StateApi>;

impl IHoldersState<TokenId, TokenAmount, StateApi> for State {
    fn holders(&self) -> &StateMap<Address, HolderStateT, StateApi> { &self.holders }

    fn holders_mut(
        &mut self,
    ) -> &mut StateMap<Address, HolderState<TokenId, TokenAmount, StateApi>, StateApi> {
        &mut self.holders
    }
}

impl ITokensSecurityState<TokenId, TokenState, StateApi> for State {
    fn security_tokens(&self) -> &StateMap<TokenId, TokenSecurityState, StateApi> {
        &self.tokens_security
    }

    fn security_tokens_mut(&mut self) -> &mut StateMap<TokenId, TokenSecurityState, StateApi> {
        &mut self.tokens_security
    }
}

impl IHoldersSecurityState<TokenId, TokenAmount, StateApi> for State {
    fn state(&self) -> &HoldersSecurityState<TokenId, TokenAmount, StateApi> {
        &self.holders_security
    }

    fn state_mut(&mut self) -> &mut HoldersSecurityState<TokenId, TokenAmount, StateApi> {
        &mut self.holders_security
    }
}

impl ISponsorsState<StateApi> for State {
    fn sponsors(&self) -> &StateSet<ContractAddress, StateApi> { &self.sponsors }

    fn sponsors_mut(&mut self) -> &mut StateSet<ContractAddress, StateApi> { &mut self.sponsors }
}
