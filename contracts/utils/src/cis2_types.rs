use concordium_cis2::{
    TokenAmountU32 as Cis2TokenAmountU32, TokenAmountU8 as Cis2TokenAmountU8, TokenIdU32, TokenIdU8,
};

pub type TokenId = TokenIdU8;
pub type NftTokenAmount = Cis2TokenAmountU8;
pub type SftTokenId = TokenIdU32;
pub type SftTokenAmount = Cis2TokenAmountU32;
