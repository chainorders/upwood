pub mod types;

use concordium_cis2::*;
use concordium_std::*;
pub use concordium_std::{MetadataUrl, Timestamp};
use types::*;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    pub addresses: StateMap<Address, HolderState, S>,
    pub treasury:  Address,
}

#[derive(Serialize)]
pub struct HolderState {
    pub nonce:    u64,
    pub is_agent: bool,
}
impl HolderState {
    #[inline]
    pub fn nonce(&self) -> u64 { self.nonce }

    #[inline]
    pub fn increment_nonce(&mut self) -> u64 {
        self.nonce += 1;
        self.nonce
    }
}

#[init(
    contract = "offchain_rewards",
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
    let state = State {
        addresses: state_builder.new_map(),
        treasury:  params.treasury,
    };
    logger.log(&Event::Init(params))?;
    Ok(state)
}

#[receive(
    contract = "offchain_rewards",
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
        .map(|a| a.is_agent)
        .unwrap_or(false);
    Ok(is_agent)
}

#[receive(
    contract = "offchain_rewards",
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
    host.state_mut()
        .addresses
        .entry(params.address)
        .or_insert(HolderState {
            nonce:    0,
            is_agent: true,
        });
    logger.log(&Event::AgentAdded(params.address))?;

    Ok(())
}

#[receive(
    contract = "offchain_rewards",
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
        .entry(address)
        .occupied_or(Error::InvalidAddress)?
        .modify(|a| a.is_agent = false);
    logger.log(&&Event::AgentRemoved(address))?;

    Ok(())
}

#[receive(
    contract = "offchain_rewards",
    name = "claimReward",
    enable_logger,
    mutable,
    parameter = "ClaimRequest",
    error = "Error",
    crypto_primitives
)]
pub fn claim_reward(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
    crypto_primitives: &CryptoPrimitives,
) -> ContractResult<()> {
    let claim_req: ClaimRequest = ctx.parameter_cursor().get()?;
    ensure!(
        claim_req.claim.contract_address.eq(&ctx.self_address()),
        Error::InvalidContractAddress
    );
    ensure!(
        ctx.sender().matches_account(&claim_req.claim.account),
        Error::Unauthorized
    );
    let hash = claim_req
        .claim
        .hash(|data| crypto_primitives.hash_sha2_256(&data).0)?;
    ensure!(
        host.check_account_signature(claim_req.signer, &claim_req.signature, &hash)
            .map_err(|_| { Error::CheckSignature })?,
        Error::InvalidSignature
    );

    let (treasury, nonce) = {
        let state = host.state_mut();
        ensure!(
            state
                .addresses
                .get(&claim_req.signer.into())
                .map(|a| a.is_agent)
                .unwrap_or(false),
            Error::UnauthorizedInvalidAgent
        );

        let mut claimer = state
            .addresses
            .entry(claim_req.claim.account.into())
            .or_insert(HolderState {
                nonce:    0,
                is_agent: false,
            });
        ensure!(
            claimer.nonce.eq(&claim_req.claim.account_nonce),
            Error::InvalidNonce
        );
        let nonce = claimer.increment_nonce();

        (state.treasury, nonce)
    };

    host.invoke_contract(
        &claim_req.claim.reward_token_contract,
        &TransferParams(vec![Transfer {
            amount:   claim_req.claim.reward_amount,
            token_id: claim_req.claim.reward_token_id,
            data:     AdditionalData::empty(),
            to:       claim_req.claim.account.into(),
            from:     treasury,
        }]),
        EntrypointName::new_unchecked("transfer"),
        Amount::zero(),
    )
    .map_err(|_| Error::InvokeContract)?;

    logger.log(&Event::Claimed(ClaimedEvent {
        account_address: claim_req.claim.account,
        nonce,
        reward_id: claim_req.claim.reward_id,
        reward_amount: claim_req.claim.reward_amount,
        reward_token_id: claim_req.claim.reward_token_id,
        reward_token_contract: claim_req.claim.reward_token_contract,
    }))?;

    Ok(())
}
