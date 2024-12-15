use concordium_cis2::{Cis2Event, MintEvent};
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
    contract = "security_sft_single",
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
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::Mint]));
    ensure!(is_authorized, Error::Unauthorized);

    let compliance = state.compliance;
    let identity_registry = state.identity_registry;

    for MintParam {
        address: owner,
        amount,
    } in params.owners
    {
        let owner = Address::Account(owner);
        ensure!(amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        ensure!(TRACKED_TOKEN_ID.eq(&params.token_id), Error::InvalidTokenId);

        if let Some(identity_registry) = identity_registry {
            ensure!(
                identity_registry_client::is_verified(host, &identity_registry, &owner)?,
                Error::UnVerifiedIdentity
            );
        }
        let compliance_token = TokenUId::new(params.token_id, self_address);
        if let Some(compliance) = compliance {
            let compliance_can_transfer =
                compliance_client::can_transfer(host, &compliance, &CanTransferParam {
                    token_id: compliance_token,
                    amount,
                    to: owner,
                })?;
            ensure!(compliance_can_transfer, Error::InCompliantTransfer);
        }

        let (state, state_builder) = host.state_and_builder();
        // Mint tokens
        state
            .addresses
            .entry(owner)
            .or_insert_with(|| HolderState::new_active(state_builder))
            .try_modify(|holder| holder.add_assign_unfrozen(amount))?;
        // Update minted supply
        state.token.add_assign_supply(amount)?;

        if let Some(compliance) = compliance {
            compliance_client::minted(host, &compliance, &MintedParam {
                token_id: TokenUId::new(params.token_id, self_address),
                amount,
                owner,
            })?;
        }

        logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
            token_id: params.token_id,
            amount,
            owner,
        })))?;
    }

    Ok(())
}
