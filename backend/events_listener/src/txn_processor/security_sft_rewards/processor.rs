use chrono::{DateTime, Utc};
use concordium_protocols::concordium_cis2_security::Cis2SecurityEvent;
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_sft_rewards::types::{AgentRole, Event, TokenAmount, TokenId};
use shared::db::security_sft_rewards::{
    ContractReward, RewardClaimed, RewardToken,
};
use shared::db_shared::DbConn;
use tracing::{debug, instrument};

use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_security;
use crate::txn_processor::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};
pub const TRACKED_TOKEN_ID: Decimal = Decimal::ZERO;

fn process_cis2_security_event(
    conn: &mut DbConn,
    now: DateTime<Utc>,
    contract: &ContractAddress,
    event: Cis2SecurityEvent<TokenId, TokenAmount, AgentRole>,
) -> Result<(), ProcessorError> {
    cis2_security::processor::process_parsed_event(conn, contract, event, now)
}

#[instrument(
    name="security_sft_rewards_process_events",
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
        let parsed_event = event.parse::<Event>().expect("Failed to parse event");
        debug!(
            "Processing event for contract: {}/{}",
            contract.index, contract.subindex
        );
        debug!("Event details: {:#?}", parsed_event);

        match parsed_event {
            Event::AgentAdded(a) => {
                process_cis2_security_event(conn, now, contract, Cis2SecurityEvent::AgentAdded(a))?
            }
            Event::AgentRemoved(a) => process_cis2_security_event(
                conn,
                now,
                contract,
                Cis2SecurityEvent::AgentRemoved(a),
            )?,
            Event::Cis2(event) => {
                process_cis2_security_event(conn, now, contract, Cis2SecurityEvent::Cis2(event))?
            }
            Event::ComplianceAdded(a) => process_cis2_security_event(
                conn,
                now,
                contract,
                Cis2SecurityEvent::ComplianceAdded(a),
            )?,
            Event::IdentityRegistryAdded(a) => process_cis2_security_event(
                conn,
                now,
                contract,
                Cis2SecurityEvent::IdentityRegistryAdded(a),
            )?,
            Event::Paused(a) => {
                process_cis2_security_event(conn, now, contract, Cis2SecurityEvent::Paused(a))?
            }
            Event::UnPaused(a) => {
                process_cis2_security_event(conn, now, contract, Cis2SecurityEvent::UnPaused(a))?
            }
            Event::Recovered(a) => {
                process_cis2_security_event(conn, now, contract, Cis2SecurityEvent::Recovered(a))?
            }
            Event::TokenFrozen(a) => {
                process_cis2_security_event(conn, now, contract, Cis2SecurityEvent::TokenFrozen(a))?
            }
            Event::TokenUnFrozen(a) => process_cis2_security_event(
                conn,
                now,
                contract,
                Cis2SecurityEvent::TokenUnFrozen(a),
            )?,
            Event::RewardAdded(event) => {
                ContractReward {
                    contract_address:        contract.to_decimal(),
                    rewarded_token_contract: event.rewarded_token_contract.to_decimal(),
                    rewarded_token_id:       event.rewarded_token_id.to_decimal(),
                    reward_amount:           event.reward_amount.to_decimal(),
                    create_time:             now.naive_utc(),
                    update_time:             now.naive_utc(),
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
                    create_time:             now.naive_utc(),
                    update_time:             now.naive_utc(),
                }
                .insert(conn)?;
            }
            Event::RewardClaimed(event) => {
                ContractReward::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.rewarded_token_contract.to_decimal(),
                    event.rewarded_token_id.to_decimal(),
                    event.reward_amount.to_decimal(),
                    now.naive_utc(),
                )?;
                RewardToken::sub_amount(
                    conn,
                    contract.to_decimal(),
                    event.token_id.to_decimal(),
                    event.rewarded_token_contract.to_decimal(),
                    event.rewarded_token_id.to_decimal(),
                    event.reward_amount.to_decimal(),
                    now.naive_utc(),
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
                    create_time: now.naive_utc(),
                    update_time: now.naive_utc(),
                }
                .insert(conn)?;
            }
        }
    }

    Ok(())
}
