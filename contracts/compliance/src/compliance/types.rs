use concordium_cis2::{TokenAmountU64, TokenIdVec};
use concordium_std::ContractAddress;

use super::error::Error;

pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = TokenAmountU64;
pub type TokenId = TokenIdVec;
pub type Module = ContractAddress;
