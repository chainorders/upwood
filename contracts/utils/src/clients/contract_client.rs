use concordium_cis2::{
    StandardIdentifier, SupportResult, SupportsQueryParams, SupportsQueryResponse,
};
use concordium_std::*;

/// Errors which can be returned by the any Concordium Contract.
#[derive(Debug)]
pub enum ContractClientError<T> {
    /// Invoking the contract returned the given error.
    CallContractError(CallContractError<T>),
    /// The ok response from the contract could not be parsed.
    ParseResult,
    /// The contract did not return a response. This can happen if the contract
    /// is v0
    NoResponse,
    /// The error response from the contract could not be parsed.
    ParseResultError,
    /// The contract returned an invalid response. This can happen if contract
    /// returns an unexpected response. Ex returning a single result when
    /// multiple are queried.
    InvalidResponse,
}

impl<E: Deserial, R: Read> TryFrom<CallContractError<R>> for ContractClientError<E> {
    type Error = ParseError;

    fn try_from(value: CallContractError<R>) -> Result<Self, Self::Error> {
        match value {
            CallContractError::AmountTooLarge => {
                Ok(ContractClientError::CallContractError(CallContractError::AmountTooLarge))
            }
            CallContractError::MissingAccount => {
                Ok(ContractClientError::CallContractError(CallContractError::MissingAccount))
            }
            CallContractError::MissingContract => {
                Ok(ContractClientError::CallContractError(CallContractError::MissingContract))
            }
            CallContractError::MissingEntrypoint => {
                Ok(ContractClientError::CallContractError(CallContractError::MissingEntrypoint))
            }
            CallContractError::MessageFailed => {
                Ok(ContractClientError::CallContractError(CallContractError::MessageFailed))
            }
            CallContractError::LogicReject {
                reason,
                mut return_value,
            } => Ok(ContractClientError::CallContractError(CallContractError::LogicReject {
                reason,
                return_value: E::deserial(&mut return_value).map_err(|_| ParseError {})?,
            })),
            CallContractError::Trap => {
                Ok(ContractClientError::CallContractError(CallContractError::Trap))
            }
        }
    }
}

pub trait IContractState: Serial + DeserialWithState<StateApi> {}

pub trait IContractClient<State: IContractState> {
    /// Returns the contract address.
    fn contract_address(&self) -> ContractAddress;

    /// Returns the standard identifier.
    fn standard_identifier(&self) -> StandardIdentifier;

    /// Checks if the contract supports the standard identifier.
    ///
    /// # Errors
    ///
    /// Returns `ContractClientError` if there is an error checking the support.
    fn is_valid(&self, host: &Host<State>) -> Result<bool, ContractClientError<()>> {
        self.supports(host, self.standard_identifier())
    }

    /// Checks if the contract supports the given identifier.
    ///
    /// # Errors
    ///
    /// Returns `ContractClientError` if there is an error checking the support.
    fn supports(
        &self,
        host: &Host<State>,
        identifier: StandardIdentifier,
    ) -> Result<bool, ContractClientError<()>> {
        let parameter = SupportsQueryParams {
            queries: vec![identifier.to_owned()],
        };

        let res: Result<SupportsQueryResponse, _> = self.invoke_contract_read_only(
            host,
            EntrypointName::new_unchecked("supports"),
            &parameter,
        );

        match res {
            Ok(res) => {
                if res.results.len() != 1 {
                    bail!(ContractClientError::InvalidResponse);
                }
                match &res.results[0] {
                    SupportResult::NoSupport => Ok(false),
                    SupportResult::Support => Ok(true),
                    // Reason for returning false here is that the current contract (response of
                    // `contract_address`) does not support the given identifier.
                    // Hence any requests to the current contract will fail.
                    SupportResult::SupportBy(contracts) => {
                        Ok(contracts.contains(&self.contract_address()))
                    }
                }
            }
            Err(e) => bail!(e),
        }
    }

    /// Invokes a contract in read-only mode. The state of the Callee contract
    /// should not change.
    ///
    /// # Errors
    ///
    /// Returns `ContractClientError` if there is an error invoking the
    /// contract.
    fn invoke_contract_read_only<P: Serial, R: Deserial, E: Deserial>(
        &self,
        host: &Host<State>,
        method: EntrypointName,
        parameter: &P,
    ) -> Result<R, ContractClientError<E>> {
        let res = host.invoke_contract_read_only(
            &self.contract_address(),
            parameter,
            method,
            Amount::from_ccd(0),
        );

        let res = match res {
            Ok(res) => res,
            Err(err) => {
                let err = ContractClientError::try_from(err);
                match err {
                    Ok(err) => return Err(err),
                    Err(_) => return Err(ContractClientError::ParseResultError),
                }
            }
        };

        match res {
            // Since the contract should return a response. If it doesn't, it is an error.
            Some(mut res) => R::deserial(&mut res).map_err(|_| ContractClientError::ParseResult),
            None => bail!(ContractClientError::NoResponse),
        }
    }
}
