use concordium_std::*;

use super::{state::State, types::ContractResult};

#[derive(Serialize, SchemaType)]
pub struct SupportsPermitQueryParams {
    /// The list of supportPermit queries.
    #[concordium(size_length = 2)]
    pub queries: Vec<OwnedEntrypointName>,
}

#[derive(Debug, Serialize, SchemaType)]
#[concordium(transparent)]
pub struct SupportsQueryResponse {
    /// List of support results corresponding to the list of queries.
    #[concordium(size_length = 2)]
    pub results: Vec<bool>,
}

/// Returns true Or false for each query in the list of queries.
/// The result is true if the contract supports execution a function and false
/// otherwise.
#[receive(
    contract = "rwa_sponsor",
    name = "supportsPermit",
    parameter = "SupportsPermitQueryParams",
    return_value = "SupportsQueryResponse",
    error = "super::error::Error"
)]
fn contract_supports_permit(
    ctx: &ReceiveContext,
    _host: &Host<State>,
) -> ContractResult<SupportsQueryResponse> {
    let params: SupportsPermitQueryParams = ctx.parameter_cursor().get()?;
    // Since this contract has been implemented as a layer of sponsor contract
    // It supports all the functions
    // The actual authorization of which functions can be called is left to the
    // callee
    Ok(SupportsQueryResponse {
        results: params.queries.into_iter().map(|_| true).collect(),
    })
}
