pub mod error;
mod state;
pub mod types;

use concordium_cis2::*;
use concordium_protocols::concordium_cis2_security::cis2_security_client::Cis2SecurityClient;
use concordium_protocols::concordium_cis2_security::Burn;
use concordium_std::*;
pub use concordium_std::{MetadataUrl, Timestamp};
use error::Error;
use state::{HolderState, State};
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
    let reward_token = params.reward_token;
    let state = State {
        reward_token,
        curr_token_id: 0.into(),
        tokens: state_builder.new_map(),
        addresses: state_builder.new_map(),
    };
    logger.log(&Event::RewardTokenUpdated(params))?;

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
    let is_agent = host
        .state()
        .addresses
        .get(&agent.address)
        .is_some_and(|a| a.is_agent);
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
    let (state, state_builder) = host.state_and_builder();
    state
        .addresses
        .entry(params.address)
        .or_insert_with(|| HolderState::new(state_builder))
        .modify(|a| a.is_agent = true);
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
    host.state_mut()
        .addresses
        .get_mut(&address)
        .map_or(Err(Error::InvalidAddress), |mut a| {
            a.is_agent = false;
            Ok(())
        })?;
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
        state
            .addresses
            .entry(sender)
            .or_insert_with(|| HolderState::new(state_builder))
            .modify(|a| match update {
                OperatorUpdate::Add => a.operators.insert(operator),
                OperatorUpdate::Remove => a.operators.remove(&operator),
            });

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
        let is_operator = state
            .addresses
            .get(&query.owner)
            .map_or(false, |a| a.operators.contains(&query.address));
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
        let balance = state.addresses.get(&query.address).map_or(0.into(), |a| {
            if a.balances.contains(&query.token_id) {
                1.into()
            } else {
                0.into()
            }
        });
        res.push(balance);
        ensure!(
            state.tokens.get(&query.token_id).is_some(),
            Error::InvalidTokenId
        );
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
        let metadata_url = state
            .tokens
            .get(&query)
            .ok_or(Error::InvalidTokenId)?
            .clone();
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
            let mut from_holder = state
                .addresses
                .get_mut(&from)
                .ok_or(Error::InvalidAddress)?;
            ensure!(
                from.eq(&sender) || from_holder.operators.contains(&sender),
                Error::Unauthorized
            );
            ensure!(
                from_holder.balances.remove(&token_id),
                Error::InsufficientFunds
            );
        };

        state
            .addresses
            .entry(to.address())
            .or_insert_with(|| HolderState::new(state_builder))
            .modify(|a| a.balances.insert(token_id));

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
pub struct MintParams {
    pub signed_metadata: SignedMetadata,
    pub signer:          AccountAddress,
    pub signature:       AccountSignatures,
}

#[derive(Serialize, SchemaType)]
pub struct SignedMetadata {
    pub contract_address: ContractAddress,
    pub metadata_url:     MetadataUrl,
    pub account:          AccountAddress,
    pub account_nonce:    u64,
}

impl From<&SignedMetadata> for Vec<u8> {
    fn from(val: &SignedMetadata) -> Self {
        let mut data = Vec::new();
        val.serial(&mut data).unwrap();
        data
    }
}

impl SignedMetadata {
    pub fn hash<T>(&self, hasher: T) -> ContractResult<[u8; 32]>
    where T: FnOnce(Vec<u8>) -> [u8; 32] {
        let hash: Vec<u8> = self.into();
        let hash = hasher(hash);
        Ok(hash)
    }
}

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
        signed_metadata,
        signer: agent,
        signature,
    } = ctx.parameter_cursor().get()?;
    ensure!(
        signed_metadata.contract_address.eq(&ctx.self_address()),
        Error::InvalidContractAddress
    );
    let hash = signed_metadata.hash(|data| crypto_primitives.hash_sha2_256(&data).0)?;
    ensure!(
        host.check_account_signature(agent, &signature, &hash)
            .map_err(|_| Error::CheckSignature)?,
        Error::InvalidSignature
    );

    let (state, builder) = host.state_and_builder();
    ensure!(
        state
            .addresses
            .get(&agent.into())
            .is_some_and(|a| a.is_agent),
        Error::UnauthorizedInvalidAgent
    );
    let (..) = mint_token(
        state,
        builder,
        signed_metadata.account.into(),
        signed_metadata.metadata_url,
        Some(signed_metadata.account_nonce),
        logger,
    )?;
    let reward_token = state.reward_token;
    host.invoke_burn_single(&reward_token.contract, Burn {
        amount:   TokenAmountU64(1),
        token_id: reward_token.id,
        owner:    signed_metadata.account.into(),
    })
    .map_err(|_| Error::BurnError)?;

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct MintAgentParams {
    pub metadata_url: ContractMetadataUrl,
    pub account:      AccountAddress,
}

#[receive(
    contract = "nft_multi_rewarded",
    name = "mintAgent",
    mutable,
    parameter = "MintAgentParams",
    error = "Error",
    enable_logger
)]
pub fn mint_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let MintAgentParams {
        account: owner,
        metadata_url,
    }: MintAgentParams = ctx.parameter_cursor().get()?;
    let owner: Address = owner.into();
    let (state, builder) = host.state_and_builder();
    let is_agent = state
        .addresses
        .get(&ctx.sender())
        .map(|a| a.is_agent)
        .unwrap_or(false);
    ensure!(is_agent, Error::UnauthorizedInvalidAgent);

    mint_token(state, builder, owner, metadata_url.into(), None, logger)?;
    let reward_token = state.reward_token;
    host.invoke_burn_single(&reward_token.contract, Burn {
        amount: TokenAmountU64(1),
        token_id: reward_token.id,
        owner,
    })
    .map_err(|_| Error::BurnError)?;
    Ok(())
}

/// Mints a new token and assigns it to the given owner.
/// Returns the minted token ID and the new nonce.
fn mint_token(
    state: &mut State,
    builder: &mut StateBuilder,
    owner: Address,
    metadata_url: MetadataUrl,
    account_nonce: Option<u64>,
    logger: &mut Logger,
) -> ContractResult<(TokenIdU64, u64)> {
    let curr_token_id = state.curr_token_id;
    state
        .tokens
        .entry(curr_token_id)
        .vacant_or(Error::InvalidTokenId)?
        .insert(metadata_url.clone());

    let nonce = state
        .addresses
        .entry(owner)
        .or_insert_with(|| HolderState::new(builder))
        .try_modify(|holder| {
            if let Some(account_nonce) = account_nonce {
                ensure!(holder.nonce.eq(&account_nonce), Error::InvalidNonce);
            }
            holder.balances.insert(curr_token_id);
            holder.nonce += 1;
            Ok(holder.nonce)
        })?;
    state.curr_token_id = TokenIdU64(curr_token_id.0 + 1);

    // logs
    logger.log(&Event::Cis2(Cis2Event::TokenMetadata(TokenMetadataEvent {
        token_id: curr_token_id,
        metadata_url,
    })))?;
    logger.log(&Event::Cis2(Cis2Event::Mint(MintEvent {
        token_id: curr_token_id,
        amount: 1.into(),
        owner,
    })))?;
    logger.log(&Event::NonceUpdated(owner, nonce))?;

    Ok((curr_token_id, nonce))
}
