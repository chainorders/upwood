use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use nft_multi_rewarded::types::Event;
use rust_decimal::Decimal;
use shared::db::cis2_security::Agent;
use shared::db::nft_multi_rewarded::{AddressNonce, NftMultiRewardedContract};
use shared::db_shared::DbConn;
use tracing::{info, instrument, trace};

use crate::processors::cis2_utils::{ContractAddressToDecimal, TokenIdToDecimal};
use crate::processors::{cis2_security, ProcessorError};

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/nft-multi-rewarded/contract.wasm.v1"
    ))
    .expect("Failed to parse nft-multi-rewarded module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_nft_multi_rewarded".to_string())
}

#[allow(clippy::too_many_arguments)]
#[instrument(
    name="nft",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    block_height: Decimal,
    block_time: NaiveDateTime,
    txn_index: Decimal,
    txn_sender: &str,
    txn_instigator: &str,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    let contract = contract.to_decimal();
    for event in events {
        let parsed_event = event.parse::<Event>().expect("Failed to parse event");
        trace!("Event details: {:#?}", parsed_event);

        match parsed_event {
            Event::AgentAdded(agent) => {
                let agent = match Agent::find(conn, contract, &agent.to_string())? {
                    Some(agent) => agent,
                    None => Agent {
                        agent_address: agent.to_string(),
                        cis2_address:  contract,
                        roles:         vec![],
                    }
                    .insert(conn)?,
                };
                info!("Agent: {} added", agent.agent_address.to_string());
            }
            Event::AgentRemoved(e) => {
                Agent::find(conn, contract, &e.to_string())?
                    .ok_or_else(|| ProcessorError::Cis2AgentNotFound {
                        contract,
                        agent: e.to_string(),
                    })?
                    .delete(conn)?;
                info!("Agent: {} removed", e.to_string());
            }
            Event::Cis2(event) => {
                cis2_security::process_events_cis2(
                    conn,
                    block_height,
                    block_time,
                    txn_index,
                    txn_sender,
                    txn_instigator,
                    contract,
                    event,
                )?;
            }
            Event::RewardTokenUpdated(e) => {
                NftMultiRewardedContract::new(
                    contract,
                    e.reward_token.contract.to_decimal(),
                    e.reward_token.id.to_decimal(),
                    block_time,
                )
                .upsert(conn)?;
                info!("Reward token updated: {:?}", e.reward_token);
            }
            Event::NonceUpdated(address, nonce) => {
                AddressNonce {
                    address:          address.to_string(),
                    contract_address: contract,
                    nonce:            nonce as i64,
                }
                .upsert(conn)?;
                info!("Nonce updated: {}, nonce: {}", address.to_string(), nonce);
            }
        }
    }

    Ok(())
}
