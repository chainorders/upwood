use super::{error::*, mint::mint_internal, state::State, types::*};
use concordium_cis2::{AdditionalData, Cis2Client, Receiver, TokenAmountU64, Transfer};
use concordium_rwa_utils::{
    concordium_cis2_security::TokenDeposited, token_deposits_state::IDepositedTokensState,
};
use concordium_std::*;

#[receive(
    contract = "rwa_security_sft",
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

    let deposited_token_uid = NftTokenOwnerUId {
        token_id: NftTokenUId {
            contract: sender,
            id:       params.token_id,
        },
        owner:    Receiver::Account(from),
    };
    host.state_mut().inc_deposits(deposited_token_uid.clone(), params.amount);
    logger.log(&Event::Deposited(TokenDeposited {
        token_id: deposited_token_uid.clone().token_id,
        owner:    from,
        amount:   TokenAmountU64(params.amount.0.into()),
    }))?;

    if params.data.size().ne(&0u32) {
        let mut cursor = Cursor::new(params.data.to_owned());
        let data: Result<MintParam, ParseError> = MintParam::deserial(&mut cursor);
        if let Ok(mint_param) = data {
            ensure!(
                params.from.matches_account(&mint_param.deposited_token_owner),
                Error::Unauthorized
            );
            ensure!(
                deposited_token_uid.matches(&mint_param.deposited_token_id),
                Error::InvalidDepositData
            );
            mint_internal(ctx.self_address(), mint_param, host, logger)?
        }
    }

    Ok(())
}

#[receive(
    contract = "rwa_security_sft",
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
    let token_owner_uid = params.token_id.to_token_owner_uid(params.owner.into());

    host.state_mut().dec_deposits(&token_owner_uid, params.amount)?;
    Cis2Client::new(params.token_id.contract)
        .transfer::<_, _, _, ()>(host, Transfer {
            amount:   params.amount,
            from:     Address::Contract(ctx.self_address()),
            to:       token_owner_uid.owner,
            token_id: token_owner_uid.token_id.id.to_owned(),
            data:     AdditionalData::empty(),
        })
        .map_err(|_| Error::Cis2WithdrawError)?;
    logger.log(&Event::Withdraw(TokenDeposited {
        token_id: params.token_id,
        owner:    params.owner,
        amount:   TokenAmountU64(params.amount.0.into()),
    }))?;

    Ok(())
}

#[receive(
    contract = "rwa_security_sft",
    name = "balanceOfDeposited",
    parameter = "BalanceOfDepositParams",
    error = "super::error::Error",
    return_value = "NftTokenAmount"
)]
pub fn balance_of_deposited(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<NftTokenAmount> {
    let params: BalanceOfDepositParams = ctx.parameter_cursor().get()?;
    Ok(host
        .state()
        .balance_of_deposited(&params.token_id.to_token_owner_uid(params.address.into())))
}
