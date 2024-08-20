use concordium_cis2::{TokenMetadataQueryParams, TokenMetadataQueryResponse};
use concordium_rwa_utils::state_implementations::cis2_state::ICis2SingleState;
use concordium_std::*;

use super::state::State;
use super::types::{ContractResult, TokenId};

/// Retrieves the metadata for a token.
///
/// # Returns
///
/// Returns `ContractResult<TokenMetadataQueryResponse>` containing the metadata
/// for each queried token.
///
/// # Errors
///
/// This method will return a `ParseError` if it fails to parse the input
/// this method will return an `InvalidTokenId` if the token does not exist.
/// parameters.
#[receive(
    contract = "security_sft_rewards",
    name = "tokenMetadata",
    parameter = "TokenMetadataQueryParams<TokenId>",
    return_value = "TokenMetadataQueryResponse",
    error = "super::error::Error"
)]
pub fn token_metadata(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<TokenMetadataQueryResponse> {
    let TokenMetadataQueryParams { queries }: TokenMetadataQueryParams<TokenId> =
        ctx.parameter_cursor().get()?;
    let state = host.state();
    let res: Vec<_> = queries.iter().map(|_| state.metadata_url()).collect();

    Ok(TokenMetadataQueryResponse(res))
}
