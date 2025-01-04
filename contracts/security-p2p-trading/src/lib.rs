use concordium_cis2::{AdditionalData, TokenAmountU64, TokenIdU64, TokenIdUnit, Transfer};
use concordium_protocols::concordium_cis2_ext::cis2_client::Cis2Client;
use concordium_protocols::concordium_cis2_security::{AgentWithRoles, TokenUId};
use concordium_protocols::rate::Rate;
use concordium_std::*;

/// The type of the Token Amount used in the EuroE. This should match the type used in the `euroe` contract.
pub type CurrencyTokenAmount = TokenAmountU64;
/// The type of the token id used in the EuroE contract. This should match the type used in the `euroe` contract.
pub type CurrencyTokenId = TokenIdUnit;
pub type CurrencyTokenAddress = TokenUId<CurrencyTokenId>;
/// The type of the Token Amount used in the contract. This should match the type used in the `security-sft-multi` contract.
pub type TokenAmount = TokenAmountU64;
/// The type of the token id used in the contract. This should match the type used in the `security-sft-multi` contract.
pub type SecurityTokenId = TokenIdU64;
pub type SecurityTokenAddress = TokenUId<SecurityTokenId>;
pub type SecurityTokenAmount = TokenAmountU64;
pub type ContractResult<T> = Result<T, Error>;

/// Represents Sell event.
/// This is the event that is emitted when a user deposits tokens to be sold.
#[derive(Serialize, SchemaType, Debug)]
pub struct SellEvent {
    pub token_contract:  ContractAddress,
    pub token_id:        SecurityTokenId,
    pub seller:          AccountAddress,
    pub token_amount:    TokenAmount,
    pub rate:            Rate,
    pub currency_amount: CurrencyTokenAmount,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum Event {
    Initialized(CurrencyTokenAddress),
    AgentAdded(AgentWithRoles<AgentRole>),
    AgentRemoved(Address),
    MarketAdded(AddMarketParams),
    Sell(SellEvent),
    MarketRemoved(SecurityTokenAddress),
}

#[derive(Serial, Reject, SchemaType)]
pub enum Error {
    ParseError,
    Unauthorized,
    InvalidConversion,
    LogError,
    AgentExists,
    InvalidMarket,
    TokenTransfer,
    CurrencyTransfer,
    InvalidSellRate,
}
impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}
impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}

#[derive(Serialize, SchemaType, Debug, Clone, Copy)]
pub enum AgentRole {
    AddMarket,
    RemoveMarket,
    Operator,
}

#[derive(Serialize, Debug, SchemaType, Clone, Copy)]
pub struct Market {
    /// Buyer
    pub buyer: AccountAddress,
    /// Rate at which the tokens are being bought.
    pub rate:  Rate,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    pub currency_token: CurrencyTokenAddress,
    pub agents:         StateMap<Address, StateSet<AgentRole, S>, S>,
    pub markets:        StateMap<SecurityTokenAddress, Market, S>,
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
    pub currency: CurrencyTokenAddress,
    pub agents:   Vec<AgentWithRoles<AgentRole>>,
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

        for agent in params.agents.iter() {
            let _ = agents.insert(agent.address, {
                let mut roles = state_builder.new_set();
                for role in agent.roles.iter() {
                    roles.insert(*role);
                }
                roles
            });
        }

        agents
    };
    let state = State {
        currency_token: params.currency,
        agents,
        markets: state_builder.new_map(),
    };

    logger.log(&Event::Initialized(params.currency))?;
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

#[derive(Serialize, SchemaType, Debug)]
pub struct AddMarketParams {
    pub token:  SecurityTokenAddress,
    pub market: Market,
}

#[receive(
    contract = "security_p2p_trading",
    name = "addMarket",
    mutable,
    parameter = "AddMarketParams",
    enable_logger
)]
fn add_market(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let market: AddMarketParams = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        state.has_agent(&ctx.sender(), AgentRole::AddMarket),
        Error::Unauthorized
    );

    let existing = state.markets.insert(market.token, market.market);
    if existing.is_some() {
        logger.log(&Event::MarketRemoved(market.token))?;
    }
    logger.log(&Event::MarketAdded(market))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "removeMarket",
    mutable,
    parameter = "SecurityTokenAddress",
    enable_logger
)]
fn remove_market(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let market: SecurityTokenAddress = ctx.parameter_cursor().get()?;
    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_agent(&ctx.sender(), AgentRole::RemoveMarket),
        Error::Unauthorized
    );
    state
        .markets
        .remove_and_get(&market)
        .ok_or(Error::InvalidMarket)?;
    logger.log(&Event::MarketRemoved(market))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "getMarket",
    parameter = "SecurityTokenAddress"
)]
fn get_market(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Market> {
    let token: SecurityTokenAddress = ctx.parameter_cursor().get()?;
    let state = host.state();
    let market = state.markets.get(&token).ok_or(Error::InvalidMarket)?;
    Ok(*market)
}

#[derive(Serialize, SchemaType)]
pub struct SellParams {
    pub token:  SecurityTokenAddress,
    pub amount: SecurityTokenAmount,
    pub rate:   Rate,
}

#[receive(
    contract = "security_p2p_trading",
    name = "sell",
    mutable,
    parameter = "SellParams",
    error = "Error",
    enable_logger
)]
pub fn sell(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: SellParams = ctx.parameter_cursor().get()?;
    let seller = match ctx.sender() {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };
    let (currency_token, currency_amount, buyer) = {
        let state = host.state_mut();
        let market = state
            .markets
            .get_mut(&params.token)
            .ok_or(Error::InvalidMarket)?;
        ensure!(market.rate.eq(&params.rate), Error::InvalidSellRate);
        let (currency_amount, _) = market
            .rate
            .convert_token_amount_with_rem(&params.amount)
            .map_err(|_| Error::InvalidConversion)?;
        (state.currency_token, currency_amount, market.buyer)
    };

    // Transfer the tokens to the buyer and the currency to the seller.
    host.invoke_transfer_single(&currency_token.contract, Transfer {
        amount:   currency_amount,
        token_id: currency_token.id,
        from:     buyer.into(),
        to:       seller.into(),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::CurrencyTransfer)?;
    host.invoke_transfer_single(&params.token.contract, Transfer {
        amount:   params.amount,
        token_id: params.token.id,
        from:     seller.into(),
        to:       buyer.into(),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::TokenTransfer)?;

    // Log the sell event.
    logger.log(&Event::Sell(SellEvent {
        token_contract: params.token.contract,
        token_id: params.token.id,
        seller,
        token_amount: params.amount,
        rate: params.rate,
        currency_amount,
    }))?;
    Ok(())
}
