use concordium_cis2::{
    AdditionalData, OnReceivingCis2DataParams, Receiver, TokenAmountU64, TokenIdVec, Transfer,
};
use concordium_protocols::concordium_cis2_ext::cis2_client::{self, CisClientError};
use concordium_protocols::concordium_cis2_security::TokenUId;
use concordium_protocols::rate::Rate;
use concordium_rwa_utils::conversions::to_additional_data;
use concordium_std::ops::SubAssign;
use concordium_std::*;

pub type TokenAmount = TokenAmountU64;
pub type AnyTokenUId = TokenUId<TokenIdVec>;

#[derive(Serialize, SchemaType)]
pub struct SellEvent {
    pub from:   AccountAddress,
    pub amount: TokenAmount,
    pub rate:   Rate,
}

#[derive(Serialize, SchemaType)]
pub struct SellCancelledEvent {
    pub from:   AccountAddress,
    pub amount: TokenAmount,
}

#[derive(Serialize, SchemaType)]
pub struct Exchange {
    pub payer:       AccountAddress,
    pub pay_amount:  TokenAmount,
    pub sell_amount: TokenAmount,
    pub seller:      AccountAddress,
}

#[derive(Serialize, SchemaType)]
pub enum Event {
    Sell(SellEvent),
    SellCancelled(SellCancelledEvent),
    Exchange(Exchange),
}

#[derive(Serial, Reject, SchemaType)]
pub enum Error {
    ParseError,
    Unauthorized,
    Cis2CallError,
    InvalidToken,
    SellPositionExists,
    SellPositionMissing,
    InvalidConversion,
    InvalidAmount,
    LogError,
}
impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}
impl From<CisClientError> for Error {
    fn from(_: CisClientError) -> Self { Error::Cis2CallError }
}
impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}
pub type ContractResult<T> = Result<T, Error>;

#[derive(Serialize, Clone, SchemaType, PartialEq, Debug)]
pub struct Deposit {
    pub amount: TokenAmount,
    pub rate:   Rate,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    token:    AnyTokenUId,
    currency: AnyTokenUId,
    deposits: StateMap<AccountAddress, Deposit, S>,
}

#[derive(Serialize, SchemaType)]
pub struct InitParam {
    pub token:    AnyTokenUId,
    pub currency: AnyTokenUId,
}

#[init(
    contract = "security_p2p_trading",
    event = "Event",
    error = "Error",
    parameter = "InitParam"
)]
pub fn init(ctx: &InitContext, state_builder: &mut StateBuilder) -> InitResult<State> {
    let params: InitParam = ctx.parameter_cursor().get()?;
    Ok(State {
        token:    params.token,
        currency: params.currency,
        deposits: state_builder.new_map(),
    })
}

#[derive(Serialize, SchemaType)]
pub struct TransferSellParams {
    pub amount: TokenAmount,
    pub rate:   Rate,
}

#[receive(
    contract = "security_p2p_trading",
    name = "transferSell",
    mutable,
    parameter = "TransferSellParams",
    error = "Error"
)]
pub fn transfer_sell(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: TransferSellParams = ctx.parameter_cursor().get()?;
    let token = host.state().token.clone();
    cis2_client::transfer_single(host, &token.contract, Transfer {
        token_id: token.id,
        amount:   params.amount,
        from:     ctx.sender(),
        to:       Receiver::from_contract(
            ctx.self_address(),
            OwnedEntrypointName::new_unchecked("sell".into()),
        ),
        data:     to_additional_data(params.rate).map_err(|_| Error::ParseError)?,
    })?;
    Ok(())
}

pub type SellReceiveParams = OnReceivingCis2DataParams<TokenIdVec, TokenAmount, Rate>;

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
    let state = host.state();

    let incoming_token = TokenUId {
        id:       params.token_id.clone(),
        contract: match ctx.sender() {
            Address::Account(_) => bail!(Error::Unauthorized),
            Address::Contract(c) => c,
        },
    };
    ensure!(incoming_token.eq(&state.token), Error::InvalidToken);
    let from = match params.from {
        Address::Account(from) => from,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };
    host.state_mut()
        .deposits
        .entry(from)
        .vacant_or(Error::SellPositionExists)?
        .insert(Deposit {
            amount: params.amount,
            rate:   params.data,
        });
    logger.log(&Event::Sell(SellEvent {
        from,
        amount: params.amount,
        rate: params.data,
    }))?;
    Ok(())
}

#[receive(
    contract = "security_p2p_trading",
    name = "cancelSell",
    mutable,
    error = "Error",
    enable_logger
)]
pub fn cancel_sell(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let sender = match ctx.sender() {
        Address::Account(from) => from,
        Address::Contract(_) => bail!(Error::Unauthorized),
    };
    let deposit_token = host.state().token.clone();
    let deposits = host
        .state_mut()
        .deposits
        .remove_and_get(&sender)
        .ok_or(Error::SellPositionMissing)?;
    cis2_client::transfer_single(host, &deposit_token.contract, Transfer {
        token_id: deposit_token.id,
        amount:   deposits.amount,
        from:     ctx.self_address().into(),
        to:       Receiver::Account(sender),
        data:     AdditionalData::empty(),
    })?;
    logger.log(&Event::SellCancelled(SellCancelledEvent {
        from:   sender,
        amount: deposits.amount,
    }))?;
    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct ForceCancelSellParams {
    pub from: AccountAddress,
    pub to:   AccountAddress,
}

#[receive(
    contract = "security_p2p_trading",
    name = "forceCancelSell",
    mutable,
    parameter = "ForceCancelSellParams",
    error = "Error",
    enable_logger
)]
pub fn force_cancel_sell(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let ForceCancelSellParams { from, to } = ctx.parameter_cursor().get()?;
    let deposit_token = host.state().token.clone();
    let deposits = host
        .state_mut()
        .deposits
        .remove_and_get(&from)
        .ok_or(Error::SellPositionMissing)?;
    cis2_client::transfer_single(host, &deposit_token.contract, Transfer {
        token_id: deposit_token.id,
        amount:   deposits.amount,
        from:     ctx.self_address().into(),
        to:       Receiver::Account(to),
        data:     AdditionalData::empty(),
    })?;
    logger.log(&Event::SellCancelled(SellCancelledEvent {
        from,
        amount: deposits.amount,
    }))?;
    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct TransferExchangeParams {
    pub pay: TokenAmount,
    pub get: ExchangeParams,
}

#[derive(Serialize, SchemaType)]
pub struct ExchangeParams {
    pub from: AccountAddress,
    pub rate: Rate,
}

#[receive(
    contract = "security_p2p_trading",
    name = "transferExchange",
    mutable,
    parameter = "TransferExchangeParams",
    error = "Error"
)]
pub fn transfer_exchange(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: TransferExchangeParams = ctx.parameter_cursor().get()?;
    let currency = host.state().currency.clone();
    cis2_client::transfer_single(host, &currency.contract, Transfer {
        token_id: currency.id,
        amount:   params.pay,
        from:     ctx.sender(),
        to:       Receiver::from_contract(
            ctx.self_address(),
            OwnedEntrypointName::new_unchecked("exchange".into()),
        ),
        data:     to_additional_data(params.get).map_err(|_| Error::ParseError)?,
    })?;
    Ok(())
}

pub type ExchangeReceiveParams = OnReceivingCis2DataParams<TokenIdVec, TokenAmount, ExchangeParams>;

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
        amount: pay_amount,
        from: payer,
        token_id: currency_token_id,
        data: ExchangeParams { from: seller, rate },
    } = ctx.parameter_cursor().get()?;
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

    let (token, pay_amount, sell_amount) = {
        let state = host.state_mut();
        ensure!(currency.eq(&state.currency), Error::InvalidToken);

        let mut deposit = state
            .deposits
            .get_mut(&seller)
            .ok_or(Error::SellPositionMissing)?;
        ensure!(deposit.rate.eq(&rate), Error::SellPositionMissing);

        let (sell_amount, un_converted_pay_amount) = deposit
            .rate
            .convert(&pay_amount.0)
            .map_err(|_| Error::InvalidConversion)?;

        let sell_amount = TokenAmountU64(sell_amount);
        ensure_eq!(un_converted_pay_amount, 0, Error::InvalidAmount);
        ensure!(sell_amount.le(&deposit.amount), Error::InvalidAmount);
        ensure!(sell_amount.gt(&TokenAmountU64(0)), Error::InvalidAmount);
        deposit.amount.sub_assign(sell_amount);

        (state.token.clone(), pay_amount, sell_amount)
    };

    cis2_client::transfer_single(host, &currency.contract, Transfer {
        token_id: currency.id.clone(),
        amount:   pay_amount,
        from:     ctx.self_address().into(),
        to:       Receiver::Account(seller),
        data:     AdditionalData::empty(),
    })?;
    cis2_client::transfer_single(host, &token.contract, Transfer {
        token_id: token.id,
        amount:   sell_amount,
        from:     ctx.self_address().into(),
        to:       Receiver::Account(payer),
        data:     AdditionalData::empty(),
    })?;
    logger.log(&Event::Exchange(Exchange {
        payer,
        pay_amount,
        sell_amount,
        seller,
    }))?;

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct GetDepositParams {
    pub from: AccountAddress,
}

#[receive(
    contract = "security_p2p_trading",
    name = "getDeposit",
    parameter = "GetDepositParams",
    return_value = "Deposit"
)]
pub fn get_deposit(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Deposit> {
    let params: GetDepositParams = ctx.parameter_cursor().get()?;
    let deposit = host
        .state()
        .deposits
        .get(&params.from)
        .ok_or(Error::SellPositionMissing)?;

    Ok(deposit.clone())
}
