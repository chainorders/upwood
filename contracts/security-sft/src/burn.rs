use super::{error::Error, event::Event, state::State, types::*};
use concordium_cis2::{BurnEvent, Cis2Event};
use concordium_rwa_utils::{
    clients::compliance_client::{ComplianceContract, IComplianceClient},
    compliance_types::Token,
    holders_security_state::IHoldersSecurityState,
    holders_state::IHoldersState,
    tokens_security_state::ITokensSecurityState,
    tokens_state::ITokensState,
};
use concordium_std::*;

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
/// Returns `Error::Unauthorized` if the sender is not authorized to burn the
/// tokens. Returns `Error::Custom(CustomContractError::PausedToken)` if the
/// token is paused. Returns `Error::InsufficientFunds` if the owner does not
/// have enough tokens.
#[receive(
    contract = "rwa_security_sft",
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

        host.state_mut().sub_balance(owner, token_id, amount)?;
        ComplianceContract(host.state().compliance()).burned(
            host,
            Token::new(token_id, ctx.self_address()),
            owner,
            amount,
        )?;

        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id,
            owner,
        })))?;
    }

    Ok(())
}
