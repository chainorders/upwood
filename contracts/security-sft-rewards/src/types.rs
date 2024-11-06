use concordium_cis2::{Cis2Event, TokenAmountU64, TokenIdU32, TokenIdVec};
use concordium_protocols::concordium_cis2_ext;
use concordium_protocols::concordium_cis2_security::{
    self, AgentUpdatedEvent, ComplianceAdded, IdentityRegistryAdded, Paused, RecoverEvent,
    TokenFrozen,
};
use concordium_protocols::rate::Rate;
use concordium_std::*;

use super::error::Error;

pub type ContractResult<R> = Result<R, Error>;
pub type TokenAmount = concordium_cis2::TokenAmountU64;
pub type TokenId = TokenIdU32;

#[derive(Serialize, SchemaType, Debug)]
#[concordium(repr(u8))]
pub enum Event {
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
    AgentRemoved(AgentUpdatedEvent<AgentRole>),

    /// Event triggered when an agent is added.
    #[concordium(tag = 250)]
    AgentAdded(AgentUpdatedEvent<AgentRole>),

    /// Event forwarded from the CIS2 contract.
    #[concordium(forward = cis2_events)]
    Cis2(Cis2Event<TokenId, TokenAmount>),

    /// Event triggered when a reward is added.
    #[concordium(tag = 200)]
    RewardAdded(RewardAddedEvent),

    #[concordium(tag = 201)]
    RewardClaimed(RewardClaimedEvent),
}

#[derive(Serialize, SchemaType, Debug)]
pub struct RewardAddedEvent {
    pub token_id:                TokenId,
    pub rewarded_token_contract: ContractAddress,
    pub rewarded_token_id:       TokenIdVec,
    pub reward_amount:           TokenAmountU64,
    pub reward_rate:             Rate,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct RewardClaimedEvent {
    pub token_id:                TokenId,
    pub amount:                  TokenAmount,
    pub rewarded_token_contract: ContractAddress,
    pub rewarded_token_id:       TokenIdVec,
    pub reward_amount:           TokenAmountU64,
    pub owner:                   Address,
}

#[derive(Debug, Serialize, SchemaType, PartialEq, Eq, Clone, Copy)]
pub enum AgentRole {
    SetIdentityRegistry,
    SetCompliance,
    AddAgent,
    Mint,
    /// The role to force a burn of tokens. This roles also means that while
    /// burning the agent will be able to unfreeze tokens
    ForcedBurn,
    /// The role to force a transfer of tokens. This roles also means that while
    /// transferring the agent will be able to unfreeze tokens
    ForcedTransfer,
    Freeze,
    UnFreeze,
    HolderRecovery,
    Pause,
    UnPause,
    Rewarder,
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentRole::AddAgent => write!(f, "AddAgent"),
            AgentRole::SetIdentityRegistry => write!(f, "SetIdentityRegistry"),
            AgentRole::SetCompliance => write!(f, "SetCompliance"),
            AgentRole::Mint => write!(f, "Mint"),
            AgentRole::ForcedBurn => write!(f, "ForcedBurn"),
            AgentRole::ForcedTransfer => write!(f, "ForcedTransfer"),
            AgentRole::Freeze => write!(f, "Freeze"),
            AgentRole::UnFreeze => write!(f, "UnFreeze"),
            AgentRole::HolderRecovery => write!(f, "HolderRecovery"),
            AgentRole::Pause => write!(f, "Pause"),
            AgentRole::UnPause => write!(f, "UnPause"),
            AgentRole::Rewarder => write!(f, "Rewarder"),
        }
    }
}

impl AgentRole {
    /// Returns a list of roles that can be assigned to the owner of the
    /// contract. This should ideally be all the roles.
    pub fn owner_roles() -> Vec<Self> {
        vec![
            Self::SetIdentityRegistry,
            Self::SetCompliance,
            Self::AddAgent,
            Self::Mint,
            Self::ForcedBurn,
            Self::ForcedTransfer,
            Self::Freeze,
            Self::UnFreeze,
            Self::HolderRecovery,
            Self::Pause,
            Self::UnPause,
            Self::Rewarder,
        ]
    }
}

pub type Agent = concordium_cis2_security::AgentWithRoles<AgentRole>;
pub type BurnParams = concordium_cis2_security::BurnParams<TokenId, TokenAmount>;
pub type Burn = concordium_cis2_security::Burn<TokenId, TokenAmount>;
pub type FreezeParams = concordium_cis2_security::FreezeParams<TokenId, TokenAmount>;
pub type TransferParams = concordium_cis2::TransferParams<TokenId, TokenAmount>;
pub type PauseParams = concordium_cis2_security::PauseParams<TokenId>;
pub type PauseParam = concordium_cis2_security::PauseParam<TokenId>;
pub type IsPausedResponse = concordium_cis2_security::IsPausedResponse;
pub type BalanceOfQueryParams = concordium_cis2::BalanceOfQueryParams<TokenId>;
pub type BalanceOfQueryResponse = concordium_cis2::BalanceOfQueryResponse<TokenAmount>;
pub type MintParams = concordium_cis2_security::MintParams<TokenId, TokenAmount>;
pub type MintParam = concordium_cis2_security::MintParam<TokenAmount>;
pub use concordium_cis2_ext::ContractMetadataUrl;
pub use concordium_cis2_security::RecoverParam;

pub const TRACKED_TOKEN_ID: TokenId = TokenIdU32(0);
pub const MIN_REWARD_TOKEN_ID: TokenId = TokenIdU32(1);

#[derive(Serialize, SchemaType)]
pub struct InitParam {
    pub identity_registry:         ContractAddress,
    pub compliance:                ContractAddress,
    pub sponsors:                  Option<ContractAddress>,
    pub metadata_url:              ContractMetadataUrl,
    pub blank_reward_metadata_url: ContractMetadataUrl,
}
