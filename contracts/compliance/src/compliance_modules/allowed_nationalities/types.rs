use concordium_rwa_utils::{cis2_types, common_types};

use crate::compliance;

pub type AttributeTag = common_types::AttributeTag;
pub type AttributeValue = common_types::AttributeValue;
pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = cis2_types::NftTokenAmount;
pub type TokenId = cis2_types::TokenId;
pub type Error = compliance::error::Error;
