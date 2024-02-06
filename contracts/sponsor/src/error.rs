use concordium_std::*;

use concordium_rwa_utils::concordium_cis3::SupportsCis3Error;

#[derive(SchemaType, Serialize, Reject)]
pub enum Error {
    ParseError,
    LogError,
    WrongContract,
    Expired,
    NonceMismatch,
    WrongSignature,
    SerializationError,
    AccountMissing,
    CallContractError,
    CIS3NotImplemented,
    CIS3CheckError,
}

impl From<CheckAccountSignatureError> for Error {
    fn from(_: CheckAccountSignatureError) -> Self { Error::AccountMissing }
}

impl<T> From<CallContractError<T>> for Error {
    fn from(_: CallContractError<T>) -> Self { Error::CallContractError }
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}

impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}

impl From<SupportsCis3Error> for Error {
    fn from(_: SupportsCis3Error) -> Self { Error::CIS3CheckError }
}
