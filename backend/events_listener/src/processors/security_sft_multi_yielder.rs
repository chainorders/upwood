use chrono::NaiveDateTime;
use concordium_protocols::concordium_cis2_security::AgentWithRoles;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_sft_multi_yielder::{
    AgentRole, Event, UpsertYieldParams, YieldCalculation, YieldRemovedEvent,
};
use shared::db::cis2_security::Agent;
use shared::db::security_sft_multi_yielder::{Yield, YieldType};
use shared::db_shared::DbConn;
use tracing::instrument;

use super::cis2_utils::{ContractAddressToDecimal, TokenIdToDecimal};
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
                                (YieldType::SimpleInterest, rate)
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
                Yield::insert_batch(conn, &yields)?;
            }
            Event::YieldRemoved(YieldRemovedEvent {
                token_contract,
                token_id,
            }) => Yield::delete_batch(
                conn,
                contract.to_decimal(),
                token_contract.to_decimal(),
                token_id.to_decimal(),
            )?,
        }
    }

    Ok(())
}

fn role_to_string(r: AgentRole) -> String {
    match r {
        AgentRole::AddYield => "AddYield".to_string(),
        AgentRole::RemoveYield => "RemoveYield".to_string(),
        AgentRole::Operator => "Operator".to_string(),
    }
}
