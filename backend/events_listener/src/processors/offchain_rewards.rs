use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use offchain_rewards::types::Event;
use rust_decimal::Decimal;
use shared::db::offchain_rewards::{
    OffchainRewardClaim, OffchainRewardee, OffchainRewardsContact, OffchainRewardsContractAgent,
};
use shared::db_shared::DbConn;
use tracing::{instrument, warn};

use super::cis2_utils::{ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal};
use crate::processors::ProcessorError;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/offchain-rewards/contract.wasm.v1"
    ))
    .expect("Failed to parse offchain-rewards module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_offchain_rewards".to_string())
}

#[allow(clippy::too_many_arguments)]
#[instrument(
    name="offchain_rewards",
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
        match event {
            Event::Init(init_param) => {
                let db_contract = OffchainRewardsContact::find(conn, contract.to_decimal())?;
                match db_contract {
                    Some(_) => {
                        return Err(ProcessorError::ContractAlreadyExists(*contract));
                    }
                    None => {
                        let contract = OffchainRewardsContact {
                            contract_address: contract.to_decimal(),
                            treasury_address: init_param.treasury.to_string(),
                            create_time:      block_time,
                            update_time:      block_time,
                        };
                        contract.insert(conn)?;
                    }
                }
            }
            Event::AgentAdded(address) => {
                let db_agent = OffchainRewardsContractAgent::find(
                    conn,
                    contract.to_decimal(),
                    address.to_string(),
                )?;
                match db_agent {
                    Some(_) => {
                        warn!(
                            "Agent already exists for offchain rewards contract {} and address {}",
                            contract, address
                        );
                    }
                    None => {
                        let agent = OffchainRewardsContractAgent {
                            agent_address:    address.to_string(),
                            contract_address: contract.to_decimal(),
                            create_time:      block_time,
                            update_time:      block_time,
                        };
                        agent.insert(conn)?;
                    }
                }
            }
            Event::AgentRemoved(address) => {
                let db_agent = OffchainRewardsContractAgent::find(
                    conn,
                    contract.to_decimal(),
                    address.to_string(),
                )?;
                match db_agent {
                    Some(agent) => {
                        agent.delete(conn)?;
                    }
                    None => {
                        warn!(
                            "Agent does not exist for offchain rewards contract {} and address {}",
                            contract, address
                        );
                    }
                }
            }
            Event::Claimed(event) => {
                let rewardee = OffchainRewardee::find(
                    conn,
                    contract.to_decimal(),
                    &event.account_address.to_string(),
                )?;
                let rewardee = match rewardee {
                    Some(rewardee) => OffchainRewardee {
                        nonce: event.nonce.into(),
                        update_time: block_time,
                        ..rewardee
                    }
                    .update(conn)?,
                    None => OffchainRewardee {
                        account_address:  event.account_address.to_string(),
                        contract_address: contract.to_decimal(),
                        nonce:            event.nonce.into(),
                        create_time:      block_time,
                        update_time:      block_time,
                    }
                    .insert(conn)?,
                };
                OffchainRewardClaim {
                    account_address: rewardee.account_address,
                    block_height,
                    contract_address: rewardee.contract_address,
                    nonce: rewardee.nonce,
                    reward_amount: event.reward_amount.to_decimal(),
                    reward_token_contract_address: event.reward_token_contract.to_decimal(),
                    reward_token_id: event.reward_token_id.to_decimal(),
                    txn_index,
                    create_time: block_time,
                    id: uuid::Uuid::new_v4(),
                    reward_id: event.reward_id,
                }
                .insert(conn)?;
            }
        }
    }
    Ok(())
}
