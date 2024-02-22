use std::ops::Sub;

use concordium_cis2::{
    AdditionalData, Cis2Event, MintEvent, OnReceivingCis2Params, Receiver, TokenAmountU32,
    TokenAmountU8, TokenMetadataEvent,
};
use concordium_std::*;

use concordium_rwa_utils::{
    agents_state::IsAgentsState,
    clients::{
        compliance_client::{ComplianceContract, IComplianceClient},
        identity_registry_client::{IdentityRegistryClient, IdentityRegistryContract},
    },
    compliance_types::Token,
    holders_security_state::IHoldersSecurityState,
    holders_state::IHoldersState,
    token_deposits_state::IDepositedTokensState,
    tokens_state::{ITokensState, IsTokenAmount},
};

use super::{
    error::*,
    event::*,
    state::{State, TokenState},
    types::*,
};

#[derive(Serialize, SchemaType)]
pub struct AddParam {
    pub deposit_token_id: NftTokenUId,
    pub metadata_url:     ContractMetadataUrl,
    pub fractions_rate:   Rate,
}

#[derive(Serialize, SchemaType)]
pub struct AddParams {
    pub tokens: Vec<AddParam>,
}

/// Add a new token to the contract.
/// The token is then identified by the wrapped token id and the metadata url.
#[receive(
    contract = "rwa_security_sft",
    name = "addTokens",
    enable_logger,
    mutable,
    parameter = "AddParams",
    error = "super::error::Error"
)]
pub fn add_tokens(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    // Sender of this transaction should be registered as an agent in the contract
    ensure!(host.state().is_agent(&ctx.sender()), Error::Unauthorized);
    let params: AddParams = ctx.parameter_cursor().get()?;
    for AddParam {
        deposit_token_id,
        metadata_url,
        fractions_rate,
    } in params.tokens
    {
        ensure!(fractions_rate.is_valid(), Error::InvalidFractionsRate);
        let metadata_url: MetadataUrl = metadata_url.into();
        let state = host.state_mut();
        let token_id = state.get_wrapped_id(&deposit_token_id).unwrap_or_else(|| {
            let token_id = state.get_and_increment_token_id();
            state.add_wrapped_id(deposit_token_id.clone(), token_id);
            token_id
        });
        let existing = state.add_or_replace_token(token_id, TokenState {
            metadata_url: metadata_url.to_owned(),
            fractions_rate,
            deposit_token_id,
            supply: TokenAmount::zero(),
        });

        // Updating Metadata of token in use is not allowed
        if let Some(existing) = existing {
            ensure!(existing.supply.is_zero(), Error::InvalidTokenId);
        }

        logger.log(&Event::Cis2(Cis2Event::TokenMetadata(TokenMetadataEvent {
            token_id,
            metadata_url,
        })))?;
    }

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct MintParam {
    /// The token id of the deposited token.
    pub deposited_token_id:    NftTokenUId,
    /// The owner of the deposited token.
    pub deposited_token_owner: AccountAddress,
    /// The amount of the deposited token.
    pub deposited_amount:      NftTokenAmount,
    /// The owner of the minted token.
    pub owner:                 Receiver,
}

/// Mint the given amount of tokens to the owner. Locking the deposited tokens.
#[receive(
    contract = "rwa_security_sft",
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
    ensure!(ctx.sender().matches_account(&params.deposited_token_owner), Error::Unauthorized);

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
    let identity_registry = IdentityRegistryContract(state.identity_registry());
    ensure!(identity_registry.is_verified(host, &owner_address)?, Error::UnVerifiedIdentity);

    let compliance = ComplianceContract(state.compliance());
    let token_id =
        host.state().get_wrapped_id(&params.deposited_token_id).ok_or(Error::InvalidTokenId)?;

    let token_state = host.state().token(&token_id)?;
    let (mint_amount, un_converted_amount) = token_state
        .fractions_rate
        .convert(&params.deposited_amount.0.into())
        .map_err(|_| Error::InvalidFractionsRate)?;
    let mint_amount = TokenAmount::from(mint_amount as u32);
    let un_converted_amount = TokenAmountU8::from(un_converted_amount as u8);
    let converted_amount = params.deposited_amount.sub(un_converted_amount);

    ensure!(mint_amount.ge(&TokenAmountU32(0)), Error::InvalidAmount);
    
    let compliance_token = Token::new(token_id, curr_contract_address);
    ensure!(
        compliance.can_transfer(host, compliance_token, owner_address, mint_amount)?,
        Error::InCompliantTransfer
    );

    // Minting Logic
    let (state, state_builder) = host.state_and_builder();
    state.inc_locked_deposits(
        &params.deposited_token_id.to_token_owner_uid(params.deposited_token_owner.into()),
        converted_amount,
    )?;
    state.add_balance(owner_address, &token_id, mint_amount, state_builder)?;
    // Notify compliance that the token has been minted
    compliance.minted(host, compliance_token, owner_address, mint_amount)?;
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
