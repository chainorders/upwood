use concordium_protocols::concordium_cis2_security;

pub type ContractResult<T> = Result<T, super::error::Error>;
pub type AttributeTag = concordium_cis2_security::AttributeTag;
pub type AttributeValue = concordium_cis2_security::AttributeValue;
pub type Identity = concordium_cis2_security::Identity;
pub type Issuer = concordium_cis2_security::Issuer;
pub use concordium_cis2_security::{IdentityAttribute, IdentityCredential};
use concordium_std::{Address, SchemaType, Serialize};
pub type CredentialId = concordium_std::PublicKeyEd25519;
/// Represents an event that is triggered when an identity is updated.
#[derive(Serialize, SchemaType, Debug)]
#[concordium(transparent)]
pub struct IdentityUpdatedEvent {
    /// The address associated with the identity.
    pub address: Address,
}

/// Represents an event that is triggered when an issuer is updated.
#[derive(Serialize, SchemaType, Debug)]
#[concordium(transparent)]
pub struct IssuerUpdatedEvent {
    /// The issuer that was updated.
    pub issuer: Issuer,
}

/// Represents an event that is triggered when an agent is updated (Added /
/// Removed).
#[derive(Serialize, SchemaType, Debug)]
pub struct AgentUpdatedEvent {
    /// The agent that was updated.
    pub agent: Address,
}

/// Represents the different types of events that can be triggered in the
/// contract.
#[derive(Serialize, SchemaType, Debug)]
#[concordium(repr(u8))]
pub enum Event {
    /// Triggered when a new identity is registered.
    IdentityRegistered(IdentityUpdatedEvent),
    /// Triggered when an identity is removed.
    IdentityRemoved(IdentityUpdatedEvent),
    /// Triggered when a new issuer is added.
    IssuerAdded(IssuerUpdatedEvent),
    /// Triggered when an issuer is removed.
    IssuerRemoved(IssuerUpdatedEvent),
    /// Triggered when a new agent is added.
    AgentAdded(AgentUpdatedEvent),
    /// Triggered when an agent is removed.
    AgentRemoved(AgentUpdatedEvent),
}
/// Parameters for registering an identity.
#[derive(Serialize, SchemaType)]
pub struct RegisterIdentityParams {
    pub identity: Identity,
    pub address:  Address,
}
