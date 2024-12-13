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
        let balance: TokenAmount = match state.address(&query.address) {
            None => TokenAmount::zero(),
            Some(holder) => match holder.active() {
                None => TokenAmount::zero(),
                Some(holder_state) => {
                    if TRACKED_TOKEN_ID.eq(&query.token_id) {
                        holder_state.balance.total()
                    } else {
                        ensure!(
                            state.reward_tokens.get(&query.token_id).is_some(),
                            Error::InvalidRewardTokenId
                        );
                        holder_state
                            .reward_balances
                            .get(&query.token_id)
                            .map(|t| t.total())
                            .unwrap_or(TokenAmount::zero())
                    }
                }
            },
        };
        res.push(balance);
    }
    Ok(concordium_cis2::BalanceOfQueryResponse(res))
}
