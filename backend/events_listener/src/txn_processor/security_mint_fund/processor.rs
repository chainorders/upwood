use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use security_mint_fund::Event;
use shared::db::DbConn;
use tracing::{debug, instrument};

use super::db;
use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_utils::{ContractAddressToDecimal, TokenAmountToDecimal};

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
                db::SecurityMintFundContract::new(
                    contract.to_decimal(),
                    event.token,
                    event.investment_token,
                    event.currency_token,
                    event.rate,
                    event.fund_state,
                    now,
                )
                .insert(conn)?;
            }
            Event::FundStateUpdated(fund_state) => {
                db::SecurityMintFundContract::update_state(
                    conn,
                    contract.to_decimal(),
                    fund_state,
                    now,
                )?;
            }
            Event::Invested(event) => {
                db::Investor::new(
                    contract.to_decimal(),
                    &event.investor,
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )
                .upsert(conn)?;
                db::SecurityMintFundContract::add_investment_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
            }
            Event::InvestmentCancelled(event) => {
                db::Investor::cancel_investment(
                    conn,
                    contract.to_decimal(),
                    &event.investor,
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
                db::SecurityMintFundContract::sub_investment_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
            }
            Event::InvestmentClaimed(event) => {
                db::Investor::claim_investment(
                    conn,
                    contract.to_decimal(),
                    &event.investor,
                    event.security_amount.to_decimal(),
                    now,
                )?;
                db::SecurityMintFundContract::sub_token_amount(
                    conn,
                    contract.to_decimal(),
                    event.security_amount.to_decimal(),
                    now,
                )?;
            }
            Event::InvestmentDisbursed(event) => {
                db::SecurityMintFundContract::sub_currency_amount(
                    conn,
                    contract.to_decimal(),
                    event.currency_amount.to_decimal(),
                    now,
                )?;
                db::InvestmentRecordInsert {
                    contract_address:       contract.to_decimal(),
                    investor:               event.receiver.address().to_string(),
                    create_time:            now.naive_utc(),
                    currency_amount:        Some(event.currency_amount.to_decimal()),
                    token_amount:           None,
                    investment_record_type: db::InvestmentRecordType::Disbursed,
                }
                .insert(conn)?;
            }
        }
    }

    Ok(())
}
