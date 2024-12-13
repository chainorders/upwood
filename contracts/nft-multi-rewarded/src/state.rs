use concordium_protocols::concordium_cis2_security::TokenUId;
use concordium_std::{
    Address, Deletable, DeserialWithState, HasStateApi, MetadataUrl, Serial, StateApi,
    StateBuilder, StateMap, StateSet,
};

use super::types::{RewardTokenId, TokenId};

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the security NFT contract.
pub struct State<S=StateApi> {
    pub reward_token:  TokenUId<RewardTokenId>,
    pub curr_token_id: TokenId,
    pub tokens:        StateMap<TokenId, MetadataUrl, S>,
    pub addresses:     StateMap<Address, HolderState<S>, S>,
}

#[derive(DeserialWithState, Serial, Deletable)]
#[concordium(state_parameter = "S")]
pub struct HolderState<S=StateApi> {
    pub operators: StateSet<Address, S>,
    pub balances:  StateSet<TokenId, S>,
    pub nonce:     u64,
    pub is_agent:  bool,
}
impl<S: HasStateApi> HolderState<S> {
    #[inline]
    pub fn new(state_builder: &mut StateBuilder<S>) -> Self {
        HolderState {
            operators: state_builder.new_set(),
            balances:  state_builder.new_set(),
            nonce:     0,
            is_agent:  false,
        }
    }
}
