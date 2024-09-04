pub mod error;
mod state;
pub mod types;

use concordium_cis2::*;
use concordium_protocols::concordium_cis2_ext::PlusSubOne;
use concordium_protocols::concordium_cis2_security::TokenUId;
use concordium_std::*;
use error::Error;
use state::{AddressState, State};
use types::*;

const SUPPORTS_STANDARDS: [StandardIdentifier<'static>; 2] =
    [CIS0_STANDARD_IDENTIFIER, CIS2_STANDARD_IDENTIFIER];

#[init(
    contract = "nft_multi_rewarded",
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
    let state = State {
        reward_token:  params.reward_token.clone(),
        curr_token_id: 0.into(),
        tokens:        state_builder.new_map(),
        addresses:     state_builder.new_map(),
    };
    logger.log(&Event::Init(params))?;

    Ok(state)
}

#[receive(
    contract = "nft_multi_rewarded",
    name = "isAgent",
    parameter = "Agent",
    return_value = "bool",
    error = "Error"
)]
pub fn is_agent(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let agent: Agent = ctx.parameter_cursor().get()?;
    let is_agent = host.state().address(&agent.address).is_some();
    Ok(is_agent)
}

#[receive(
    contract = "nft_multi_rewarded",
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
    host.state_mut()
        .add_address(params.address, AddressState::Agent)?;
    logger.log(&Event::AgentAdded(params.address))?;

    Ok(())
}

#[receive(
    contract = "nft_multi_rewarded",
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
    let state = host.state_mut().remove_and_get_address(&address)?;
    state.agent().ok_or(Error::InvalidAddress)?;
    logger.log(&&Event::AgentRemoved(address))?;

    Ok(())
}

#[receive(
    contract = "nft_multi_rewarded",
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
        let mut holder = state.address_or_insert_holder(&sender, state_builder);
        let holder = holder.holder_mut().ok_or(Error::InvalidAddress)?;

        match update {
            OperatorUpdate::Add => holder.add_operator(operator),
            OperatorUpdate::Remove => holder.remove_operator(&operator),
        }
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

#[receive(
    contract = "nft_multi_rewarded",
    name = "operatorOf",
    parameter = "OperatorOfQueryParams",
    return_value = "OperatorOfQueryResponse",
    error = "Error"
)]
pub fn operator_of(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<OperatorOfQueryResponse> {
    let OperatorOfQueryParams { queries }: OperatorOfQueryParams = ctx.parameter_cursor().get()?;
    let state = host.state();
    let mut res = Vec::with_capacity(queries.len());

    for query in queries {
        let is_operator = state.address(&query.owner).map_or(false, |h| {
            h.holder().map_or(false, |h| h.has_operator(&query.address))
        });
        res.push(is_operator);
    }

    Ok(OperatorOfQueryResponse(res))
}

#[receive(
    contract = "nft_multi_rewarded",
    name = "balanceOf",
    parameter = "types::BalanceOfQueryParams",
    return_value = "types::BalanceOfQueryResponse",
    error = "Error"
)]
pub fn balance_of(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<types::BalanceOfQueryResponse> {
    let types::BalanceOfQueryParams { queries } = ctx.parameter_cursor().get()?;
    let mut res: Vec<TokenAmount> = Vec::with_capacity(queries.len());
    let state = host.state();
    for query in queries {
        state.token(&query.token_id).ok_or(Error::InvalidTokenId)?;
        let balance: TokenAmount = state
            .address(&query.address)
            .and_then(|a| a.holder().map(|h| h.balance(&query.token_id)))
            .map(|v| if v { 1.into() } else { 0.into() })
            .unwrap_or(0.into());
        res.push(balance);
    }
    Ok(concordium_cis2::BalanceOfQueryResponse(res))
}

#[receive(
    contract = "nft_multi_rewarded",
    name = "supports",
    parameter = "SupportsQueryParams",
    return_value = "SupportsQueryResponse",
    error = "Error"
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
    contract = "nft_multi_rewarded",
    name = "tokenMetadata",
    parameter = "TokenMetadataQueryParams<TokenId>",
    return_value = "TokenMetadataQueryResponse",
    error = "Error"
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
        let metadata_url = state.token(&query).ok_or(Error::InvalidTokenId)?.clone();
        res.push(metadata_url);
    }

    Ok(TokenMetadataQueryResponse(res))
}

#[receive(
    contract = "nft_multi_rewarded",
    name = "transfer",
    enable_logger,
    mutable,
    parameter = "types::TransferParams",
    error = "Error"
)]
pub fn transfer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let sender = ctx.sender();
    let transfers: types::TransferParams = ctx.parameter_cursor().get()?;
    let transfers = transfers.0;

    for concordium_cis2::Transfer {
        to,
        from,
        amount,
        token_id,
        data,
    } in transfers
    {
        ensure!(amount.eq(&1.into()), Error::InvalidAmount);

        // Transfer token
        let (state, state_builder) = host.state_and_builder();
        {
            let mut from_holder = state.address_mut(&from).ok_or(Error::InvalidAddress)?;
            let from_holder = from_holder.holder_mut().ok_or(Error::InvalidAddress)?;
            ensure!(
                from.eq(&sender) || from_holder.has_operator(&sender),
                Error::Unauthorized
            );
            from_holder.remove_balance(&token_id);
        };

        state
            .address_or_insert_holder(&to.address(), state_builder)
            .holder_mut()
            .ok_or(Error::InvalidAddress)?
            .add_balance(token_id);

        logger.log(&Event::Cis2(Cis2Event::Transfer(TransferEvent {
            amount,
            token_id,
            from,
            to: to.address(),
        })))?;

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
            )
            .map_err(|_| Error::TransferInvokeError)?;
        }
    }

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct SignedMetadataUrls {
    pub metadata_urls: Vec<MetadataUrl>,
    pub agent_account: AccountAddress,
    pub signature:     AccountSignatures,
}
pub type MintParams = OnReceivingCis2DataParams<TokenIdVec, TokenAmountU64, SignedMetadataUrls>;

#[receive(
    contract = "nft_multi_rewarded",
    name = "mint",
    enable_logger,
    mutable,
    parameter = "MintParams",
    error = "Error",
    crypto_primitives
)]
pub fn mint(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
    crypto_primitives: &CryptoPrimitives,
) -> ContractResult<()> {
    let MintParams {
        token_id,
        amount,
        from,
        data: signed_metadata_urls,
    }: MintParams = ctx.parameter_cursor().get()?;
    ensure!(
        amount
            .0
            .eq(&(signed_metadata_urls.metadata_urls.len() as u64)),
        Error::InvalidAmount
    );
    let data_to_hash = to_data(&signed_metadata_urls.metadata_urls)?;
    let hash: HashSha3256 = crypto_primitives.hash_sha3_256(&data_to_hash);
    ensure!(
        host.check_account_signature(
            signed_metadata_urls.agent_account,
            &signed_metadata_urls.signature,
            &hash.0
        )
        .map_err(|_| { Error::UnauthorizedCheckSignature })?,
        Error::UnauthorizedInvalidSignature
    );

    let reward_token = TokenUId {
        id:       token_id,
        contract: match ctx.sender() {
            Address::Account(_) => bail!(Error::Unauthorized),
            Address::Contract(contract) => contract,
        },
    };
    let state: &State = host.state();
    ensure!(
        state.reward_token.eq(&reward_token),
        Error::InvalidRewardToken
    );
    ensure!(
        state
            .address(&signed_metadata_urls.agent_account.into())
            .map(|a| a.agent().is_some())
            .is_some_and(|v| v),
        Error::UnauthorizedInvalidAgent
    );

    let (state, builder) = host.state_and_builder();
    for metadata_url in signed_metadata_urls.metadata_urls {
        let curr_token_id = state.curr_token_id;
        state.curr_token_id = curr_token_id.plus_one();

        state.add_token(curr_token_id, metadata_url.clone())?;
        let mut holder = state.address_or_insert_holder(&from, builder);
        holder
            .holder_mut()
            .ok_or(Error::InvalidAddress)?
            .add_balance(curr_token_id);

        logger.log(&Event::Cis2(Cis2Event::TokenMetadata(TokenMetadataEvent {
            token_id: curr_token_id,
            metadata_url,
        })))?;
        logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
            token_id: curr_token_id,
            amount:   1.into(),
            owner:    from,
        })))?;
    }

    Ok(())
}

fn to_data(metadata_urls: &[MetadataUrl]) -> ContractResult<Vec<u8>> {
    let mut data = Vec::new();
    for url in metadata_urls {
        url.serial(&mut data)
            .map_err(|_| Error::MetadataUrlSerialization)?;
    }
    Ok(data)
}
