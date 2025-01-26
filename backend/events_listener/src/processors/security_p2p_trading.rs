use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_p2p_trading::{AddMarketParams, AgentRole, Event, ExchangeEvent};
use shared::db::cis2_security::Agent;
use shared::db::security_p2p_trading::{ExchangeRecord, Market, P2PTradeContract, Trader};
use shared::db_shared::DbConn;
use tracing::{info, instrument, trace};
use uuid::Uuid;

use crate::processors::cis2_utils::{
    ContractAddressToDecimal, RateToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
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

#[allow(clippy::too_many_arguments)]
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
    _txn_sender: &str,
    _txn_instigator: &str,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let event = event.parse::<Event>().expect("Failed to parse event");
        trace!("Event details: {:#?}", event);

        match event {
            Event::Initialized(currency_token) => {
                P2PTradeContract {
                    contract_address:                contract.to_decimal(),
                    currency_token_id:               currency_token.id.to_decimal(),
                    currency_token_contract_address: currency_token.contract.to_decimal(),
                    create_time:                     block_time,
                    total_sell_currency_amount:      0.into(),
                }
                .insert(conn)?;
                info!("initialized");
            }
            Event::AgentAdded(agent) => {
                Agent::new(
                    agent.address,
                    block_time,
                    contract.to_decimal(),
                    agent.roles.iter().map(agent_role_to_string).collect(),
                )
                .insert(conn)?;
            }
            Event::AgentRemoved(agent) => {
                Agent::delete(conn, contract.to_decimal(), &agent)?;
            }
            Event::MarketAdded(AddMarketParams { token, market }) => {
                let contract = P2PTradeContract::find(conn, contract.to_decimal())?.ok_or(
                    ProcessorError::TradeContractNotFound {
                        contract: contract.to_decimal(),
                    },
                )?;
                Market {
                    contract_address: contract.contract_address,
                    token_contract_address: token.contract.to_decimal(),
                    token_id: token.id.to_decimal(),
                    currency_token_id: contract.currency_token_id,
                    currency_token_contract_address: contract.currency_token_contract_address,
                    liquidity_provider: market.liquidity_provider.to_string(),
                    buy_rate_numerator: market.buy_rate.numerator.into(),
                    buy_rate_denominator: market.buy_rate.denominator.into(),
                    sell_rate_numerator: market.sell_rate.numerator.into(),
                    sell_rate_denominator: market.sell_rate.denominator.into(),
                    total_sell_currency_amount: 0.into(),
                    total_sell_token_amount: 0.into(),
                    create_time: block_time,
                    update_time: block_time,
                }
                .insert(conn)?;
            }
            Event::MarketRemoved(market) => {
                Market::delete(
                    conn,
                    contract.to_decimal(),
                    market.id.to_decimal(),
                    market.contract.to_decimal(),
                )?;
            }
            Event::Exchanged(ExchangeEvent {
                token_amount,
                rate,
                seller,
                buyer,
                token_contract,
                token_id,
                currency_amount,
            }) => {
                let market = Market::find(
                    conn,
                    contract.to_decimal(),
                    token_id.to_decimal(),
                    token_contract.to_decimal(),
                )?
                .map(|mut market| {
                    market.total_sell_currency_amount += currency_amount.to_decimal();
                    market.total_sell_token_amount += token_amount.to_decimal();
                    market.update_time = block_time;
                    market
                })
                .ok_or(ProcessorError::MarketNotFound {
                    contract:       contract.to_decimal(),
                    token_id:       token_id.to_decimal(),
                    token_contract: token_contract.to_decimal(),
                })?
                .update(conn)?;
                Trader::find(
                    conn,
                    contract.to_decimal(),
                    token_id.to_decimal(),
                    token_contract.to_decimal(),
                    seller.to_string(),
                )?
                .map(|mut seller| {
                    seller.token_out_amount += token_amount.to_decimal();
                    seller.currency_in_amount += currency_amount.to_decimal();
                    seller.update_time = block_time;
                    seller
                })
                .unwrap_or_else(|| Trader {
                    contract_address: contract.to_decimal(),
                    token_contract_address: token_contract.to_decimal(),
                    token_id: token_id.to_decimal(),
                    trader: seller.to_string(),
                    token_in_amount: 0.into(),
                    token_out_amount: token_amount.to_decimal(),
                    currency_in_amount: currency_amount.to_decimal(),
                    currency_out_amount: 0.into(),
                    currency_token_id: market.currency_token_id,
                    currency_token_contract_address: market.currency_token_contract_address,
                    create_time: block_time,
                    update_time: block_time,
                })
                .upsert(conn)?;
                Trader::find(
                    conn,
                    contract.to_decimal(),
                    token_id.to_decimal(),
                    token_contract.to_decimal(),
                    buyer.to_string(),
                )?
                .map(|mut buyer| {
                    buyer.token_in_amount += token_amount.to_decimal();
                    buyer.currency_out_amount += currency_amount.to_decimal();
                    buyer.update_time = block_time;
                    buyer
                })
                .unwrap_or_else(|| Trader {
                    contract_address: contract.to_decimal(),
                    token_contract_address: token_contract.to_decimal(),
                    token_id: token_id.to_decimal(),
                    trader: buyer.to_string(),
                    token_in_amount: token_amount.to_decimal(),
                    token_out_amount: 0.into(),
                    currency_in_amount: 0.into(),
                    currency_out_amount: currency_amount.to_decimal(),
                    currency_token_id: market.currency_token_id,
                    currency_token_contract_address: market.currency_token_contract_address,
                    create_time: block_time,
                    update_time: block_time,
                })
                .upsert(conn)?;
                ExchangeRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    token_id: token_id.to_decimal(),
                    token_contract_address: token_contract.to_decimal(),
                    currency_token_id: market.currency_token_id,
                    currency_token_contract_address: market.currency_token_contract_address,
                    seller: seller.to_string(),
                    buyer: buyer.to_string(),
                    currency_amount: currency_amount.to_decimal(),
                    token_amount: token_amount.to_decimal(),
                    create_time: block_time,
                    rate: rate.to_decimal(),
                }
                .insert(conn)?;
            }
        }
    }

    Ok(())
}

fn agent_role_to_string(agent_role: &AgentRole) -> String {
    match agent_role {
        AgentRole::AddMarket => "AddMarket".to_string(),
        AgentRole::RemoveMarket => "RemoveMarket".to_string(),
        AgentRole::Operator => "Operator".to_string(),
    }
}
