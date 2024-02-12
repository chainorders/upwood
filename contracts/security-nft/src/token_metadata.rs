use concordium_cis2::{TokenMetadataQueryParams, TokenMetadataQueryResponse};
use concordium_std::*;

use concordium_rwa_utils::tokens_state::ITokensState;

use super::{
    state::State,
    types::{ContractResult, TokenId},
};

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
    contract = "rwa_security_nft",
    name = "tokenMetadata",
    parameter = "TokenMetadataQueryParams<TokenId>",
    return_value = "TokenMetadataQueryResponse",
    error = "super::error::Error"
)]
pub fn token_metadata(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<TokenMetadataQueryResponse> {
    let TokenMetadataQueryParams {
        queries,
    }: TokenMetadataQueryParams<TokenId> = ctx.parameter_cursor().get()?;

    let state = host.state();
    let res: Result<Vec<_>, _> = queries.iter().map(|q| state.token(q)).collect();

    Ok(TokenMetadataQueryResponse(res?))
}
