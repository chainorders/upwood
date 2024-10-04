use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use shared::db::DbConn;
use security_mint_fund::Event;
use tracing::{debug, instrument};

use super::db;
use crate::txn_listener::listener::ProcessorError;

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
                db::insert_fund(
                    conn,
                    db::Contract::new(
                        contract,
                        event.token,
                        event.investment_token,
                        event.currency_token,
                        event.rate,
                        event.fund_state,
                        now,
                    ),
                )?;
            }
            Event::FundStateUpdated(fund_state) => {
                db::update_fund_state(conn, contract, fund_state, now)?;
            }
            Event::Invested(event) => {
                db::insert_investor_or_update_add_investment(
                    conn,
                    db::Investor::new(
                        contract,
                        &event.investor,
                        event.currency_amount,
                        event.security_amount,
                        now,
                    ),
                )?;
                db::update_fund_add_investment_amount(
                    conn,
                    contract,
                    &event.currency_amount,
                    &event.security_amount,
                    now,
                )?;
                db::insert_investment_record(
                    conn,
                    db::InvestmentRecordInsert::new(
                        contract,
                        &event.investor.into(),
                        Some(&event.currency_amount),
                        Some(&event.security_amount),
                        db::InvestmentRecordType::Invested,
                        now,
                    ),
                )?;
            }
            Event::InvestmentCancelled(event) => {
                db::update_investor_sub_investment(
                    conn,
                    contract,
                    &event.investor,
                    &event.currency_amount,
                    &event.security_amount,
                    now,
                )?;
                db::update_fund_sub_investment_amount(
                    conn,
                    contract,
                    &event.currency_amount,
                    &event.security_amount,
                    now,
                )?;
                db::insert_investment_record(
                    conn,
                    db::InvestmentRecordInsert::new(
                        contract,
                        &event.investor.into(),
                        Some(&event.currency_amount),
                        Some(&event.security_amount),
                        db::InvestmentRecordType::Cancelled,
                        now,
                    ),
                )?;
            }
            Event::InvestmentClaimed(event) => {
                db::update_investor_sub_investment_token_amount(
                    conn,
                    contract,
                    &event.investor,
                    &event.security_amount,
                    now,
                )?;
                db::update_fund_sub_token_amount(conn, contract, &event.security_amount, now)?;
                db::insert_investment_record(
                    conn,
                    db::InvestmentRecordInsert::new(
                        contract,
                        &event.investor.into(),
                        None,
                        Some(&event.security_amount),
                        db::InvestmentRecordType::Claimed,
                        now,
                    ),
                )?;
            }
            Event::InvestmentDisbursed(event) => {
                db::update_fund_sub_currency_amount(conn, contract, &event.currency_amount, now)?;
                db::insert_investment_record(
                    conn,
                    db::InvestmentRecordInsert::new(
                        contract,
                        &event.receiver.address(),
                        Some(&event.currency_amount),
                        None,
                        db::InvestmentRecordType::Claimed,
                        now,
                    ),
                )?;
            }
        }
    }

    Ok(())
}
