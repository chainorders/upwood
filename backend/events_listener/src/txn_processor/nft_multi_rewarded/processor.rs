use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use nft_multi_rewarded::types::Event;
use shared::db::DbConn;
use tracing::{debug, instrument};

use super::db;
use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_security;
use crate::txn_processor::cis2_utils::{ContractAddressToDecimal, TokenIdToDecimal};

#[instrument(
    name="nft_multi_rewarded_process_events",
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
            Event::AgentAdded(e) => {
                cis2_security::db::Agent::new(e, now, contract.to_decimal(), vec![])
                    .insert(conn)?;
            }
            Event::AgentRemoved(e) => {
                cis2_security::db::Agent::delete(conn, contract.to_decimal(), &e)?;
            }
            Event::Cis2(event) => {
                cis2_security::processor::process_events_cis2(conn, now, contract, event)?;
            }
            Event::RewardTokenUpdated(e) => {
                db::NftMultiRewardedContract::new(
                    contract.to_decimal(),
                    e.reward_token.contract.to_decimal(),
                    e.reward_token.id.to_decimal(),
                    now,
                )
                .upsert(conn)?;
            }
            Event::NonceUpdated(address, nonce) => {
                db::AddressNonce {
                    address:          address.to_string(),
                    contract_address: contract.to_decimal(),
                    nonce:            nonce as i64,
                }
                .upsert(conn)?;
            }
        }
    }

    Ok(())
}
