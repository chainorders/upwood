use concordium_cis2::{
    TokenAmountU32, TokenAmountU64, TokenAmountU8, TokenIdU32, TokenIdU8, TokenIdUnit, TokenIdVec,
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

    fn max_value() -> Self { TokenAmountU32(u32::MAX) }
}

impl IsTokenAmount for TokenAmountU64 {
    fn zero() -> Self { TokenAmountU64(0) }

    fn max_value() -> Self { TokenAmountU64(u64::MAX) }
}

impl IsTokenId for TokenIdU8 {}
impl IsTokenId for TokenIdU32 {}
impl IsTokenId for TokenIdVec {}
impl IsTokenId for TokenIdUnit {}

/// Represents the metadata URL and hash of a token.
#[derive(SchemaType, Serial, Clone, Deserial)]
pub struct ContractMetadataUrl {
    pub url:  String,
    pub hash: Option<String>,
}

impl From<ContractMetadataUrl> for MetadataUrl {
    fn from(val: ContractMetadataUrl) -> Self {
        MetadataUrl {
            url:  val.url,
            hash: {
                if let Some(hash) = val.hash {
                    let mut hash_bytes = [0u8; 32];
                    match hex::decode_to_slice(hash, &mut hash_bytes) {
                        Ok(_) => Some(hash_bytes),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            },
        }
    }
}
