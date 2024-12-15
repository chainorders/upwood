use concordium_cis2::{BurnEvent, Cis2Event, MintEvent};
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::{
    compliance_client, identity_registry_client, CanTransferParam, MintedParam, TokenUId,
};
use concordium_std::*;

use super::error::*;
use super::state::State;
use super::types::*;
use crate::state::HolderState;

#[receive(
    contract = "security_sft_rewards",
    name = "mint",
    enable_logger,
    mutable,
    parameter = "MintParams",
    error = "Error"
)]
pub fn mint(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let self_address = ctx.self_address();
    let params: MintParams = ctx.parameter_cursor().get()?;

    let state = host.state();
    let is_authorized = state
        .address(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::Mint]));
    ensure!(is_authorized, Error::Unauthorized);

    let compliance = state.compliance;
    let identity_registry_client = state.identity_registry;
    let (_, max_reward_token_id) = state.rewards_ids_range;

    for MintParam {
        address: owner,
        amount,
    } in params.owners
    {
        let owner = Address::Account(owner);
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        ensure!(TRACKED_TOKEN_ID.eq(&params.token_id), Error::InvalidTokenId);
        ensure!(
            identity_registry_client::is_verified(host, &identity_registry_client, &owner)?,
            Error::UnVerifiedIdentity
        );
        let compliance_token = TokenUId::new(params.token_id, self_address);
        let compliance_can_transfer =
            compliance_client::can_transfer(host, &compliance, &CanTransferParam {
                token_id: compliance_token,
                amount,
                to: owner,
            })?;
        ensure!(compliance_can_transfer, Error::InCompliantTransfer);

        let (state, state_builder) = host.state_and_builder();
        let to_burn = {
            // Mint tokens
            let mut holder = state
                .addresses
                .entry(owner)
                .or_insert(HolderState::new_active(state_builder));
            holder.add_assign_unfrozen(amount)?;
            let to_burn = holder
                .reward_balances_mut()?
                .entry(max_reward_token_id)
                .or_default()
                .add_assign_unfrozen(amount);
            to_burn
        };
        logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
            token_id: params.token_id,
            amount,
            owner,
        })))?;
        logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
            token_id: max_reward_token_id,
            amount,
            owner,
        })))?;
        if to_burn.gt(&TokenAmount::zero()) {
            logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
                amount: to_burn,
                token_id: max_reward_token_id,
                owner,
            })))?;
        }

        {
            // Update minted supply
            state.token.add_assign_supply(amount)?;
            state
                .reward_tokens
                .entry(max_reward_token_id)
                .occupied_or(Error::InvalidRewardTokenId)?
                .add_assign_supply(amount);
        }

        compliance_client::minted(host, &compliance, &MintedParam {
            token_id: TokenUId::new(params.token_id, self_address),
            amount,
            owner,
        })?;
    }

    Ok(())
}
