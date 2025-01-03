use concordium_cis2::{OnReceivingCis2DataParams, Receiver, TokenAmountU64, TokenIdVec};
use concordium_protocols::concordium_cis2_security::{AgentWithRoles, TokenUId};
use concordium_protocols::rate::{ExchangeError, Rate};
use concordium_std::*;

pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = TokenAmountU64;
pub type CurrencyTokenAmount = TokenAmountU64;
pub type AnyTokenUId = TokenUId<TokenIdVec>;

#[derive(Serialize, SchemaType, Debug, Clone, Copy)]
pub enum AgentRole {
    AddFund,
    RemoveFund,
    UpdateFundState,
    Operator,
}

impl AgentRole {
    pub fn owner() -> Vec<Self> { vec![Self::AddFund, Self::RemoveFund, Self::UpdateFundState] }
}

#[derive(Serialize, SchemaType, Debug)]
pub struct InvestedEvent {
    pub fund_id:         FundId,
    pub investor:        AccountAddress,
    pub security_amount: TokenAmount,
    pub currency_amount: CurrencyTokenAmount,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct FundAddedEvent {
    pub fund_id:        FundId,
    pub token:          AnyTokenUId,
    pub rate:           Rate,
    pub security_token: AnyTokenUId,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum Event {
    Initialized(AnyTokenUId),
    AgentAdded(AgentWithRoles<AgentRole>),
    AgentRemoved(Address),
    FundAdded(FundAddedEvent),
    FundRemoved(FundId),
    FundStateUpdated(UpdateFundStateParams),
    Invested(InvestedEvent),
    InvestmentClaimed(InvestedEvent),
    InvestmentCancelled(InvestedEvent),
}

#[derive(Serialize, SchemaType, Debug, Clone)]
pub struct InitParam {
    pub currency_token: AnyTokenUId,
    pub agents:         Vec<AgentWithRoles<AgentRole>>,
}

#[derive(Serialize, SchemaType, Reject)]
pub enum Error {
    UnAuthorized,
    ParseError,
    LogError,
    CurrencyTokenTransfer,
    InvalidConversion,
    InvalidFundState,
    TokenMint,
    TokenBurn,
    TokenBalance,
    TokenUnFreeze,
    InvalidFundId,
    InvalidInvestor,
    NonExistentToken,
    AgentExists,
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}
impl From<ExchangeError> for Error {
    fn from(_: ExchangeError) -> Self { Error::InvalidConversion }
}
impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}
/// Current state fo the func represented by this contract.
#[derive(Serialize, Clone, Debug)]
pub enum FundState {
    Open,
    Success(FundSuccessState),
    Fail,
}

#[derive(Serialize, Clone, Debug)]
pub struct FundSuccessState {
    pub funds_receiver: Receiver,
}

pub type FundId = u32;
pub type InvestReceiveParams = OnReceivingCis2DataParams<TokenIdVec, CurrencyTokenAmount, FundId>;

#[derive(Serialize, SchemaType, Debug)]
pub struct UpdateFundStateParams {
    pub fund_id: FundId,
    pub state:   UpdateFundState,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum UpdateFundState {
    Success(Receiver),
    Fail,
}

#[derive(Serialize, SchemaType)]
pub struct AddFundParams {
    pub token:          AnyTokenUId,
    pub rate:           Rate,
    pub security_token: AnyTokenUId,
}

#[derive(Serialize, SchemaType)]
pub struct ClaimInvestmentParam {
    pub fund_id:  FundId,
    pub investor: AccountAddress,
}

#[derive(Serialize, SchemaType)]
pub struct ClaimInvestmentParams {
    pub investments: Vec<ClaimInvestmentParam>,
}

/// Parameters for `transferInvest` function.
/// This is the parameters which will be used to transfer `Currency` token to this contract.
#[derive(Serialize, SchemaType)]
pub struct TransferInvestParams {
    /// Amount of currency token to be invested in current contract's fund.
    pub amount:  CurrencyTokenAmount,
    pub fund_id: FundId,
}
