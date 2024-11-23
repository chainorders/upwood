use concordium_cis2::{BurnEvent, Cis2Event, MintEvent};
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::{
    compliance_client, BurnedParam, TokenFrozen, TokenUId,
};
use concordium_std::*;

use super::error::Error;
use super::state::State;
use super::types::*;

/// Burns the specified amount of the given token from the given owner's
/// account.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// - Returns `Error::Unauthorized` if the sender is not authorized to burn the tokens.
/// - Returns `Error::Custom(CustomContractError::PausedToken)` if the token is paused.
/// - Returns `Error::InsufficientFunds` if the owner does not have enough tokens.
#[receive(
    contract = "security_sft_rewards",
    name = "burn",
    parameter = "BurnParams",
    error = "Error",
    enable_logger,
    mutable
)]
pub fn burn(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let sender = ctx.sender();
    let params: BurnParams = ctx.parameter_cursor().get()?;
    let state = host.state();
    let compliance: ContractAddress = state.compliance;
    let (_, max_reward_token_id) = state.rewards_ids_range;

    for Burn {
        token_id,
        amount,
        owner,
    } in params.0
    {
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        let state = host.state_mut();
        let reward_carry = {
            let mut holder = state.address_mut(&owner).ok_or(Error::InvalidAddress)?;
            let holder = holder.active_mut().ok_or(Error::RecoveredAddress)?;
            let is_authorized = owner.eq(&sender) || holder.has_operator(&sender);
            ensure!(is_authorized, Error::Unauthorized);

            holder.sub_assign_unfrozen_balance(&token_id, amount)?;
            holder.sub_assign_unfrozen_balance_signed(&max_reward_token_id, amount)
        };

        if reward_carry.gt(&TokenAmount::zero()) {
            logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
                amount: reward_carry,
                token_id: max_reward_token_id,
                owner,
            })))?;
        }

        state.sub_assign_supply(&token_id, amount)?;
        state.sub_assign_supply_signed(&max_reward_token_id, amount)?;

        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id,
            owner,
        })))?;
        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id: max_reward_token_id,
            owner,
        })))?;

        compliance_client::burned(host, &compliance, &BurnedParam {
            token_id: TokenUId::new(token_id, ctx.self_address()),
            amount,
            owner,
        })?;
    }

    Ok(())
}

#[receive(
    contract = "security_sft_rewards",
    name = "forcedBurn",
    parameter = "BurnParams",
    error = "Error",
    enable_logger,
    mutable
)]
/// Forcibly burns tokens from the specified owners.
///
/// This function allows an authorized agent to burn tokens from any owner, regardless of the owner's approval status.
///
/// # Parameters
/// - `BurnParams`: A struct containing the parameters for the forced burn operation.
///
/// # Errors
/// - `Error::Unauthorized`: If the sender is not authorized to perform forced burns.
/// - Other errors related to token existence, pausing, and balance checking.
///
/// # Events
/// - `Cis2Event::Burn`: Emitted for each token that is burned.
/// - `TokenUnFrozen`: Emitted for each token that is unfrozen as a result of the burn.
pub fn forced_burn(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let zero_amount = TokenAmount::zero();
    let params: BurnParams = ctx.parameter_cursor().get()?;
    let state = host.state();
    let is_authorized = state
        .address(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::ForcedBurn]));
    ensure!(is_authorized, Error::Unauthorized);

    let compliance: ContractAddress = state.compliance;
    let (_, max_reward_token_id) = state.rewards_ids_range;

    for Burn {
        token_id,
        amount,
        owner,
    } in params.0
    {
        ensure!(amount.gt(&zero_amount), Error::InvalidAmount);
        let state = host.state_mut();
        let (un_frozen_amount, reward_carry) = {
            let mut holder = state.address_mut(&owner).ok_or(Error::InvalidAddress)?;
            let holder = holder.active_mut().ok_or(Error::RecoveredAddress)?;
            let un_frozen_amount = holder.un_freeze_balance_to_match(&token_id, amount)?;
            holder.sub_assign_unfrozen_balance(&token_id, amount)?;
            let reward_carry =
                holder.sub_assign_unfrozen_balance_signed(&max_reward_token_id, amount);
            (un_frozen_amount, reward_carry)
        };
        state.sub_assign_supply(&token_id, amount)?;
        state.sub_assign_supply_signed(&max_reward_token_id, amount)?;

        if un_frozen_amount.gt(&zero_amount) {
            logger.log(&Event::TokenUnFrozen(TokenFrozen {
                token_id,
                amount: un_frozen_amount,
                address: owner,
            }))?;
        }

        if reward_carry.gt(&zero_amount) {
            logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
                amount: reward_carry,
                token_id: max_reward_token_id,
                owner,
            })))?;
        }

        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id,
            owner,
        })))?;
        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id,
            owner,
        })))?;

        compliance_client::burned(host, &compliance, &BurnedParam {
            token_id: TokenUId::new(token_id, ctx.self_address()),
            amount,
            owner,
        })?;
    }

    Ok(())
}
