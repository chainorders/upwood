use concordium_cis2::{
    TokenAmountU32, TokenAmountU64 as Cis2TokenAmountU64, TokenAmountU8, TokenIdU32, TokenIdU8,
    TokenIdVec,
};
use concordium_std::*;
/// Trait representing a token amount.
///
/// This trait is used to define the behavior of token amounts.
pub trait IsTokenAmount:
    concordium_cis2::IsTokenAmount
    + PartialOrd
    + ops::SubAssign
    + Copy
    + ops::AddAssign
    + ops::Sub<Output = Self> {
    /// Returns the zero value of the token amount.
    fn zero() -> Self;

    /// Returns the maximum value of the token amount.
    /// This should return `1` for NFTs.
    fn max_value() -> Self;

    /// Subtracts the given amount from self. Returns None if the amount is too
    /// large.
    ///
    /// # Arguments
    ///
    /// * `other` - The amount to subtract.
    ///
    /// # Returns
    ///
    /// Returns `Some(())` if the subtraction was successful, `None` otherwise.
    fn checked_sub_assign(&mut self, other: Self) -> Option<()> {
        if other.le(self) {
            self.sub_assign(other);
            Some(())
        } else {
            None
        }
    }

    /// Adds the given amount to self. Returns None if the amount is too large.
    ///
    /// # Arguments
    ///
    /// * `other` - The amount to add.
    ///
    /// # Returns
    ///
    /// Returns `Some(())` if the addition was successful, `None` otherwise.
    fn checked_add_assign(&mut self, other: Self) -> Option<()> {
        if other.le(&Self::max_value().sub(*self)) {
            self.add_assign(other);
            Some(())
        } else {
            None
        }
    }

    /// Returns true if the amount is zero.
    fn is_zero(&self) -> bool { self.eq(&Self::zero()) }
}

pub trait IsTokenId: concordium_cis2::IsTokenId + Clone {}

impl IsTokenAmount for TokenAmountU8 {
    fn zero() -> Self { TokenAmountU8(0) }

    fn max_value() -> Self { TokenAmountU8(1) }
}

impl IsTokenAmount for TokenAmountU32 {
    fn zero() -> Self { TokenAmountU32(0) }

    fn max_value() -> Self { TokenAmountU32(1000000) }
}

impl IsTokenAmount for Cis2TokenAmountU64 {
    fn zero() -> Self { Cis2TokenAmountU64(0) }

    fn max_value() -> Self { Cis2TokenAmountU64(u64::MAX) }
}

impl IsTokenId for TokenIdU8 {}
impl IsTokenId for TokenIdU32 {}
impl IsTokenId for TokenIdVec {}