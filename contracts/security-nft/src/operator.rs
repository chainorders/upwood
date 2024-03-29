use super::{error::Error, event::Event, state::State, types::ContractResult};
use concordium_cis2::*;
use concordium_rwa_utils::holders_state::IHoldersState;
use concordium_std::*;

/// Updates the operator status for the sender.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_security_nft",
    name = "updateOperator",
    mutable,
    enable_logger,
    parameter = "UpdateOperatorParams",
    error = "super::error::Error"
)]
pub fn update_operator(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let UpdateOperatorParams {
        0: updates,
    }: UpdateOperatorParams = ctx.parameter_cursor().get()?;
    let (state, state_builder) = host.state_and_builder();
    let sender = ctx.sender();
    ensure!(sender.is_account(), Error::InvalidAddress);

    for UpdateOperator {
        operator,
        update,
    } in updates
    {
        match update {
            OperatorUpdate::Add => state.add_operator(sender, operator, state_builder),
            OperatorUpdate::Remove => state.remove_operator(sender, &operator),
        }

        logger.log(&Event::Cis2(Cis2Event::UpdateOperator(UpdateOperatorEvent {
            operator,
            update,
            owner: sender,
        })))?;
    }

    Ok(())
}

// Returns true if the given address is an operator for the given owner.
///
/// # Returns
///
/// Returns `ContractResult<OperatorOfQueryResponse>` containing a boolean
/// indicating whether the given address is an operator for the given owner.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_security_nft",
    name = "operatorOf",
    parameter = "OperatorOfQueryParams",
    return_value = "OperatorOfQueryResponse",
    error = "super::error::Error"
)]
pub fn operator_of(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<OperatorOfQueryResponse> {
    let OperatorOfQueryParams {
        queries,
    }: OperatorOfQueryParams = ctx.parameter_cursor().get()?;
    let state = host.state();
    let res: Vec<bool> = queries.iter().map(|q| state.is_operator(&q.owner, &q.address)).collect();

    Ok(OperatorOfQueryResponse(res))
}
