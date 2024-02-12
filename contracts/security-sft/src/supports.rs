use concordium_cis2::{
    StandardIdentifier, SupportResult, SupportsQueryParams, SupportsQueryResponse,
    CIS0_STANDARD_IDENTIFIER, CIS2_STANDARD_IDENTIFIER,
};
use concordium_std::*;

use concordium_rwa_utils::{
    concordium_cis3::CIS3_STANDARD_IDENTIFIER, sponsors_state::ISponsorsState,
};

use super::{state::State, types::ContractResult};

const SUPPORTS_STANDARDS: [StandardIdentifier<'static>; 2] =
    [CIS0_STANDARD_IDENTIFIER, CIS2_STANDARD_IDENTIFIER];

/// Determines whether the contract supports a specific concordium standard.
///
/// # Returns
///
/// Returns `ContractResult<SupportsQueryResponse>` containing the support
/// status for each queried standard.
///
/// # Errors
///
/// This method will return an error if:
/// * `ParseError` - The parameter cursor cannot parse the
///   `SupportsQueryParams`.
#[receive(
    contract = "rwa_security_sft",
    name = "supports",
    parameter = "SupportsQueryParams",
    return_value = "SupportsQueryResponse",
    error = "super::error::Error"
)]
fn supports(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<SupportsQueryResponse> {
    let params: SupportsQueryParams = ctx.parameter_cursor().get()?;
    let mut response = Vec::with_capacity(params.queries.len());
    for std_id in params.queries {
        if SUPPORTS_STANDARDS.contains(&std_id.as_standard_identifier()) {
            response.push(SupportResult::Support);
        } else if std_id.as_standard_identifier().eq(&CIS3_STANDARD_IDENTIFIER) {
            response.push(SupportResult::SupportBy(host.state().list_sponsors()));
        } else {
            response.push(SupportResult::NoSupport)
        }
    }

    Ok(SupportsQueryResponse::from(response))
}
