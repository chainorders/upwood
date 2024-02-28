use concordium_cis2::StandardIdentifier;
use concordium_std::*;

pub const CIS3_STANDARD_IDENTIFIER: StandardIdentifier<'static> =
    StandardIdentifier::new_unchecked("CIS-3");

#[derive(SchemaType, Serialize, PartialEq, Debug)]
pub struct PermitMessage {
    pub contract_address: ContractAddress,
    pub nonce: u64,
    pub timestamp: Timestamp,
    pub entry_point: OwnedEntrypointName,
    #[concordium(size_length = 2)]
    pub payload: Vec<u8>,
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

#[derive(SchemaType, Serialize, Debug)]
pub struct PermitParam {
    pub signature: AccountSignatures,
    pub signer: AccountAddress,
    pub message: PermitMessage,
}

#[cfg(test)]
mod tests {
    use concordium_std::{ContractAddress, Cursor, Deserial, OwnedEntrypointName, Timestamp};

    use super::PermitMessage;

    #[test]
    fn permit_message_serde() {
        let msg = PermitMessage {
            contract_address: ContractAddress::new(1, 2),
            nonce: 23,
            timestamp: Timestamp::from_timestamp_millis(1234),
            entry_point: OwnedEntrypointName::new_unchecked("transfer".to_owned()),
            payload: vec![0, 1, 2, 3, 4],
        };
        let msg_bytes = msg.bytes().unwrap();
        let mut cur2 = Cursor::new(&msg_bytes);
        let msg_deserial = PermitMessage::deserial(&mut cur2).unwrap();
        assert_eq!(msg, msg_deserial);
    }
}
