use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use diesel::Connection;
use rust_decimal::Decimal;
use security_p2p_trading::{AddMarketParams, AgentRole, Event, ExchangeEvent};
use shared::db::cis2_security::Agent;
use shared::db::security_p2p_trading::{
    ExchangeRecord, Market, MarketType, P2PTradeContract, Trader,
};
use shared::db_shared::{DbConn, DbResult};
use tracing::{info, instrument, trace, warn};
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
        let event = event.parse::<Event>();
        let event = match event {
            Ok(event) => event,
            Err(e) => {
                warn!("Failed to parse event: {:?}", e);
                continue;
            }
        };
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
                info!(
                    "Trading Contract initialized: {:?} with currency: {:?}",
                    contract, currency_token
                );
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
            Event::MarketAdded(AddMarketParams {
                token_contract,
                market,
            }) => {
                let market = conn.transaction::<_, ProcessorError, _>(|conn| {
                    let contract = P2PTradeContract::find(conn, contract.to_decimal())?.ok_or(
                        ProcessorError::TradeContractNotFound {
                            contract: contract.to_decimal(),
                        },
                    )?;

                    {
                        // Check if the market already exists
                        let market = Market::find(
                            conn,
                            contract.contract_address,
                            token_contract.to_decimal(),
                        )?;
                        if let Some(market) = market {
                            // If it exists, remove it
                            market.delete(conn)?;
                            info!("Market removed: {:?}", market);
                        };
                    }

                    // Insert the new market
                    let market = match market {
                        security_p2p_trading::Market::Mint(market) => Market {
                            market_type: MarketType::Mint,
                            contract_address: contract.contract_address,
                            currency_token_id: contract.currency_token_id,
                            currency_token_contract_address: contract
                                .currency_token_contract_address,
                            liquidity_provider: market.liquidity_provider.to_string(),
                            token_contract_address: token_contract.to_decimal(),
                            token_id: None,
                            token_id_calculation_start: Some(market.token_id.start.millis.into()),
                            token_id_calculation_diff_millis: Some(
                                market.token_id.diff_millis.into(),
                            ),
                            sell_rate_numerator: Some(market.rate.numerator.into()),
                            sell_rate_denominator: Some(market.rate.denominator.into()),
                            buy_rate_numerator: None,
                            buy_rate_denominator: None,
                            max_token_amount: market.max_token_amount.to_decimal(),
                            max_currency_amount: None,
                            currency_in_amount: 0.into(),
                            currency_out_amount: 0.into(),
                            token_in_amount: 0.into(),
                            token_out_amount: 0.into(),
                            create_time: block_time,
                            update_time: block_time,
                        },
                        security_p2p_trading::Market::Transfer(market) => Market {
                            market_type: MarketType::Transfer,
                            contract_address: contract.contract_address,
                            currency_token_id: contract.currency_token_id,
                            currency_token_contract_address: contract
                                .currency_token_contract_address,
                            liquidity_provider: market.liquidity_provider.to_string(),
                            token_contract_address: token_contract.to_decimal(),
                            token_id: Some(market.token_id.to_decimal()),
                            token_id_calculation_start: None,
                            token_id_calculation_diff_millis: None,
                            buy_rate_numerator: Some(market.buy_rate.numerator.into()),
                            buy_rate_denominator: Some(market.buy_rate.denominator.into()),
                            sell_rate_numerator: Some(market.sell_rate.numerator.into()),
                            sell_rate_denominator: Some(market.sell_rate.denominator.into()),
                            max_token_amount: market.max_token_amount.to_decimal(),
                            max_currency_amount: Some(market.max_currency_amount.to_decimal()),
                            create_time: block_time,
                            update_time: block_time,
                            token_in_amount: 0.into(),
                            currency_in_amount: 0.into(),
                            token_out_amount: 0.into(),
                            currency_out_amount: 0.into(),
                        },
                    };
                    let market = market.insert(conn)?;
                    Ok(market)
                })?;
                info!("Market added: {:?}", market);
            }
            Event::MarketRemoved(market) => {
                conn.transaction(|conn| {
                    let market = Market::find(conn, contract.to_decimal(), market.to_decimal())?;
                    match market {
                        Some(market) => {
                            market.delete(conn)?;
                            info!("Market removed: {:?}", market);
                        }
                        None => {
                            warn!("Market not found: {:?}", market);
                        }
                    }
                    DbResult::Ok(())
                })?;
            }
            Event::Exchanged(ExchangeEvent {
                token_amount,
                rate,
                seller,
                buyer,
                token_contract,
                token_id,
                currency_amount,
                exchange_type,
            }) => {
                conn.transaction::<_, ProcessorError, _>(|conn| {
                    let contract = P2PTradeContract::find(conn, contract.to_decimal())?.ok_or(
                        ProcessorError::TradeContractNotFound {
                            contract: contract.to_decimal(),
                        },
                    )?;

                    let mut market =
                        Market::find(conn, contract.contract_address, token_contract.to_decimal())?
                            .ok_or(ProcessorError::MarketNotFound {
                                contract:       contract.contract_address,
                                token_contract: token_contract.to_decimal(),
                            })?;
                    match exchange_type {
                        security_p2p_trading::ExchangeType::Buy
                        | security_p2p_trading::ExchangeType::Mint => {
                            market.currency_in_amount += currency_amount.to_decimal();
                            market.token_out_amount += token_amount.to_decimal();
                            market.max_token_amount -= token_amount.to_decimal();
                            market.max_currency_amount = market
                                .max_currency_amount
                                .map(|v| v + currency_amount.to_decimal());
                        }
                        security_p2p_trading::ExchangeType::Sell => {
                            market.currency_out_amount += currency_amount.to_decimal();
                            market.token_in_amount += token_amount.to_decimal();
                            market.max_token_amount += token_amount.to_decimal();
                            market.max_currency_amount = market
                                .max_currency_amount
                                .map(|v| v - currency_amount.to_decimal());
                        }
                    }
                    market.update_time = block_time;
                    market.update(conn)?;

                    Trader::find(
                        conn,
                        contract.contract_address,
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
                        contract_address: contract.contract_address,
                        token_contract_address: token_contract.to_decimal(),
                        token_id: token_id.to_decimal(),
                        trader: seller.to_string(),
                        token_in_amount: 0.into(),
                        token_out_amount: token_amount.to_decimal(),
                        currency_in_amount: currency_amount.to_decimal(),
                        currency_out_amount: 0.into(),
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        create_time: block_time,
                        update_time: block_time,
                    })
                    .upsert(conn)?;

                    Trader::find(
                        conn,
                        contract.contract_address,
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
                        contract_address: contract.contract_address,
                        token_contract_address: token_contract.to_decimal(),
                        token_id: token_id.to_decimal(),
                        trader: buyer.to_string(),
                        token_in_amount: token_amount.to_decimal(),
                        token_out_amount: 0.into(),
                        currency_in_amount: 0.into(),
                        currency_out_amount: currency_amount.to_decimal(),
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        create_time: block_time,
                        update_time: block_time,
                    })
                    .upsert(conn)?;

                    ExchangeRecord {
                        id: Uuid::new_v4(),
                        block_height,
                        txn_index,
                        contract_address: contract.contract_address,
                        token_id: token_id.to_decimal(),
                        token_contract_address: token_contract.to_decimal(),
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        seller: seller.to_string(),
                        buyer: buyer.to_string(),
                        currency_amount: currency_amount.to_decimal(),
                        token_amount: token_amount.to_decimal(),
                        create_time: block_time,
                        rate: rate.to_decimal(),
                        exchange_record_type: match exchange_type {
                            security_p2p_trading::ExchangeType::Buy => {
                                shared::db::security_p2p_trading::ExchangeRecordType::Buy
                            }
                            security_p2p_trading::ExchangeType::Sell => {
                                shared::db::security_p2p_trading::ExchangeRecordType::Sell
                            }
                            security_p2p_trading::ExchangeType::Mint => {
                                shared::db::security_p2p_trading::ExchangeRecordType::Mint
                            }
                        },
                    }
                    .insert(conn)?;

                    Ok(())
                })?;

                info!(
                    "Exchanged: {:?}",
                    (
                        seller.to_string(),
                        buyer.to_string(),
                        token_amount,
                        currency_amount
                    )
                );
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
