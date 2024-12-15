use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_std::*;

use super::error::Error;
use super::state::State;
use super::types::*;

/// Queries the balance of the specified token IDs for the given addresses.
///
/// This function takes a list of `BalanceOfQueryParams` and
/// returns a `BalanceOfQueryResponse` containing the token balances for each query.
///
/// # Returns
/// A `ContractResult` containing the token balances for each query.
#[receive(
    contract = "security_sft_rewards",
    name = "balanceOf",
    parameter = "BalanceOfQueryParams",
    return_value = "BalanceOfQueryResponse",
    error = "super::error::Error"
)]
pub fn balance_of(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<BalanceOfQueryResponse> {
    let BalanceOfQueryParams { queries } = ctx.parameter_cursor().get()?;
    let mut res: Vec<TokenAmount> = Vec::with_capacity(queries.len());
    let state = host.state();
    for query in queries {
        match (
            TRACKED_TOKEN_ID.eq(&query.token_id),
            state.addresses.get(&query.address),
        ) {
            (true, Some(h)) => res.push(h.balance_total()),
            (true, None) => res.push(TokenAmount::zero()),
            (false, Some(h)) => {
                state
                    .reward_tokens
                    .get(&query.token_id)
                    .ok_or(Error::InvalidTokenId)?;
                res.push(
                    h.reward_balances()?
                        .get(&query.token_id)
                        .map_or(TokenAmount::zero(), |a| a.un_frozen.as_amount()),
                );
            }
            (false, None) => {
                state
                    .reward_tokens
                    .get(&query.token_id)
                    .ok_or(Error::InvalidTokenId)?;
                res.push(TokenAmount::zero());
            }
        }
    }
    Ok(concordium_cis2::BalanceOfQueryResponse(res))
}
