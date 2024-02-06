use concordium_std::*;

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
