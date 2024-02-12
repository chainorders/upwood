#![allow(clippy::manual_range_patterns)]
use concordium_cis2::{Cis2Event, IsTokenAmount, IsTokenId};
use concordium_std::{schema::SchemaType, *};

use super::types::{TokenAmount, TokenId, NftTokenAmount, NftTokenUId};
/// Represents an event that is triggered when an agent is updated (Added /
/// Removed).
#[derive(Serialize, SchemaType, Debug)]
pub struct AgentUpdatedEvent {
    pub agent: Address,
}

/// Represents the event when tokens are frozen / un frozen.
#[derive(Serialize, SchemaType, Debug)]
pub struct TokenFrozen<T: IsTokenId + SchemaType, A: IsTokenAmount + SchemaType> {
    pub token_id: T,
    pub amount:   A,
    pub address:  Address,
}

/// Represents the event when a token is paused / unpaused.
#[derive(Serialize, SchemaType, Debug)]
pub struct Paused<T: IsTokenId + SchemaType> {
    pub token_id: T,
}

/// Represents the event when an identity registry is added.
#[derive(Serialize, SchemaType, Debug)]
#[concordium(transparent)]
pub struct IdentityRegistryAdded(pub ContractAddress);

/// Represents the event when compliance is added.
#[derive(Serialize, SchemaType, Debug)]
#[concordium(transparent)]
pub struct ComplianceAdded(pub ContractAddress);

/// Represents an event for recovering a lost account.
#[derive(Serialize, SchemaType, Debug)]
pub struct RecoverEvent {
    /// The address of the lost account.
    pub lost_account: Address,
    /// The address of the new account.
    pub new_account:  Address,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct TokenDeposited {
    pub token_id: NftTokenUId,
    pub owner:    AccountAddress,
    pub amount:   NftTokenAmount,
}


#[derive(Serialize, SchemaType, Debug)]
#[concordium(repr(u8))]
pub enum Event {
    #[concordium(tag = 240)]
    Deposited(TokenDeposited),
    #[concordium(tag = 241)]
    Withdraw(TokenDeposited),
    /// Event triggered when an account is recovered.
    #[concordium(tag = 242)]
    Recovered(RecoverEvent),

    /// Event triggered when an identity registry is added.
    #[concordium(tag = 243)]
    IdentityRegistryAdded(IdentityRegistryAdded),

    /// Event triggered when compliance is added.
    #[concordium(tag = 244)]
    ComplianceAdded(ComplianceAdded),

    /// Event triggered when a token is unpaused.
    #[concordium(tag = 245)]
    UnPaused(Paused<TokenId>),

    /// Event triggered when a token is paused.
    #[concordium(tag = 246)]
    Paused(Paused<TokenId>),

    /// Event triggered when tokens are frozen.
    #[concordium(tag = 247)]
    TokenFrozen(TokenFrozen<TokenId, TokenAmount>),

    /// Event triggered when tokens are unfrozen.
    #[concordium(tag = 248)]
    TokenUnFrozen(TokenFrozen<TokenId, TokenAmount>),

    /// Event triggered when an agent is removed.
    #[concordium(tag = 249)]
    AgentRemoved(AgentUpdatedEvent),

    /// Event triggered when an agent is added.
    #[concordium(tag = 250)]
    AgentAdded(AgentUpdatedEvent),

    /// Event forwarded from the CIS2 contract.
    #[concordium(forward = cis2_events)]
    Cis2(Cis2Event<TokenId, TokenAmount>),
}
