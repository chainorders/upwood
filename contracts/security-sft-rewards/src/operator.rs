use concordium_cis2::*;
use concordium_std::*;

use super::error::Error;
use super::state::State;
use super::types::{ContractResult, Event};
use crate::state::HolderState;

/// Updates the operator status for the sender.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_rewards",
    name = "updateOperator",
    mutable,
    enable_logger,
    parameter = "UpdateOperatorParams",
    error = "Error"
)]
pub fn update_operator(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let UpdateOperatorParams { 0: updates }: UpdateOperatorParams = ctx.parameter_cursor().get()?;
    let sender = ctx.sender();
    let (state, state_builder) = host.state_and_builder();

    for UpdateOperator { operator, update } in updates {
        match update {
            OperatorUpdate::Add => state
                .addresses
                .entry(sender)
                .or_insert_with(|| HolderState::new_active(state_builder))
                .add_operator(operator)?,
            OperatorUpdate::Remove => state
                .addresses
                .entry(sender)
                .occupied_or(Error::InvalidAddress)?
                .remove_operator(&operator)?,
        }
        logger.log(&Event::Cis2(Cis2Event::UpdateOperator(
            UpdateOperatorEvent {
                operator,
                update,
                owner: sender,
            },
        )))?;
    }
    Ok(())
}

/// # Returns
///
/// Returns `ContractResult<OperatorOfQueryResponse>` containing a boolean
/// indicating whether the given address is an operator for the given owner.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_rewards",
    name = "operatorOf",
    parameter = "OperatorOfQueryParams",
    return_value = "OperatorOfQueryResponse",
    error = "super::error::Error"
)]
pub fn operator_of(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<OperatorOfQueryResponse> {
    let OperatorOfQueryParams { queries }: OperatorOfQueryParams = ctx.parameter_cursor().get()?;
    let state = host.state();
    let mut res = Vec::with_capacity(queries.len());

    for query in queries {
        res.push(
            state
                .addresses
                .get(&query.owner)
                .ok_or(Error::InvalidAddress)?
                .has_operator(&query.address),
        );
    }

    Ok(OperatorOfQueryResponse(res))
}
