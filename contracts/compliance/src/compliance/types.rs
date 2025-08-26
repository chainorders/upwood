use concordium_cis2::{TokenAmountU64, TokenIdVec};
use concordium_std::{Address, ContractAddress, SchemaType, Serialize};

use super::error::Error;

pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = TokenAmountU64;
pub type TokenId = TokenIdVec;
pub type Module = ContractAddress;
#[derive(Serialize, SchemaType)]
pub struct AgentUpdatedEvent {
    pub agent: Address,
}

#[derive(Serialize, SchemaType)]
pub enum Event {
    AgentRemoved(AgentUpdatedEvent),
    AgentAdded(AgentUpdatedEvent),
    ModuleAdded(ContractAddress),
    ModuleRemoved(ContractAddress),
}

#[derive(Serialize, SchemaType)]
pub struct InitParams {
    pub modules: Vec<Module>,
}
