use concordium_cis2::{Cis2Event, MintEvent};
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::{
    compliance_client, identity_registry_client, CanTransferParam, MintedParam, TokenUId,
};
use concordium_rwa_utils::state_implementations::agent_with_roles_state::IAgentWithRolesState;
use concordium_rwa_utils::state_implementations::cis2_security_state::ICis2SingleSecurityState;
use concordium_std::*;

use super::error::*;
use super::state::State;
use super::types::*;

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
    ensure!(
        state.is_agent(&ctx.sender(), vec![AgentRole::Mint]),
        Error::Unauthorized
    );

    let compliance = state.compliance;
    let identity_registry_client = state.identity_registry;
    let token_id = params.token_id;

    for owner in params.owners {
        let owner_address = Address::Account(owner.address);
        let mint_amount = owner.amount;

        ensure!(mint_amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        ensure!(
            identity_registry_client::is_verified(host, identity_registry_client, &owner_address)?,
            Error::UnVerifiedIdentity
        );
        let compliance_token = TokenUId::new(token_id, self_address);
        let compliance_can_transfer =
            compliance_client::can_transfer(host, compliance, &CanTransferParam {
                token_id: compliance_token,
                amount:   mint_amount,
                to:       owner_address,
            })?;
        ensure!(compliance_can_transfer, Error::InCompliantTransfer);

        // Minting Logic
        let (state, state_builder) = host.state_and_builder();
        state.mint(mint_amount, &owner_address, state_builder)?;
        // Notify compliance that the token has been minted
        compliance_client::minted(host, compliance, &MintedParam {
            token_id: compliance_token,
            amount:   mint_amount,
            owner:    owner_address,
        })?;
        // Log the mint event
        logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
            token_id,
            amount: mint_amount,
            owner: owner_address,
        })))?;
    }

    Ok(())
}
