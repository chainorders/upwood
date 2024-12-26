use attributes::NATIONALITY;
use concordium_protocols::concordium_cis2_security::identity_registry_client::IdentityRegistryClient;
use concordium_protocols::concordium_cis2_security::{CanTransferParam, IdentityAttribute};
use concordium_std::*;
use state::*;
use types::*;

mod state;
pub mod types;

#[init(
    contract = "rwa_compliance_module_allowed_nationalities",
    error = "Error",
    parameter = "InitParams"
)]
pub fn init(ctx: &InitContext, state_builder: &mut StateBuilder) -> ContractResult<State> {
    let params: InitParams = ctx.parameter_cursor().get()?;

    Ok(State::new(
        params.identity_registry,
        params.nationalities,
        state_builder,
    ))
}

#[receive(
    contract = "rwa_compliance_module_allowed_nationalities",
    name = "burned",
    error = "Error"
)]
fn burned(_: &ReceiveContext, _: &Host<State>) -> ContractResult<()> { Ok(()) }

#[receive(
    contract = "rwa_compliance_module_allowed_nationalities",
    name = "canTransfer",
    parameter = "CanTransferParam<TokenId, TokenAmount>",
    return_value = "bool",
    error = "Error"
)]
fn can_transfer(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let params: CanTransferParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    let state = host.state();

    let id = host.invoke_identity_registry_get_identity(&state.identity_registry(), params.to)?;
    for IdentityAttribute { tag, value } in id.attributes {
        if tag.eq(&NATIONALITY.0) {
            return Ok(state.is_allowed(value));
        }
    }

    Ok(false)
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
    contract = "rwa_compliance_module_allowed_nationalities",
    name = "minted",
    error = "Error"
)]
fn minted(_: &ReceiveContext, _: &Host<State>) -> ContractResult<()> { Ok(()) }

#[receive(
    contract = "rwa_compliance_module_allowed_nationalities",
    name = "transferred",
    error = "Error"
)]
fn transferred(_: &ReceiveContext, _: &Host<State>) -> ContractResult<()> { Ok(()) }
