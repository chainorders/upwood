use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_mint_fund::Event;
use shared::db::security_mint_fund::{
    InvestmentRecordInsert, InvestmentRecordType, Investor, SecurityMintFundContract,
    SecurityMintFundState,
};
use shared::db_shared::DbConn;
use tracing::{debug, instrument};

use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};

#[instrument(
    name="security_mint_fund_process_events",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    now: DateTime<Utc>,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let event = event.parse::<Event>().expect("Failed to parse event");
        debug!(
            "Processing event for contract: {}/{}",
            contract.index, contract.subindex
        );
        debug!("Event details: {:#?}", event);

        match event {
            Event::Initialized(event) => {
                SecurityMintFundContract {
                    contract_address: contract.to_decimal(),
                    token_id: event.token.id.to_decimal(),
                    token_contract_address: event.token.contract.to_decimal(),
                    investment_token_id: event.investment_token.id.to_decimal(),
                    investment_token_contract_address: event.investment_token.contract.to_decimal(),
                    currency_token_id: event.currency_token.id.to_decimal(),
                    currency_token_contract_address: event.currency_token.contract.to_decimal(),
                    rate: rate_to_decimal(event.rate.numerator, event.rate.denominator),
                    fund_state: to_db_fund_state(event.fund_state),
                    currency_amount: Decimal::ZERO,
                    token_amount: Decimal::ZERO,
                    create_time: now.naive_utc(),
                    update_time: now.naive_utc(),
                }
                .insert(conn)?;
            }
            Event::FundStateUpdated(fund_state) => {
                SecurityMintFundContract::update_state(
                    conn,
                    contract.to_decimal(),
                    to_db_fund_state(fund_state),
                    now,
                )?;
            }
            Event::Invested(event) => {
                Investor::new(
                    contract.to_decimal(),
                    &event.investor,
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )
                .upsert(conn)?;
                SecurityMintFundContract::add_investment_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
            }
            Event::InvestmentCancelled(event) => {
                Investor::cancel_investment(
                    conn,
                    contract.to_decimal(),
                    &event.investor,
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
                SecurityMintFundContract::sub_investment_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
            }
            Event::InvestmentClaimed(event) => {
                Investor::claim_investment(
                    conn,
                    contract.to_decimal(),
                    &event.investor,
                    event.security_amount.to_decimal(),
                    now,
                )?;
                SecurityMintFundContract::sub_token_amount(
                    conn,
                    contract.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
            }
            Event::InvestmentDisbursed(event) => {
                SecurityMintFundContract::sub_currency_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    now,
                )?;
                InvestmentRecordInsert {
                    contract_address:       contract.to_decimal(),
                    investor:               event.receiver.address().to_string(),
                    create_time:            now.naive_utc(),
                    currency_amount:        Some(event.currency_amount.to_decimal()),
                    token_amount:           None,
                    investment_record_type: InvestmentRecordType::Disbursed,
                }
                .insert(conn)?;
            }
        }
    }

    Ok(())
}

fn to_db_fund_state(fund_state: security_mint_fund::FundState) -> SecurityMintFundState {
    match fund_state {
        security_mint_fund::FundState::Open => SecurityMintFundState::Open,
        security_mint_fund::FundState::Success(_) => SecurityMintFundState::Success,
        security_mint_fund::FundState::Fail => SecurityMintFundState::Fail,
    }
}
