use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use shared::db::DbConn;
use security_p2p_trading::Event;
use tracing::{debug, instrument};

use super::db;
use crate::txn_listener::listener::ProcessorError;

#[instrument(
    name="security_p2p_trading_process_events",
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
                db::insert_contract(
                    conn,
                    db::Contract::new(contract, &event.token, &event.currency, now),
                )?;
            }
            Event::Sell(event) => {
                db::insert_deposit_or_update_add_amount(
                    conn,
                    db::Deposit::new(contract, &event.from, &event.amount, &event.rate, now),
                )?;
                db::update_contract_add_amount(conn, contract, &event.amount, now)?;
                db::insert_trading_record(
                    conn,
                    db::TradingRecordInsert::new_sell(
                        contract,
                        &event.from,
                        &event.amount,
                        &event.rate,
                        now,
                    ),
                )?;
            }
            Event::SellCancelled(event) => {
                db::update_deposit_sub_amount(conn, contract, &event.from, &event.amount, now)?;
                db::update_contract_sub_amount(conn, contract, &event.amount, now)?;
                db::insert_trading_record(
                    conn,
                    db::TradingRecordInsert::new_sell_cancelled(
                        contract,
                        &event.from,
                        &event.amount,
                        now,
                    ),
                )?;
            }
            Event::Exchange(event) => {
                db::update_deposit_sub_amount(
                    conn,
                    contract,
                    &event.seller,
                    &event.sell_amount,
                    now,
                )?;
                db::update_contract_sub_amount(conn, contract, &event.sell_amount, now)?;
                db::insert_trading_records(conn, vec![
                    db::TradingRecordInsert::new_exchange_sell(
                        contract,
                        &event.seller,
                        &event.sell_amount,
                        &event.payer,
                        &event.pay_amount,
                        now,
                    ),
                    db::TradingRecordInsert::new_exchange_buy(
                        contract,
                        &event.seller,
                        &event.sell_amount,
                        &event.payer,
                        &event.pay_amount,
                        now,
                    ),
                ])?;
            }
        }
    }

    Ok(())
}
