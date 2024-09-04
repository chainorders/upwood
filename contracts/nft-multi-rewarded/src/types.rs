use concordium_cis2::{Cis2Event, TokenIdU32, TokenIdU64, TokenIdVec};
use concordium_protocols::concordium_cis2_ext;
use concordium_protocols::concordium_cis2_security::{self, TokenUId};
use concordium_std::{Address, SchemaType, Serialize};

use super::error::Error;

pub type ContractResult<R> = Result<R, Error>;
pub type TokenAmount = concordium_cis2::TokenAmountU8;
pub type TokenId = TokenIdU64;
pub type RewardTokenId = TokenIdU32;
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

#[derive(Serialize, SchemaType)]
pub struct InitParam {
    pub reward_token: TokenUId<TokenIdVec>,
}

#[derive(Serialize, SchemaType)]
pub enum Event {
    Init(InitParam),
    AgentAdded(Address),
    AgentRemoved(Address),
    Cis2(Cis2Event<TokenId, TokenAmount>),
}
