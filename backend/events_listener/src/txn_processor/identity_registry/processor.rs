use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use concordium_rwa_identity_registry::event::Event;
use shared::db::identity_registry::{Agent, Identity, Issuer};
use shared::db_shared::DbConn;
use tracing::{debug, instrument};

use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_utils::ContractAddressToDecimal;

/// Processes the events of the rwa-identity-registry contract.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the `DbConn` connection.
/// * `now` - The current time as a `DateTime<Utc>`.
/// * `contract` - A reference to the `ContractAddress` of the contract whose
///   events are to be processed.
/// * `events` - A slice of `ContractEvent`s to be processed.
///
/// # Returns
///
/// * A `Result` indicating the success or failure of the operation.
#[instrument(
    name="identity_registry_process_events",
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
                Agent::new(e.agent, now, contract.to_decimal()).insert(conn)?;
            }
            Event::AgentRemoved(e) => {
                Agent::delete(conn, contract.to_decimal(), &e.agent)?;
            }
            Event::IdentityRegistered(e) => {
                Identity::new(&e.address, now, contract.to_decimal()).insert(conn)?;
            }
            Event::IdentityRemoved(e) => {
                Identity::delete(conn, contract.to_decimal(), &e.address)?;
            }
            Event::IssuerAdded(e) => {
                Issuer::new(e.issuer.to_decimal(), now, contract.to_decimal()).insert(conn)?;
            }
            Event::IssuerRemoved(e) => {
                Issuer::delete(conn, contract.to_decimal(), e.issuer.to_decimal())?;
            }
        }
    }

    Ok(())
}
