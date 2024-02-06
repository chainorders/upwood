use concordium_std::*;

use super::{error::Error, state::State, types::*};
use concordium_rwa_utils::{
    clients::compliance_client::{ComplianceContract, IComplianceClient},
    compliance_types::*,
};

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
    error = "super::error::Error"
)]
fn minted(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<()> {
    let params: MintedParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    let state = host.state();

    for module in state.modules.iter() {
        ensure!(ctx.sender().matches_contract(&params.token_id.contract), Error::Unauthorized);

        ComplianceContract(module.to_owned()).minted(
            host,
            params.token_id,
            params.owner,
            params.amount,
        )?;
    }

    Ok(())
}
