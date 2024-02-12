use concordium_std::{HasStateApi, Serialize};

use crate::clients::contract_client::IContractState;

use super::{
    holders_security_state::IHoldersSecurityState, holders_state::IsTokenId,
    tokens_security_state::ITokensSecurityState, tokens_state::IsTokenAmount,
};

/// Trait representing the security state of the Cis2 contract.
/// It combines the functionality of `ITokensSecurityState` and
/// `IHoldersSecurityState` traits.
pub trait ICis2SecurityState<
    T: IsTokenId,
    A: IsTokenAmount,
    TTokenState: Serialize + Clone,
    S: HasStateApi,
>:
    ITokensSecurityState<T, TTokenState, S> + IHoldersSecurityState<T, A, S> + IContractState {
}

pub enum Cis2SecurityStateError {
    InvalidTokenId,
    InsufficientFunds,
    InvalidAmount,
}
