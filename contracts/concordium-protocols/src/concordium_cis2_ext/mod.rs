pub mod cis2_client;

use concordium_cis2::{
    AdditionalData, TokenAmountU32, TokenAmountU64, TokenAmountU8, TokenIdU32, TokenIdU64, TokenIdU8, TokenIdUnit, TokenIdVec
};
use concordium_std::ops::{Add, AddAssign, Sub};
use concordium_std::*;

/// Trait representing a token amount.
///
/// This trait is used to define the behavior of token amounts.
pub trait IsTokenAmount: concordium_cis2::IsTokenAmount+PartialOrd+ops::SubAssign+Copy+cmp::Ord+ops::AddAssign+ops::Sub<Output=Self>+ops::Add<Output=Self>
{
    /// Returns the zero value of the token amount.
    fn zero() -> Self;

    /// Returns true if the amount is zero.
    fn is_zero(&self) -> bool { self.eq(&Self::zero()) }
}

pub trait IsTokenId: concordium_cis2::IsTokenId+Clone+PartialOrd {}

impl IsTokenAmount for TokenAmountU8 {
    fn zero() -> Self { TokenAmountU8(0) }
}

impl IsTokenAmount for TokenAmountU32 {
    fn zero() -> Self { TokenAmountU32(0) }
}

impl IsTokenAmount for TokenAmountU64 {
    fn zero() -> Self { TokenAmountU64(0) }
}

impl IsTokenId for TokenIdU8 {}
impl IsTokenId for TokenIdU32 {}
impl IsTokenId for TokenIdVec {}
impl IsTokenId for TokenIdUnit {}
impl IsTokenId for TokenIdU64 {}

/// Represents the metadata URL and hash of a token.
#[derive(Debug, SchemaType, Serial, Clone, Deserial)]
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

pub trait PlusSubOne<T> {
    fn plus_one(&self) -> T;
    fn plus_one_assign(&mut self);
    fn sub_one(&self) -> T;
}

impl PlusSubOne<TokenIdU32> for TokenIdU32 {
    fn plus_one(&self) -> Self { TokenIdU32(self.0.add(1)) }

    fn sub_one(&self) -> Self { TokenIdU32(self.0.sub(1)) }

    fn plus_one_assign(&mut self) { self.0.add_assign(1) }
}

impl PlusSubOne<TokenIdU64> for TokenIdU64 {
    fn plus_one(&self) -> Self { TokenIdU64(self.0.add(1)) }

    fn sub_one(&self) -> Self { TokenIdU64(self.0.sub(1)) }

    fn plus_one_assign(&mut self) { self.0.add_assign(1) }
}


pub trait ToAdditionalData {
    fn to_additional_data(&self) -> Result<AdditionalData, ()>;
}

impl <S> ToAdditionalData for S
where
    S: Serial
{
    fn to_additional_data(&self) -> Result<AdditionalData, ()> {
        let mut bytes = Vec::new();
        self.serial(&mut bytes)?;
        Ok(bytes.into())
    }
}
