use concordium_cis2::{
    AdditionalData, OnReceivingCis2DataParams, Receiver, TokenAmountU64, TokenIdVec, Transfer,
};
use concordium_protocols::concordium_cis2_ext::cis2_client::Cis2Client;
use concordium_protocols::concordium_cis2_ext::ToAdditionalData;
use concordium_protocols::concordium_cis2_security::{AgentWithRoles, TokenUId};
use concordium_protocols::rate::Rate;
use concordium_std::*;

pub type CurrencyTokenAmount = TokenAmountU64;
pub type TokenAmount = TokenAmountU64;
pub type AnyTokenUId = TokenUId<TokenIdVec>;
pub type ContractResult<T> = Result<T, Error>;

/// Represents Sell event.
/// This is the event that is emitted when a user deposits tokens to be sold.
#[derive(Serialize, SchemaType, Debug)]
pub struct SellEvent {
    pub market:   ContractAddress,
    pub token_id: TokenIdVec,
    /// The address of the user who deposited the tokens.
    pub from:     AccountAddress,
    /// The amount of tokens that were deposited.
    pub amount:   TokenAmount,
    /// The rate at which a buyer can convert deposited tokens to currency token.
    pub rate:     Rate,
}

/// Represents SellCancelled event.
/// This is the event that is emitted when a user cancels a sell position.
#[derive(Serialize, SchemaType, Debug)]
pub struct SellCancelledEvent {
    pub market:   ContractAddress,
    pub token_id: TokenIdVec,
    // The address of the user who cancelled the sell position.
    pub from:     AccountAddress,
    // The amount of tokens that were returned to the user.
    pub amount:   TokenAmount,
}

/// Represents Exchange event.
/// This event is emitted when a user exchanges tokens with another user.
#[derive(Serialize, SchemaType, Debug)]
pub struct ExchangeEvent {
    pub payer:       AccountAddress,
    pub pay_amount:  CurrencyTokenAmount,
    pub market:      ContractAddress,
    pub token_id:    TokenIdVec,
    pub sell_amount: TokenAmount,
    pub seller:      AccountAddress,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum Event {
    Initialized(AnyTokenUId),
    AgentAdded(AgentWithRoles<AgentRole>),
    AgentRemoved(Address),
    MarketAdded(ContractAddress),
    Sell(SellEvent),
    SellCancelled(SellCancelledEvent),
    Exchange(ExchangeEvent),
    MarketRemoved(ContractAddress),
}

#[derive(Serial, Reject, SchemaType)]
pub enum Error {
    ParseError,
    Unauthorized,
    SellPositionExists,
    SellPositionMissing,
    InvalidConversion,
    InvalidAmount,
    LogError,
    AgentExists,
    InvalidMarket,
    InvalidCurrency,
    MarketInUse,
    TokenTransfer,
    CurrencyTransfer,
}
impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}
impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}

#[derive(Serialize, SchemaType, Clone, PartialEq, Eq, Debug)]
pub struct Deposit {
    pub amount: TokenAmount,
    pub rate:   Rate,
}

#[derive(Serialize, SchemaType, Debug, Clone, Copy)]
pub enum AgentRole {
    AddMarket,
    RemoveMarket,
    Operator,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    pub currency_token: AnyTokenUId,
    pub agents:         StateMap<Address, StateSet<AgentRole, S>, S>,
    pub markets: StateMap<ContractAddress, StateMap<(AccountAddress, TokenIdVec), Deposit, S>, S>,
}

impl State {
    pub fn has_agent(&self, agent: &Address, role: AgentRole) -> bool {
        self.agents
            .get(agent)
            .map_or(false, |roles| roles.contains(&role))
    }
}

/// Initialization parameters for the contract.
#[derive(Serialize, SchemaType, Clone, Debug)]
pub struct InitParam {
    /// The token that is being used to pay for the tokens being sold.
    pub currency: AnyTokenUId,
}

#[init(
    contract = "security_p2p_trading",
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
    let agents = {
        let mut agents = state_builder.new_map();
        // Insert owner as agent
        let _ = agents.insert(ctx.init_origin().into(), {
            let mut roles = state_builder.new_set();
            roles.insert(AgentRole::AddMarket);
            roles.insert(AgentRole::RemoveMarket);
            roles.insert(AgentRole::Operator);
            roles
        });
        agents
    };
    let state = State {
        currency_token: params.currency.clone(),
        agents,
        markets: state_builder.new_map(),
    };
    logger.log(&Event::Initialized(params.currency.clone()))?;
    for agent in state.agents.iter() {
        logger.log(&Event::AgentAdded(AgentWithRoles {
            address: *agent.0,
            roles:   agent.1.iter().map(|r| *r).collect(),
        }))?;
    }
    Ok(state)
}

#[receive(
    contract = "security_p2p_trading",
    name = "addAgent",
    mutable,
    parameter = "AgentWithRoles<AgentRole>",
    enable_logger
)]
fn add_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let agent: AgentWithRoles<AgentRole> = ctx.parameter_cursor().get()?;
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );

    let (state, state_builder) = host.state_and_builder();
    let roles = {
        let mut roles_state = state_builder.new_set();
        for role in agent.roles.iter() {
            roles_state.insert(*role);
        }
        roles_state
    };
    state
        .agents
        .entry(agent.address)
        .vacant_or(Error::AgentExists)?
        .insert(roles);
    logger.log(&Event::AgentAdded(agent))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "removeAgent",
    mutable,
    parameter = "Address",
    enable_logger
)]
fn remove_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let agent: Address = ctx.parameter_cursor().get()?;
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let (state, _) = host.state_and_builder();
    state.agents.remove(&agent);
    logger.log(&Event::AgentRemoved(agent))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "addMarket",
    mutable,
    parameter = "ContractAddress",
    enable_logger
)]
fn add_market(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let market: ContractAddress = ctx.parameter_cursor().get()?;
    let (state, state_builder) = host.state_and_builder();
    ensure!(
        state.has_agent(&ctx.sender(), AgentRole::AddMarket),
        Error::Unauthorized
    );
    state
        .markets
        .entry(market)
        .vacant_or(Error::AgentExists)?
        .insert(state_builder.new_map());
    logger.log(&Event::MarketAdded(market))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "removeMarket",
    mutable,
    parameter = "ContractAddress",
    enable_logger
)]
fn remove_market(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let market_contract: ContractAddress = ctx.parameter_cursor().get()?;
    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_agent(&ctx.sender(), AgentRole::RemoveMarket),
        Error::Unauthorized
    );
    let market = state
        .markets
        .remove_and_get(&market_contract)
        .ok_or(Error::InvalidMarket)?;
    ensure!(market.is_empty(), Error::MarketInUse);
    logger.log(&Event::MarketRemoved(market_contract))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "marketInUse",
    parameter = "ContractAddress"
)]
pub fn market_in_use(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let market_contract: ContractAddress = ctx.parameter_cursor().get()?;
    let market = host
        .state()
        .markets
        .get(&market_contract)
        .ok_or(Error::InvalidMarket)?;
    Ok(!market.is_empty())
}

#[derive(Serialize, SchemaType)]
pub struct SellPositionOfParams {
    pub market:   ContractAddress,
    pub token_id: TokenIdVec,
    pub seller:   AccountAddress,
}

#[receive(
    contract = "security_p2p_trading",
    name = "sellPositionOf",
    parameter = "SellPositionOfParams"
)]
pub fn sell_position_of(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Deposit> {
    let params: SellPositionOfParams = ctx.parameter_cursor().get()?;
    let deposit = host
        .state()
        .markets
        .get(&params.market)
        .ok_or(Error::InvalidMarket)?
        .get(&(params.seller, params.token_id))
        .ok_or(Error::SellPositionMissing)?
        .clone();
    Ok(deposit)
}

/// Represents a sell position.
#[derive(Serialize, SchemaType)]
pub struct TransferSellParams {
    /// The contract address of the token being sold / security token contract address.
    pub market:   ContractAddress,
    /// The token id of the token being sold / security token id.
    pub token_id: TokenIdVec,
    /// The amount of tokens that are being sold.
    pub amount:   TokenAmount,
    /// The rate at which the tokens are being sold.
    pub rate:     Rate,
}

/// This function is called when a user wants to sell tokens.
/// To be able to call this function the user must have added current contract as an operator in the token contract for the token which is to be sold.
/// The function will transfer the tokens to be sold from the user to the current contract.
/// Tokens are received in the `sell` entrypoint.
#[receive(
    contract = "security_p2p_trading",
    name = "transferSell",
    mutable,
    parameter = "TransferSellParams",
    error = "Error"
)]
pub fn transfer_sell(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: TransferSellParams = ctx.parameter_cursor().get()?;
    let to = Receiver::from_contract(
        ctx.self_address(),
        OwnedEntrypointName::new_unchecked("sell".into()),
    );
    let data = params.rate.to_additional_data().ok_or(Error::ParseError)?;
    host.invoke_transfer_single(&params.market, Transfer {
        token_id: params.token_id,
        amount: params.amount,
        from: ctx.sender(),
        to,
        data,
    })
    .map_err(|_| Error::TokenTransfer)?;
    Ok(())
}

pub type SellReceiveParams = OnReceivingCis2DataParams<TokenIdVec, CurrencyTokenAmount, Rate>;

/// This function is called when a user wants to sell tokens.
/// This function can only be called by a smart contract.
/// In order to call this function the token holder should transfer the tokens to be sold to the current contract and token contract should call this function.
/// Current contract will only allow trusted token contracts to call this function.
#[receive(
    contract = "security_p2p_trading",
    name = "sell",
    mutable,
    parameter = "SellReceiveParams",
    error = "Error",
    enable_logger
)]
pub fn sell(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: SellReceiveParams = ctx.parameter_cursor().get()?;
    let market_contract = match ctx.sender() {
        Address::Account(_) => bail!(Error::Unauthorized),
        Address::Contract(from) => from,
    };
    let from = match params.from {
        Address::Account(from) => from,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };

    let mut market = host
        .state_mut()
        .markets
        .get_mut(&market_contract)
        .ok_or(Error::InvalidMarket)?;
    market
        .entry((from, params.token_id.clone()))
        .vacant_or(Error::SellPositionExists)?
        .insert(Deposit {
            amount: params.amount,
            rate:   params.data,
        });

    logger.log(&Event::Sell(SellEvent {
        from,
        amount: params.amount,
        rate: params.data,
        token_id: params.token_id,
        market: market_contract,
    }))?;
    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct CancelSellParams {
    pub market: ContractAddress,
    pub params: Vec<CancelSellParam>,
}

#[derive(Serialize, SchemaType)]
pub struct CancelSellParam {
    pub token_id: TokenIdVec,
    pub seller:   AccountAddress,
}

/// This function should be called by the owner of the tokens being sold to cancel the sell position.
/// The function will transfer the tokens back to the user.
#[receive(
    contract = "security_p2p_trading",
    name = "cancelSell",
    parameter = "CancelSellParams",
    mutable,
    error = "Error",
    enable_logger
)]
pub fn cancel_sell(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let CancelSellParams {
        market: market_contract,
        params,
    }: CancelSellParams = ctx.parameter_cursor().get()?;
    for param in params {
        let state = host.state_mut();
        if !ctx.sender().matches_account(&param.seller) {
            ensure!(
                state.has_agent(&ctx.sender(), AgentRole::Operator),
                Error::Unauthorized
            );
        }
        let deposit_amount = {
            let mut market = state
                .markets
                .get_mut(&market_contract)
                .ok_or(Error::InvalidMarket)?;
            let deposits = market
                .get_mut()
                .remove_and_get(&(param.seller, param.token_id.clone()))
                .ok_or(Error::SellPositionMissing)?;
            deposits.amount
        };

        host.invoke_transfer_single(&market_contract, Transfer {
            token_id: param.token_id.clone(),
            amount:   deposit_amount,
            from:     ctx.self_address().into(),
            to:       Receiver::Account(param.seller),
            data:     AdditionalData::empty(),
        })
        .map_err(|_| Error::TokenTransfer)?;
        logger.log(&Event::SellCancelled(SellCancelledEvent {
            market:   market_contract,
            token_id: param.token_id,
            from:     param.seller,
            amount:   deposit_amount,
        }))?;
    }
    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct ForceCancelSellParams {
    pub from: AccountAddress,
    pub to:   AccountAddress,
}

#[derive(Serialize, SchemaType)]
pub struct TransferExchangeParams {
    pub pay: TokenAmount,
    pub get: ExchangeParams,
}

#[derive(Serialize, SchemaType)]
pub struct ExchangeParams {
    pub market:   ContractAddress,
    pub token_id: TokenIdVec,
    pub from:     AccountAddress,
    pub amount:   TokenAmount,
}

/// This function should be called by the buyer of the tracked security token.
/// The function will transfer currency token from the sender to the contract. The transfer is received by the `exchange` entrypoint.
/// To be able to call this function the sender must have the current contract added as an operator to the currency token contract.
#[receive(
    contract = "security_p2p_trading",
    name = "transferExchange",
    mutable,
    parameter = "TransferExchangeParams",
    error = "Error"
)]
pub fn transfer_exchange(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: TransferExchangeParams = ctx.parameter_cursor().get()?;
    let currency = host.state().currency_token.clone();
    host.invoke_transfer_single(&currency.contract, Transfer {
        token_id: currency.id.clone(),
        amount:   params.pay,
        from:     ctx.sender(),
        to:       Receiver::from_contract(
            ctx.self_address(),
            OwnedEntrypointName::new_unchecked("exchange".into()),
        ),
        data:     params.get.to_additional_data().ok_or(Error::ParseError)?,
    })
    .map_err(|_| Error::CurrencyTransfer)?;
    Ok(())
}

pub type ExchangeReceiveParams = OnReceivingCis2DataParams<TokenIdVec, TokenAmount, ExchangeParams>;

/// This function is called by the currency token contract when the currency token is received.
/// This would happen when the currency token holder transfers currency token to the current contract.
/// The function will calculate the amounts of the security token to be transferred to the buyer and the currency token to be transferred to the seller
/// and will settle the amounts.
#[receive(
    contract = "security_p2p_trading",
    name = "exchange",
    mutable,
    parameter = "ExchangeReceiveParams",
    error = "Error",
    enable_logger
)]
pub fn exchange(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let ExchangeReceiveParams {
        amount: pay_currency_amount,
        from: payer,
        token_id: currency_token_id,
        data:
            ExchangeParams {
                market: market_contract,
                token_id,
                from: seller,
                amount: buy_token_amount,
            },
    } = ctx.parameter_cursor().get()?;
    ensure!(
        buy_token_amount.gt(&TokenAmountU64(0)),
        Error::InvalidAmount
    );

    let currency = TokenUId {
        id:       currency_token_id,
        contract: match ctx.sender() {
            Address::Account(_) => bail!(Error::Unauthorized),
            Address::Contract(c) => c,
        },
    };
    let payer = match payer {
        Address::Account(payer) => payer,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };

    let state = host.state_mut();
    ensure!(currency.eq(&state.currency_token), Error::InvalidCurrency);
    let (pay_curr_amount, sell_token_amount) = state
        .markets
        .get_mut(&market_contract)
        .ok_or(Error::InvalidMarket)?
        .entry((seller, token_id.clone()))
        .occupied_or(Error::SellPositionMissing)?
        .try_modify(|deposit| {
            ensure!(buy_token_amount.le(&deposit.amount), Error::InvalidAmount);
            let pay_curr_amount = deposit
                .rate
                .convert_token_amount(buy_token_amount)
                .map_err(|_| Error::InvalidConversion)?;
            ensure!(
                pay_curr_amount.eq(&pay_currency_amount),
                Error::InvalidAmount
            );
            deposit.amount -= buy_token_amount;
            Ok((pay_curr_amount, buy_token_amount))
        })?;

    host.invoke_transfer_single(&market_contract, Transfer {
        token_id: token_id.clone(),
        amount:   sell_token_amount,
        from:     ctx.self_address().into(),
        to:       Receiver::Account(payer),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::TokenTransfer)?;
    host.invoke_transfer_single(&currency.contract, Transfer {
        token_id: currency.id.clone(),
        amount:   pay_curr_amount,
        from:     ctx.self_address().into(),
        to:       Receiver::Account(seller),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::CurrencyTransfer)?;
    logger.log(&Event::Exchange(ExchangeEvent {
        market: market_contract,
        token_id,
        payer,
        pay_amount: pay_curr_amount,
        sell_amount: sell_token_amount,
        seller,
    }))?;

    Ok(())
}
