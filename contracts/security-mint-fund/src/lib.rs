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
pub mod types;
use concordium_cis2::{AdditionalData, Receiver, Transfer};
use concordium_protocols::concordium_cis2_ext::cis2_client::Cis2Client;
use concordium_protocols::concordium_cis2_ext::{IsTokenAmount, ToAdditionalData};
use concordium_protocols::concordium_cis2_security::cis2_security_client::Cis2SecurityClient;
use concordium_protocols::concordium_cis2_security::{
    AgentWithRoles, Burn, FreezeParam, MintParam, TokenAmountSecurity, TokenUId,
};
use concordium_protocols::rate::Rate;
use concordium_std::ops::AddAssign;
use concordium_std::*;
use types::*;

#[derive(Serial, DeserialWithState, Debug)]
#[concordium(state_parameter = "S")]
pub struct Fund<S> {
    pub state:          FundState,
    pub investments:    StateMap<AccountAddress, CurrencyTokenAmount, S>,
    pub token:          AnyTokenUId,
    /// This is the rate  which will be used to convert from `currency_token` token to `security_token` Token.
    pub rate:           Rate,
    /// This is the token which will be minted after successful completion of the fund.
    pub security_token: AnyTokenUId,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    pub last_fund_id:   FundId,
    /// Agents who can interact with this contract.
    /// Agents are allowed to Start a new fund, Update the fund & force the claim for a user Or force the return of amount to a user.
    pub agents:         StateMap<Address, StateSet<AgentRole, S>, S>,
    /// This is the token which is used to fund this contract.
    /// This will usualle be EuroE Token
    pub currency_token: AnyTokenUId,
    /// Map of all the funds which are currently in this contract.
    /// Key is the token which will be minted in locked state after investment. This is the initial token minted by the contract upon investment.
    pub funds:          StateMap<FundId, Fund<S>, S>,
}

impl State<StateApi> {
    pub fn has_agent(&self, address: Address, role: AgentRole) -> bool {
        self.agents
            .get(&address)
            .map_or(false, |roles| roles.contains(&role))
    }
}

#[init(
    contract = "security_mint_fund",
    error = "Error",
    parameter = "InitParam",
    event = "Event",
    enable_logger
)]
fn init(
    ctx: &InitContext,
    state_builder: &mut StateBuilder,
    logger: &mut Logger,
) -> InitResult<State> {
    let params: InitParam = ctx.parameter_cursor().get()?;
    logger.log(&Event::Initialized(params.currency_token.clone()))?;
    let agents = {
        let mut agents = state_builder.new_map();
        let roles = {
            let mut roles = state_builder.new_set();
            for role in AgentRole::owner().iter() {
                roles.insert(*role);
            }
            roles
        };
        let _ = agents.insert(ctx.init_origin().into(), roles);
        logger.log(&Event::AgentAdded(AgentWithRoles {
            address: ctx.init_origin().into(),
            roles:   AgentRole::owner(),
        }))?;

        for agent in params.agents.iter() {
            let roles = {
                let mut roles = state_builder.new_set();
                for role in agent.roles.iter() {
                    roles.insert(*role);
                }
                roles
            };
            let _ = agents.insert(agent.address, roles);
            logger.log(&Event::AgentAdded(agent.clone()))?;
        }

        agents
    };
    Ok(State {
        currency_token: params.currency_token,
        last_fund_id: 0,
        agents,
        funds: state_builder.new_map(),
    })
}

#[receive(
    contract = "security_mint_fund",
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
        Error::UnAuthorized
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
    contract = "security_mint_fund",
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
        Error::UnAuthorized
    );
    let (state, _) = host.state_and_builder();
    state.agents.remove(&agent);
    logger.log(&Event::AgentRemoved(agent))?;
    Ok(())
}

#[receive(
    contract = "security_mint_fund",
    name = "addFund",
    mutable,
    parameter = "AddFundParams",
    enable_logger
)]
fn add_fund(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: AddFundParams = ctx.parameter_cursor().get()?;
    let (state, state_builder) = host.state_and_builder();
    ensure!(
        state.has_agent(ctx.sender(), AgentRole::AddFund),
        Error::UnAuthorized
    );

    let fund_id = state.last_fund_id;
    let existing = state.funds.insert(fund_id, Fund {
        state:          FundState::Open,
        investments:    state_builder.new_map(),
        token:          params.token.clone(),
        rate:           params.rate,
        security_token: params.security_token.clone(),
    });
    ensure!(existing.is_none(), Error::InvalidFundId);
    state.last_fund_id = fund_id + 1;

    // ensure that the token exists
    let _ = host
        .invoke_token_metadata_single(&params.token.contract, params.token.id.clone())
        .map_err(|_| Error::NonExistentToken)?;
    let _ = host
        .invoke_token_metadata_single(
            &params.security_token.contract,
            params.security_token.id.clone(),
        )
        .map_err(|_| Error::NonExistentToken)?;

    logger.log(&Event::FundAdded(FundAddedEvent {
        fund_id,
        token: params.token,
        rate: params.rate,
        security_token: params.security_token,
    }))?;
    Ok(())
}

#[receive(
    contract = "security_mint_fund",
    name = "removeFund",
    mutable,
    parameter = "FundId",
    enable_logger
)]
fn remove_fund(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let fund_id: FundId = ctx.parameter_cursor().get()?;
    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_agent(ctx.sender(), AgentRole::RemoveFund),
        Error::UnAuthorized
    );
    let fund = state
        .funds
        .remove_and_get(&fund_id)
        .ok_or(Error::InvalidFundId)?;
    ensure!(fund.investments.is_empty(), Error::InvalidFundState);
    logger.log(&Event::FundRemoved(fund_id))?;
    Ok(())
}

#[receive(
    contract = "security_mint_fund",
    name = "updateFundState",
    mutable,
    parameter = "UpdateFundStateParams",
    enable_logger
)]
fn update_fund_state(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: UpdateFundStateParams = ctx.parameter_cursor().get()?;
    let state = host.state();
    ensure!(
        state.has_agent(ctx.sender(), AgentRole::UpdateFundState),
        Error::UnAuthorized
    );
    let fund = state
        .funds
        .get(&params.fund_id)
        .ok_or(Error::InvalidFundId)?;
    {
        let fund_state = match (&fund.state, &params.state) {
            (FundState::Open, UpdateFundState::Success(params)) => {
                FundState::Success(FundSuccessState {
                    funds_receiver: params.clone(),
                })
            }
            (FundState::Open, UpdateFundState::Fail) => FundState::Fail,
            (FundState::Fail, _) => bail!(Error::InvalidFundState),
            (FundState::Success(_), _) => bail!(Error::InvalidFundState),
        };
        host.state_mut()
            .funds
            .entry(params.fund_id)
            .and_modify(|fund| {
                fund.state = fund_state;
            });
    }

    logger.log(&Event::FundStateUpdated(params))?;
    Ok(())
}

#[receive(
    contract = "security_mint_fund",
    name = "transferInvest",
    mutable,
    parameter = "TransferInvestParams"
)]
fn transfer_invest(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    let currency_token = host.state().currency_token.clone();
    let params: TransferInvestParams = ctx.parameter_cursor().get()?;
    host.invoke_transfer_single(&currency_token.contract, Transfer {
        token_id: currency_token.id,
        amount:   params.amount,
        to:       Receiver::Contract(
            ctx.self_address(),
            OwnedEntrypointName::new_unchecked("invest".into()),
        ),
        from:     ctx.sender(),
        data:     params
            .fund_id
            .to_additional_data()
            .ok_or(Error::ParseError)?,
    })
    .map_err(|_| Error::CurrencyTokenTransfer)?;

    Ok(())
}

#[receive(
    contract = "security_mint_fund",
    name = "invest",
    mutable,
    parameter = "InvestReceiveParams",
    enable_logger
)]
fn invest(ctx: &ReceiveContext, host: &mut Host<State>, logger: &mut Logger) -> ContractResult<()> {
    let InvestReceiveParams {
        amount: curr_amount,
        data: fund_id,
        token_id,
        from,
    } = ctx.parameter_cursor().get()?;
    let from = match from {
        Address::Account(a) => a,
        Address::Contract(_) => bail!(Error::UnAuthorized),
    };
    let currency_token = TokenUId {
        id:       token_id,
        contract: match ctx.sender() {
            Address::Account(_) => bail!(Error::UnAuthorized),
            Address::Contract(c) => c,
        },
    };

    let (wrapped_amount, wrapped_token) = {
        let state = host.state_mut();
        let mut fund = state.funds.get_mut(&fund_id).ok_or(Error::InvalidFundId)?;
        ensure!(
            state.currency_token.eq(&currency_token),
            Error::UnAuthorized
        );
        match fund.state {
            FundState::Open => {
                // Add the investment to the fund
                fund.investments
                    .entry(from)
                    .or_insert_with(CurrencyTokenAmount::zero)
                    .modify(|amount| amount.add_assign(curr_amount));
                // Convert the currency amount to wrapped amount
                let security_amount = fund.rate.convert_currency_amount(&curr_amount)?;
                (security_amount, fund.token.clone())
            }
            _ => bail!(Error::InvalidFundState),
        }
    };

    host.invoke_mint_single(&wrapped_token.contract, wrapped_token.id, MintParam {
        address: from.into(),
        amount:  TokenAmountSecurity::new_un_frozen(wrapped_amount),
    })
    .map_err(|_| Error::TokenMint)?;

    logger.log(&Event::Invested(InvestedEvent {
        fund_id,
        currency_amount: curr_amount,
        security_amount: wrapped_amount,
        investor: from,
    }))?;
    Ok(())
}

#[receive(
    contract = "security_mint_fund",
    name = "claimInvestment",
    mutable,
    parameter = "ClaimInvestmentParams",
    enable_logger
)]
fn claim_investment(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let sender = ctx.sender();
    let params: ClaimInvestmentParams = ctx.parameter_cursor().get()?;
    for investment in params.investments {
        let (token, currency_token, currency_amount, security_token, security_amount, fund_state) = {
            let state = host.state_mut();
            if !sender.matches_account(&investment.investor) {
                ensure!(
                    &state.has_agent(sender, AgentRole::Operator),
                    Error::UnAuthorized
                );
            }
            let currency_token = state.currency_token.clone();
            let mut fund = state
                .funds
                .get_mut(&investment.fund_id)
                .ok_or(Error::InvalidFundId)?;
            // Invested currency amount
            let currency_amount = fund
                .investments
                .remove_and_get(&investment.investor)
                .ok_or(Error::InvalidInvestor)?;
            // Promised security amount
            let security_amount = fund.rate.convert_currency_amount(&currency_amount)?;
            (
                fund.token.clone(),
                currency_token,
                currency_amount,
                fund.security_token.clone(),
                security_amount,
                fund.state.clone(),
            )
        };

        match fund_state {
            FundState::Open => bail!(Error::InvalidFundState),
            FundState::Success(FundSuccessState { funds_receiver }) => {
                // Transfer the currency amount to the receiver
                host.invoke_transfer_single(&currency_token.contract, Transfer {
                    amount:   currency_amount,
                    token_id: currency_token.id.clone(),
                    from:     ctx.self_address().into(),
                    to:       funds_receiver.clone(),
                    data:     AdditionalData::empty(),
                })
                .map_err(|_| Error::CurrencyTokenTransfer)?;

                if security_token.eq(&token) {
                    // Only Un Freeze Tokens
                    host.invoke_un_freeze_single(
                        &security_token.contract,
                        investment.investor.into(),
                        FreezeParam {
                            token_amount: security_amount,
                            token_id:     security_token.id.clone(),
                        },
                    )
                    .map_err(|_| Error::TokenUnFreeze)?;
                } else {
                    // Burn the initially minted tokens
                    host.invoke_burn_single(&token.contract, Burn {
                        token_id: token.id.clone(),
                        amount:   security_amount,
                        owner:    investment.investor.into(),
                    })
                    .map_err(|_| Error::TokenBurn)?;
                    // Mint the security tokens
                    host.invoke_mint_single(
                        &security_token.contract,
                        security_token.id.clone(),
                        MintParam {
                            address: investment.investor.into(),
                            amount:  TokenAmountSecurity::new_un_frozen(security_amount),
                        },
                    )
                    .map_err(|_| Error::TokenMint)?;
                }

                logger.log(&Event::InvestmentClaimed(InvestedEvent {
                    fund_id: investment.fund_id,
                    security_amount,
                    investor: investment.investor,
                    currency_amount,
                }))?;
            }
            FundState::Fail => {
                // Return the Invested currency amount
                host.invoke_transfer_single(&currency_token.contract, Transfer {
                    amount:   currency_amount,
                    token_id: currency_token.id.clone(),
                    from:     ctx.self_address().into(),
                    to:       investment.investor.into(),
                    data:     AdditionalData::empty(),
                })
                .map_err(|_| Error::CurrencyTokenTransfer)?;
                host.invoke_burn_single(&token.contract, Burn {
                    token_id: token.id,
                    amount:   security_amount,
                    owner:    investment.investor.into(),
                })
                .map_err(|_| Error::TokenBurn)?;

                logger.log(&Event::InvestmentCancelled(InvestedEvent {
                    fund_id: investment.fund_id,
                    security_amount,
                    investor: investment.investor,
                    currency_amount,
                }))?;
            }
        }
    }

    Ok(())
}
