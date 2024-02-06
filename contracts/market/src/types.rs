use concordium_cis2::{TokenAmountU64, TokenIdVec};
use concordium_std::*;

use super::error::Error;

pub type TokenId = TokenIdVec;

#[derive(Serialize, SchemaType, Clone, Debug)]
pub struct TokenUId {
    pub contract: ContractAddress,
    pub id:       TokenId,
}

impl PartialEq for TokenUId {
    fn eq(&self, other: &Self) -> bool {
        self.contract.eq(&other.contract) && self.id.eq(&other.id)
    }
}
impl Eq for TokenUId {}

impl TokenUId {
    pub fn new(contract: ContractAddress, id: TokenId) -> Self {
        Self {
            contract,
            id,
        }
    }
}
/// Represents the amount of a token. This should be large enough to accommodate
/// for any token amount which can be received Or exchanged by the contract.
pub type Cis2TokenAmount = TokenAmountU64;

pub type ContractResult<T> = Result<T, Error>;

#[derive(Serialize, SchemaType, Clone)]
pub struct Rate {
    pub numerator:   u64,
    pub denominator: u64,
}

impl Rate {
    pub fn new(numerator: u64, denominator: u64) -> Result<Self, ExchangeError> {
        if denominator == 0 {
            return Err(ExchangeError::InvalidRate);
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Is Less than `1`
    pub fn le_1(&self) -> bool { self.numerator <= self.denominator }

    /// is Valid
    pub fn is_valid(&self) -> bool { self.numerator != 0 && self.denominator != 0 }

    pub fn convert(&self, amount: &u64) -> Result<(u64, u64), ExchangeError> {
        let numerator = amount.checked_mul(self.numerator).ok_or(ExchangeError::InvalidRate)?;
        let converted_amount =
            numerator.checked_div(self.denominator).ok_or(ExchangeError::InvalidRate)?;
        let remainder =
            numerator.checked_rem(self.denominator).ok_or(ExchangeError::InvalidRate)?;
        Ok((converted_amount, remainder))
    }
}

#[derive(Serialize, SchemaType, Clone)]
pub enum ExchangeRate {
    Ccd(Rate),
    Cis2((TokenUId, Rate)),
}

impl ExchangeRate {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Ccd(rate) => rate.is_valid(),
            Self::Cis2((_, rate)) => rate.is_valid(),
        }
    }
}

#[derive(Debug)]
pub enum ExchangeError {
    TokenNotListed,
    InsufficientSupply,
    InvalidRate,
}

#[cfg(test)]
mod tests {
    use super::Rate;

    #[test]
    fn calculate_amounts_rem() {
        let rate = Rate::new(1, 3).expect("valid rate");
        assert_eq!(rate.convert(&2).expect("valid amount"), (0, 2));
    }

    #[test]
    fn calculate_amounts() {
        let rate = Rate::new(1, 3).expect("valid rate");
        assert_eq!(rate.convert(&3).expect("valid amount"), (1, 0));
    }
}
