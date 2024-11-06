use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use security_p2p_trading::Event;
use shared::db::DbConn;
use tracing::{debug, instrument};

use super::db;
use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_utils::{ContractAddressToDecimal, TokenAmountToDecimal};

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
                db::P2PTradeContract::new(
                    contract.to_decimal(),
                    &event.token,
                    &event.currency,
                    &event.rate,
                    now,
                )
                .insert(conn)?;
            }
            Event::RateUpdated(rate) => {
                db::P2PTradeContract::update_rate(conn, contract.to_decimal(), &rate, now)?;
            }
            Event::Sell(event) => {
                db::Deposit::new(
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    now,
                )
                .upsert(conn)?;
                db::P2PTradeContract::add_amount(
                    conn,
                    contract.to_decimal(),
                    event.amount.to_decimal(),
                    now,
                )?;
                db::TradingRecordInsert::new_sell(
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    now,
                )
                .insert(conn)?;
            }
            Event::SellCancelled(event) => {
                db::Deposit::sub_amount(
                    conn,
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    now,
                )?;
                db::P2PTradeContract::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.amount.to_decimal(),
                    now,
                )?;
                db::TradingRecordInsert::new_sell_cancelled(
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    now,
                )
                .insert(conn)?;
            }
            Event::Exchange(event) => {
                db::Deposit::sub_amount(
                    conn,
                    contract.to_decimal(),
                    &event.seller,
                    event.sell_amount.to_decimal(),
                    now,
                )?;
                db::P2PTradeContract::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.sell_amount.to_decimal(),
                    now,
                )?;
                db::TradingRecordInsert::insert_batch(conn, vec![
                    db::TradingRecordInsert::new_exchange_sell(
                        contract.to_decimal(),
                        &event.seller,
                        event.sell_amount.to_decimal(),
                        &event.payer,
                        event.pay_amount.to_decimal(),
                        now,
                    ),
                    db::TradingRecordInsert::new_exchange_buy(
                        contract.to_decimal(),
                        &event.seller,
                        event.sell_amount.to_decimal(),
                        &event.payer,
                        event.pay_amount.to_decimal(),
                        now,
                    ),
                ])?;
            }
        }
    }

    Ok(())
}
