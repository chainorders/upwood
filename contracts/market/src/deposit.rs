use concordium_cis2::{
    AdditionalData, Cis2Client, OnReceivingCis2Params, Receiver, TokenIdVec, Transfer,
};
use concordium_std::*;

use crate::state::TokenOwnerUId;

use super::{
    error::*,
    event::{Event, TokenDeposited},
    exchange::{exchange_internal, ExchangeParams},
    list::{list_internal, ListParams},
    state::State,
    types::{Cis2TokenAmount, ContractResult, TokenUId},
};

pub type DepositParams = OnReceivingCis2Params<TokenIdVec, Cis2TokenAmount>;

#[receive(
    contract = "rwa_market",
    name = "deposit",
    mutable,
    parameter = "DepositParams",
    error = "super::error::Error",
    enable_logger
)]
pub fn deposit(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let sender = match ctx.sender() {
        Address::Account(_) => bail!(Error::Unauthorized),
        Address::Contract(sender) => sender,
    };

    let params: DepositParams = ctx.parameter_cursor().get()?;
    let from = match params.from {
        Address::Account(address) => address,
        Address::Contract(_) => bail!(Error::OnlyAccount),
    };

    let deposit_token_uid = TokenUId::new(sender, params.token_id.to_owned());
    host.state_mut().add_or_increase_deposits(deposit_token_uid.to_owned(), params.amount, from);
    logger.log(&Event::Deposited(TokenDeposited {
        token_id: deposit_token_uid.to_owned(),
        owner:    from,
        amount:   params.amount,
    }))?;

    if params.data.size().ne(&0u32) {
        let mut cursor = Cursor::new(params.data.to_owned());
        let data: Result<ListParams, ParseError> = ListParams::deserial(&mut cursor);
        if let Ok(data) = data {
            ensure!(params.from.matches_account(&data.owner), Error::Unauthorized);
            ensure!(deposit_token_uid.eq(&data.token_id), Error::InvalidDepositData);
            list_internal(data, host, logger)?
        } else {
            cursor.offset = 0;
            let data = ExchangeParams::deserial(&mut cursor);
            if let Ok(data) = data {
                ensure!(params.from.matches_account(&data.payer), Error::Unauthorized);
                ensure!(host.state().can_be_paid_by(&deposit_token_uid), Error::InvalidDepositData);
                exchange_internal(
                    ctx.self_address(),
                    ctx.owner(),
                    data,
                    Amount::zero(),
                    host,
                    logger,
                )?
            }
        }
    }

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct WithdrawParams {
    pub token_id: TokenUId,
    pub owner:    AccountAddress,
    pub amount:   Cis2TokenAmount,
}

#[receive(
    contract = "rwa_market",
    name = "withdraw",
    mutable,
    parameter = "WithdrawParams",
    error = "super::error::Error",
    enable_logger
)]
pub fn withdraw(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    let params: WithdrawParams = ctx.parameter_cursor().get()?;
    ensure!(ctx.sender().matches_account(&params.owner), Error::Unauthorized);
    ensure!(
        host.state().unlisted_amount(&params.token_id, &params.owner).ge(&params.amount),
        Error::InsufficientDeposits
    );
    host.state_mut().decrease_deposit(
        &TokenOwnerUId::new(params.token_id.to_owned(), params.owner),
        &params.amount,
    );
    Cis2Client::new(params.token_id.contract)
        .transfer::<_, _, _, ()>(host, Transfer {
            amount:   params.amount,
            from:     Address::Contract(ctx.self_address()),
            to:       Receiver::Account(params.owner),
            token_id: params.token_id.id.to_owned(),
            data:     AdditionalData::empty(),
        })
        .map_err(|_| Error::Cis2WithdrawError)?;
    logger.log(&Event::Withdraw(TokenDeposited {
        token_id: params.token_id,
        owner:    params.owner,
        amount:   params.amount,
    }))?;

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct BalanceOfDepositParams {
    pub token_id: TokenUId,
    pub address:  AccountAddress,
}

#[receive(
    contract = "rwa_market",
    name = "balanceOfDeposited",
    parameter = "BalanceOfDepositParams",
    error = "super::error::Error",
    return_value = "Cis2TokenAmount"
)]
pub fn balance_of_deposited(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<Cis2TokenAmount> {
    let params: BalanceOfDepositParams = ctx.parameter_cursor().get()?;
    Ok(host.state().deposited_amount(&params.token_id, &params.address))
}

