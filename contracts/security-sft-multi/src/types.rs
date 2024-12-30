use concordium_cis2::TokenIdU64;
use concordium_protocols::concordium_cis2_ext;
use concordium_protocols::concordium_cis2_security::{self, Cis2SecurityEvent, SecurityParams};
use concordium_std::{SchemaType, Serialize};

use super::error::Error;
pub type ContractResult<R> = Result<R, Error>;
pub type TokenAmount = concordium_cis2::TokenAmountU64;
pub type TokenId = TokenIdU64;
pub type Event = Cis2SecurityEvent<TokenId, TokenAmount, AgentRole>;

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
    AddToken,
    Operator,
    RemoveToken,
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentRole::SetIdentityRegistry => write!(f, "SetIdentityRegistry"),
            AgentRole::SetCompliance => write!(f, "SetCompliance"),
            AgentRole::AddAgent => write!(f, "AddAgent"),
            AgentRole::Mint => write!(f, "Mint"),
            AgentRole::ForcedBurn => write!(f, "ForcedBurn"),
            AgentRole::ForcedTransfer => write!(f, "ForcedTransfer"),
            AgentRole::Freeze => write!(f, "Freeze"),
            AgentRole::UnFreeze => write!(f, "UnFreeze"),
            AgentRole::HolderRecovery => write!(f, "HolderRecovery"),
            AgentRole::Pause => write!(f, "Pause"),
            AgentRole::UnPause => write!(f, "UnPause"),
            AgentRole::AddToken => write!(f, "AddToken"),
            AgentRole::Operator => write!(f, "Operator"),
            AgentRole::RemoveToken => write!(f, "RemoveToken"),
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
            Self::AddToken,
            Self::Operator,
            Self::RemoveToken,
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

#[derive(Serialize, SchemaType)]
pub struct InitParam {
    pub security: Option<SecurityParams>,
    pub agents:   Vec<Agent>,
}
