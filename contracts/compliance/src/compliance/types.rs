use concordium_rwa_utils::cis2_types;
use concordium_std::ContractAddress;

use super::error::Error;

pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = cis2_types::NftTokenAmount;
pub type TokenId = cis2_types::TokenId;
pub type Module = ContractAddress;
