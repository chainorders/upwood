use concordium_cis2::{AdditionalData, TokenAmountU64, TokenIdU64, TokenIdVec, Transfer};
use concordium_protocols::concordium_cis2_ext::cis2_client::Cis2Client;
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::cis2_security_client::Cis2SecurityClient;
use concordium_protocols::concordium_cis2_security::{
    AgentWithRoles, Burn, MintParam, TokenAmountSecurity,
};
use concordium_protocols::rate::{ExchangeError, Rate};
use concordium_std::collections::BTreeMap;
use concordium_std::ops::AddAssign;
use concordium_std::*;

pub type YieldTokenAmount = TokenAmountU64;
pub type ContractResult<T> = Result<T, Error>;
/// This is the security token Id. This should match `TokenId` Type of `security_sft_multi` contract
pub type SecurityTokenId = TokenIdU64;
/// This is the security token amount. This should match `TokenAmount` Type of `security_sft_multi` contract
pub type SecurityTokenAmount = TokenAmountU64;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    /// The treasury account that holds yield tokens.
    /// This is the account from which the yield tokens are transferred to the user.
    pub treasury: Address,
    pub agents:   StateMap<Address, StateSet<AgentRole, S>, S>,
    pub yields: StateMap<
        // Security token contract address
        ContractAddress,
        // Rewards
        StateBTreeMap<
            // When the reward is to be given
            SecurityTokenId,
            Vec<YieldState>,
        >,
        S,
    >,
}

impl State {
    pub fn is_agent(&self, address: &Address, role: AgentRole) -> bool {
        self.agents
            .get(address)
            .is_some_and(|roles_state| roles_state.contains(&role))
    }
}

#[derive(Serialize, SchemaType, Debug, Clone, Copy)]
pub enum AgentRole {
    AddYield,
    RemoveYield,
    Operator,
    UpdateTreasury,
}

impl AgentRole {
    pub fn owner() -> Vec<Self> {
        vec![
            Self::AddYield,
            Self::RemoveYield,
            Self::Operator,
            Self::UpdateTreasury,
        ]
    }
}

#[derive(Serialize, SchemaType, Debug, Clone)]
pub struct YieldState {
    pub contract:    ContractAddress,
    pub token_id:    TokenIdVec,
    /// The rate at which the reward is is calculated compared to the total amount of the token.
    pub calculation: YieldCalculation,
}

#[derive(Serialize, SchemaType, Debug, Clone, Copy)]
pub enum YieldCalculation {
    Quantity(Rate),
    SimpleInterest(Rate),
}

impl YieldCalculation {
    pub fn calculate_amount(
        &self,
        security_amount: &TokenAmountU64,
        duration_ticks: u64,
    ) -> Result<TokenAmountU64, ExchangeError> {
        let (amount, _) = match self {
            YieldCalculation::Quantity(rate) => rate.convert_token_amount_with_rem(security_amount),
            YieldCalculation::SimpleInterest(rate) => rate
                .convert_token_amount_with_rem(&TokenAmountU64(security_amount.0 * duration_ticks)),
        }?;

        Ok(amount)
    }
}

#[derive(Serialize, SchemaType, Debug, Reject)]
pub enum Error {
    UnAuthorized,
    ParseError,
    LogError,
    AgentExists,
    InvalidYield,
    NoYield,
    YieldCalculationError,
    YieldDistribution,
    TokenBurn,
    TokenMint,
}
impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::LogError }
}
impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::ParseError }
}

#[derive(Serialize, SchemaType, Debug)]
pub enum Event {
    AgentAdded(AgentWithRoles<AgentRole>),
    AgentRemoved(Address),
    YieldAdded(UpsertYieldParams),
    YieldRemoved(YieldRemovedEvent),
    YieldDistributed(YieldDistributedEvent),
    TreasuryUpdated(Address),
}

#[derive(Serialize, SchemaType, Debug)]
pub struct YieldDistributedEvent {
    pub from_token: TokenIdU64,
    pub to_token:   TokenIdU64,
    pub contract:   ContractAddress,
    pub amount:     TokenAmountU64,
    pub to:         AccountAddress,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct YieldAddedEvent {
    /// The security token contract address.
    pub token_contract:       ContractAddress,
    pub yield_token_contract: ContractAddress,
    pub yield_token_id:       TokenIdVec,
    pub yield_calculation:    YieldCalculation,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct YieldRemovedEvent {
    pub token_contract: ContractAddress,
    pub token_id:       SecurityTokenId,
}

#[derive(Serialize, SchemaType)]
pub struct InitParam {
    pub treasury: Address,
    pub agents:   Vec<AgentWithRoles<AgentRole>>,
}

#[init(
    contract = "security_sft_multi_yielder",
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
    logger.log(&Event::TreasuryUpdated(params.treasury))?;
    Ok(State {
        treasury: params.treasury,
        agents,
        yields: state_builder.new_map(),
    })
}

#[receive(
    contract = "security_sft_multi_yielder",
    name = "setTreasury",
    mutable,
    parameter = "Address",
    enable_logger
)]
fn set_treasury(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let treasury: Address = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        state.is_agent(&ctx.sender(), AgentRole::UpdateTreasury),
        Error::UnAuthorized
    );
    state.treasury = treasury;
    logger.log(&Event::TreasuryUpdated(treasury))?;
    Ok(())
}

#[receive(contract = "security_sft_multi_yielder", name = "getTreasury")]
fn get_treasury(_: &ReceiveContext, host: &Host<State>) -> ContractResult<Address> {
    Ok(host.state().treasury)
}

#[receive(
    contract = "security_sft_multi_yielder",
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
    contract = "security_sft_multi_yielder",
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

#[derive(Serialize, SchemaType, Debug)]
pub struct UpsertYieldParams {
    pub token_contract: ContractAddress,
    pub token_id:       SecurityTokenId,
    pub yields:         Vec<YieldState>,
}

#[receive(
    contract = "security_sft_multi_yielder",
    name = "upsertYield",
    mutable,
    parameter = "UpsertYieldParams",
    enable_logger
)]
fn upsert_yield(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let UpsertYieldParams {
        token_contract,
        token_id,
        yields,
    }: UpsertYieldParams = ctx.parameter_cursor().get()?;
    ensure!(
        host.state().is_agent(&ctx.sender(), AgentRole::AddYield),
        Error::UnAuthorized
    );
    let (state, state_builder) = host.state_and_builder();
    let existing = state
        .yields
        .entry(token_contract)
        .or_insert_with(|| state_builder.new_btree_map())
        .modify(|contract_yield| contract_yield.insert(token_id, yields.clone()));

    if existing.is_some() {
        logger.log(&Event::YieldRemoved(YieldRemovedEvent {
            token_contract,
            token_id,
        }))?;
    }

    logger.log(&Event::YieldAdded(UpsertYieldParams {
        token_contract,
        token_id,
        yields,
    }))?;

    Ok(())
}

#[derive(Serialize, SchemaType, Debug)]
pub struct RemoveYieldParams {
    pub token_contract: ContractAddress,
    pub token_id:       SecurityTokenId,
}

#[receive(
    contract = "security_sft_multi_yielder",
    name = "removeYield",
    mutable,
    parameter = "RemoveYieldParams",
    enable_logger
)]
fn remove_yield(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let RemoveYieldParams {
        token_contract,
        token_id,
    }: RemoveYieldParams = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        state.is_agent(&ctx.sender(), AgentRole::RemoveYield),
        Error::UnAuthorized
    );

    let existing = state
        .yields
        .entry(token_contract)
        .occupied_or(Error::InvalidYield)?
        .modify(|contract_yield| contract_yield.remove_and_get(&token_id))
        .is_some();
    ensure!(existing, Error::InvalidYield);

    logger.log(&Event::YieldRemoved(YieldRemovedEvent {
        token_contract,
        token_id,
    }))?;

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct YieldParams {
    pub owner:  AccountAddress,
    pub yields: Vec<YieldParam>,
}

#[derive(Serialize, SchemaType)]
pub struct YieldParam {
    pub token_ver_from: TokenIdU64,
    pub token_ver_to:   TokenIdU64,
    pub token_contract: ContractAddress,
    pub amount:         TokenAmountU64,
}

#[receive(
    contract = "security_sft_multi_yielder",
    name = "yieldFor",
    mutable,
    parameter = "YieldParams",
    enable_logger
)]
fn yield_for(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let YieldParams {
        owner,
        yields: mut params,
    }: YieldParams = ctx.parameter_cursor().get()?;

    let sender = ctx.sender();
    let state = host.state();
    let treasury = state.treasury;

    if !sender.matches_account(&owner) {
        let is_agent = state.is_agent(&sender, AgentRole::Operator);
        ensure!(is_agent, Error::UnAuthorized);
    }

    let mut yields = BTreeMap::new();
    // Calculate the yields for the user across contracts.
    for param in params.iter_mut() {
        let contract_yields = state
            .yields
            .get(&param.token_contract)
            .ok_or(Error::InvalidYield)?;

        let mut token_ver_from = param.token_ver_from;
        while let Some(next_token_ver) = contract_yields
            .higher(&token_ver_from)
            .filter(|v| v.le(&param.token_ver_to))
        {
            let next_yields = contract_yields.get(&next_token_ver).unwrap();
            for next_yield in next_yields.iter() {
                let amount = next_yield
                    .calculation
                    .calculate_amount(
                        &param.amount,
                        // Every version is assumed to be a tick.
                        next_token_ver.0 - token_ver_from.0,
                    )
                    .map_err(|_| Error::YieldCalculationError)?;

                if amount.is_zero() {
                    continue;
                }

                yields
                    .entry((next_yield.contract, next_yield.token_id.clone()))
                    .or_insert(TokenAmountU64::zero())
                    .add_assign(amount);
            }

            token_ver_from = *next_token_ver;
        }

        param.token_ver_to = token_ver_from;
    }

    ensure!(!yields.is_empty(), Error::NoYield);
    // Transfer the yields to the user.
    for ((contract, token_id), amount) in yields.into_iter() {
        host.invoke_transfer_single(&contract, Transfer {
            token_id,
            from: treasury,
            to: owner.into(),
            amount,
            data: AdditionalData::empty(),
        })
        .map_err(|_| Error::YieldDistribution)?;
    }

    // Updating the security token versions
    for YieldParam {
        token_ver_from,
        token_ver_to,
        token_contract,
        amount,
    } in params
        .into_iter()
        .filter(|p| p.token_ver_from != p.token_ver_to)
    {
        // Burn the current version of secruity token.
        host.invoke_burn_single(&token_contract, Burn {
            amount,
            token_id: token_ver_from,
            owner: owner.into(),
        })
        .map_err(|_| Error::TokenBurn)?;
        // Mint the next version of security token.
        host.invoke_mint_single(&token_contract, token_ver_to, MintParam {
            address: owner.into(),
            amount:  TokenAmountSecurity::new_un_frozen(amount),
        })
        .map_err(|_| Error::TokenMint)?;
        logger.log(&Event::YieldDistributed(YieldDistributedEvent {
            from_token: token_ver_from,
            to_token: token_ver_to,
            contract: token_contract,
            amount,
            to: owner,
        }))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use concordium_cis2::TokenAmountU64;
    use concordium_protocols::rate::Rate;

    use crate::YieldCalculation;

    #[test]
    pub fn test_yield_calculations() {
        let rate = Rate {
            numerator:   100,
            denominator: 1,
        };
        let quantity = YieldCalculation::Quantity(rate);
        let simple_interest = YieldCalculation::SimpleInterest(rate);

        let amount = TokenAmountU64(33);
        let duration = 10;

        let quantity_amount = quantity.calculate_amount(&amount, duration).unwrap();
        let simple_interest_amount = simple_interest.calculate_amount(&amount, duration).unwrap();

        assert_eq!(quantity_amount, TokenAmountU64(33 * 100));
        assert_eq!(simple_interest_amount, TokenAmountU64(10 * 100 * 33));

        let rate = Rate {
            numerator:   1,
            denominator: 25,
        };
        let quantity = YieldCalculation::Quantity(rate);
        let simple_interest = YieldCalculation::SimpleInterest(rate);

        let amount = TokenAmountU64(33);
        let duration = 10;

        let quantity_amount = quantity.calculate_amount(&amount, duration).unwrap();
        let simple_interest_amount = simple_interest.calculate_amount(&amount, duration).unwrap();

        assert_eq!(quantity_amount, TokenAmountU64(33 / 25));
        assert_eq!(simple_interest_amount, TokenAmountU64(10 * 33 / 25));

        let rate = Rate {
            numerator:   5,
            denominator: 1,
        };
        let quantity = YieldCalculation::Quantity(rate);
        let quantity_amount = quantity.calculate_amount(&1000.into(), 0).unwrap();
        assert_eq!(quantity_amount, TokenAmountU64(5000));
    }
}
