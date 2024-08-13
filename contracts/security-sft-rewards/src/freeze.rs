use concordium_protocols::concordium_cis2_security::TokenFrozen;
use concordium_rwa_utils::state_implementations::{
    agent_with_roles_state::IAgentWithRolesState,
    holders_security_state::IHoldersSecurityState,
    tokens_state::{ITokensState, TokenStateResult},
};
use concordium_std::*;

use super::{error::*, state::State, types::*};

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
    contract = "security_sft_rewards",
    name = "freeze",
    mutable,
    enable_logger,
    parameter = "FreezeParams",
    error = "Error"
)]
pub fn freeze(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let (state, state_builder) = host.state_and_builder();
    // Sender of this transaction should be a Trusted Agent
    ensure!(state.is_agent(&ctx.sender(), vec![AgentRole::Freeze]), Error::Unauthorized);

    let FreezeParams {
        owner,
        tokens,
    }: FreezeParams = ctx.parameter_cursor().get()?;
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
    contract = "security_sft_rewards",
    name = "unFreeze",
    mutable,
    enable_logger,
    parameter = "FreezeParams",
    error = "Error"
)]
pub fn un_freeze(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state_mut();
    // Sender of this transaction should be a Trusted Agent
    ensure!(state.is_agent(&ctx.sender(), vec![AgentRole::UnFreeze]), Error::Unauthorized);

    let FreezeParams {
        owner,
        tokens,
    }: FreezeParams = ctx.parameter_cursor().get()?;
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
/// Returns `ContractResult<FrozenResponse>` containing the frozen
/// balance of the given token for the given addresses.
///
/// # Errors
///
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_rewards",
    name = "balanceOfFrozen",
    parameter = "FrozenParams",
    return_value = "FrozenResponse",
    error = "Error"
)]
pub fn balance_of_frozen(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<FrozenResponse> {
    let state = host.state();
    let FrozenParams {
        owner,
        tokens: token_ids,
    }: FrozenParams = ctx.parameter_cursor().get()?;

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
/// Returns `ContractResult<FrozenResponse>` containing the
/// unfrozen balance of the given token for the given addresses.
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_rewards",
    name = "balanceOfUnFrozen",
    parameter = "FrozenParams",
    return_value = "FrozenResponse",
    error = "Error"
)]
pub fn balance_of_un_frozen(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<FrozenResponse> {
    let state = host.state();
    let FrozenParams {
        owner,
        tokens: token_ids,
    }: FrozenParams = ctx.parameter_cursor().get()?;

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
