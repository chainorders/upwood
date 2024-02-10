use concordium_cis2::{IsTokenAmount, IsTokenId};
use concordium_std::*;

use concordium_rwa_utils::{
    agents_state::IsAgentsState,
    holders_security_state::IHoldersSecurityState,
    tokens_state::{ITokensState, TokenStateResult},
};

use super::{error::*, event::*, state::State, types::*};

#[derive(Serialize, SchemaType)]
pub struct FreezeParam<T: IsTokenId, A: IsTokenAmount> {
    pub token_id:     T,
    pub token_amount: A,
}

#[derive(Serialize, SchemaType)]
pub struct FreezeParams<T: IsTokenId, A: IsTokenAmount> {
    pub owner:  Address,
    pub tokens: Vec<FreezeParam<T, A>>,
}

#[derive(Serialize, SchemaType)]
pub struct FrozenParams<T: IsTokenId> {
    pub owner:  Address,
    pub tokens: Vec<T>,
}

#[derive(Serialize, SchemaType)]
pub struct FrozenResponse<T: IsTokenAmount> {
    pub tokens: Vec<T>,
}

/// Freezes the given amount of given tokenIds for the given address.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender is not an agent.
/// Returns `Error::InsufficientFunds` if the owner does not have enough
/// unfrozen balance. Returns `Error::ParseError` if the parameters could not be
/// parsed.
#[receive(
    contract = "rwa_security_sft",
    name = "freeze",
    mutable,
    enable_logger,
    parameter = "FreezeParams<TokenId, TokenAmount>",
    error = "super::error::Error"
)]
pub fn freeze(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let (state, state_builder) = host.state_and_builder();
    // Sender of this transaction should be a Trusted Agent
    ensure!(state.is_agent(&ctx.sender()), Error::Unauthorized);

    let FreezeParams {
        owner,
        tokens,
    }: FreezeParams<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    for token in tokens {
        state.ensure_token_exists(&token.token_id)?;
        state.ensure_has_sufficient_unfrozen_balance(
            &owner,
            &token.token_id,
            &token.token_amount,
        )?;

        state.freeze(owner, &token.token_id, token.token_amount, state_builder)?;
        logger.log(&Event::TokenFrozen(TokenFrozen {
            token_id: token.token_id,
            amount:   token.token_amount,
            address:  owner,
        }))?;
    }

    Ok(())
}

/// Unfreezes the given amount of given tokenIds for the given address.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender is not an agent.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_security_sft",
    name = "unFreeze",
    mutable,
    enable_logger,
    parameter = "FreezeParams<TokenId, TokenAmount>",
    error = "super::error::Error"
)]
pub fn un_freeze(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state_mut();
    // Sender of this transaction should be a Trusted Agent
    ensure!(state.is_agent(&ctx.sender()), Error::Unauthorized);

    let FreezeParams {
        owner,
        tokens,
    }: FreezeParams<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    for token in tokens {
        state.un_freeze(owner, token.token_id, token.token_amount)?;
        logger.log(&Event::TokenUnFrozen(TokenFrozen {
            token_id: token.token_id,
            amount:   token.token_amount,
            address:  owner,
        }))?;
    }

    Ok(())
}

/// Returns the frozen balance of the given token for the given addresses.
///
/// # Returns
///
/// Returns `ContractResult<FrozenResponse<TokenAmount>>` containing the frozen
/// balance of the given token for the given addresses.
///
/// # Errors
///
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_security_sft",
    name = "balanceOfFrozen",
    parameter = "FrozenParams<TokenId>",
    return_value = "FrozenResponse<TokenAmount>",
    error = "super::error::Error"
)]
pub fn balance_of_frozen(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<FrozenResponse<TokenAmount>> {
    let state = host.state();
    let FrozenParams {
        owner,
        tokens: token_ids,
    }: FrozenParams<TokenId> = ctx.parameter_cursor().get()?;

    let tokens = token_ids
        .iter()
        .map(|token_id| {
            state.ensure_token_exists(token_id).map(|_| state.balance_of_frozen(&owner, token_id))
        })
        .collect::<TokenStateResult<Vec<_>>>()?;

    Ok(FrozenResponse {
        tokens,
    })
}

/// Returns the unfrozen balance of the given token for the given addresses.
///
/// # Returns
///
/// Returns `ContractResult<FrozenResponse<TokenAmount>>` containing the
/// unfrozen balance of the given token for the given addresses.
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_security_sft",
    name = "balanceOfUnFrozen",
    parameter = "FrozenParams<TokenId>",
    return_value = "FrozenResponse<TokenAmount>",
    error = "super::error::Error"
)]
pub fn balance_of_un_frozen(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<FrozenResponse<TokenAmount>> {
    let state = host.state();
    let FrozenParams {
        owner,
        tokens: token_ids,
    }: FrozenParams<TokenId> = ctx.parameter_cursor().get()?;

    let tokens = token_ids
        .iter()
        .map(|token_id| {
            state.ensure_token_exists(token_id).map(|_| state.balance_of_unfrozen(&owner, token_id))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(FrozenResponse {
        tokens,
    })
}
