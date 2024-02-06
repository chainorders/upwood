use concordium_std::{attributes::NATIONALITY, *};

use super::{state::State, types::*};
use concordium_rwa_utils::{
    clients::identity_registry_client::{IdentityRegistryClient, IdentityRegistryContract},
    common_types::{Identity, IdentityAttribute},
    compliance_types::*,
};

#[receive(
    contract = "rwa_compliance_module_allowed_nationalities",
    name = "canTransfer",
    parameter = "CanTransferParam<TokenId, TokenAmount>",
    return_value = "bool",
    error = "super::types::Error"
)]
fn can_transfer(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let params: CanTransferParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    let state = host.state();
    let idr = IdentityRegistryContract(state.identity_registry());
    let id: Identity = idr.get_identity(host, params.to)?;
    for IdentityAttribute {
        tag,
        value,
    } in id.attributes
    {
        if tag.eq(&NATIONALITY.0) {
            return Ok(state.is_allowed(value));
        }
    }

    Ok(false)
}
