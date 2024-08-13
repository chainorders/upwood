use concordium_cis2::{BurnEvent, Cis2Event};
use concordium_protocols::concordium_cis2_security::{compliance_client, BurnedParam, Token, TokenFrozen};
use concordium_rwa_utils::{
    state_implementations::{
        agent_with_roles_state::IAgentWithRolesState,
        holders_security_state::IHoldersSecurityState, holders_state::IHoldersState,
        tokens_security_state::ITokensSecurityState, tokens_state::ITokensState,
    },
};
use concordium_std::*;

use super::{error::Error, state::State, types::*};

/// Burns the specified amount of the given token from the given owner's
/// account.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// - Returns `Error::Unauthorized` if the sender is not authorized to burn the
/// tokens.
/// - Returns `Error::Custom(CustomContractError::PausedToken)` if the
/// token is paused.
/// - Returns `Error::InsufficientFunds` if the owner does not
/// have enough tokens.
#[receive(
    contract = "security_sft_rewards",
    name = "burn",
    parameter = "BurnParams",
    error = "super::error::Error",
    enable_logger,
    mutable
)]
pub fn burn(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let sender = ctx.sender();
    let params: BurnParams = ctx.parameter_cursor().get()?;

    for Burn {
        token_id,
        amount,
        owner,
    } in params.0
    {
        let state = host.state();
        let is_authorized =
        // Sender is the Owner of the token
        owner.eq(&sender)
        // Sender is an operator of the owner
        || state.is_operator(&owner, &sender);

        ensure!(is_authorized, Error::Unauthorized);
        state.ensure_token_exists(&token_id)?;
        state.ensure_not_paused(&token_id)?;
        state.ensure_has_sufficient_unfrozen_balance(&owner, &token_id, &amount)?;
        let compliance = state.compliance();

        host.state_mut().sub_balance(owner, token_id, amount)?;
        compliance_client::burned(host, compliance, &BurnedParam {
            token_id: Token::new(token_id, ctx.self_address()),
            amount,
            owner,
        })?;

        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id,
            owner,
        })))?;
    }

    Ok(())
}

#[receive(
    contract = "security_sft_rewards",
    name = "forcedBurn",
    parameter = "BurnParams",
    error = "super::error::Error",
    enable_logger,
    mutable
)]
pub fn forced_burn(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: BurnParams = ctx.parameter_cursor().get()?;

    let state = host.state();
    ensure!(state.is_agent(&ctx.sender(), vec![&AgentRole::ForcedBurn]), Error::Unauthorized);

    for Burn {
        token_id,
        amount,
        owner,
    } in params.0
    {
        let state = host.state();
        state.ensure_token_exists(&token_id)?;
        state.ensure_not_paused(&token_id)?;
        // Only the balance is checked. The frozen balance is not checked.
        state.ensure_has_sufficient_balance(&owner, &token_id, &amount)?;
        let compliance = state.compliance();

        let state = host.state_mut();
        state.sub_balance(owner, token_id, amount)?;
        let unfrozen_balance = state.adjust_frozen_balance(owner, token_id)?;

        compliance_client::burned(host, compliance, &BurnedParam {
            token_id: Token::new(token_id, ctx.self_address()),
            amount,
            owner,
        })?;

        logger.log(&Event::TokenUnFrozen(TokenFrozen {
            token_id,
            amount: unfrozen_balance,
            address: owner,
        }))?;
        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id,
            owner,
        })))?;
    }

    Ok(())
}
