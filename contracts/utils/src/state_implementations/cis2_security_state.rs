use super::{
    holders_security_state::IHoldersSecurityState, tokens_security_state::ITokensSecurityState,
};
use crate::clients::contract_client::IContractState;
use concordium_protocols::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use concordium_std::{HasStateApi, Serialize};

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
