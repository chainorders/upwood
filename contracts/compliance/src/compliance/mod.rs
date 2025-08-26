pub mod error;
mod state;
pub mod types;

use concordium_cis2::{
    StandardIdentifier, SupportResult, SupportsQueryParams, SupportsQueryResponse,
    CIS0_STANDARD_IDENTIFIER,
};
use concordium_protocols::concordium_cis2_security::compliance_client::ComplianceClient;
use concordium_protocols::concordium_cis2_security::{
    BurnedParam, CanTransferParam, MintedParam, TransferredParam, COMPLIANCE_STANDARD_IDENTIFIER,
};
use concordium_std::*;
use error::*;
use state::*;
use types::*;

const SUPPORTS_STANDARDS: [StandardIdentifier<'static>; 2] =
    [CIS0_STANDARD_IDENTIFIER, COMPLIANCE_STANDARD_IDENTIFIER];

#[init(
    contract = "rwa_compliance",
    event = "Event",
    error = "Error",
    parameter = "InitParams",
    enable_logger
)]
pub fn init(
    ctx: &InitContext,
    state_builder: &mut StateBuilder,
    logger: &mut Logger,
) -> ContractResult<State> {
    let params: InitParams = ctx.parameter_cursor().get()?;
    for module in params.modules.iter() {
        logger.log(&Event::ModuleAdded(module.to_owned()))?;
    }
    let mut state = State::new(params.modules, state_builder);
    state.agents.insert(Address::Account(ctx.init_origin()));

    Ok(state)
}

/// Handles the `supports` event in the `rwa_compliance` contract.
///
/// This function is called to check if the contract supports a given standard.
/// It iterates over all standards in the `SUPPORTS_STANDARDS` array, and checks
/// if the queried standard is in the array.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_compliance",
    name = "supports",
    parameter = "SupportsQueryParams",
    return_value = "SupportsQueryResponse",
    error = "Error"
)]
fn supports(ctx: &ReceiveContext, _: &Host<State>) -> ContractResult<SupportsQueryResponse> {
    let params: SupportsQueryParams = ctx.parameter_cursor().get()?;
    let mut response = Vec::with_capacity(params.queries.len());
    for std_id in params.queries {
        if SUPPORTS_STANDARDS.contains(&std_id.as_standard_identifier()) {
            response.push(SupportResult::Support);
        } else {
            response.push(SupportResult::NoSupport)
        }
    }

    Ok(SupportsQueryResponse::from(response))
}

/// Returns true if the given address is an agent.
///
/// # Returns
///
/// Returns `ContractResult<Vec<Address>>` containing the list of agents.
#[receive(
    contract = "rwa_compliance",
    name = "isAgent",
    return_value = "bool",
    parameter = "Address",
    error = "Error"
)]
pub fn is_agent(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let address: Address = ctx.parameter_cursor().get()?;
    Ok(host.state().agents.contains(&address))
}

#[receive(
    contract = "rwa_compliance",
    name = "agents",
    return_value = "Vec<Address>",
    error = "Error"
)]
/// Returns the list of agents.
pub fn agents(_ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Vec<Address>> {
    Ok(host.state.agents.iter().map(|a| *a).collect())
}

/// Adds the given address as an agent.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "rwa_compliance",
    name = "addAgent",
    mutable,
    enable_logger,
    parameter = "Address",
    error = "Error"
)]
pub fn add_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let agent: Address = ctx.parameter_cursor().get()?;
    ensure!(
        host.state_mut().agents.insert(agent),
        Error::AgentAlreadyExists
    );
    logger.log(&Event::AgentAdded(AgentUpdatedEvent { agent }))?;

    Ok(())
}

/// Removes the given address as an agent.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "rwa_compliance",
    name = "removeAgent",
    mutable,
    enable_logger,
    parameter = "Address",
    error = "Error"
)]
pub fn remove_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let agent: Address = ctx.parameter_cursor().get()?;
    ensure!(host.state_mut().agents.remove(&agent), Error::AgentNotFound);
    logger.log(&Event::AgentRemoved(AgentUpdatedEvent { agent }))?;

    Ok(())
}

#[receive(
    contract = "rwa_compliance",
    name = "addModule",
    parameter = "ContractAddress",
    error = "Error",
    mutable,
    enable_logger
)]
pub fn add_module(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        host.state().agents.contains(&ctx.sender()),
        Error::Unauthorized
    );
    let module: ContractAddress = ctx.parameter_cursor().get()?;
    host.state_mut().add_module(module);
    logger.log(&Event::ModuleAdded(module))?;
    Ok(())
}

#[receive(
    contract = "rwa_compliance",
    name = "removeModule",
    parameter = "ContractAddress",
    error = "Error",
    mutable,
    enable_logger
)]
pub fn remove_module(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        host.state().agents.contains(&ctx.sender()),
        Error::Unauthorized
    );
    let module: ContractAddress = ctx.parameter_cursor().get()?;
    host.state_mut().remove_module(&module);
    logger.log(&Event::ModuleRemoved(module))?;
    Ok(())
}

#[receive(
    contract = "rwa_compliance",
    name = "modules",
    return_value = "Vec<ContractAddress>"
)]
pub fn modules(_: &ReceiveContext, host: &Host<State>) -> ContractResult<Vec<ContractAddress>> {
    Ok(host.state().modules())
}

/// Handles the `burned` event in the `rwa_compliance` contract.
///
/// This function is called when tokens are burned. It iterates over all modules
/// in the state, and calls the `burned` function on the `ComplianceContract`
/// for each module.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
/// Returns `Error::Unauthorized` if the sender of the event does not match the
/// contract of the token.
#[receive(
    contract = "rwa_compliance",
    name = "burned",
    parameter = "BurnedParam<TokenId, TokenAmount>",
    error = "Error",
    mutable
)]
fn burned(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: BurnedParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    ensure!(
        ctx.sender().matches_contract(&params.token_id.contract),
        Error::Unauthorized
    );

    let modules = host.state().modules();
    for module in modules {
        host.invoke_compiliance_burned(&module, &params)?;
    }

    Ok(())
}

/// Handles the `minted` event in the `rwa_compliance` contract.
///
/// This function is called when tokens are minted. It iterates over all modules
/// in the state, and calls the `minted` function on the `ComplianceContract`
/// for each module.
///
/// # Errors
/// Returns `Error::ParseError` if the parameters could not be parsed.
/// Returns `Error::Unauthorized` if the sender of the event does not match the
/// contract of the token.
#[receive(
    contract = "rwa_compliance",
    name = "minted",
    parameter = "MintedParam<TokenId, TokenAmount>",
    error = "Error",
    mutable
)]
fn minted(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: MintedParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    for module in host.state.modules().iter() {
        ensure!(
            ctx.sender().matches_contract(&params.token_id.contract),
            Error::Unauthorized
        );
        host.invoke_compiliance_minted(module, &params)?;
    }

    Ok(())
}

/// Handles the `transferred` event in the `rwa_compliance` contract.
///
/// This function is called when tokens are transferred. It iterates over all
/// modules in the state, and calls the `transferred` function on the
/// `ComplianceContract` for each module.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender of the event does not match the
/// contract of the token. Returns `Error::ParseError` if the parameters could
/// not be parsed.
#[receive(
    contract = "rwa_compliance",
    name = "transferred",
    parameter = "TransferredParam<TokenId, TokenAmount>",
    error = "Error",
    mutable
)]
fn transferred(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: TransferredParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    let modules = host.state().modules();

    for module in modules {
        ensure!(
            ctx.sender().matches_contract(&params.token_id.contract),
            Error::Unauthorized
        );
        host.invoke_compiliance_transferred(&module, &params)?;
    }

    Ok(())
}

/// Handles the `can_transfer` event in the `rwa_compliance` contract.
///
/// This function is called to check if a transfer of tokens can be made. It
/// iterates over all modules in the state, and calls the `can_transfer`
/// function on the `ComplianceContract` for each module.
#[receive(
    contract = "rwa_compliance",
    name = "canTransfer",
    parameter = "CanTransferParam<TokenId, TokenAmount>",
    return_value = "bool",
    error = "Error"
)]
fn can_transfer(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let params: CanTransferParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    let state = host.state();

    for module in state.modules.iter() {
        let can_transfer = host.invoke_compiliance_can_transfer(&module, &params)?;
        if !can_transfer {
            return Ok(false);
        }
    }

    Ok(true)
}
