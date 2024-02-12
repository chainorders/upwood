use std::num::NonZeroI32;

use concordium_std::*;

use concordium_rwa_utils::{
    cis2_state::Cis2StateError, clients::contract_client::ContractClientError,
    holders_security_state::HolderSecurityStateError, holders_state::HolderStateError,
    token_deposits_state::DepositedStateError, tokens_security_state::TokenSecurityError,
    tokens_state::TokenStateError,
};

#[derive(SchemaType)]
pub enum Error {
    InvalidTokenId,
    /// The balance of the token owner is insufficient for the transfer (Error
    /// code: -42000002).
    InsufficientFunds,
    /// Sender is unauthorized to call this function (Error code: -42000003).
    Unauthorized,
    /// Triggered when there is an error parsing a value.
    ParseError,
    /// Triggered when there is an error logging a value.
    LogError,
    /// Triggered when the receiver of the token is not verified.
    UnVerifiedIdentity,
    /// Triggered when the transfer is non-compliant.
    InCompliantTransfer,
    /// Triggered when there is an error calling the Compliance Contract.
    ComplianceError,
    /// Triggered when there is an error invoking a contract.
    CallContractError,
    /// Triggered when the token is paused.
    PausedToken,
    /// Triggered when the amount for NFT is not 1.
    InvalidAmount,
    /// Triggered when the provided address is invalid.
    InvalidAddress,
    /// Triggered when an agent already exists.
    AgentAlreadyExists,
    /// Triggered when an agent could not be found.
    AgentNotFound,
    OnlyAccount,
    InvalidDepositData,
    Cis2WithdrawError,
    InsufficientDeposits,
    NotDeposited,
    InsufficientFractionalized,
    InvalidFractionsRate
}

impl Error {
    fn error_code(&self) -> NonZeroI32 {
        NonZeroI32::new(match self {
            Error::InvalidTokenId => -42000001,
            Error::InsufficientFunds => -42000002,
            Error::Unauthorized => -42000003,
            Error::ParseError => -1,
            Error::LogError => -2,
            Error::UnVerifiedIdentity => -3,
            Error::InCompliantTransfer => -4,
            Error::ComplianceError => -5,
            Error::CallContractError => -6,
            Error::PausedToken => -7,
            Error::InvalidAmount => -8,
            Error::InvalidAddress => -9,
            Error::AgentAlreadyExists => -10,
            Error::AgentNotFound => -11,
            Error::OnlyAccount => -12,
            Error::InvalidDepositData => -13,
            Error::Cis2WithdrawError => -14,
            Error::InsufficientDeposits => -15,
            Error::NotDeposited => -16,
            Error::InsufficientFractionalized => -17,
            Error::InvalidFractionsRate => -18,
        })
        .unwrap()
    }
}

impl From<Error> for Reject {
    fn from(err: Error) -> Self {
        Reject {
            error_code: err.error_code(),
            ..Default::default()
        }
    }
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}

impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}

impl From<TokenStateError> for Error {
    fn from(_: TokenStateError) -> Self { Error::InvalidTokenId }
}

impl From<HolderStateError> for Error {
    fn from(_: HolderStateError) -> Self { Error::InsufficientFunds }
}

impl<T> From<ContractClientError<T>> for Error {
    fn from(_: ContractClientError<T>) -> Self { Error::CallContractError }
}

impl From<TokenSecurityError> for Error {
    fn from(value: TokenSecurityError) -> Self {
        match value {
            TokenSecurityError::PausedToken => Error::PausedToken,
        }
    }
}

impl From<HolderSecurityStateError> for Error {
    fn from(e: HolderSecurityStateError) -> Self {
        match e {
            HolderSecurityStateError::AmountTooLarge => Error::InsufficientFunds,
            HolderSecurityStateError::AmountOverflow => Error::InvalidAmount,
            HolderSecurityStateError::AddressAlreadyRecovered => Error::InvalidAddress,
            HolderSecurityStateError::InvalidRecoveryAddress => Error::InvalidAddress,
        }
    }
}

impl<T> From<CallContractError<T>> for Error {
    fn from(_: CallContractError<T>) -> Self { Error::CallContractError }
}

impl From<Cis2StateError> for Error {
    fn from(value: Cis2StateError) -> Self {
        match value {
            Cis2StateError::InvalidTokenId => Error::InvalidTokenId,
            Cis2StateError::InsufficientFunds => Error::InsufficientFunds,
            Cis2StateError::InvalidAmount => Error::InvalidAmount,
        }
    }
}

impl From<DepositedStateError> for Error {
    fn from(value: DepositedStateError) -> Self {
        match value {
            DepositedStateError::TokenNotFound => Error::NotDeposited,
            DepositedStateError::InsufficientDeposits => Error::InsufficientDeposits,
            DepositedStateError::InsufficientLocked => Error::InsufficientFractionalized,
        }
    }
}
