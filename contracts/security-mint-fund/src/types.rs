use concordium_cis2::{
    OnReceivingCis2DataParams, Receiver, TokenAmountU64, TokenIdU64, TokenIdUnit,
};
use concordium_protocols::concordium_cis2_security::{AgentWithRoles, TokenUId};
use concordium_protocols::rate::{ExchangeError, Rate};
use concordium_std::*;

pub type ContractResult<T> = Result<T, Error>;
pub type TokenAmount = TokenAmountU64;
pub type CurrencyTokenAmount = TokenAmountU64;
pub type CurrencyTokenId = TokenIdUnit;
/// TokenUId for `Currency` token. This token type should match EuroE token type.
pub type CurrencyTokenUId = TokenUId<CurrencyTokenId>;
/// TokenUId for `Security` token. This token type should match Security Sft Multi token type.
pub type SecurityTokenUId = TokenUId<TokenIdU64>;

#[derive(Serialize, SchemaType, Debug, Clone, Copy)]
pub enum AgentRole {
    AddFund,
    RemoveFund,
    UpdateFundState,
    Operator,
}

impl AgentRole {
    pub fn owner() -> Vec<Self> {
        vec![
            Self::AddFund,
            Self::RemoveFund,
            Self::UpdateFundState,
            Self::Operator,
        ]
    }
}

#[derive(Serialize, SchemaType, Debug)]
pub struct InvestedEvent {
    pub security_token:  SecurityTokenUId,
    pub investor:        AccountAddress,
    pub security_amount: TokenAmount,
    pub currency_amount: CurrencyTokenAmount,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct FundAddedEvent {
    pub token:          SecurityTokenUId,
    pub rate:           Rate,
    pub security_token: SecurityTokenUId,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum Event {
    /// Emitted when the contract is initialized with the currency token
    /// Triggered in the init function
    Initialized(CurrencyTokenUId),

    /// Emitted when a new agent with roles is added to the contract
    /// Triggered in both init function (for initial agents) and add_agent function
    AgentAdded(AgentWithRoles<AgentRole>),

    /// Emitted when an agent is removed from the contract
    /// Triggered in the remove_agent function
    AgentRemoved(Address),

    /// Emitted when a new fund is added to the contract
    /// Triggered in the add_fund function
    FundAdded(FundAddedEvent),

    /// Emitted when a fund is removed from the contract
    /// Triggered in the remove_fund function
    FundRemoved(SecurityTokenUId),

    /// Emitted when a fund's state is updated to either Success or Fail
    /// Triggered in the update_fund_state function
    FundStateUpdated(UpdateFundStateParams),

    /// Emitted when an investor makes an investment in an Open fund
    /// Triggered in the invest function after adding investment and minting tokens
    Invested(InvestedEvent),

    /// Emitted when an investment is claimed from a successful fund
    /// Triggered in the claim_investment function when fund is in Success state
    InvestmentClaimed(InvestedEvent),

    /// Emitted when an investment is cancelled and funds returned to investor
    /// Triggered in the claim_investment function when fund is in Fail state
    InvestmentCancelled(InvestedEvent),
}

#[derive(Serialize, SchemaType, Debug, Clone)]
pub struct InitParam {
    pub currency_token: CurrencyTokenUId,
    pub agents:         Vec<AgentWithRoles<AgentRole>>,
}

#[derive(Serialize, SchemaType, Reject)]
pub enum Error {
    /// Thrown when a caller doesn't have the required agent role or permissions to perform an action
    UnAuthorized,
    /// Thrown when there's an error parsing function parameters or token data
    ParseError,
    /// Thrown when the contract fails to log events
    LogError,
    /// Thrown when transferring currency tokens fails (during investment or claiming)
    CurrencyTokenTransfer,
    /// Thrown when rate conversion between currency and security token amounts fails
    InvalidConversion,
    /// Thrown when an operation is attempted in an incompatible fund state
    /// (e.g., claiming investment when fund is in Open state)
    InvalidFundState,
    /// Thrown when minting security tokens fails
    TokenMint,
    /// Thrown when burning security tokens fails
    TokenBurn,
    /// Thrown when checking token balance fails
    TokenBalance,
    /// Thrown when unfreezing tokens during investment claiming fails
    TokenUnFreeze,
    /// Thrown when referring to a fund that doesn't exist in the contract state
    InvalidFundId,
    /// Thrown when claiming investment for an address that has no investment in the fund
    InvalidInvestor,
    /// Thrown when referring to a token that doesn't exist
    NonExistentToken,
    /// Thrown when attempting to add an agent that already exists
    AgentExists,
    /// Thrown when attempting to add a fund with a token ID that's already registered
    FundExists,
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

pub type InvestReceiveParams =
    OnReceivingCis2DataParams<CurrencyTokenId, CurrencyTokenAmount, SecurityTokenUId>;

#[derive(Serialize, SchemaType, Debug)]
pub struct UpdateFundStateParams {
    pub security_token: SecurityTokenUId,
    pub state:          UpdateFundState,
}

#[derive(Serialize, SchemaType, Debug)]
pub enum UpdateFundState {
    Success(Receiver),
    Fail,
}

#[derive(Serialize, SchemaType)]
pub struct AddFundParams {
    pub token:          SecurityTokenUId,
    pub rate:           Rate,
    pub security_token: SecurityTokenUId,
}

#[derive(Serialize, SchemaType)]
pub struct ClaimInvestmentParam {
    pub security_token: SecurityTokenUId,
    pub investor:       AccountAddress,
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
    pub amount:         CurrencyTokenAmount,
    pub security_token: SecurityTokenUId,
}
