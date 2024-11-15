use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_p2p_trading::Event;
use shared::db::security_p2p_trading::{
    P2PTradeContract, Trade, Trader, TradingRecord, TradingRecordType,
};
use shared::db_shared::DbConn;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::processors::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};
use crate::processors::ProcessorError;

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
                let rate = rate_to_decimal(event.rate.numerator, event.rate.denominator);
                // Only a single deposit / sell position exists per trader
                let trader = Trader {
                    contract_address: contract.to_decimal(),
                    rate,
                    token_amount: event.amount.to_decimal(),
                    trader_address: event.from.to_string(),
                    create_time: now.naive_utc(),
                    update_time: now.naive_utc(),
                }
                .upsert(conn)?;
                P2PTradeContract::add_amount(
                    conn,
                    contract.to_decimal(),
                    event.amount.to_decimal(),
                    now,
                )?;
                TradingRecord {
                    id: Uuid::new_v4(),
                    contract_address: contract.to_decimal(),
                    trader_address: event.from.to_string(),
                    token_amount: event.amount.to_decimal(),
                    currency_amount: event.amount.to_decimal() * rate,
                    token_amount_balance: trader.token_amount,
                    currency_amount_balance: trader.token_amount * rate,
                    record_type: TradingRecordType::Sell,
                    create_time: now.naive_utc(),
                }
                .insert(conn)?;
            }
            Event::SellCancelled(event) => {
                let trader = Trader::sub_amount(
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
                TradingRecord {
                    id: Uuid::new_v4(),
                    contract_address: contract.to_decimal(),
                    trader_address: event.from.to_string(),
                    token_amount: event.amount.to_decimal(),
                    currency_amount: event.amount.to_decimal() * trader.rate,
                    token_amount_balance: trader.token_amount,
                    currency_amount_balance: trader.token_amount * trader.rate,
                    record_type: TradingRecordType::SellCancel,
                    create_time: now.naive_utc(),
                }
                .insert(conn)?;
            }
            Event::Exchange(event) => {
                let trader = Trader::sub_amount(
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
                TradingRecord {
                    id: Uuid::new_v4(),
                    contract_address: contract.to_decimal(),
                    trader_address: event.seller.to_string(),
                    token_amount: event.sell_amount.to_decimal(),
                    currency_amount: event.sell_amount.to_decimal() * trader.rate,
                    token_amount_balance: trader.token_amount,
                    currency_amount_balance: trader.token_amount * trader.rate,
                    record_type: TradingRecordType::Exchange,
                    create_time: now.naive_utc(),
                }
                .insert(conn)?;
                Trade {
                    id:               Uuid::new_v4(),
                    contract_address: contract.to_decimal(),
                    seller_address:   event.seller.to_string(),
                    buyer_address:    event.payer.to_string(),
                    token_amount:     event.sell_amount.to_decimal(),
                    currency_amount:  event.pay_amount.to_decimal(),
                    rate:             trader.rate,
                    create_time:      now.naive_utc(),
                }
                .insert(conn)?;
            }
        }
    }

    Ok(())
}
