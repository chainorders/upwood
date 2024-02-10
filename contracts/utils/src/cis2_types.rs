use concordium_cis2::{
    TokenAmountU32 as Cis2TokenAmountU32, TokenAmountU64 as Cis2TokenAmountU64,
    TokenAmountU8 as Cis2TokenAmountU8, TokenIdU32, TokenIdU8, TokenIdVec,
};

use super::{holders_state::IsTokenId, tokens_state::IsTokenAmount};

pub type TokenId = TokenIdU8;
pub type NftTokenAmount = Cis2TokenAmountU8;

/// Trait implementation for a NFT token amount.
impl IsTokenAmount for NftTokenAmount {
    fn zero() -> Self { Cis2TokenAmountU8(0) }

    fn max_value() -> Self { Cis2TokenAmountU8(1) }
}

pub type SftTokenId = TokenIdU32;
pub type SftTokenAmount = Cis2TokenAmountU32;

/// Trait implementation for a SFT token amount.
impl IsTokenAmount for SftTokenAmount {
    fn zero() -> Self { Cis2TokenAmountU32(0) }

    fn max_value() -> Self { Cis2TokenAmountU32(1000000) }
}

impl IsTokenAmount for Cis2TokenAmountU64 {
    fn zero() -> Self { Cis2TokenAmountU64(0) }

    fn max_value() -> Self { Cis2TokenAmountU64(u64::MAX) }
}

impl IsTokenId for TokenId {}
impl IsTokenId for SftTokenId {}
impl IsTokenId for TokenIdVec {}
