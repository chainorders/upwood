use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_mint_fund::Event;
use shared::db::security_mint_fund::{
    InvestmentRecord, InvestmentRecordType, Investor, SecurityMintFundContract,
    SecurityMintFundState,
};
use shared::db_shared::DbConn;
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::processors::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};
use crate::processors::ProcessorError;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/security-mint-fund/contract.wasm.v1"
    ))
    .expect("Failed to parse security-mint-fund module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_mint_fund".to_string())
}

#[instrument(
    name="mint_fund",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    block_height: Decimal,
    block_time: NaiveDateTime,
    txn_index: Decimal,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let event = event.parse::<Event>().expect("Failed to parse event");

        match event {
            Event::Initialized(event) => {
                let (fund_state, receiver_address) = to_db_fund_state(event.fund_state);
                SecurityMintFundContract {
                    contract_address: contract.to_decimal(),
                    token_id: event.token.id.to_decimal(),
                    token_contract_address: event.token.contract.to_decimal(),
                    investment_token_id: event.investment_token.id.to_decimal(),
                    investment_token_contract_address: event.investment_token.contract.to_decimal(),
                    currency_token_id: event.currency_token.id.to_decimal(),
                    currency_token_contract_address: event.currency_token.contract.to_decimal(),
                    rate: rate_to_decimal(event.rate.numerator, event.rate.denominator),
                    fund_state,
                    receiver_address,
                    currency_amount: Decimal::ZERO,
                    token_amount: Decimal::ZERO,
                    create_time: block_time,
                    update_time: block_time,
                }
                .insert(conn)?;
                info!("initialized");
            }
            Event::FundStateUpdated(fund_state) => {
                let (fund_state, receiver_address) = to_db_fund_state(fund_state);
                SecurityMintFundContract::update_state(
                    conn,
                    contract.to_decimal(),
                    fund_state,
                    receiver_address,
                    block_time,
                )?;
                info!("state updated: to: {:?}", fund_state);
            }
            Event::Invested(event) => {
                let investor = Investor::new(
                    contract.to_decimal(),
                    &event.investor,
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    block_time,
                )
                .upsert(conn)?;
                InvestmentRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    investor: investor.investor.to_string(),
                    currency_amount: event.currency_amount.to_decimal(),
                    token_amount: event.security_amount.to_decimal(),
                    currency_amount_balance: investor.currency_amount,
                    token_amount_balance: investor.token_amount,
                    investment_record_type: InvestmentRecordType::Invested,
                    create_time: block_time,
                }
                .insert(conn)?;
                SecurityMintFundContract::add_investment_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    block_time,
                )?;
                info!(
                    "Investment received: from: {}, currency amount: {}, token amount: {}",
                    event.investor.to_string(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal()
                );
            }
            Event::InvestmentCancelled(event) => {
                let investor = Investor::cancel_investment(
                    conn,
                    contract.to_decimal(),
                    &event.investor,
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    block_time,
                )?;
                InvestmentRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    investor: event.investor.to_string(),
                    currency_amount: event.currency_amount.to_decimal(),
                    token_amount: event.security_amount.to_decimal(),
                    currency_amount_balance: investor.currency_amount,
                    token_amount_balance: investor.token_amount,
                    investment_record_type: InvestmentRecordType::Cancelled,
                    create_time: block_time,
                }
                .insert(conn)?;
                SecurityMintFundContract::sub_investment_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    block_time,
                )?;
                info!(
                    "Investment cancelled: from: {}, currency amount: {}, token amount: {}",
                    event.investor.to_string(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal()
                );
            }
            Event::InvestmentClaimed(event) => {
                let investor =
                    Investor::find(conn, contract.to_decimal(), &event.investor.to_string())?
                        .ok_or(ProcessorError::InvestorNotFound {
                            investor: event.investor.to_string(),
                            contract: contract.to_decimal(),
                        })?;
                if event
                    .security_amount
                    .to_decimal()
                    .ne(&investor.token_amount)
                {
                    // This should not happen, but if it does, we log it
                    // and continue processing the event.
                    warn!(
                        "Investment claim mismatch: from: {}, token amount: {}, expected: {}",
                        event.investor.to_string(),
                        event.security_amount.to_decimal(),
                        investor.token_amount
                    );
                }

                SecurityMintFundContract::sub_token_amount(
                    conn,
                    contract.to_decimal(),
                    investor.token_amount,
                    block_time,
                )?;
                let curr_currency_amount = investor.currency_amount;
                let curr_token_amount = investor.token_amount;
                let investor: Investor = investor.claim_investment(conn, block_time)?;
                InvestmentRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    investor: event.investor.to_string(),
                    currency_amount: curr_currency_amount,
                    token_amount: curr_token_amount,
                    currency_amount_balance: investor.currency_amount,
                    token_amount_balance: investor.token_amount,
                    investment_record_type: InvestmentRecordType::Claimed,
                    create_time: block_time,
                }
                .insert(conn)?;
                info!(
                    "Investment claimed: from: {}, token amount: {}, currency amount: {}",
                    event.investor.to_string(),
                    curr_token_amount,
                    curr_currency_amount
                );
            }
            Event::InvestmentDisbursed(event) => {
                SecurityMintFundContract::sub_currency_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    block_time,
                )?;
                info!(
                    "Investment disbursed: currency amount: {}",
                    event.currency_amount.to_decimal()
                );
            }
        }
    }

    Ok(())
}

fn to_db_fund_state(
    fund_state: security_mint_fund::FundState,
) -> (SecurityMintFundState, Option<String>) {
    match fund_state {
        security_mint_fund::FundState::Open => (SecurityMintFundState::Open, None),
        security_mint_fund::FundState::Success(receiver) => (
            SecurityMintFundState::Success,
            Some(receiver.address().to_string()),
        ),
        security_mint_fund::FundState::Fail => (SecurityMintFundState::Fail, None),
    }
}
