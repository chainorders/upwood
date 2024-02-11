use concordium_cis2::{TokenAmountU64, TokenIdVec};
use concordium_rwa_utils::common_types;

use crate::compliance;

pub type AttributeTag = common_types::AttributeTag;
pub type AttributeValue = common_types::AttributeValue;
pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = TokenAmountU64;
pub type TokenId = TokenIdVec;
pub type Error = compliance::error::Error;
