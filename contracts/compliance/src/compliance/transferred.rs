use concordium_protocols::concordium_cis2_security::TransferredParam;
use concordium_rwa_utils::clients::compliance_client::{ComplianceContract, IComplianceClient};
use concordium_std::*;

use super::{error::Error, state::State, types::*};

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
    error = "super::error::Error"
)]
fn transferred(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<()> {
    let params: TransferredParam<TokenId, TokenAmount> = ctx.parameter_cursor().get()?;
    let state = host.state();
    let token_id = params.token_id;

    for module in state.modules.iter() {
        ensure!(ctx.sender().matches_contract(&token_id.contract), Error::Unauthorized);

        ComplianceContract(module.to_owned()).transferred(
            host,
            token_id.clone(),
            params.from,
            params.to,
            params.amount,
        )?;
    }

    Ok(())
}
