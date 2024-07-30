use super::{error::*, state::State, types::*};
use concordium_cis2::{
    AdditionalData, Cis2Event, MintEvent, OnReceivingCis2Params, Receiver, TokenAmountU8,
    TokenMetadataEvent,
};
use concordium_rwa_utils::{
    agents_state::IsAgentsState,
    clients::{
        compliance_client::{ComplianceContract, IComplianceClient},
        identity_registry_client::{IdentityRegistryClient, IdentityRegistryContract},
    },
    compliance_types::Token,
    holders_security_state::IHoldersSecurityState,
    holders_state::IHoldersState,
};
use concordium_std::*;

const TOKEN_AMOUNT_1: TokenAmountU8 = TokenAmountU8(1);

/// Mints the given amount of given tokenIds for the given address.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns error `Unauthorized` if the sender is not an agent.
/// Returns error `UnVerifiedIdentity` if the owner is not verified.
/// Returns error `InCompliantTransfer` if the transfer is non-compliant.
/// Returns error `ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_security_nft",
    name = "mint",
    enable_logger,
    mutable,
    parameter = "MintParams",
    error = "super::error::Error"
)]
pub fn mint(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let self_address = ctx.self_address();
    let params: MintParams = ctx.parameter_cursor().get()?;
    let owner_address = params.owner.address();

    let state = host.state();
    // Sender of this transaction should be registered as an agent in the contract
    ensure!(state.is_agent(&ctx.sender()), Error::Unauthorized);

    // Ensure that the owner is not recovered
    state.ensure_not_recovered(&owner_address)?;

    // Ensure that the owner is verified
    let identity_registry = IdentityRegistryContract(state.identity_registry());
    ensure!(identity_registry.is_verified(host, &owner_address)?, Error::UnVerifiedIdentity);

    let compliance = ComplianceContract(state.compliance());
    for MintParam {
        metadata_url,
    } in params.tokens
    {
        let metadata_url: MetadataUrl = metadata_url.into();
        // Generate a token with the metdata url
        let token_id = host.state_mut().generate_add_token(metadata_url.clone())?;
        let compliance_token = Token::new(token_id, self_address);
        // Check if the owner can hold the token
        ensure!(
            compliance.can_transfer(host, compliance_token, owner_address, TOKEN_AMOUNT_1)?,
            Error::InCompliantTransfer
        );
        let (state, state_builder) = host.state_and_builder();
        state.add_balance(owner_address, &token_id, TOKEN_AMOUNT_1, state_builder)?;
        // Notify compliance that the token has been minted
        compliance.minted(host, compliance_token, owner_address, TOKEN_AMOUNT_1)?;
        logger.log(&Event::Cis2(Cis2Event::TokenMetadata(TokenMetadataEvent {
            token_id,
            metadata_url,
        })))?;
        logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
            token_id,
            amount: TOKEN_AMOUNT_1,
            owner: owner_address,
        })))?;

        // If the receiver is a contract: invoke the receive hook function.
        if let Receiver::Contract(address, function) = &params.owner {
            let parameter = OnReceivingCis2Params {
                token_id,
                amount: TOKEN_AMOUNT_1,
                // From self because the minting is being done from deposited tokens in custody of
                // the current contract
                from: self_address.into(),
                data: AdditionalData::empty(),
            };
            host.invoke_contract(
                address,
                &parameter,
                function.as_entrypoint_name(),
                Amount::zero(),
            )?;
        }
    }

    Ok(())
}
