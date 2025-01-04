use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_p2p_trading::{AddMarketParams, AgentRole, Event, SellEvent};
use shared::db::cis2_security::Agent;
use shared::db::security_p2p_trading::{Market, P2PTradeContract, SellRecord};
use shared::db_shared::DbConn;
use tracing::{info, instrument, trace};
use uuid::Uuid;

use crate::processors::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, RateToDecimal, TokenAmountToDecimal,
    TokenIdToDecimal,
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
                Market {
                    contract_address: contract.to_decimal(),
                    token_contract_address: token.contract.to_decimal(),
                    token_id: token.id.to_decimal(),
                    buyer: market.buyer.to_string(),
                    rate: rate_to_decimal(market.rate.numerator, market.rate.denominator),
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
            Event::Sell(SellEvent {
                token_amount,
                rate,
                seller,
                token_contract,
                token_id,
                currency_amount,
            }) => {
                let _ = Market::find(
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
                SellRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    token_id: token_id.to_decimal(),
                    token_contract_address: token_contract.to_decimal(),
                    seller: seller.to_string(),
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
