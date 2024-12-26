use concordium_cis2::{TokenAmountU64, TokenIdVec};
use concordium_protocols::concordium_cis2_security;
use concordium_std::{ContractAddress, SchemaType, Serialize};

use crate::compliance;

pub type AttributeTag = concordium_cis2_security::AttributeTag;
pub type AttributeValue = concordium_cis2_security::AttributeValue;
pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = TokenAmountU64;
pub type TokenId = TokenIdVec;
pub type Error = compliance::error::Error;
#[derive(Serialize, SchemaType)]
pub struct InitParams {
    pub nationalities:     Vec<AttributeValue>,
    pub identity_registry: ContractAddress,
}
