//! This contract works as a controller for another security token
//!
//! This contract has following state
//! * Open
//! * Success
//! * Fail
//!
//! The contract is initialized with Open State.
//! Which means a Concordium Address and transfer `Currency` tokens to this contract.
//! Representing a Promise of investment in the specified `Security Token`
//!
//! Upon receiving some amount A1 of `CurrencyToken`,
//! amount A2  of `Token` will be calculated using the specified `Rate`
//! and will be minted to the sender of `CurrencyToken`
//!
//! At Some point in future time the owner of the Contract can set the state of the this contract to `Success` Or `Fail`
//!
//! At any point in the future the investor can request to
//! * Cancel their Investment, Only if the state of the contract is `Open` or `Fail` this request will
//!     * return their invested `Currency` token
//!     * burn their `Tokens`
//! * Claim their Investment, Only if the state of the contract is `Success` this request will
//!     * mint their `Investment` token
//!     * burn their `Token`

use concordium_cis2::{
    AdditionalData, BalanceOfQuery, OnReceivingCis2Params, Receiver, TokenAmountU64, TokenIdVec,
    Transfer,
};
use concordium_protocols::concordium_cis2_ext::cis2_client;
use concordium_protocols::concordium_cis2_security::{
    cis2_security_client, Burn, FreezeParam, MintParam, TokenUId,
};
use concordium_protocols::rate::{ExchangeError, Rate};
use concordium_std::*;

pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = TokenAmountU64;
pub type CurrencyTokenAmount = TokenAmountU64;
pub type AnyTokenUId = TokenUId<TokenIdVec>;

#[derive(Serialize, SchemaType, Reject)]
pub enum Error {
    UnAuthorized,
    ParseError,
    CurrencyTokenTransfer,
    InvalidConversion,
    InvalidFundState,
    TokenMint,
    TokenFreeze,
    TokenForceBurn,
    TokenBalance,
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}
impl From<ExchangeError> for Error {
    fn from(_: ExchangeError) -> Self { Error::InvalidConversion }
}
/// Current state fo the func represented by this contract.
#[derive(Serialize, SchemaType, Clone)]
pub enum FundState {
    Open,
    Success(Receiver),
    Fail,
}

impl FundState {
    pub fn accepting_investments(&self) -> bool { matches!(self, FundState::Open) }

    pub fn can_cancel_investment(&self) -> bool {
        matches!(self, FundState::Open | FundState::Fail)
    }

    pub fn can_claim_investment(&self) -> bool { matches!(self, FundState::Success(_)) }
}

#[derive(Serialize, SchemaType)]
struct State {
    /// Token representing the security. Security token represented by `investment_token` will be minted after completion of this fund.
    /// Upon completion of this contract's fund this is is the token which will be burned.
    pub token:            AnyTokenUId,
    /// This is the token which is used to fund this contract.
    pub currency_token:   AnyTokenUId,
    /// The actual security token to be minted after successful completion of this contract's fund.
    pub investment_token: AnyTokenUId,
    /// This is the rate  which will be used to convert from `Currency` token to `Investment` Token.
    pub rate:             Rate,
    /// Current state of the fund.
    pub fund_state:       FundState,
}

#[init(
    contract = "security_mint_fund",
    error = "Error",
    parameter = "State"
)]
fn init(ctx: &InitContext, _: &mut StateBuilder) -> InitResult<State> {
    let params: State = ctx.parameter_cursor().get()?;
    Ok(params)
}

/// Parameters for `transferInvest` function.
/// This is the parameters which will be used to transfer `Currency` token to this contract.
#[derive(Serialize, SchemaType)]
pub struct TransferInvestParams {
    /// Amount of currency token to be invested in current contract's fund.
    amount: CurrencyTokenAmount,
}

#[receive(contract = "security_mint_fund", name = "transferInvest", mutable)]
fn transfer_invest(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let currency_token = host.state().currency_token.clone();
    let params: TransferInvestParams = ctx.parameter_cursor().get()?;
    cis2_client::transfer_single(host, &currency_token.contract, Transfer {
        token_id: currency_token.id,
        amount:   params.amount,
        to:       Receiver::Contract(
            ctx.self_address(),
            OwnedEntrypointName::new_unchecked("invest".into()),
        ),
        from:     ctx.sender(),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::CurrencyTokenTransfer)?;

    Ok(())
}

pub type InvestReceiveParams = OnReceivingCis2Params<TokenIdVec, CurrencyTokenAmount>;

#[receive(
    contract = "security_mint_fund",
    name = "invest",
    mutable,
    parameter = "InvestReceiveParams"
)]
fn invest(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: InvestReceiveParams = ctx.parameter_cursor().get()?;
    ensure!(
        params.amount.gt(&TokenAmountU64(0)),
        Error::InvalidConversion
    );

    let from = match params.from {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::UnAuthorized),
    };
    let currency_token = TokenUId {
        id:       params.token_id,
        contract: match ctx.sender() {
            Address::Account(_) => bail!(Error::UnAuthorized),
            Address::Contract(c) => c,
        },
    };

    let (amount, token) = {
        let state = host.state();
        ensure!(
            state.fund_state.accepting_investments(),
            Error::InvalidFundState
        );
        ensure!(
            state.currency_token.eq(&currency_token),
            Error::UnAuthorized
        );

        let (security_amount, un_converted_currency_amount) =
            state.rate.convert(&params.amount.0)?;
        ensure_eq!(un_converted_currency_amount, 0, Error::InvalidConversion);
        ensure!(security_amount > 0, Error::InvalidConversion);
        let security_amount = TokenAmountU64(security_amount);

        (security_amount, state.token.clone())
    };

    cis2_security_client::mint_single(host, &token.contract, token.id.clone(), MintParam {
        address: from,
        amount,
    })
    .map_err(|_| Error::TokenMint)?;
    cis2_security_client::freeze_single(host, &token.contract, from.into(), FreezeParam {
        token_id:     token.id,
        token_amount: amount,
    })
    .map_err(|_| Error::TokenFreeze)?;
    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct CancelInvestParams {
    pub currency_amount: CurrencyTokenAmount,
}

#[receive(contract = "security_mint_fund", name = "cancelInvest", mutable)]
fn cancel_invest(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let sender = match ctx.sender() {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::UnAuthorized),
    };
    let params: CancelInvestParams = ctx.parameter_cursor().get()?;
    ensure!(
        params.currency_amount.gt(&TokenAmountU64(0)),
        Error::InvalidConversion
    );

    let (security_amount, token, currency_amount, currency_token) = {
        let state = host.state();
        ensure!(
            state.fund_state.can_cancel_investment(),
            Error::InvalidFundState
        );

        let (security_amount, un_converted_currency_amount) =
            state.rate.convert(&params.currency_amount.0)?;
        ensure_eq!(un_converted_currency_amount, 0, Error::InvalidConversion);
        ensure!(security_amount > 0, Error::InvalidConversion);
        let security_amount = TokenAmountU64(security_amount);

        (
            security_amount,
            state.token.clone(),
            params.currency_amount,
            state.currency_token.clone(),
        )
    };

    cis2_security_client::force_burn_single(host, &token.contract, Burn {
        token_id: token.id,
        amount:   security_amount,
        owner:    sender.into(),
    })
    .map_err(|_| Error::TokenForceBurn)?;
    cis2_client::transfer_single(host, &currency_token.contract, Transfer {
        token_id: currency_token.id,
        amount:   currency_amount,
        from:     ctx.self_address().into(),
        to:       sender.into(),
        data:     AdditionalData::empty(),
    })
    .map_err(|_| Error::CurrencyTokenTransfer)?;

    Ok(())
}

#[receive(
    contract = "security_mint_fund",
    name = "updateFundState",
    mutable,
    parameter = "FundState"
)]
fn update_fund_state(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let params: FundState = ctx.parameter_cursor().get()?;
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::UnAuthorized
    );
    let fund_state = match (&host.state().fund_state, params) {
        (FundState::Open, FundState::Open) => bail!(Error::InvalidFundState),
        (FundState::Open, fund_state) => fund_state,
        (FundState::Fail, _) => bail!(Error::InvalidFundState),
        (FundState::Success(_), _) => bail!(Error::InvalidFundState),
    };
    host.state_mut().fund_state = fund_state.clone();

    if let FundState::Success(funds_receiver) = fund_state {
        let currency_token = host.state().currency_token.clone();
        let balance_of_currency: CurrencyTokenAmount =
            cis2_client::balance_of_single(host, &currency_token.contract, BalanceOfQuery {
                token_id: currency_token.id.clone(),
                address:  ctx.self_address().into(),
            })
            .map_err(|_| Error::TokenBalance)?;
        cis2_client::transfer_single(host, &currency_token.contract, Transfer {
            token_id: currency_token.id,
            amount:   balance_of_currency,
            from:     ctx.self_address().into(),
            to:       funds_receiver,
            data:     AdditionalData::empty(),
        })
        .map_err(|_| Error::CurrencyTokenTransfer)?;
    }
    Ok(())
}

#[receive(contract = "security_mint_fund", name = "claimInvestment", mutable)]
fn claim_investment(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let sender = match ctx.sender() {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::UnAuthorized),
    };

    let state = host.state();
    let token = state.token.clone();
    let investment_token = state.investment_token.clone();
    ensure!(
        state.fund_state.can_claim_investment(),
        Error::InvalidFundState
    );

    let security_amount: TokenAmount =
        cis2_client::balance_of_single(host, &token.contract, BalanceOfQuery {
            token_id: token.id.clone(),
            address:  sender.into(),
        })
        .map_err(|_| Error::TokenBalance)?;
    cis2_security_client::force_burn_single(host, &token.contract, Burn {
        token_id: token.id,
        amount:   security_amount,
        owner:    sender.into(),
    })
    .map_err(|_| Error::TokenForceBurn)?;
    cis2_security_client::mint_single(
        host,
        &investment_token.contract,
        investment_token.id,
        MintParam {
            address: sender,
            amount:  security_amount,
        },
    )
    .map_err(|_| Error::TokenMint)?;
    Ok(())
}
