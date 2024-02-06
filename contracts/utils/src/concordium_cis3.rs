use concordium_cis2::{
    StandardIdentifier, SupportResult, SupportsQueryParams, SupportsQueryResponse,
};
use concordium_std::*;

pub const CIS3_STANDARD_IDENTIFIER: StandardIdentifier<'static> =
    StandardIdentifier::new_unchecked("CIS-3");

#[derive(SchemaType, Serialize, PartialEq, Debug)]
pub struct PermitMessage {
    pub contract_address: ContractAddress,
    pub nonce:            u64,
    pub timestamp:        Timestamp,
    pub entry_point:      OwnedEntrypointName,
    #[concordium(size_length = 2)]
    pub payload:          Vec<u8>,
}

impl PermitMessage {
    // It has a `Unit` error type because implementation of the `Write` trait for
    // `Cursor` has `Unit` error type.
    pub fn bytes(&self) -> Result<Vec<u8>, ParseError> {
        let mut bytes: Vec<u8> = vec![];
        let mut out = Cursor::new(&mut bytes);
        // TODO: remove dependency on re-serialization of the message
        Self::serial(self, &mut out).map_err(|_| ParseError::default())?;
        Ok(bytes)
    }
}

#[derive(SchemaType, Serialize)]
pub struct PermitParam {
    pub signature: AccountSignatures,
    pub signer:    AccountAddress,
    pub message:   PermitMessage,
}

pub enum SupportsCis3Error {
    V0Contract,
    InvalidResponseLength,
    CallContractError,
    ParseError,
}

impl<T> From<CallContractError<T>> for SupportsCis3Error {
    fn from(_: CallContractError<T>) -> Self { SupportsCis3Error::CallContractError }
}

impl From<ParseError> for SupportsCis3Error {
    fn from(_: ParseError) -> Self { SupportsCis3Error::ParseError }
}

pub fn supports_cis3_by<S>(
    host: &mut impl HasHost<S>,
    host_contract: &ContractAddress,
    implementor_contract: &ContractAddress,
) -> Result<bool, SupportsCis3Error> {
    let supports_parameter = SupportsQueryParams {
        queries: vec![CIS3_STANDARD_IDENTIFIER.to_owned()],
    };
    let (_, supports_res) = host.invoke_contract(
        host_contract,
        &supports_parameter,
        EntrypointName::new_unchecked("supports"),
        Amount::from_micro_ccd(0),
    )?;
    let SupportsQueryResponse {
        results: supports_res,
    } = match supports_res {
        Some(mut supports_res) => supports_res.get()?,
        None => return Err(SupportsCis3Error::V0Contract),
    };
    let supports_res = match supports_res.first() {
        Some(supports_res) => match supports_res {
            SupportResult::NoSupport => false,
            SupportResult::Support => false,
            SupportResult::SupportBy(contracts) => contracts.contains(implementor_contract),
        },
        None => return Err(SupportsCis3Error::InvalidResponseLength),
    };

    Ok(supports_res)
}

#[cfg(test)]
mod tests {
    use concordium_std::{ContractAddress, Cursor, Deserial, OwnedEntrypointName, Timestamp};

    use super::PermitMessage;

    #[test]
    fn permit_message_serde() {
        let msg = PermitMessage {
            contract_address: ContractAddress::new(1, 2),
            nonce:            23,
            timestamp:        Timestamp::from_timestamp_millis(1234),
            entry_point:      OwnedEntrypointName::new_unchecked("transfer".to_owned()),
            payload:          vec![0, 1, 2, 3, 4],
        };
        let msg_bytes = msg.bytes().unwrap();
        let mut cur2 = Cursor::new(&msg_bytes);
        let msg_deserial = PermitMessage::deserial(&mut cur2).unwrap();
        assert_eq!(msg, msg_deserial);
    }
}
