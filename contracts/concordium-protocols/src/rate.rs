use concordium_cis2::TokenAmountU64;
use concordium_std::{SchemaType, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum ExchangeError {
    InvalidRate,
}

#[derive(Serialize, SchemaType, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rate {
    pub numerator:   u64,
    pub denominator: u64,
}

impl Default for Rate {
    fn default() -> Self {
        Self {
            numerator:   1,
            denominator: 1,
        }
    }
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

    fn convert(&self, amount: &u64) -> Result<(u64, u64), ExchangeError> {
        let numerator = amount
            .checked_mul(self.numerator)
            .ok_or(ExchangeError::InvalidRate)?;
        let converted_amount = numerator
            .checked_div(self.denominator)
            .ok_or(ExchangeError::InvalidRate)?;
        let remainder = numerator
            .checked_rem(self.denominator)
            .ok_or(ExchangeError::InvalidRate)?;
        Ok((converted_amount, remainder))
    }

    fn convert_inverse(&self, amount: &u64) -> Result<(u64, u64), ExchangeError> {
        let numerator = amount
            .checked_mul(self.denominator)
            .ok_or(ExchangeError::InvalidRate)?;
        let converted_amount = numerator
            .checked_div(self.numerator)
            .ok_or(ExchangeError::InvalidRate)?;
        let remainder = numerator
            .checked_rem(self.numerator)
            .ok_or(ExchangeError::InvalidRate)?;
        Ok((converted_amount, remainder))
    }

    /// Convert the given security amount to a currency amount.
    /// Returns the converted currency amount.
    /// If the full conversion is not possible, an error is returned.
    pub fn convert_token_amount(
        &self,
        security_amount: &TokenAmountU64,
    ) -> Result<TokenAmountU64, ExchangeError> {
        let (converted_security_amount, unconverted_security_amount) =
            self.convert(&security_amount.0)?;
        if unconverted_security_amount != 0 {
            return Err(ExchangeError::InvalidRate);
        }
        Ok(TokenAmountU64(converted_security_amount))
    }

    pub fn convert_token_amount_with_rem(
        &self,
        security_amount: &TokenAmountU64,
    ) -> Result<(TokenAmountU64, TokenAmountU64), ExchangeError> {
        let (converted_security_amount, unconverted_security_amount) =
            self.convert(&security_amount.0)?;
        Ok((
            TokenAmountU64(converted_security_amount),
            TokenAmountU64(unconverted_security_amount),
        ))
    }

    /// Convert the given currency amount to a security amount.
    /// Returns the converted security amount.
    /// If the full conversion is not possible, an error is returned.
    pub fn convert_currency_amount(
        &self,
        currency_amount: &TokenAmountU64,
    ) -> Result<TokenAmountU64, ExchangeError> {
        let (converted_currency_amount, unconverted_currency_amount) =
            self.convert_inverse(&currency_amount.0)?;
        if unconverted_currency_amount != 0 {
            return Err(ExchangeError::InvalidRate);
        }
        Ok(TokenAmountU64(converted_currency_amount))
    }
}

#[cfg(test)]
mod tests {
    use super::Rate;
    use crate::rate::ExchangeError;

    #[test]
    fn calculate_amounts_rem() {
        let rate = Rate::new(1, 3).expect("valid rate");
        assert_eq!(rate.convert(&2).expect("valid amount"), (0, 2));
        assert_eq!(rate.convert_inverse(&2).expect("valid amount"), (6, 0));
        assert_eq!(
            rate.convert_token_amount(&2.into()),
            Err(ExchangeError::InvalidRate)
        );
        assert_eq!(rate.convert_currency_amount(&2.into()), Ok(6.into()));
    }

    #[test]
    fn calculate_amounts() {
        let rate = Rate::new(1, 3).expect("valid rate");
        assert_eq!(rate.convert(&3).expect("valid amount"), (1, 0));
        assert_eq!(rate.convert_inverse(&3).expect("valid amount"), (9, 0));
        assert_eq!(rate.convert_token_amount(&3.into()), Ok(1.into()));
        assert_eq!(rate.convert_currency_amount(&3.into()), Ok(9.into()));
    }
}
