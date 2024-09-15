use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use concordium_rwa_backend_shared::db::DbConn;
use concordium_rwa_identity_registry::event::Event;
use tracing::{debug, instrument};

use super::db::{self};
use crate::txn_listener::listener::ProcessorError;

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
#[instrument(skip_all, fields(contract = %contract, events = events.len()))]
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
                db::insert_agent(conn, db::Agent::new(e.agent, now, contract))?;
            }
            Event::AgentRemoved(e) => {
                db::remove_agent(conn, contract, &e.agent)?;
            }
            Event::IdentityRegistered(e) => {
                db::insert_identity(conn, db::Identity::new(&e.address, now, contract))?;
            }
            Event::IdentityRemoved(e) => {
                db::remove_identity(conn, contract, &e.address)?;
            }
            Event::IssuerAdded(e) => {
                db::insert_issuer(conn, db::Issuer::new(&e.issuer, now, contract))?;
            }
            Event::IssuerRemoved(e) => {
                db::remove_issuer(conn, contract, &e.issuer)?;
            }
        }
    }

    Ok(())
}
