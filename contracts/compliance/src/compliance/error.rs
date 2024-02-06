use concordium_std::*;

use concordium_rwa_utils::clients::contract_client::ContractClientError;

#[derive(Serial, Reject, SchemaType)]
pub enum Error {
    ParseError,
    LogError,
    InvalidModule,
    CallContractError,
    Unauthorized,
    AgentAlreadyExists,
    AgentNotFound,
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}

impl From<ContractClientError<()>> for Error {
    fn from(e: ContractClientError<()>) -> Self {
        match e {
            ContractClientError::NoResponse => Error::InvalidModule,
            ContractClientError::InvalidResponse => Error::InvalidModule,
            ContractClientError::CallContractError(_) => Error::CallContractError,
            // these should not happen
            ContractClientError::ParseResult => Error::ParseError,
            ContractClientError::ParseResultError => Error::ParseError,
        }
    }
}

impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}
