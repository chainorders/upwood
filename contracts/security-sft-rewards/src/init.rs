use concordium_protocols::concordium_cis2_security::{ComplianceAdded, IdentityRegistryAdded};
use concordium_rwa_utils::state_implementations::{
    agent_with_roles_state::IAgentWithRolesState, holders_security_state::IHoldersSecurityState,
};
use concordium_std::*;

use super::{
    error::Error,
    state::State,
    types::{Agent, AgentRole, ContractResult, Event, InitParam},
};

/// Initializes the contract with the given parameters.
///
/// # Returns
///
/// Returns `InitResult<State>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[init(
    contract = "security_sft_rewards",
    event = "Event",
    error = "Error",
    parameter = "InitParam",
    enable_logger
)]
pub fn init(
    ctx: &InitContext,
    state_builder: &mut StateBuilder,
    logger: &mut Logger,
) -> InitResult<State> {
    let params: InitParam = ctx.parameter_cursor().get()?;
    let owner = Address::Account(ctx.init_origin());
    let state = State::new(
        params.identity_registry,
        params.compliance,
        params.sponsors,
        // Adds owner as an agent
        vec![Agent {
            address: owner,
            roles:   AgentRole::owner_roles(),
        }],
        params.metadata_url.into(),
        state_builder,
    );

    logger.log(&Event::IdentityRegistryAdded(IdentityRegistryAdded(params.identity_registry)))?;
    logger.log(&Event::ComplianceAdded(ComplianceAdded(params.compliance)))?;

    Ok(state)
}

/// Returns the address of the identity registry contract.
///
/// # Returns
///
/// Returns `ContractResult<ContractAddress>` containing the address of the
/// identity registry contract.
#[receive(
    contract = "security_sft_rewards",
    name = "identityRegistry",
    return_value = "ContractAddress"
)]
pub fn identity_registry(
    _: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<ContractAddress> {
    Ok(host.state().identity_registry())
}

#[receive(
    contract = "security_sft_rewards",
    name = "setIdentityRegistry",
    mutable,
    enable_logger,
    parameter = "ContractAddress",
    error = "super::error::Error"
)]
pub fn set_identity_registry(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let identity_registry: ContractAddress = ctx.parameter_cursor().get()?;
    ensure!(
        host.state().is_agent(&ctx.sender(), vec![&AgentRole::SetIdentityRegistry]),
        Error::Unauthorized
    );

    // IdentityRegistryClient::new(identity_registry).is_identity_registry()?;

    host.state_mut().set_identity_registry(identity_registry);
    logger.log(&Event::IdentityRegistryAdded(IdentityRegistryAdded(identity_registry)))?;

    Ok(())
}

/// Returns the address of the compliance contract.
///
/// # Returns
///
/// Returns `ContractResult<ContractAddress>` containing the address of the
/// compliance contract.
#[receive(contract = "security_sft_rewards", name = "compliance", return_value = "ContractAddress")]
pub fn compliance(_: &ReceiveContext, host: &Host<State>) -> ContractResult<ContractAddress> {
    Ok(host.state().compliance())
}

#[receive(
    contract = "security_sft_rewards",
    name = "setCompliance",
    mutable,
    enable_logger,
    parameter = "ContractAddress",
    error = "super::error::Error"
)]
pub fn set_compliance(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let compliance: ContractAddress = ctx.parameter_cursor().get()?;
    ensure!(
        host.state().is_agent(&ctx.sender(), vec![&AgentRole::SetCompliance]),
        Error::Unauthorized
    );

    host.state_mut().set_compliance(compliance);
    logger.log(&Event::ComplianceAdded(ComplianceAdded(compliance)))?;

    Ok(())
}
