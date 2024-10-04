use concordium_std::num::NonZeroI32;
use concordium_std::{LogError, ParseError, Reject, SchemaType};

#[derive(SchemaType)]
pub enum Error {
    /// Triggered when there is an error parsing a value.
    ParseError,
    /// Triggered when there is an error logging a value.
    LogError,
    /// Triggered when the receiver of the token is not verified.
    InvalidTokenId,
    InsufficientFunds,
    /// Sender is unauthorized to call this function (Error code: -42000003).
    Unauthorized,
    /// Triggered when the amount for NFT is not 1.
    InvalidAmount,
    /// Triggered when the provided address is invalid.
    InvalidAddress,
    TransferInvokeError,
    InvalidRewardToken,
    UnauthorizedInvalidAgent,
    UnauthorizedCheckSignature,
    UnauthorizedInvalidSignature,
    MetadataUrlSerialization,
}

impl Error {
    fn error_code(&self) -> NonZeroI32 {
        NonZeroI32::new(match self {
            // CIS2 Errors codes
            Error::InvalidTokenId => -42000001,
            Error::InsufficientFunds => -42000002,
            Error::Unauthorized => -42000003,
            // General Errors codes
            Error::ParseError => -1,
            Error::LogError => -2,
            Error::InvalidAmount => -3,
            Error::InvalidAddress => -4,
            Error::TransferInvokeError => -6,
            Error::InvalidRewardToken => -7,
            Error::UnauthorizedInvalidAgent => -8,
            Error::UnauthorizedCheckSignature => -9,
            Error::UnauthorizedInvalidSignature => -10,
            Error::MetadataUrlSerialization => -11,
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
