use concordium_cis2::{Cis2Event, TokenIdU64, TokenIdVec};
use concordium_protocols::concordium_cis2_ext;
use concordium_protocols::concordium_cis2_security::{self, TokenUId};
use concordium_std::{Address, SchemaType, Serialize};

use super::error::Error;

pub type ContractResult<R> = Result<R, Error>;
pub type TokenAmount = concordium_cis2::TokenAmountU8;
pub type TokenId = TokenIdU64;
pub type RewardTokenId = TokenIdVec;
pub type RewardTokenAmount = concordium_cis2::TokenAmountU64;

pub type Agent = concordium_cis2_security::Agent;
pub type BurnParams = concordium_cis2_security::BurnParams<TokenId, TokenAmount>;
pub type Burn = concordium_cis2_security::Burn<TokenId, TokenAmount>;
pub type TransferParams = concordium_cis2::TransferParams<TokenId, TokenAmount>;
pub type BalanceOfQueryParams = concordium_cis2::BalanceOfQueryParams<TokenId>;
pub type BalanceOfQueryResponse = concordium_cis2::BalanceOfQueryResponse<TokenAmount>;
pub type MintParams = concordium_cis2_security::MintParams<TokenId, TokenAmount>;
pub type MintParam = concordium_cis2_security::MintParam<TokenAmount>;
pub use concordium_cis2_ext::ContractMetadataUrl;
pub use concordium_std::ContractAddress;

#[derive(Serialize, SchemaType, Debug)]
pub struct InitParam {
    /// token id of the fungible token to be used as reward token.
    /// upon receiving this token nft's would be allowed to mint equal to the amount of reward token received.
    /// the reward token would be burned.
    pub reward_token: TokenUId<RewardTokenId>,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum Event {
    RewardTokenUpdated(InitParam),
    AgentAdded(Address),
    AgentRemoved(Address),
    NonceUpdated(Address, u64),
    Cis2(Cis2Event<TokenId, TokenAmount>),
}
