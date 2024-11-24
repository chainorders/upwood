use chrono::NaiveDateTime;
use concordium_protocols::concordium_cis2_security::Cis2SecurityEvent;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_sft_rewards::types::{AgentRole, Event, TokenAmount, TokenId};
use shared::db::security_sft_rewards::{ContractReward, RewardClaimed, RewardToken};
use shared::db_shared::DbConn;
use tracing::{info, instrument, trace};

use crate::processors::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};
use crate::processors::{cis2_security, ProcessorError};
pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/security-sft-rewards/contract.wasm.v1"
    ))
    .expect("Failed to parse security-sft-rewards module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_sft_rewards".to_string())
}

fn process_cis2_security_event(
    conn: &mut DbConn,
    block_height: Decimal,
    block_time: NaiveDateTime,
    txn_index: Decimal,
    contract: &ContractAddress,
    event: Cis2SecurityEvent<TokenId, TokenAmount, AgentRole>,
) -> Result<(), ProcessorError> {
    cis2_security::process_parsed_event(conn, contract, event, block_time, block_height, txn_index)
}

#[instrument(
    name="security_sft_rewards",
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
        let parsed_event = event.parse::<Event>().expect("Failed to parse event");
        trace!("Event details: {:#?}", parsed_event);

        match parsed_event {
            Event::AgentAdded(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::AgentAdded(a),
            )?,
            Event::AgentRemoved(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::AgentRemoved(a),
            )?,
            Event::Cis2(event) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::Cis2(event),
            )?,
            Event::ComplianceAdded(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::ComplianceAdded(a),
            )?,
            Event::IdentityRegistryAdded(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::IdentityRegistryAdded(a),
            )?,
            Event::Paused(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::Paused(a),
            )?,
            Event::UnPaused(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::UnPaused(a),
            )?,
            Event::Recovered(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::Recovered(a),
            )?,
            Event::TokenFrozen(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::TokenFrozen(a),
            )?,
            Event::TokenUnFrozen(a) => process_cis2_security_event(
                conn,
                block_height,
                block_time,
                txn_index,
                contract,
                Cis2SecurityEvent::TokenUnFrozen(a),
            )?,
            Event::RewardAdded(event) => {
                ContractReward {
                    contract_address:        contract.to_decimal(),
                    rewarded_token_contract: event.rewarded_token_contract.to_decimal(),
                    rewarded_token_id:       event.rewarded_token_id.to_decimal(),
                    reward_amount:           event.reward_amount.to_decimal(),
                    create_time:             block_time,
                    update_time:             block_time,
                }
                .upsert_add_amount(conn)?;
                RewardToken {
                    contract_address:        contract.to_decimal(),
                    token_id:                event.token_id.to_decimal(),
                    reward_amount:           event.reward_amount.to_decimal(),
                    reward_rate:             rate_to_decimal(
                        event.reward_rate.numerator,
                        event.reward_rate.denominator,
                    ),
                    rewarded_token_contract: event.rewarded_token_contract.to_decimal(),
                    rewarded_token_id:       event.rewarded_token_id.to_decimal(),
                    create_time:             block_time,
                    update_time:             block_time,
                }
                .insert(conn)?;
                info!(
                    "Reward added on token: {}, rewarded token: {}/{}, amount: {}, rate: {}/{}",
                    event.token_id,
                    event.rewarded_token_contract,
                    event.rewarded_token_id,
                    event.reward_amount.to_decimal(),
                    event.reward_rate.numerator,
                    event.reward_rate.denominator
                );
            }
            Event::RewardClaimed(event) => {
                ContractReward::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.rewarded_token_contract.to_decimal(),
                    event.rewarded_token_id.to_decimal(),
                    event.reward_amount.to_decimal(),
                    block_time,
                )?;
                RewardToken::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.token_id.to_decimal(),
                    event.rewarded_token_contract.to_decimal(),
                    event.rewarded_token_id.to_decimal(),
                    event.reward_amount.to_decimal(),
                    block_time,
                )?;
                RewardClaimed {
                    id: uuid::Uuid::new_v4(),
                    contract_address: contract.to_decimal(),
                    token_id: event.token_id.to_decimal(),
                    holder_address: event.owner.to_string(),
                    token_amount: event.amount.to_decimal(),
                    rewarded_token_contract: event.rewarded_token_contract.to_decimal(),
                    rewarded_token_id: event.rewarded_token_id.to_decimal(),
                    reward_amount: event.reward_amount.to_decimal(),
                    create_time: block_time,
                    update_time: block_time,
                }
                .insert(conn)?;
                info!(
                    "Reward claimed on token: {}, rewarded token: {}/{}, amount: {}, by: {}",
                    event.token_id,
                    event.rewarded_token_contract,
                    event.rewarded_token_id,
                    event.reward_amount.to_decimal(),
                    event.owner.to_string()
                );
            }
        }
    }

    Ok(())
}
