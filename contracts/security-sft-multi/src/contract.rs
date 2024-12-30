use concordium_cis2::*;
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::contract_logic::{
    authorize_and_burn, authorize_and_mint, authorize_and_transfer, Cis2SecurityState,
    SecurityTokenState,
};
use concordium_protocols::concordium_cis2_security::identity_registry_client::IdentityRegistryClient;
use concordium_protocols::concordium_cis2_security::{
    AddTokenParams, AgentUpdatedEvent, ComplianceAdded, FreezeParam, IdentityRegistryAdded, Paused,
    RecoverEvent, TokenFrozen,
};
use concordium_std::*;

use super::error::Error;
use super::state::State;
use super::types::{BalanceOfQueryParams, BalanceOfQueryResponse, *};
use crate::state::{HolderState, HolderStateActive};
const SUPPORTS_STANDARDS: [StandardIdentifier<'static>; 2] =
    [CIS0_STANDARD_IDENTIFIER, CIS2_STANDARD_IDENTIFIER];

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
    contract = "security_sft_multi",
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
            HolderState::Active(HolderStateActive::new_with_roles(
                state_builder,
                &AgentRole::owner_roles(),
            )),
        );
        logger.log(&Event::AgentAdded(AgentUpdatedEvent {
            agent: owner,
            roles: AgentRole::owner_roles(),
        }))?;
        for agent in params.agents {
            let _ = addresses.insert(
                agent.address,
                HolderState::Active(HolderStateActive::new_with_roles(
                    state_builder,
                    &agent.roles,
                )),
            );
            logger.log(&Event::AgentAdded(AgentUpdatedEvent {
                agent: agent.address,
                roles: agent.roles,
            }))?;
        }
        addresses
    };
    let state = State {
        security: params.security,
        addresses,
        tokens: state_builder.new_map(),
    };

    if let Some(security_params) = state.security {
        logger.log(&Event::IdentityRegistryAdded(IdentityRegistryAdded(
            security_params.identity_registry,
        )))?;
        logger.log(&Event::ComplianceAdded(ComplianceAdded(
            security_params.compliance,
        )))?;
    }

    Ok(state)
}

/// Returns the address of the identity registry contract.
///
/// # Returns
///
/// Returns `ContractResult<ContractAddress>` containing the address of the identity registry contract.
#[receive(
    contract = "security_sft_multi",
    name = "identityRegistry",
    return_value = "ContractAddress"
)]
pub fn identity_registry(
    _: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<Option<ContractAddress>> {
    Ok(host.state().security.map(|s| s.identity_registry))
}

#[receive(
    contract = "security_sft_multi",
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
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::SetIdentityRegistry]));
    ensure!(is_authorized, Error::Unauthorized);

    let identity_registry: ContractAddress = ctx.parameter_cursor().get()?;
    match host.state_mut().security {
        Some(mut security) => {
            security.identity_registry = identity_registry;
        }
        None => {
            bail!(Error::SecurityNotSet);
        }
    }
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
    contract = "security_sft_multi",
    name = "compliance",
    return_value = "ContractAddress"
)]
pub fn compliance(
    _: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<Option<ContractAddress>> {
    Ok(host.state().security.map(|s| s.compliance))
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
    contract = "security_sft_multi",
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
    let state = host.state_mut();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::SetCompliance]));
    ensure!(is_authorized, Error::Unauthorized);

    let compliance: ContractAddress = ctx.parameter_cursor().get()?;
    match &mut state.security {
        Some(security) => {
            security.compliance = compliance;
        }
        None => {
            bail!(Error::SecurityNotSet);
        }
    }
    logger.log(&Event::ComplianceAdded(ComplianceAdded(compliance)))?;

    Ok(())
}

/// Returns true if the given address is an agent.
///
/// # Returns
///
/// Returns `ContractResult<Vec<Address>>` containing the list of agents.
#[receive(
    contract = "security_sft_multi",
    name = "isAgent",
    parameter = "Agent",
    return_value = "bool",
    error = "Error"
)]
pub fn is_agent(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let agent: Agent = ctx.parameter_cursor().get()?;
    let is_agent = host
        .state()
        .addresses
        .get(&agent.address)
        .is_some_and(|a| a.is_agent(&agent.roles));
    Ok(is_agent)
}

/// Adds the given address as an agent.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "security_sft_multi",
    name = "addAgent",
    mutable,
    enable_logger,
    parameter = "Agent",
    error = "Error"
)]
pub fn add_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: Agent = ctx.parameter_cursor().get()?;
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let (state, state_builder) = host.state_and_builder();
    state
        .addresses
        .entry(params.address)
        .or_insert(HolderState::new_active(state_builder))
        .try_modify(|holder| holder.set_agent_roles(&params.roles))?;
    logger.log(&Event::AgentAdded(AgentUpdatedEvent {
        agent: params.address,
        roles: params.roles,
    }))?;

    Ok(())
}

/// Removes the given address as an agent.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "security_sft_multi",
    name = "removeAgent",
    mutable,
    enable_logger,
    parameter = "Address",
    error = "Error"
)]
pub fn remove_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let address: Address = ctx.parameter_cursor().get()?;
    host.state_mut()
        .addresses
        .entry(address)
        .occupied_or(Error::InvalidAddress)?
        .clear_agent_roles()?;

    logger.log(&Event::AgentRemoved(AgentUpdatedEvent {
        agent: address,
        roles: vec![],
    }))?;

    Ok(())
}

/// Freezes the given amount of given tokenIds for the given address.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender is not an agent.
/// Returns `Error::InsufficientFunds` if the owner does not have enough unfrozen balance.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "freeze",
    mutable,
    enable_logger,
    parameter = "FreezeParams",
    error = "Error"
)]
pub fn freeze(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state_mut();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::Freeze]));
    ensure!(is_authorized, Error::Unauthorized);

    let FreezeParams {
        owner: owner_address,
        tokens: freezes,
    }: FreezeParams = ctx.parameter_cursor().get()?;

    let mut owner = state
        .addresses
        .get_mut(&owner_address)
        .ok_or(Error::InvalidAddress)?;

    for FreezeParam {
        token_id,
        token_amount,
    } in freezes
    {
        ensure!(token_amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        owner.freeze(token_id, token_amount)?;
        logger.log(&Event::TokenFrozen(TokenFrozen {
            token_id,
            amount: token_amount,
            address: owner_address,
        }))?
    }

    Ok(())
}

/// Unfreezes the given amount of given tokenIds for the given address.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender is not an agent.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "unFreeze",
    mutable,
    enable_logger,
    parameter = "FreezeParams",
    error = "Error"
)]
pub fn un_freeze(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::UnFreeze]));
    ensure!(is_authorized, Error::Unauthorized);

    let FreezeParams {
        owner: owner_address,
        tokens: freezes,
    }: FreezeParams = ctx.parameter_cursor().get()?;

    let state = host.state_mut();
    let mut owner = state
        .addresses
        .get_mut(&owner_address)
        .ok_or(Error::InvalidAddress)?;

    for FreezeParam {
        token_amount,
        token_id,
    } in freezes
    {
        ensure!(token_amount.gt(&TokenAmount::zero()), Error::InvalidAmount);
        owner.un_freeze(token_id, token_amount)?;
        logger.log(&Event::TokenUnFrozen(TokenFrozen {
            token_id,
            amount: token_amount,
            address: owner_address,
        }))?
    }

    Ok(())
}

/// Returns the frozen balance of the given token for the given addresses.
///
/// # Returns
///
/// Returns `ContractResult<BalanceOfQueryResponse>` containing the frozen balance of the given token for the given addresses.
///
/// # Errors
///
/// - `Error::TokenDoesNotExist`: If any of the specified tokens do not exist.
/// - `Error::InvalidAddress`: If any of the provided addresses are invalid.
/// - `Error::ParseError`: If the input parameters could not be parsed correctly.
#[receive(
    contract = "security_sft_multi",
    name = "balanceOfFrozen",
    parameter = "BalanceOfQueryParams",
    return_value = "BalanceOfQueryResponse",
    error = "Error"
)]
pub fn balance_of_frozen(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<BalanceOfQueryResponse> {
    let BalanceOfQueryParams { queries } = ctx.parameter_cursor().get()?;

    let mut amounts = Vec::with_capacity(queries.len());
    let state = host.state();
    for query in queries {
        ensure!(
            state.tokens.get(&query.token_id).is_some(),
            Error::InvalidTokenId
        );
        let balance = state
            .addresses
            .get(&query.address)
            .map_or_else(TokenAmount::zero, |holder| {
                holder.frozen_balance(query.token_id)
            });
        amounts.push(balance);
    }

    Ok(concordium_cis2::BalanceOfQueryResponse(amounts))
}

/// Returns the unfrozen balance of the given token for the given addresses.
///
/// # Returns
///
/// Returns `ContractResult<BalanceOfQueryResponse>` containing the unfrozen balance of the given token for the given addresses.
///
/// # Errors
///
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "balanceOfUnFrozen",
    parameter = "BalanceOfQueryParams",
    return_value = "BalanceOfQueryResponse",
    error = "Error"
)]
pub fn balance_of_un_frozen(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<BalanceOfQueryResponse> {
    let BalanceOfQueryParams { queries } = ctx.parameter_cursor().get()?;

    let mut amounts = Vec::with_capacity(queries.len());
    let state = host.state();
    for query in queries {
        ensure!(
            state.tokens.get(&query.token_id).is_some(),
            Error::InvalidTokenId
        );
        let balance = state
            .addresses
            .get(&query.address)
            .map_or_else(TokenAmount::zero, |holder| {
                holder.un_frozen_balance(query.token_id)
            });
        amounts.push(balance);
    }

    Ok(concordium_cis2::BalanceOfQueryResponse(amounts))
}

/// Pauses the given tokenIds.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender is not an agent.
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "pause",
    mutable,
    enable_logger,
    parameter = "PauseParams",
    error = "super::error::Error"
)]
pub fn pause(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state_mut();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::Pause]));
    ensure!(is_authorized, Error::Unauthorized);

    let PauseParams { tokens }: PauseParams = ctx.parameter_cursor().get()?;
    for PauseParam { token_id } in tokens {
        state
            .tokens
            .get_mut(&token_id)
            .ok_or(Error::InvalidTokenId)?
            .pause();
        logger.log(&Event::Paused(Paused { token_id }))?;
    }

    Ok(())
}

/// Unpauses the given tokens.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender is not an agent.
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "unPause",
    mutable,
    enable_logger,
    parameter = "PauseParams",
    error = "super::error::Error"
)]
pub fn un_pause(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let state = host.state_mut();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::UnPause]));
    ensure!(is_authorized, Error::Unauthorized);

    let PauseParams { tokens }: PauseParams = ctx.parameter_cursor().get()?;
    for PauseParam { token_id } in tokens {
        state
            .tokens
            .get_mut(&token_id)
            .ok_or(Error::InvalidTokenId)?
            .un_pause();
        logger.log(&Event::UnPaused(Paused { token_id }))?;
    }

    Ok(())
}

/// Returns true if the given tokens are paused.
///
/// # Returns
///
/// Returns `ContractResult<IsPausedResponse>` containing a boolean for each token indicating whether it is paused.
///
/// # Errors
///
/// Returns `Error::TokenDoesNotExist` if the token does not exist.
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "isPaused",
    parameter = "PauseParams",
    return_value = "IsPausedResponse",
    error = "super::error::Error"
)]
pub fn is_paused(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<IsPausedResponse> {
    let PauseParams { tokens }: PauseParams = ctx.parameter_cursor().get()?;
    let mut res = IsPausedResponse {
        tokens: Vec::with_capacity(tokens.len()),
    };

    let state = host.state();
    for token in tokens {
        let paused = state
            .tokens
            .get(&token.token_id)
            .ok_or(Error::InvalidTokenId)?
            .paused;
        res.tokens.push(paused);
    }

    Ok(res)
}

#[receive(
    contract = "security_sft_multi",
    name = "recover",
    mutable,
    enable_logger,
    parameter = "RecoverParam",
    error = "Error"
)]
pub fn recover(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let RecoverParam {
        lost_account,
        new_account,
    }: RecoverParam = ctx.parameter_cursor().get()?;
    let state = host.state();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::HolderRecovery]));
    ensure!(is_authorized, Error::Unauthorized);

    if let Some(security) = state.security {
        ensure!(
            host.invoke_identity_registry_is_verified(&security.identity_registry, &new_account)?,
            Error::UnVerifiedIdentity
        );
    }

    host.state_mut().recover(lost_account, new_account)?;
    logger.log(&Event::Recovered(RecoverEvent {
        lost_account,
        new_account,
    }))?;

    Ok(())
}

#[receive(
    contract = "security_sft_multi",
    name = "recoveryAddress",
    parameter = "Address",
    error = "Error",
    return_value = "Option<Address>"
)]
pub fn recovery_address(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<Option<Address>> {
    let address: Address = ctx.parameter_cursor().get()?;
    let recovery_address = host
        .state()
        .addresses
        .get(&address)
        .and_then(|holder| holder.recovery_address());
    Ok(recovery_address)
}

/// Updates the operator status for the sender.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was successful.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "updateOperator",
    mutable,
    enable_logger,
    parameter = "UpdateOperatorParams",
    error = "Error"
)]
pub fn update_operator(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let UpdateOperatorParams { 0: updates }: UpdateOperatorParams = ctx.parameter_cursor().get()?;
    let sender = ctx.sender();
    let (state, state_builder) = host.state_and_builder();

    for UpdateOperator { operator, update } in updates {
        state
            .addresses
            .entry(sender)
            .or_insert(HolderState::Active(HolderStateActive::new(state_builder)))
            .try_modify(|holder| match update {
                OperatorUpdate::Add => holder.add_operator(operator),
                OperatorUpdate::Remove => holder.remove_operator(&operator),
            })?;
        logger.log(&Event::Cis2(Cis2Event::UpdateOperator(
            UpdateOperatorEvent {
                operator,
                update,
                owner: sender,
            },
        )))?;
    }
    Ok(())
}

/// # Returns
///
/// Returns `ContractResult<OperatorOfQueryResponse>` containing a boolean
/// indicating whether the given address is an operator for the given owner.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "security_sft_multi",
    name = "operatorOf",
    parameter = "OperatorOfQueryParams",
    return_value = "OperatorOfQueryResponse",
    error = "super::error::Error"
)]
pub fn operator_of(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<OperatorOfQueryResponse> {
    let OperatorOfQueryParams { queries }: OperatorOfQueryParams = ctx.parameter_cursor().get()?;
    let state = host.state();
    let mut res = Vec::with_capacity(queries.len());

    for query in queries {
        let is_operator = state
            .addresses
            .get(&query.owner)
            .is_some_and(|a| a.has_operator(&query.address));
        res.push(is_operator);
    }

    Ok(OperatorOfQueryResponse(res))
}

/// Retrieves the metadata for a token.
///
/// # Returns
///
/// Returns `ContractResult<TokenMetadataQueryResponse>` containing the metadata
/// for each queried token.
///
/// # Errors
///
/// This method will return a `ParseError` if it fails to parse the input
/// this method will return an `InvalidTokenId` if the token does not exist.
/// parameters.
#[receive(
    contract = "security_sft_multi",
    name = "tokenMetadata",
    parameter = "TokenMetadataQueryParams<TokenId>",
    return_value = "TokenMetadataQueryResponse",
    error = "super::error::Error"
)]
pub fn token_metadata(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<TokenMetadataQueryResponse> {
    let TokenMetadataQueryParams { queries }: TokenMetadataQueryParams<TokenId> =
        ctx.parameter_cursor().get()?;
    let state = host.state();
    let mut res = Vec::with_capacity(queries.len());
    for query in queries {
        res.push(
            state
                .tokens
                .get(&query)
                .ok_or(Error::InvalidTokenId)?
                .metadata_url()
                .clone(),
        );
    }

    Ok(TokenMetadataQueryResponse(res))
}

/// Determines whether the contract supports a specific concordium standard.
///
/// # Returns
///
/// Returns `ContractResult<SupportsQueryResponse>` containing the support
/// status for each queried standard.
///
/// # Errors
///
/// This method will return an error if:
/// * `ParseError` - The parameter cursor cannot parse the `SupportsQueryParams`.
#[receive(
    contract = "security_sft_multi",
    name = "supports",
    parameter = "SupportsQueryParams",
    return_value = "SupportsQueryResponse",
    error = "super::error::Error"
)]
fn supports(ctx: &ReceiveContext, _: &Host<State>) -> ContractResult<SupportsQueryResponse> {
    let params: SupportsQueryParams = ctx.parameter_cursor().get()?;
    let mut response = Vec::with_capacity(params.queries.len());
    for std_id in params.queries {
        if SUPPORTS_STANDARDS.contains(&std_id.as_standard_identifier()) {
            response.push(SupportResult::Support);
        } else {
            response.push(SupportResult::NoSupport)
        }
    }

    Ok(SupportsQueryResponse::from(response))
}

#[receive(
    contract = "security_sft_multi",
    name = "addToken",
    enable_logger,
    mutable,
    parameter = "AddTokenParams<TokenId, ContractMetadataUrl>",
    error = "Error"
)]
pub fn add_token(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: AddTokenParams<TokenId, ContractMetadataUrl> = ctx.parameter_cursor().get()?;

    let state = host.state_mut();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::AddToken]));
    ensure!(is_authorized, Error::Unauthorized);

    let token_id = params.token_id;
    let token_metadata = params.token_metadata;

    state
        .tokens
        .entry(token_id)
        .or_insert_with(|| SecurityTokenState {
            metadata_url: token_metadata.clone().into(),
            supply:       TokenAmount::zero(),
            paused:       false,
        });

    logger.log(&Event::Cis2(Cis2Event::TokenMetadata(TokenMetadataEvent {
        metadata_url: token_metadata.into(),
        token_id,
    })))?;

    Ok(())
}

#[receive(
    contract = "security_sft_multi",
    name = "removeToken",
    enable_logger,
    mutable,
    parameter = "TokenId",
    error = "Error"
)]
pub fn remove_token(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let token_id: TokenId = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    let is_authorized = state
        .addresses
        .get(&ctx.sender())
        .is_some_and(|a| a.is_agent(&[AgentRole::RemoveToken]));
    ensure!(is_authorized, Error::Unauthorized);

    state
        .tokens
        .remove_and_get(&token_id)
        .filter(|t| t.supply.is_zero())
        .ok_or(Error::InvalidTokenId)?;

    logger.log(&Event::TokenRemoved(token_id))?;

    Ok(())
}

#[receive(
    contract = "security_sft_multi",
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
    let security = state.security;

    for MintParam {
        address: owner,
        amount,
    } in params.owners
    {
        authorize_and_mint(
            host,
            security,
            params.token_id,
            owner.address(),
            amount,
            self_address,
        )?;

        if let Receiver::Contract(contract, entrypoint) = &owner {
            host.invoke_contract(
                contract,
                &OnReceivingCis2Params {
                    token_id: params.token_id,
                    amount,
                    from: ctx.self_address().into(),
                    data: AdditionalData::empty(),
                },
                entrypoint.as_entrypoint_name(),
                Amount::zero(),
            )?;
        }

        logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
            token_id: params.token_id,
            amount:   amount.total(),
            owner:    owner.address(),
        })))?;
        if amount.frozen.gt(&TokenAmount::zero()) {
            logger.log(&Event::TokenFrozen(TokenFrozen {
                token_id: params.token_id,
                amount:   amount.frozen,
                address:  owner.address(),
            }))?;
        }
    }

    Ok(())
}

#[receive(
    contract = "security_sft_multi",
    name = "transfer",
    enable_logger,
    mutable,
    parameter = "concordium_cis2::TransferParams::<TokenId, TokenAmount>",
    error = "Error"
)]
pub fn transfer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let concordium_cis2::TransferParams::<TokenId, TokenAmount>(transfers) =
        ctx.parameter_cursor().get()?;
    let self_address = ctx.self_address();
    let sender = ctx.sender();

    let state = host.state();
    let sender_address = state.addresses.get(&sender);
    let sender_is_operator_agent = sender_address
        .as_ref()
        .map(|s| s.is_agent(&[AgentRole::Operator]))
        .unwrap_or(false);
    let sender_is_forced_transfer_agent = sender_address
        .as_ref()
        .map(|s| s.is_agent(&[AgentRole::ForcedTransfer]))
        .unwrap_or(false);
    let security = state.security;

    for Transfer {
        to,
        from,
        amount,
        token_id,
        data,
    } in transfers
    {
        let to_address = to.address();
        let state = host.state();
        concordium_dbg!(
            "transfer: from: {:?}, to: {:?}, amount: {:?}, token_id: {:?}",
            from,
            to_address,
            amount,
            token_id
        );
        let sender_is_operator = state
            .addresses
            .get(&from)
            .map(|a| a.has_operator(&sender))
            .ok_or(Error::InvalidAddress)?;
        let sender_can_operate = sender.eq(&from) || sender_is_operator;

        let unfrozen_amount = authorize_and_transfer(
            host,
            security,
            sender_can_operate,
            sender_is_operator_agent,
            sender_is_forced_transfer_agent,
            self_address,
            token_id,
            from,
            to_address,
            amount,
        )?;

        if let Receiver::Contract(to_contract, entrypoint) = to {
            host.invoke_contract(
                &to_contract,
                &OnReceivingCis2Params {
                    token_id,
                    amount,
                    from,
                    data,
                },
                entrypoint.as_entrypoint_name(),
                Amount::zero(),
            )?;
        }

        if unfrozen_amount.gt(&TokenAmount::zero()) {
            logger.log(&Event::TokenUnFrozen(TokenFrozen {
                token_id,
                amount: unfrozen_amount,
                address: from,
            }))?;
        }
        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id,
            from,
            to: to_address,
        })))?;
    }

    Ok(())
}

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
    contract = "security_sft_multi",
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
    let params: BurnParams = ctx.parameter_cursor().get()?;
    let self_address = ctx.self_address();
    let sender = ctx.sender();

    let state = host.state();
    let sender_address = state.addresses.get(&sender);
    let sender_is_operator_agent = sender_address
        .as_ref()
        .map(|s| s.is_agent(&[AgentRole::Operator]))
        .unwrap_or(false);
    let sender_is_forced_burn_agent = sender_address
        .as_ref()
        .map(|s| s.is_agent(&[AgentRole::ForcedBurn]))
        .unwrap_or(false);
    let security = state.security;

    for Burn {
        token_id,
        amount,
        owner,
    } in params.0
    {
        let state = host.state();
        let sender_is_operator = state
            .addresses
            .get(&owner)
            .ok_or(Error::InvalidAddress)?
            .has_operator(&sender);
        let sender_can_operate = sender.eq(&owner) || sender_is_operator;

        let unfrozen_amount = authorize_and_burn(
            host,
            security,
            sender_can_operate,
            sender_is_operator_agent,
            sender_is_forced_burn_agent,
            self_address,
            token_id,
            owner,
            amount,
        )?;

        if unfrozen_amount.gt(&TokenAmount::zero()) {
            logger.log(&Event::TokenUnFrozen(TokenFrozen {
                token_id,
                amount: unfrozen_amount,
                address: owner,
            }))?;
        }
        logger.log(&Event::Cis2(Cis2Event::Burn(BurnEvent {
            amount,
            token_id,
            owner,
        })))?;
    }

    Ok(())
}

/// Queries the balance of the specified token IDs for the given addresses.
///
/// This function takes a list of `BalanceOfQueryParams` and
/// returns a `BalanceOfQueryResponse` containing the token balances for each query.
///
/// # Returns
/// A `ContractResult` containing the token balances for each query.
#[receive(
    contract = "security_sft_multi",
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
        state
            .tokens
            .get(&query.token_id)
            .ok_or(Error::InvalidTokenId)?;
        let balance: TokenAmount = state
            .addresses
            .get(&query.address)
            .map_or(TokenAmount::zero(), |a| a.balance_total(query.token_id));
        res.push(balance);
    }
    Ok(concordium_cis2::BalanceOfQueryResponse(res))
}
