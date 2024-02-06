use concordium_std::*;

use super::{state::State, types::ContractResult};

/// Initializes the contract
#[init(contract = "rwa_sponsor", event = "super::event::Event", error = "super::error::Error")]
pub fn init(_: &InitContext, state_builder: &mut StateBuilder) -> ContractResult<State> {
    Ok(State::new(state_builder))
}
