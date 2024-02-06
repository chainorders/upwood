use concordium_cis2::{TokenAmountU8 as Cis2TokenAmountU8, TokenIdU8};
use concordium_std::*;

use super::{holders_state::IsTokenId, tokens_state::IsTokenAmount};

pub type TokenId = TokenIdU8;
pub type NftTokenAmount = Cis2TokenAmountU8;

/// Trait implementation for a NFT token amount.
impl IsTokenAmount for NftTokenAmount {
    fn zero() -> Self { Cis2TokenAmountU8(0) }

    fn max_value() -> Self { Cis2TokenAmountU8(1) }
}

#[derive(Serialize, SchemaType, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
#[concordium(transparent)]
pub struct SftTokenAmount(pub Cis2TokenAmountU8);
impl concordium_cis2::IsTokenAmount for SftTokenAmount {}

/// Trait implementation for a SFT token amount.
impl IsTokenAmount for SftTokenAmount {
    fn zero() -> Self { SftTokenAmount(Cis2TokenAmountU8(0)) }

    fn max_value() -> Self { SftTokenAmount(Cis2TokenAmountU8(u8::MAX)) }
}

impl ops::SubAssign for SftTokenAmount {
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}

impl ops::AddAssign for SftTokenAmount {
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl ops::Sub for SftTokenAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { SftTokenAmount(self.0 - rhs.0) }
}

impl IsTokenId for TokenId {}
