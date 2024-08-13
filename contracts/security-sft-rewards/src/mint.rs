use concordium_cis2::{AdditionalData, Cis2Event, MintEvent, OnReceivingCis2Params, Receiver};
use concordium_protocols::{
    concordium_cis2_ext::IsTokenAmount,
    concordium_cis2_security::{
        compliance_client, identity_registry_client, CanTransferParam, MintedParam, Token,
    },
};
use concordium_rwa_utils::state_implementations::{
    agent_with_roles_state::IAgentWithRolesState, holders_security_state::IHoldersSecurityState,
    holders_state::IHoldersState,
};
use concordium_std::*;

use super::{error::*, state::State, types::*};

/// Mint the given amount of tokens to the owner. Locking the deposited tokens.
#[receive(
    contract = "security_sft_rewards",
    name = "mint",
    enable_logger,
    mutable,
    parameter = "MintParam",
    error = "super::error::Error"
)]
pub fn mint(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let self_address = ctx.self_address();
    let params: MintParam = ctx.parameter_cursor().get()?;
    let can_mint = host.state().is_agent(&ctx.sender(), vec![&AgentRole::Mint]);
    ensure!(can_mint, Error::Unauthorized);

    mint_internal(self_address, params, host, logger)
}

pub fn mint_internal(
    curr_contract_address: ContractAddress,
    params: MintParam,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state();
    let owner_address = params.owner.address();

    // Ensure that the owner is not recovered
    state.ensure_not_recovered(&owner_address)?;
    // Ensure that the owner is verified
    ensure!(
        identity_registry_client::is_verified(host, state.identity_registry(), &owner_address)?,
        Error::UnVerifiedIdentity
    );

    let compliance = state.compliance();
    let token_id = params.token_id;
    let mint_amount = params.amount;
    ensure!(mint_amount.gt(&TokenAmount::zero()), Error::InvalidAmount);

    let compliance_token = Token::new(token_id, curr_contract_address);

    let compliance_can_transfer =
        compliance_client::can_transfer(host, compliance, &CanTransferParam {
            token_id: compliance_token,
            amount:   mint_amount,
            to:       owner_address,
        })?;
    ensure!(compliance_can_transfer, Error::InCompliantTransfer);

    // Minting Logic
    let (state, state_builder) = host.state_and_builder();
    state.add_balance(owner_address, &token_id, mint_amount, state_builder)?;
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
    // If the receiver is a contract: invoke the receive hook function.
    if let Receiver::Contract(address, function) = &params.owner {
        let parameter = OnReceivingCis2Params {
            token_id,
            amount: mint_amount,
            // From self because the minting is being done from deposited tokens in custody of the
            // current contract
            from: curr_contract_address.into(),
            data: AdditionalData::empty(),
        };
        host.invoke_contract(address, &parameter, function.as_entrypoint_name(), Amount::zero())?;
    }

    Ok(())
}
