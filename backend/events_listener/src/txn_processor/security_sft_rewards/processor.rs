use chrono::{DateTime, Utc};
use concordium_protocols::concordium_cis2_security::Cis2SecurityEvent;
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use security_sft_rewards::types::{AgentRole, Event, TokenAmount, TokenId};
use shared::db::DbConn;
use tracing::{debug, instrument};

use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_security;

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
            Event::RewardAdded(event) => todo!(),
            Event::RewardClaimed(event) => todo!(),
        }
    }

    Ok(())
}
