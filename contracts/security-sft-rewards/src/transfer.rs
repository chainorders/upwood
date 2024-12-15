use concordium_cis2::{
    BurnEvent, Cis2Event, MintEvent, OnReceivingCis2Params, Receiver, TransferEvent,
};
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::{
    compliance_client, identity_registry_client, CanTransferParam, TokenFrozen, TokenUId,
    TransferredParam,
};
use concordium_protocols::concordium_global_sponsor::{SponsoredParams, SponsoredParamsRaw};
use concordium_std::*;

use super::error::*;
use super::state::State;
use super::types::*;
use crate::state::HolderState;

/// Executes a compliant transfer of token ownership between verified accounts
///
/// This function facilitates the transfer of token ownership from one verified
/// account to another. It can be invoked by the token owner, an authorized
/// operator, or a trusted transaction sponsor.
///
/// # Arguments
///
/// * Implicitly uses `TransferParams` parsed from the parameter cursor
///
/// # Returns
///
/// * `ContractResult<()>` - Indicates the success or failure of the transfer operation
///
/// # Errors
///
/// May return the following errors:
/// * `concordium_std::ParseError` - Failed to parse `TransferParams` from the parameter cursor
/// * `Error::Unauthorized` - The sender lacks the necessary authorization for the transfer
/// * `Error::Custom(CustomContractError::PausedToken)` - The token is currently paused
/// * `Error::InsufficientFunds` - The sender's token balance is insufficient for the transfer
/// * `Error::Custom(CustomContractError::UnVerifiedIdentity)` - The receiver's identity is not
///   verified
/// * `Error::Custom(CustomContractError::InCompliantTransfer)` - The transfer violates compliance
///   rules
/// * `Error::Custom(CustomContractError::LogError)` - Failed to log the transfer event
#[receive(
    contract = "security_sft_rewards",
    name = "transfer",
    enable_logger,
    mutable,
    parameter = "TransferParams",
    error = "Error"
)]
pub fn transfer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let zero_amount = TokenAmount::zero();
    let state = host.state();
    let (sender, params) = {
        let sender = ctx.sender();
        if state.sponsor.is_some_and(|s| sender.matches_contract(&s)) {
            let params: SponsoredParamsRaw = ctx.parameter_cursor().get()?;
            let params: SponsoredParams<TransferParams> = params.try_into()?;
            (Address::Account(params.signer), params.params)
        } else {
            let params: TransferParams = ctx.parameter_cursor().get()?;
            (sender, params)
        }
    };

    let compliance = state.compliance;
    let identity_registry = state.identity_registry;
    let self_address = ctx.self_address();
    let concordium_cis2::TransferParams(transfers) = params;
    let (_, max_reward_token_id) = state.rewards_ids_range;

    for concordium_cis2::Transfer {
        to,
        from,
        amount,
        token_id,
        data,
    } in transfers
    {
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        ensure!(TRACKED_TOKEN_ID.eq(&token_id), Error::InvalidTokenId);
        ensure!(
            identity_registry_client::is_verified(host, &identity_registry, &to.address())?,
            Error::UnVerifiedIdentity
        );
        let compliance_token = TokenUId::new(token_id, self_address);
        let compliance_can_transfer =
            compliance_client::can_transfer(host, &compliance, &CanTransferParam {
                token_id: compliance_token,
                to: to.address(),
                amount,
            })?;
        ensure!(compliance_can_transfer, Error::InCompliantTransfer);

        // Transfer token
        let (state, state_builder) = host.state_and_builder();
        ensure!(!state.token.paused, Error::PausedToken);

        {
            let mut from_holder = state
                .addresses
                .get_mut(&from)
                .ok_or(Error::InvalidAddress)?;
            ensure!(
                from.eq(&sender) || from_holder.has_operator(&sender),
                Error::Unauthorized
            );
            from_holder.sub_assign_unfrozen(amount)?;
            let reward_carry = from_holder
                .reward_balances_mut()?
                .entry(max_reward_token_id)
                .or_default()
                .sub_assign_unfrozen(amount, true)?;
            if reward_carry.gt(&zero_amount) {
                // The sender does not have enough rewards to cover the transfer of rewards
                // We mint the missing rewards
                // We burn it later upon the next transfer / mint / reward claim
                logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
                    amount:   reward_carry,
                    token_id: max_reward_token_id,
                    owner:    from,
                })))?;
            }
        };

        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id,
            from,
            to: to.address(),
        })))?;
        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id: max_reward_token_id,
            from,
            to: to.address(),
        })))?;

        {
            let mut to_holder = state
                .addresses
                .entry(to.address())
                .or_insert_with(|| HolderState::new_active(state_builder));
            to_holder.add_assign_unfrozen(amount)?;
            let to_burn = to_holder
                .reward_balances_mut()?
                .entry(max_reward_token_id)
                .or_insert_with(Default::default)
                .add_assign_unfrozen(amount);
            if to_burn.gt(&zero_amount) {
                logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
                    amount:   to_burn,
                    token_id: max_reward_token_id,
                    owner:    to.address(),
                })))?;
            }
        }

        compliance_client::transferred(host, &compliance, &TransferredParam {
            token_id: compliance_token,
            from,
            to: to.address(),
            amount,
        })?;

        if let Receiver::Contract(to_contract, entrypoint) = to {
            let parameter = OnReceivingCis2Params {
                token_id,
                amount,
                from,
                data,
            };

            host.invoke_contract(
                &to_contract,
                &parameter,
                entrypoint.as_entrypoint_name(),
                Amount::zero(),
            )?;
        }
    }

    Ok(())
}

/// Forces the transfer of a specific amount of tokens from one verified account
/// to another verified. This function can be called by a trusted agent.
/// This function can be used to transfer tokens that are not compliant.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating the success or failure of the
/// operation.
///
/// # Errors
///
/// This method will return an error if:
/// * `concordium_std::ParseError` - The parameter cursor cannot parse the `TransferParams`.
/// * `Error::Unauthorized` - The sender is not authorized to perform the transfer. Sender is not an agent.
/// * `Error::Custom(CustomContractError::PausedToken)` - The token is paused.
/// * `Error::InsufficientFunds` - The sender does not have enough tokens.
/// * `Error::Custom(CustomContractError::UnVerifiedIdentity)` - The receiver's identity is not verified.
/// * `Error::Custom(CustomContractError::LogError)` - The logger failed to log the event.
#[receive(
    contract = "security_sft_rewards",
    name = "forcedTransfer",
    enable_logger,
    mutable,
    parameter = "TransferParams",
    error = "Error"
)]
pub fn forced_transfer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state();
    let is_authorized = state
        .address(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::ForcedTransfer]));
    ensure!(is_authorized, Error::Unauthorized);

    let compliance = state.compliance;
    let identity_registry = state.identity_registry;
    let self_address = ctx.self_address();
    let (_, max_reward_token_id) = state.rewards_ids_range;

    let params: TransferParams = ctx.parameter_cursor().get()?;

    for concordium_cis2::Transfer {
        to,
        from,
        amount,
        token_id,
        data,
    } in params.0
    {
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        ensure!(TRACKED_TOKEN_ID.eq(&token_id), Error::InvalidTokenId);
        ensure!(
            identity_registry_client::is_verified(host, &identity_registry, &to.address())?,
            Error::UnVerifiedIdentity
        );
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);

        let compliance_token = TokenUId::new(token_id, self_address);

        // Transfer token
        let (state, state_builder) = host.state_and_builder();
        ensure!(!state.token.paused, Error::PausedToken);

        let (un_frozen_amount, rewards_carry) = {
            let mut from_holder = state
                .addresses
                .get_mut(&from)
                .ok_or(Error::InvalidAddress)?;
            let un_frozen_amount = from_holder.un_freeze_balance_to_match(amount)?;
            from_holder.sub_assign_unfrozen(amount)?;
            let rewards_carry = from_holder
                .reward_balances_mut()?
                .entry(max_reward_token_id)
                .or_default()
                .sub_assign_unfrozen(amount, true)?;
            (un_frozen_amount, rewards_carry)
        };

        if un_frozen_amount.gt(&TokenAmount::zero()) {
            logger.log(&Event::TokenUnFrozen(TokenFrozen {
                token_id,
                amount: un_frozen_amount,
                address: from,
            }))?;
        }
        if rewards_carry.gt(&TokenAmount::zero()) {
            // The sender does not have enough rewards to cover the transfer of rewards
            // We mint the missing rewards
            // We burn it later upon the next transfer / mint / reward claim
            logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
                amount:   rewards_carry,
                token_id: max_reward_token_id,
                owner:    from,
            })))?;
        }

        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id,
            from,
            to: to.address(),
        })))?;
        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id: max_reward_token_id,
            from,
            to: to.address(),
        })))?;

        let to_burn = {
            let mut to_holder = state
                .addresses
                .entry(to.address())
                .or_insert_with(|| HolderState::new_active(state_builder));
            to_holder.add_assign_unfrozen(amount)?;
            let to_burn = to_holder
                .reward_balances_mut()?
                .entry(max_reward_token_id)
                .or_insert_with(Default::default)
                .add_assign_unfrozen(amount);
            to_burn
        };

        if to_burn.gt(&TokenAmount::zero()) {
            logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
                amount:   to_burn,
                token_id: max_reward_token_id,
                owner:    to.address(),
            })))?;
        }
        compliance_client::transferred(host, &compliance, &TransferredParam {
            token_id: compliance_token,
            from,
            to: to.address(),
            amount,
        })?;

        if let Address::Contract(_from_contract) = from {
            // TODO: there should be a way to notify that the transfer has been
            // forced Ex. A token is sent to the marketplace for
            // selling. Upon a forced transfer since marketplace
            // would not know that the it does not have the token authority
            // would continue to sell the token. Without anyone being able to
            // buy it.
        }

        if let Receiver::Contract(to_contract, entrypoint) = to {
            let parameter = OnReceivingCis2Params {
                token_id,
                amount,
                from,
                data,
            };

            host.invoke_contract(
                &to_contract,
                &parameter,
                entrypoint.as_entrypoint_name(),
                Amount::zero(),
            )?;
        }
    }

    Ok(())
}
