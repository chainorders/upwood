use concordium_protocols::concordium_cis2_security::{identity_registry_client, RecoverEvent};
use concordium_std::*;

use super::error::*;
use super::state::State;
use super::types::{AgentRole, ContractResult, Event, RecoverParam};
use crate::state::HolderState;

#[receive(
    contract = "security_sft_rewards",
    name = "recover",
    mutable,
    enable_logger,
    parameter = "RecoverParam",
    error = "Error"
)]
pub fn recover(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let RecoverParam {
        lost_account,
        new_account,
    }: RecoverParam = ctx.parameter_cursor().get()?;
    let state = host.state();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::HolderRecovery]));
    ensure!(is_authorized, Error::Unauthorized);
    ensure!(
        identity_registry_client::is_verified(host, &state.identity_registry, &new_account)?,
        Error::UnVerifiedIdentity
    );

    let state = host.state_mut();
    let lost_holder = state
        .addresses
        .insert(lost_account, HolderState::Recovered(new_account));
    let previous_new_account = match lost_holder {
        Some(HolderState::Active(lost_holder)) => state
            .addresses
            .insert(new_account, HolderState::Active(lost_holder)),
        _ => bail!(Error::InvalidAddress),
    };
    ensure!(previous_new_account.is_none(), Error::InvalidAddress);
    logger.log(&Event::Recovered(RecoverEvent {
        lost_account,
        new_account,
    }))?;

    Ok(())
}

#[receive(
    contract = "security_sft_rewards",
    name = "recoveryAddress",
    parameter = "Address",
    error = "Error",
    return_value = "Option<Address>"
)]
pub fn recovery_address(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<Option<Address>> {
    let address: Address = ctx.parameter_cursor().get()?;
    let recovery_address = host
        .state()
        .addresses
        .get(&address)
        .and_then(|holder| holder.recovery_address());
    Ok(recovery_address)
}
