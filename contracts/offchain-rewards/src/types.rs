use std::num::NonZeroI32;

use concordium_cis2::TokenIdUnit;
use concordium_protocols::concordium_cis2_security::{self};
use concordium_std::{
    AccountAddress, AccountSignatures, Address, ContractAddress, LogError, ParseError, Reject,
    SchemaType, Serial, Serialize,
};

pub type ContractResult<R> = Result<R, Error>;
pub type RewardTokenId = TokenIdUnit; // only fungible tokens allowed to be rewarded
pub type RewardTokenAmount = concordium_cis2::TokenAmountU64;

pub type Agent = concordium_cis2_security::Agent;

#[derive(Serialize, SchemaType, Debug)]
pub struct InitParam {
    pub treasury: Address,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum Event {
    Init(InitParam),
    AgentAdded(Address),
    AgentRemoved(Address),
    Claimed(ClaimedEvent),
}

#[derive(Serialize, SchemaType, Debug)]
pub struct ClaimedEvent {
    pub reward_id:             Vec<u8>,
    pub account_address:       AccountAddress,
    pub nonce:                 u64,
    pub reward_token_id:       RewardTokenId,
    pub reward_token_contract: ContractAddress,
    pub reward_amount:         RewardTokenAmount,
}

#[derive(Serialize, SchemaType)]
pub struct ClaimRequest {
    pub claim:     ClaimInfo,
    pub signer:    AccountAddress,
    pub signature: AccountSignatures,
}

impl From<&ClaimRequest> for Vec<u8> {
    fn from(val: &ClaimRequest) -> Self {
        let mut data = Vec::new();
        val.serial(&mut data).unwrap();
        data
    }
}

#[derive(Serialize, SchemaType, Debug)]
pub struct ClaimInfo {
    pub contract_address:      ContractAddress,
    pub account:               AccountAddress,
    pub account_nonce:         u64,
    pub reward_id:             Vec<u8>,
    pub reward_token_id:       RewardTokenId,
    pub reward_token_contract: ContractAddress,
    pub reward_amount:         RewardTokenAmount,
}

impl From<&ClaimInfo> for Vec<u8> {
    fn from(val: &ClaimInfo) -> Self {
        let mut data = Vec::new();
        val.serial(&mut data).unwrap();
        data
    }
}

impl ClaimInfo {
    pub fn hash<T>(&self, hasher: T) -> ContractResult<[u8; 32]>
    where T: FnOnce(Vec<u8>) -> [u8; 32] {
        let hash: Vec<u8> = self.into();
        let hash = hasher(hash);
        Ok(hash)
    }
}

#[derive(SchemaType)]
pub enum Error {
    /// Triggered when there is an error parsing a value.
    ParseError,
    /// Triggered when there is an error logging a value.
    LogError,
    /// Sender is unauthorized to call this function (Error code: -42000003).
    Unauthorized,
    /// Triggered when the provided address is invalid.
    InvalidAddress,
    UnauthorizedInvalidAgent,
    CheckSignature,
    InvalidSignature,
    InvalidNonce,
    InvalidContractAddress,
    InvokeContract,
}

impl Error {
    fn error_code(&self) -> NonZeroI32 {
        NonZeroI32::new(match self {
            // CIS2 Errors codes
            Error::Unauthorized => -42000003,
            // General Errors codes
            Error::ParseError => -1,
            Error::LogError => -2,
            Error::InvalidAddress => -3,
            Error::UnauthorizedInvalidAgent => -4,
            Error::CheckSignature => -5,
            Error::InvalidSignature => -6,
            Error::InvalidNonce => -7,
            Error::InvalidContractAddress => -8,
            Error::InvokeContract => -9,
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
