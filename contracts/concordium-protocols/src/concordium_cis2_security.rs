use concordium_cis2::{Cis2Event, IsTokenAmount, IsTokenId, Receiver, TokenAmountU64, TokenIdVec};
use concordium_std::{
    schema::SchemaType, AccountAddress, Address, ContractAddress, Cursor, SchemaType, Serialize,
};

#[derive(Serialize, SchemaType)]
pub struct PauseParams<T: IsTokenId> {
    pub tokens: Vec<T>,
}

#[derive(Serialize, SchemaType, PartialEq, Debug)]
pub struct IsPausedResponse {
    pub tokens: Vec<bool>,
}

#[derive(Debug, Serialize, Clone, SchemaType)]
pub struct Burn<T: IsTokenId, A: IsTokenAmount> {
    pub token_id: T,
    pub amount:   A,
    pub owner:    Address,
}

#[derive(Debug, Serialize, Clone, SchemaType)]
#[concordium(transparent)]
pub struct BurnParams<T: IsTokenId, A: IsTokenAmount>(
    #[concordium(size_length = 2)] pub Vec<Burn<T, A>>,
);

#[derive(Serialize, SchemaType)]
pub struct FreezeParam<T: IsTokenId, A: IsTokenAmount> {
    pub token_id:     T,
    pub token_amount: A,
}

#[derive(Serialize, SchemaType)]
pub struct FreezeParams<T: IsTokenId, A: IsTokenAmount> {
    pub owner:  Address,
    pub tokens: Vec<FreezeParam<T, A>>,
}

#[derive(Serialize, SchemaType)]
pub struct FrozenParams<T: IsTokenId> {
    pub owner:  Address,
    pub tokens: Vec<T>,
}

#[derive(Serialize, SchemaType, PartialEq, Debug)]
pub struct FrozenResponse<A: IsTokenAmount> {
    pub tokens: Vec<A>,
}

#[derive(Serialize, SchemaType)]
pub struct RecoverParam {
    pub lost_account: Address,
    pub new_account:  Address,
}

pub type Agent = Address;

#[derive(Serialize, SchemaType, Clone)]
pub struct AgentWithRoles<TAgentRole> {
    pub address: Address,
    pub roles:   Vec<TAgentRole>,
}

#[derive(Serialize, SchemaType, Clone, Debug)]
pub struct TokenUId<T> {
    pub contract: ContractAddress,
    pub id:       T,
}

impl<T: Eq> PartialEq for TokenUId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.contract.eq(&other.contract) && self.id.eq(&other.id)
    }
}
impl<T: Eq> Eq for TokenUId<T> {}

impl<T: Clone> TokenUId<T> {
    pub fn to_token_owner_uid(&self, owner: Receiver) -> TokenOwnerUId<T> {
        TokenOwnerUId::new(self.clone(), owner)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct TokenOwnerUId<T> {
    pub token_id: TokenUId<T>,
    pub owner:    Receiver,
}

impl<T> TokenOwnerUId<T> {
    pub fn new(token_id: TokenUId<T>, owner: Receiver) -> Self {
        Self {
            token_id,
            owner,
        }
    }
}

impl<T: Eq> TokenOwnerUId<T> {
    pub fn matches(&self, token_id: &TokenUId<T>) -> bool { self.token_id.eq(token_id) }
}

/// Represents an event that is triggered when an agent is updated (Added /
/// Removed).
#[derive(Serialize, SchemaType, Debug)]
pub struct AgentUpdatedEvent<R> {
    pub agent: Address,
    pub roles: Vec<R>,
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
    pub token_id: TokenUId<TokenIdVec>,
    pub owner:    AccountAddress,
    pub amount:   TokenAmountU64,
}

#[derive(Serialize, SchemaType, Debug)]
#[concordium(repr(u8))]
pub enum Cis2SecurityEvent<T, A, R>
where
    T: IsTokenId,
    A: IsTokenAmount, {
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
    UnPaused(Paused<T>),

    /// Event triggered when a token is paused.
    #[concordium(tag = 246)]
    Paused(Paused<T>),

    /// Event triggered when tokens are frozen.
    #[concordium(tag = 247)]
    TokenFrozen(TokenFrozen<T, A>),

    /// Event triggered when tokens are unfrozen.
    #[concordium(tag = 248)]
    TokenUnFrozen(TokenFrozen<T, A>),

    /// Event triggered when an agent is removed.
    #[concordium(tag = 249)]
    AgentRemoved(AgentUpdatedEvent<R>),

    /// Event triggered when an agent is added.
    #[concordium(tag = 250)]
    AgentAdded(AgentUpdatedEvent<R>),

    /// Event forwarded from the CIS2 contract.
    #[concordium(forward = cis2_events)]
    Cis2(Cis2Event<T, A>),
}
