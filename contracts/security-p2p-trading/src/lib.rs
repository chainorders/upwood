use concordium_cis2::{AdditionalData, TokenAmountU64, TokenIdU64, TokenIdUnit, Transfer};
use concordium_protocols::concordium_cis2_ext::cis2_client::Cis2Client;
use concordium_protocols::concordium_cis2_ext::ContractMetadataUrl;
use concordium_protocols::concordium_cis2_security::cis2_security_client::Cis2SecurityClient;
use concordium_protocols::concordium_cis2_security::{
    AddTokenParams, AgentWithRoles, MintParam, TokenAmountSecurity, TokenUId,
};
use concordium_protocols::rate::Rate;
use concordium_std::ops::DerefMut;
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
pub struct ExchangeEvent {
    pub token_contract:  ContractAddress,
    pub token_id:        SecurityTokenId,
    pub seller:          AccountAddress,
    pub buyer:           AccountAddress,
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
    Exchanged(ExchangeEvent),
    MarketRemoved(ContractAddress),
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
    InvalidRate,
    MintMarketNotStarted,
    AddToken,
    TokenMint,
    InvalidMarketType,
    MarketTokenLimitExceeded,
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
pub struct AddTransferMarketParam {
    pub token_id:            SecurityTokenId,
    pub liquidity_provider:  AccountAddress,
    /// The rate at which the liquidity provider buys the tokens.
    /// This is the rate at which sellers can sell their tokens to the liquidity provider.
    pub buy_rate:            Rate,
    /// The rate at which the liquidity provider sells the tokens.
    /// This is the rate at which buyers can buy tokens from the liquidity provider.
    pub sell_rate:           Rate,
    pub max_token_amount:    SecurityTokenAmount,
    pub max_currency_amount: CurrencyTokenAmount,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    pub currency_token: CurrencyTokenAddress,
    pub agents:         StateMap<Address, StateSet<AgentRole, S>, S>,
    pub markets:        StateMap<ContractAddress, Market, S>,
}

impl State {
    pub fn has_agent(&self, agent: &Address, role: AgentRole) -> bool {
        self.agents
            .get(agent)
            .map_or(false, |roles| roles.contains(&role))
    }
}

#[derive(Serialize, Clone, SchemaType)]
pub enum Market {
    Mint(MintMarket),
    Transfer(TransferMarket),
}

impl Market {
    pub fn new(market: &AddMarketParam) -> Self {
        match market {
            AddMarketParam::Mint(m) => Market::Mint(MintMarket {
                token_id:           m.token_id,
                rate:               m.rate,
                token_metadata_url: m.token_metadata_url.clone(),
                liquidity_provider: m.liquidity_provider,
                max_token_amount:   m.max_token_amount,
                token_amount:       0u64.into(),
            }),
            AddMarketParam::Transfer(m) => Market::Transfer(TransferMarket {
                token_id:            m.token_id,
                liquidity_provider:  m.liquidity_provider,
                buy_rate:            m.buy_rate,
                sell_rate:           m.sell_rate,
                max_token_amount:    m.max_token_amount,
                max_currency_amount: m.max_currency_amount,
                token_amount:        0u64.into(),
                currency_amount:     0u64.into(),
            }),
        }
    }
}

#[derive(Serialize, SchemaType, Clone)]
pub struct MintMarket {
    pub token_id:           TokenIdCalculation,
    pub rate:               Rate,
    pub token_metadata_url: ContractMetadataUrl,
    pub liquidity_provider: AccountAddress,
    pub max_token_amount:   SecurityTokenAmount,
    pub token_amount:       SecurityTokenAmount,
}

impl MintMarket {
    pub fn calculate_token_id(&self, now: Timestamp) -> Option<TokenIdU64> {
        let token_id = now
            .duration_since(self.token_id.start)
            .map(|d| d.millis() / self.token_id.diff_millis)?;
        Some(TokenIdU64(token_id))
    }
}

#[derive(Serialize, SchemaType, Clone)]
pub struct TransferMarket {
    pub token_id:            SecurityTokenId,
    pub liquidity_provider:  AccountAddress,
    /// The rate at which the liquidity provider buys the tokens.
    /// This is the rate at which sellers can sell their tokens to the liquidity provider.
    pub buy_rate:            Rate,
    /// The rate at which the liquidity provider sells the tokens.
    /// This is the rate at which buyers can buy tokens from the liquidity provider.
    pub sell_rate:           Rate,
    pub max_token_amount:    SecurityTokenAmount,
    pub max_currency_amount: CurrencyTokenAmount,
    pub token_amount:        SecurityTokenAmount,
    pub currency_amount:     CurrencyTokenAmount,
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
    pub token_contract: ContractAddress,
    pub market:         AddMarketParam,
}

#[derive(Serialize, SchemaType, Debug, Clone)]
pub enum AddMarketParam {
    Mint(AddMintMarketParam),
    Transfer(AddTransferMarketParam),
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
    let params: AddMarketParams = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        state.has_agent(&ctx.sender(), AgentRole::AddMarket),
        Error::Unauthorized
    );

    let existing = state
        .markets
        .insert(params.token_contract, Market::new(&params.market));
    if existing.is_some() {
        logger.log(&Event::MarketRemoved(params.token_contract))?;
    }
    logger.log(&Event::MarketAdded(params))?;
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
    let market: ContractAddress = ctx.parameter_cursor().get()?;
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
    parameter = "ContractAddress"
)]
fn get_market(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Market> {
    let contract: ContractAddress = ctx.parameter_cursor().get()?;
    let state = host.state();
    let market = state.markets.get(&contract).ok_or(Error::InvalidMarket)?;
    Ok(market.clone())
}

#[derive(Serialize, SchemaType)]
pub struct ExchangeParams {
    pub contract: ContractAddress,
    pub amount:   SecurityTokenAmount,
    pub rate:     Rate,
}

#[receive(
    contract = "security_p2p_trading",
    name = "sell",
    mutable,
    parameter = "ExchangeParams",
    error = "Error",
    enable_logger
)]
pub fn sell(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: ExchangeParams = ctx.parameter_cursor().get()?;
    let seller = match ctx.sender() {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };
    let (currency_token, currency_amount, liquidity_provider, token_id) = {
        let state = host.state_mut();
        let mut market = state
            .markets
            .get_mut(&params.contract)
            .ok_or(Error::InvalidMarket)?;
        let market = match market.deref_mut() {
            Market::Mint(_) => bail!(Error::InvalidMarketType),
            Market::Transfer(m) => m,
        };
        ensure!(market.buy_rate.eq(&params.rate), Error::InvalidRate);
        let (currency_amount, _) = market
            .buy_rate
            .convert_token_amount_with_rem(&params.amount)
            .map_err(|_| Error::InvalidConversion)?;
        market.currency_amount += currency_amount;
        ensure!(
            market.currency_amount <= market.max_currency_amount,
            Error::MarketTokenLimitExceeded
        );
        (
            state.currency_token,
            currency_amount,
            market.liquidity_provider,
            market.token_id,
        )
    };

    // Transfer currency from liquidity provider to seller
    host.invoke_transfer_single(&currency_token.contract, Transfer {
        amount:   currency_amount,
        token_id: currency_token.id,
        from:     liquidity_provider.into(),
        to:       seller.into(),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::CurrencyTransfer)?;
    // Transfer tokens from seller to liquidity provider
    host.invoke_transfer_single(&params.contract, Transfer {
        amount: params.amount,
        token_id,
        from: seller.into(),
        to: liquidity_provider.into(),
        data: AdditionalData::empty(),
    })
    .map_err(|_| Error::TokenTransfer)?;

    // Log the sell event.
    logger.log(&Event::Exchanged(ExchangeEvent {
        token_contract: params.contract,
        token_id,
        seller,
        buyer: liquidity_provider,
        token_amount: params.amount,
        rate: params.rate,
        currency_amount,
    }))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "buy",
    mutable,
    parameter = "ExchangeParams",
    error = "Error",
    enable_logger
)]
pub fn buy(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: ExchangeParams = ctx.parameter_cursor().get()?;
    let buyer = match ctx.sender() {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };
    let (currency_token, currency_amount, liquidity_provider, token_id) = {
        let state = host.state_mut();
        let mut market = state
            .markets
            .get_mut(&params.contract)
            .ok_or(Error::InvalidMarket)?;
        let market = match market.deref_mut() {
            Market::Mint(_) => bail!(Error::InvalidMarketType),
            Market::Transfer(m) => m,
        };
        market.token_amount += params.amount;
        ensure!(
            market.token_amount <= market.max_token_amount,
            Error::MarketTokenLimitExceeded
        );
        ensure!(market.sell_rate.eq(&params.rate), Error::InvalidRate);
        let (currency_amount, _) = market
            .sell_rate
            .convert_token_amount_with_rem(&params.amount)
            .map_err(|_| Error::InvalidConversion)?;
        (
            state.currency_token,
            currency_amount,
            market.liquidity_provider,
            market.token_id,
        )
    };

    // Transfer tokens from currency from buyer to liquidity provider
    host.invoke_transfer_single(&currency_token.contract, Transfer {
        amount:   currency_amount,
        token_id: currency_token.id,
        from:     buyer.into(),
        to:       liquidity_provider.into(),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::CurrencyTransfer)?;
    // Transfer Tokens from liquidity provider to buyer
    host.invoke_transfer_single(&params.contract, Transfer {
        amount: params.amount,
        token_id,
        from: liquidity_provider.into(),
        to: buyer.into(),
        data: AdditionalData::empty(),
    })
    .map_err(|_| Error::TokenTransfer)?;

    // Log the buy event.
    logger.log(&Event::Exchanged(ExchangeEvent {
        token_contract: params.contract,
        token_id,
        seller: liquidity_provider,
        buyer,
        token_amount: params.amount,
        rate: params.rate,
        currency_amount,
    }))?;
    Ok(())
}

#[derive(Serialize, SchemaType, Debug, Clone)]
pub struct AddMintMarketParam {
    pub token_id:           TokenIdCalculation,
    pub rate:               Rate,
    pub token_metadata_url: ContractMetadataUrl,
    pub liquidity_provider: AccountAddress,
    pub max_token_amount:   SecurityTokenAmount,
}

#[derive(Serialize, SchemaType, Debug, Clone, Copy)]
pub struct TokenIdCalculation {
    pub start:       Timestamp,
    pub diff_millis: u64,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct MintParams {
    pub token_contract: ContractAddress,
    pub amount:         SecurityTokenAmount,
    pub rate:           Rate,
}

#[receive(
    contract = "security_p2p_trading",
    name = "mint",
    mutable,
    parameter = "MintParams",
    error = "Error",
    enable_logger
)]
pub fn mint(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: MintParams = ctx.parameter_cursor().get()?;
    let buyer = match ctx.sender() {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };
    let now = ctx.metadata().block_time();

    let (currency_token, currency_amount, token_id, liquidity_provider, token_metadata_url) = {
        let state = host.state_mut();
        let mut market = state
            .markets
            .get_mut(&params.token_contract)
            .ok_or(Error::InvalidMarket)?;
        let market = match market.deref_mut() {
            Market::Mint(m) => m,
            Market::Transfer(_) => bail!(Error::InvalidMarketType),
        };
        ensure!(market.rate.eq(&params.rate), Error::InvalidRate);
        let (currency_amount, _) = market
            .rate
            .convert_token_amount_with_rem(&params.amount)
            .map_err(|_| Error::InvalidConversion)?;
        market.token_amount += params.amount;
        ensure!(
            market.token_amount <= market.max_token_amount,
            Error::MarketTokenLimitExceeded
        );

        (
            state.currency_token,
            currency_amount,
            market
                .calculate_token_id(now)
                .ok_or(Error::MintMarketNotStarted)?,
            market.liquidity_provider,
            market.token_metadata_url.clone(),
        )
    };

    if host
        .invoke_token_metadata_single(&params.token_contract, token_id)
        .is_err()
    {
        host.invoke_add_token(&params.token_contract, &AddTokenParams {
            token_id,
            token_metadata: token_metadata_url,
        })
        .map_err(|_| Error::AddToken)?;
    }

    host.invoke_transfer_single(&currency_token.contract, Transfer {
        amount:   currency_amount,
        token_id: currency_token.id,
        from:     buyer.into(),
        to:       liquidity_provider.into(),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::CurrencyTransfer)?;
    host.invoke_mint_single(&params.token_contract, token_id, MintParam {
        address: buyer.into(),
        amount:  TokenAmountSecurity {
            un_frozen: params.amount,
            ..Default::default()
        },
    })
    .map_err(|_| Error::TokenMint)?;
    logger.log(&Event::Exchanged(ExchangeEvent {
        token_contract: params.token_contract,
        token_id,
        seller: liquidity_provider,
        buyer,
        token_amount: params.amount,
        rate: params.rate,
        currency_amount,
    }))?;
    Ok(())
}
