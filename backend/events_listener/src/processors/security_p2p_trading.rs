use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_p2p_trading::Event;
use shared::db::security_p2p_trading::{
    P2PTradeContract, Seller, Trade, TradingRecord, TradingRecordType,
};
use shared::db_shared::DbConn;
use tracing::{info, instrument, trace};
use uuid::Uuid;

use crate::processors::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};
use crate::processors::ProcessorError;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/security-p2p-trading/contract.wasm.v1"
    ))
    .expect("Failed to parse security-mint-fund module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_p2p_trading".to_string())
}

#[instrument(
    name="p2p_trading",
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
        trace!("Event details: {:#?}", event);

        match event {
            Event::Initialized(event) => {
                P2PTradeContract {
                    contract_address: contract.to_decimal(),
                    token_id: event.token.id.to_decimal(),
                    token_contract_address: event.token.contract.to_decimal(),
                    currency_token_id: event.currency.id.to_decimal(),
                    currency_token_contract_address: event.currency.contract.to_decimal(),
                    token_amount: Decimal::ZERO,
                    create_time: block_time,
                    update_time: block_time,
                }
                .insert(conn)?;
                info!("initialized");
            }
            Event::Sell(event) => {
                let rate = rate_to_decimal(event.rate.numerator, event.rate.denominator);
                // Only a single deposit / sell position exists per seller
                let trader = Seller {
                    contract_address: contract.to_decimal(),
                    rate,
                    token_amount: event.amount.to_decimal(),
                    trader_address: event.from.to_string(),
                    create_time: block_time,
                    update_time: block_time,
                }
                .upsert(conn)?;
                P2PTradeContract::add_amount(
                    conn,
                    contract.to_decimal(),
                    event.amount.to_decimal(),
                    block_time,
                )?;
                TradingRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    trader_address: event.from.to_string(),
                    token_amount: event.amount.to_decimal(),
                    currency_amount: event.amount.to_decimal() * rate,
                    token_amount_balance: trader.token_amount,
                    currency_amount_balance: trader.token_amount * rate,
                    record_type: TradingRecordType::Sell,
                    create_time: block_time,
                }
                .insert(conn)?;
                info!(
                    "sell by: {}, amount: {}, rate: {}/{}",
                    event.from,
                    event.amount.to_decimal(),
                    event.rate.numerator,
                    event.rate.denominator
                );
            }
            Event::SellCancelled(event) => {
                let trader = Seller::sub_amount(
                    conn,
                    contract.to_decimal(),
                    &event.from,
                    event.amount.to_decimal(),
                    block_time,
                )?;
                P2PTradeContract::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.amount.to_decimal(),
                    block_time,
                )?;
                TradingRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    trader_address: event.from.to_string(),
                    token_amount: event.amount.to_decimal(),
                    currency_amount: event.amount.to_decimal() * trader.rate,
                    token_amount_balance: trader.token_amount,
                    currency_amount_balance: trader.token_amount * trader.rate,
                    record_type: TradingRecordType::SellCancel,
                    create_time: block_time,
                }
                .insert(conn)?;
                info!(
                    "sell cancelled by: {}, amount: {}",
                    event.from,
                    event.amount.to_decimal()
                );
            }
            Event::Exchange(event) => {
                let trader = Seller::sub_amount(
                    conn,
                    contract.to_decimal(),
                    &event.seller,
                    event.sell_amount.to_decimal(),
                    block_time,
                )?;
                P2PTradeContract::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.sell_amount.to_decimal(),
                    block_time,
                )?;
                TradingRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    trader_address: event.seller.to_string(),
                    token_amount: event.sell_amount.to_decimal(),
                    currency_amount: event.sell_amount.to_decimal() * trader.rate,
                    token_amount_balance: trader.token_amount,
                    currency_amount_balance: trader.token_amount * trader.rate,
                    record_type: TradingRecordType::Exchange,
                    create_time: block_time,
                }
                .insert(conn)?;
                Trade {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    seller_address: event.seller.to_string(),
                    buyer_address: event.payer.to_string(),
                    token_amount: event.sell_amount.to_decimal(),
                    currency_amount: event.pay_amount.to_decimal(),
                    rate: trader.rate,
                    create_time: block_time,
                }
                .insert(conn)?;
                info!(
                    "exchange by: {}, to: {}, sell amount: {}, pay amount: {}",
                    event.seller,
                    event.payer,
                    event.sell_amount.to_decimal(),
                    event.pay_amount.to_decimal()
                );
            }
        }
    }

    Ok(())
}
