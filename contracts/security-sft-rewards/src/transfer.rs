use concordium_cis2::{
    Cis2Event, OnReceivingCis2Params, Receiver, Transfer, TransferEvent, TransferParams,
};
use concordium_protocols::{
    concordium_cis2_security::{
        compliance_client, identity_registry_client, CanTransferParam, Token, TokenFrozen,
        TransferredParam,
    },
    concordium_global_sponsor::{SponsoredParams, SponsoredParamsRaw},
};
use concordium_rwa_utils::state_implementations::{
    agent_with_roles_state::IAgentWithRolesState, holders_security_state::IHoldersSecurityState,
    holders_state::IHoldersState, sponsors_state::ISponsorsState,
    tokens_security_state::ITokensSecurityState, tokens_state::ITokensState,
};
use concordium_std::*;

use super::{error::*, state::State, types::*};

/// Compliant Transfers ownership of an NFT from one verified account to another
/// verified account. This function can be called by the owner of the token or
/// an operator of the owner or the trusted sponsor of the transaction.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating the success or failure of the
/// operation.
///
/// # Errors
///
/// This method will return an error if:
/// * `concordium_std::ParseError` - The parameter cursor cannot parse the
///   `TransferParams`.
/// * `Error::Unauthorized` - The sender is not authorized to perform the
///   transfer.
/// * `Error::Custom(CustomContractError::PausedToken)` - The token is paused.
/// * `Error::InsufficientFunds` - The sender does not have enough tokens.
/// * `Error::Custom(CustomContractError::UnVerifiedIdentity)` - The receiver's
///   identity is not verified.
/// * `Error::Custom(CustomContractError::InCompliantTransfer)` - The transfer
///   is not compliant.
/// * `Error::Custom(CustomContractError::LogError)` - The logger failed to log
///   the event.
#[receive(
    contract = "security_sft_rewards",
    name = "transfer",
    enable_logger,
    mutable,
    parameter = "ContractTransferParams",
    error = "super::error::Error"
)]
pub fn transfer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state();
    let (sender, params) = {
        let sender = ctx.sender();
        if state.is_sponsor(&sender) {
            let params: SponsoredParamsRaw = ctx.parameter_cursor().get()?;
            let params: SponsoredParams<ContractTransferParams> = params.try_into()?;
            (Address::Account(params.signer), params.params)
        } else {
            let params: ContractTransferParams = ctx.parameter_cursor().get()?;
            (sender, params)
        }
    };

    let TransferParams(transfers): ContractTransferParams = params;
    let compliance = state.compliance();

    for Transfer {
        to,
        from,
        amount,
        token_id,
        data,
    } in transfers
    {
        let compliance_token = Token::new(token_id, ctx.self_address());
        let state = host.state();
        state.ensure_token_exists(&token_id)?;
        state.ensure_not_recovered(&to.address())?;
        state.ensure_not_paused(&token_id)?;
        state.ensure_has_sufficient_unfrozen_balance(&from, &token_id, &amount)?;
        ensure!(
            identity_registry_client::is_verified(host, state.identity_registry(), &to.address())?,
            Error::UnVerifiedIdentity
        );
        let compliance_can_transfer =
            compliance_client::can_transfer(host, state.compliance(), &CanTransferParam {
                token_id: compliance_token,
                to: to.address(),
                amount,
            })?;
        ensure!(compliance_can_transfer, Error::InCompliantTransfer);

        ensure!(from.eq(&sender) || state.is_operator(&from, &sender), Error::Unauthorized);

        let (state, state_builder) = host.state_and_builder();
        state.transfer(from, to.address(), &token_id, amount, state_builder)?;
        compliance_client::transferred(host, compliance, &TransferredParam {
            token_id: compliance_token,
            from,
            to: to.address(),
            amount,
        })?;

        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id,
            from,
            to: to.address(),
        })))?;

        // host.commit_state();
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
/// * `concordium_std::ParseError` - The parameter cursor cannot parse the
///   `TransferParams`.
/// * `Error::Unauthorized` - The sender is not authorized to perform the
///   transfer. Sender is not an agent.
/// * `Error::Custom(CustomContractError::PausedToken)` - The token is paused.
/// * `Error::InsufficientFunds` - The sender does not have enough tokens.
/// * `Error::Custom(CustomContractError::UnVerifiedIdentity)` - The receiver's
///   identity is not verified.
/// * `Error::Custom(CustomContractError::LogError)` - The logger failed to log
///   the event.
#[receive(
    contract = "security_sft_rewards",
    name = "forcedTransfer",
    enable_logger,
    mutable,
    parameter = "TransferParams<TokenId, TokenAmount>",
    error = "super::error::Error"
)]
pub fn forced_transfer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let TransferParams(transfers): TransferParams<TokenId, TokenAmount> =
        ctx.parameter_cursor().get()?;

    let state = host.state();
    ensure!(state.is_agent(&ctx.sender(), vec![&AgentRole::ForcedTransfer]), Error::Unauthorized);

    for Transfer {
        to,
        from,
        amount,
        token_id,
        data,
    } in transfers
    {
        let state = host.state();
        state.ensure_token_exists(&token_id)?;
        state.ensure_not_recovered(&to.address())?;
        state.ensure_not_paused(&token_id)?;
        // Only the balance is checked. The frozen balance is not checked.
        state.ensure_has_sufficient_balance(&from, &token_id, &amount)?;
        ensure!(
            identity_registry_client::is_verified(host, state.identity_registry(), &to.address())?,
            Error::UnVerifiedIdentity
        );

        let (state, state_builder) = host.state_and_builder();
        state.transfer(from, to.address(), &token_id, amount, state_builder)?;
        // Adjust the frozen balance of the sender.
        let unfrozen_balance = state.adjust_frozen_balance(from, token_id)?;
        compliance_client::transferred(host, host.state().compliance(), &TransferredParam {
            token_id: Token::new(token_id, ctx.self_address()),
            from,
            to: to.address(),
            amount,
        })?;

        logger.log(&Event::TokenUnFrozen(TokenFrozen {
            token_id,
            amount: unfrozen_balance,
            address: from,
        }))?;
        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id,
            from,
            to: to.address(),
        })))?;

        if let Address::Contract(_from_contract) = from {
            //TODO: there should be a way to notify that the transfer has been
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
