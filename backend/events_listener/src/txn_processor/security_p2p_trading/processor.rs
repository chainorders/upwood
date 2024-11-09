use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_p2p_trading::Event;
use shared::db::security_p2p_trading::{Deposit, P2PTradeContract, TradingRecordInsert};
use shared::db_shared::DbConn;
use tracing::{debug, instrument};

use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};

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
                P2PTradeContract {
                    contract_address: contract.to_decimal(),
                    token_id: event.token.id.to_decimal(),
                    token_contract_address: event.token.contract.to_decimal(),
                    currency_token_id: event.currency.id.to_decimal(),
                    currency_token_contract_address: event.currency.contract.to_decimal(),
                    token_amount: Decimal::ZERO,
                    create_time: now.naive_utc(),
                    update_time: now.naive_utc(),
                }
                .insert(conn)?;
            }
            Event::Sell(event) => {
                // Only a single deposit / sell position exists per trader
                Deposit {
                    contract_address: contract.to_decimal(),
                    rate:             rate_to_decimal(event.rate.numerator, event.rate.denominator),
                    token_amount:     event.amount.to_decimal(),
                    trader_address:   event.from.to_string(),
                    create_time:      now.naive_utc(),
                    update_time:      now.naive_utc(),
                }
                .upsert(conn)?;
                P2PTradeContract::add_amount(
                    conn,
                    contract.to_decimal(),
                    event.amount.to_decimal(),
                    now,
                )?;
                TradingRecordInsert::new_sell(
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    now,
                )
                .insert(conn)?;
            }
            Event::SellCancelled(event) => {
                Deposit::sub_amount(
                    conn,
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    now,
                )?;
                P2PTradeContract::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.amount.to_decimal(),
                    now,
                )?;
                TradingRecordInsert::new_sell_cancelled(
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    now,
                )
                .insert(conn)?;
            }
            Event::Exchange(event) => {
                Deposit::sub_amount(
                    conn,
                    contract.to_decimal(),
                    &event.seller,
                    event.sell_amount.to_decimal(),
                    now,
                )?;
                P2PTradeContract::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.sell_amount.to_decimal(),
                    now,
                )?;
                TradingRecordInsert::insert_batch(conn, vec![
                    TradingRecordInsert::new_exchange_sell(
                        contract.to_decimal(),
                        &event.seller,
                        event.sell_amount.to_decimal(),
                        &event.payer,
                        event.pay_amount.to_decimal(),
                        now,
                    ),
                    TradingRecordInsert::new_exchange_buy(
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
