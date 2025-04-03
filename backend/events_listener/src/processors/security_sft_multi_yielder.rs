use chrono::NaiveDateTime;
use concordium_protocols::concordium_cis2_security::AgentWithRoles;
use concordium_protocols::rate::Rate;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use diesel::{Connection, QueryResult};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use security_sft_multi_yielder::{
    AgentRole, Event, UpsertYieldParams, YieldCalculation, YieldDistributedEvent, YieldRemovedEvent,
};
use shared::db::cis2_security::Agent;
use shared::db::security_sft_multi_yielder::{Treasury, Yield, YieldDistribution, YieldType};
use shared::db_shared::DbConn;
use tracing::{info, instrument};
use uuid::Uuid;

use super::cis2_utils::{ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal};
use super::ProcessorError;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/security-sft-multi-yielder/contract.wasm.v1"
    ))
    .expect("Failed to parse security-sft-multi-yielder module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_sft_multi_yielder".to_string())
}

#[allow(clippy::too_many_arguments)]
#[instrument(
    name="sft_multi_yielder",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    _block_height: Decimal,
    block_time: NaiveDateTime,
    _txn_index: Decimal,
    _txn_sender: &str,
    _txn_instigator: &str,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let event = event.parse::<Event>().expect("Failed to parse event");
        match event {
            Event::AgentAdded(AgentWithRoles { address, roles }) => {
                Agent {
                    agent_address: address.to_string(),
                    cis2_address:  contract.to_decimal(),
                    roles:         roles.into_iter().map(|r| Some(role_to_string(r))).collect(),
                }
                .insert(conn)?;
            }
            Event::AgentRemoved(address) => {
                Agent::delete(conn, contract.to_decimal(), &address)?;
            }
            Event::YieldAdded(UpsertYieldParams {
                token_contract,
                token_id,
                yields,
            }) => {
                let yields = yields
                    .into_iter()
                    .map(|y| {
                        let (yield_type, rate) = match y.calculation {
                            YieldCalculation::Quantity(rate) => (YieldType::Quantity, rate),
                            YieldCalculation::SimpleInterest(rate) => {
                                (YieldType::SimpleIntrest, rate)
                            }
                        };

                        Yield {
                            contract_address: contract.to_decimal(),
                            token_contract_address: token_contract.to_decimal(),
                            token_id: token_id.to_decimal(),
                            yield_contract_address: y.contract.to_decimal(),
                            yield_token_id: y.token_id.to_decimal(),
                            yield_rate_numerator: rate.numerator.into(),
                            yield_rate_denominator: rate.denominator.into(),
                            yield_type,
                            create_time: block_time,
                        }
                    })
                    .collect::<Vec<_>>();
                conn.transaction(|conn| {
                    Yield::delete_batch(
                        conn,
                        contract.to_decimal(),
                        token_contract.to_decimal(),
                        token_id.to_decimal(),
                    )?;
                    info!(
                        "Old yields removed: {:?}",
                        (contract, token_contract, token_id)
                    );
                    Yield::insert_batch(conn, &yields)?;
                    yields
                        .iter()
                        .for_each(|yield_| info!("Yield added: {:?}", yield_));
                    QueryResult::Ok(())
                })?;
            }
            Event::YieldRemoved(YieldRemovedEvent {
                token_contract,
                token_id,
            }) => {
                Yield::delete_batch(
                    conn,
                    contract.to_decimal(),
                    token_contract.to_decimal(),
                    token_id.to_decimal(),
                )?;
                info!("Yield removed: {:?}", (contract, token_contract, token_id));
            }
            Event::YieldDistributed(YieldDistributedEvent {
                from_token,
                to_token,
                contract: token_contract,
                amount,
                to,
            }) => {
                let yield_distributions = Yield::find_batch(
                    conn,
                    contract.to_decimal(),
                    token_contract.to_decimal(),
                    from_token.to_decimal(),
                    to_token.to_decimal(),
                )?
                .into_iter()
                .map(|y| YieldDistribution {
                    id:                     Uuid::new_v4(),
                    contract_address:       y.contract_address,
                    from_token_version:     from_token.to_decimal(),
                    to_token_version:       to_token.to_decimal(),
                    token_amount:           amount.to_decimal(),
                    token_contract_address: y.token_contract_address,
                    yield_contract_address: y.yield_contract_address,
                    yield_token_id:         y.yield_token_id,
                    to_address:             to.to_string(),
                    yield_amount:           calculate_yield_amount(
                        y.yield_type,
                        y.yield_rate_numerator,
                        y.yield_rate_denominator,
                        amount.to_decimal(),
                        to_token.to_decimal() - from_token.to_decimal(),
                    ),
                    create_time:            block_time,
                })
                .collect::<Vec<_>>();
                YieldDistribution::insert_batch(conn, &yield_distributions)?;
                yield_distributions.iter().for_each(|yield_distribution| {
                    info!("Yield distributed: {:?}", yield_distribution)
                });
            }
            Event::TreasuryUpdated(address) => {
                Treasury {
                    contract_address: contract.to_decimal(),
                    treasury_address: address.to_string(),
                    create_time:      block_time,
                    update_time:      block_time,
                }
                .upsert(conn)?;
                info!("Treasury updated: {:?}", address.to_string());
            }
        }
    }

    Ok(())
}

fn role_to_string(r: AgentRole) -> String {
    match r {
        AgentRole::AddYield => "AddYield".to_string(),
        AgentRole::RemoveYield => "RemoveYield".to_string(),
        AgentRole::Operator => "Operator".to_string(),
        AgentRole::UpdateTreasury => "UpdateTreasury".to_string(),
    }
}

fn calculate_yield_amount(
    yeild_type: YieldType,
    rate_numerator: Decimal,
    rate_denominator: Decimal,
    amount: Decimal,
    token_version_diff: Decimal,
) -> Decimal {
    let rate = Rate {
        numerator:   rate_numerator
            .to_u64()
            .expect("Failed to convert numerator"),
        denominator: rate_denominator
            .to_u64()
            .expect("Failed to convert denominator"),
    };

    match yeild_type {
        YieldType::Quantity => YieldCalculation::Quantity(rate),
        YieldType::SimpleIntrest => YieldCalculation::SimpleInterest(rate),
    }
    .calculate_amount(
        &concordium_cis2::TokenAmountU64(amount.to_u64().expect("Failed to convert amount")),
        token_version_diff
            .to_u64()
            .expect("Failed to convert token_version_diff"),
    )
    .expect("Failed to calculate yield amount")
    .to_decimal()
}
