use concordium_std::*;

#[derive(SchemaType, Serialize, Reject)]
pub enum Error {
    ParseError,
    LogError,
    SerializationError,
    CallContractError,
    CIS3CheckError,
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}

impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}
