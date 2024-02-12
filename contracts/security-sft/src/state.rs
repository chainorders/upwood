use concordium_rwa_utils::{
    agents_state::IsAgentsState,
    cis2_security_state::ICis2SecurityState,
    cis2_state::ICis2State,
    clients::contract_client::IContractState,
    holders_security_state::{HoldersSecurityState, IHoldersSecurityState},
    holders_state::{HolderState, IHoldersState},
    sponsors_state::ISponsorsState,
    token_deposits_state::{DepositedTokenState, IDepositedTokensState},
    tokens_security_state::{ITokensSecurityState, TokenSecurityState},
    tokens_state::{ITokensState, TokenStateResult},
};
use concordium_std::*;

use super::types::{
    NftTokenAmount, NftTokenOwnerUId, NftTokenUId, Rate, TokenAmount, TokenId,
};

#[derive(Serialize, Clone)]
pub struct TokenState {
    pub deposit_token_id: NftTokenUId,
    pub metadata_url:     MetadataUrl,
    pub fractions_rate:   Rate,
    pub supply:           TokenAmount,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S = StateApi> {
    /// A map that saves the tokens which have been added but not yet utilized
    /// for fractionalization.
    wrapped_token_id_map:   StateMap<NftTokenUId, TokenId, S>,
    /// A map which stores the token id and amount of the token which has been
    /// sent to the contract
    deposited_tokens:       StateMap<NftTokenOwnerUId, DepositedTokenState<NftTokenAmount>, S>,
    /// A map that stores the state of each token in the contract.
    tokens:                 StateMap<TokenId, TokenState, S>,
    /// A map that stores the security state of each token in the contract.
    tokens_security_state:  StateMap<TokenId, TokenSecurityState, S>,
    /// The security state of each holder's address for each token.
    holders_security_state: HoldersSecurityState<TokenId, TokenAmount, S>,
    /// A map that stores the state of each holder's address for each token.
    holders:                StateMap<Address, HolderState<TokenId, TokenAmount, S>, S>,
    /// A set that stores the addresses of the sponsors of the contract.
    sponsors:               StateSet<ContractAddress, S>,
    /// A set that stores the addresses of the agents in the contract.
    agents:                 StateSet<Address, S>,
    /// The ID of the token associated with this contract state.
    token_id:               TokenId,
}

impl State {
    /// Creates a new state with the given parameters.
    ///
    /// # Parameters
    ///
    /// * `identity_registry`: The address of the identity registry contract.
    /// * `compliance`: The address of the compliance contract.
    /// * `sponsors`: A vector of contract addresses that are sponsors.
    /// * `agents`: A vector of agent addresses.
    /// * `state_builder`: A mutable reference to the state builder.
    ///
    /// # Returns
    ///
    /// Returns a new `State` instance.
    pub fn new(
        identity_registry: ContractAddress,
        compliance: ContractAddress,
        sponsors: Vec<ContractAddress>,
        agents: Vec<Address>,
        state_builder: &mut StateBuilder<StateApi>,
    ) -> Self {
        let mut state = State {
            agents:                 state_builder.new_set(),
            tokens:                 state_builder.new_map(),
            tokens_security_state:  state_builder.new_map(),
            holders_security_state: HoldersSecurityState::new(
                identity_registry,
                compliance,
                state_builder,
            ),
            holders:                state_builder.new_map(),
            sponsors:               state_builder.new_set(),
            token_id:               0.into(),
            wrapped_token_id_map:   state_builder.new_map(),
            deposited_tokens:       state_builder.new_map(),
        };

        for sponsor in sponsors {
            state.sponsors.insert(sponsor);
        }

        for agent in agents {
            state.agents.insert(agent);
        }

        state
    }

    /// Returns the token ID.
    ///
    /// # Returns
    ///
    /// Returns the `TokenId`.
    pub fn generate_add_token(
        &mut self,
        token_metadata: TokenState,
    ) -> TokenStateResult<TokenId> {
        let token_id = self.get_and_increment_token_id();
        self.add_token(token_id, token_metadata)?;
        Ok(token_id)
    }

    pub fn get_and_increment_token_id(&mut self) -> TokenId {
        let token_id = self.token_id;
        self.token_id.0 += 1;
        token_id
    }

    pub fn add_wrapped_id(
        &mut self,
        nft_token_id: NftTokenUId,
        token_id: TokenId,
    ) -> Option<TokenId> {
        self.wrapped_token_id_map.insert(nft_token_id, token_id)
    }

    pub fn get_wrapped_id(&self, nft_token_id: &NftTokenUId) -> Option<TokenId> {
        self.wrapped_token_id_map.get(nft_token_id).map(|t| *t)
    }
}

pub type HolderStateT = HolderState<TokenId, TokenAmount, StateApi>;

impl ITokensState<TokenId, TokenState, StateApi> for State {
    fn tokens(&self) -> &StateMap<TokenId, TokenState, StateApi> { &self.tokens }

    fn tokens_mut(&mut self) -> &mut StateMap<TokenId, TokenState, StateApi> { &mut self.tokens }
}
impl IHoldersState<TokenId, TokenAmount, StateApi> for State {
    fn holders(&self) -> &StateMap<Address, HolderStateT, StateApi> { &self.holders }

    fn holders_mut(
        &mut self,
    ) -> &mut StateMap<Address, HolderState<TokenId, TokenAmount, StateApi>, StateApi> {
        &mut self.holders
    }
}
impl IHoldersSecurityState<TokenId, TokenAmount, StateApi> for State {
    fn state(&self) -> &HoldersSecurityState<TokenId, TokenAmount, StateApi> {
        &self.holders_security_state
    }

    fn state_mut(&mut self) -> &mut HoldersSecurityState<TokenId, TokenAmount, StateApi> {
        &mut self.holders_security_state
    }
}
impl ITokensSecurityState<TokenId, TokenState, StateApi> for State {
    fn security_tokens(&self) -> &StateMap<TokenId, TokenSecurityState, StateApi> {
        &self.tokens_security_state
    }

    fn security_tokens_mut(&mut self) -> &mut StateMap<TokenId, TokenSecurityState, StateApi> {
        &mut self.tokens_security_state
    }
}
impl ICis2State<TokenId, TokenAmount, TokenState, StateApi> for State {}
impl ICis2SecurityState<TokenId, TokenAmount, TokenState, StateApi> for State {}
impl ISponsorsState<StateApi> for State {
    fn sponsors(&self) -> &StateSet<ContractAddress, StateApi> { &self.sponsors }

    fn sponsors_mut(&mut self) -> &mut StateSet<ContractAddress, StateApi> { &mut self.sponsors }
}
impl IsAgentsState<StateApi> for State {
    fn agents(&self) -> &StateSet<Address, StateApi> { &self.agents }

    fn agents_mut(&mut self) -> &mut StateSet<Address, StateApi> { &mut self.agents }
}
impl IContractState for State {}
impl IDepositedTokensState<NftTokenOwnerUId, NftTokenAmount, StateApi> for State {
    fn tokens(&self) -> &StateMap<NftTokenOwnerUId, DepositedTokenState<NftTokenAmount>, StateApi> {
        &self.deposited_tokens
    }

    fn tokens_mut(
        &mut self,
    ) -> &mut StateMap<NftTokenOwnerUId, DepositedTokenState<NftTokenAmount>, StateApi> {
        &mut self.deposited_tokens
    }
}
