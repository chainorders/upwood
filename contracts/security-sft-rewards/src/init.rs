use concordium_cis2::{Cis2Event, TokenMetadataEvent};
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::{
    AgentUpdatedEvent, ComplianceAdded, IdentityRegistryAdded,
};
use concordium_std::*;

use super::error::Error;
use super::state::State;
use super::types::{AgentRole, ContractResult, Event, InitParam};
use crate::state::{
    AddressState, HolderState, MainTokenState, RewardTokenState, TokenAmountSigned, TokenState,
};
use crate::types::{TokenAmount, MIN_REWARD_TOKEN_ID, TRACKED_TOKEN_ID};
/// Initializes the contract with the given parameters.
///
/// # Returns
///
/// Returns `InitResult<State>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[init(
    contract = "security_sft_rewards",
    event = "Event",
    error = "Error",
    parameter = "InitParam",
    enable_logger
)]
pub fn init(
    ctx: &InitContext,
    state_builder: &mut StateBuilder,
    logger: &mut Logger,
) -> InitResult<State> {
    let params: InitParam = ctx.parameter_cursor().get()?;
    let owner = Address::Account(ctx.init_origin());
    let addresses = {
        let mut addresses = state_builder.new_map();
        let _ = addresses.insert(
            owner,
            AddressState::Holder(HolderState::new_with_roles(
                state_builder,
                &AgentRole::owner_roles(),
            )),
        );
        addresses
    };
    let tokens = {
        let mut tokens = state_builder.new_map();
        let _ = tokens.insert(
            TRACKED_TOKEN_ID,
            TokenState::Main(MainTokenState {
                metadata_url: params.metadata_url.clone().into(),
                supply:       TokenAmount::zero(),
                paused:       false,
            }),
        );
        let _ = tokens.insert(
            MIN_REWARD_TOKEN_ID,
            TokenState::Reward(RewardTokenState {
                metadata_url: params.blank_reward_metadata_url.clone().into(),
                supply:       TokenAmountSigned::zero(),
                reward:       None,
            }),
        );
        tokens
    };
    let state = State {
        identity_registry: params.identity_registry,
        compliance: params.compliance,
        sponsor: params.sponsors,
        addresses,
        tokens,
        rewards_ids_range: (MIN_REWARD_TOKEN_ID, MIN_REWARD_TOKEN_ID),
    };

    logger.log(&Event::IdentityRegistryAdded(IdentityRegistryAdded(
        state.identity_registry,
    )))?;
    logger.log(&Event::ComplianceAdded(ComplianceAdded(state.compliance)))?;
    logger.log(&Event::AgentAdded(AgentUpdatedEvent {
        agent: owner,
        roles: AgentRole::owner_roles(),
    }))?;
    logger.log(&Event::Cis2(Cis2Event::TokenMetadata(TokenMetadataEvent {
        metadata_url: params.metadata_url.into(),
        token_id:     TRACKED_TOKEN_ID,
    })))?;
    logger.log(&Event::Cis2(Cis2Event::TokenMetadata(TokenMetadataEvent {
        metadata_url: params.blank_reward_metadata_url.into(),
        token_id:     MIN_REWARD_TOKEN_ID,
    })))?;
    Ok(state)
}

/// Returns the address of the identity registry contract.
///
/// # Returns
///
/// Returns `ContractResult<ContractAddress>` containing the address of the identity registry contract.
#[receive(
    contract = "security_sft_rewards",
    name = "identityRegistry",
    return_value = "ContractAddress"
)]
pub fn identity_registry(
    _: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<ContractAddress> {
    Ok(host.state().identity_registry)
}

/// Sets the address of the identity registry contract.
///
/// # Parameters
///
/// - `ContractAddress`: The address of the identity registry contract.
///
/// # Errors
///
/// Returns an `Error::Unauthorized` error if the caller is not authorized to set the identity registry.
#[receive(
    contract = "security_sft_rewards",
    name = "setIdentityRegistry",
    mutable,
    enable_logger,
    parameter = "ContractAddress",
    error = "Error"
)]
pub fn set_identity_registry(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let is_authorized = host
        .state()
        .address(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::SetIdentityRegistry]));
    ensure!(is_authorized, Error::Unauthorized);

    let identity_registry: ContractAddress = ctx.parameter_cursor().get()?;
    host.state_mut().identity_registry = identity_registry;
    logger.log(&Event::IdentityRegistryAdded(IdentityRegistryAdded(
        identity_registry,
    )))?;

    Ok(())
}

/// Returns the address of the compliance contract.
///
/// # Returns
///
/// Returns `ContractResult<ContractAddress>` containing the address of the compliance contract.
#[receive(
    contract = "security_sft_rewards",
    name = "compliance",
    return_value = "ContractAddress"
)]
pub fn compliance(_: &ReceiveContext, host: &Host<State>) -> ContractResult<ContractAddress> {
    Ok(host.state().compliance)
}

/// Sets the compliance contract address.
///
/// This function allows authorized agents to set the compliance contract address for the security SFT rewards contract.
///
/// # Parameters
///
/// - `ContractAddress`: The address of the compliance contract.
///
/// # Errors
///
/// Returns an `Error::Unauthorized` error if the caller is not authorized to set the compliance contract.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
#[receive(
    contract = "security_sft_rewards",
    name = "setCompliance",
    mutable,
    enable_logger,
    parameter = "ContractAddress",
    error = "Error"
)]
pub fn set_compliance(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let is_authorized = host
        .state()
        .address(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::SetCompliance]));
    ensure!(is_authorized, Error::Unauthorized);

    let compliance: ContractAddress = ctx.parameter_cursor().get()?;
    host.state_mut().compliance = compliance;
    logger.log(&Event::ComplianceAdded(ComplianceAdded(compliance)))?;

    Ok(())
}
