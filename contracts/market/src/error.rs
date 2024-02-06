use concordium_std::*;

use super::types::ExchangeError;

#[derive(Serial, Reject, SchemaType, Debug)]
pub enum Error {
    ParseError,
    LogError,
    /// The sender is not authorized to call.
    Unauthorized,
    InvalidExchange,
    OnlyAccount,
    InsufficientSupply,
    InvalidRate,
    NotListed,
    InsufficientDeposits,
    InsufficientPayment,
    PaymentNotRequired,
    InvalidDepositData,
    InvalidListToken,
    InvalidPaymentToken,
    InvalidCommission,
    InvalidSupply,
    InvalidExchangeRates,
    Cis2WithdrawError,
    Cis2SettlementError,
    Cis2PaymentError,
    Cis2CommissionPaymentError,
    CCDPaymentError,
    CCDCommissionPaymentError,
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}

impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}

impl From<ExchangeError> for Error {
    fn from(value: ExchangeError) -> Self {
        match value {
            ExchangeError::InsufficientSupply => Error::InsufficientSupply,
            ExchangeError::InvalidRate => Error::InvalidRate,
            ExchangeError::TokenNotListed => Error::NotListed,
        }
    }
}
