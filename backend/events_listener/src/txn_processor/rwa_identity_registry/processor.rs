use super::db::{self};
use crate::txn_listener::EventsProcessor;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::{
    smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
    ContractAddress,
};
use concordium_rwa_backend_shared::db::{DbConn, DbPool};
use concordium_rwa_identity_registry::event::{
    AgentUpdatedEvent, Event, IdentityUpdatedEvent, IssuerUpdatedEvent,
};
use log::debug;

/// `RwaIdentityRegistryProcessor` is a struct that processes events for the
/// rwa-identity-registry contract. It maintains a connection to a MongoDB
/// database and contains the module reference and name of the contract.
pub struct RwaIdentityRegistryProcessor {
    /// Module reference of the contract.
    pub module_ref:    ModuleReference,
    pub contract_name: OwnedContractName,
    pub pool:          DbPool,
}

#[async_trait]
impl EventsProcessor for RwaIdentityRegistryProcessor {
    /// Returns the name of the contract this processor is responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `OwnedContractName` of the contract.
    fn contract_name(&self) -> &OwnedContractName { &self.contract_name }

    /// Returns the module reference of the contract this processor is
    /// responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `ModuleReference` of the contract.
    fn module_ref(&self) -> &ModuleReference { &self.module_ref }

    /// Processes the events of the rwa-identity-registry contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - A reference to the `ContractAddress` of the contract
    ///   whose events are to be processed.
    /// * `events` - A slice of `ContractEvent`s to be processed.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    async fn process_events(
        &mut self,
        contract: &ContractAddress,
        events: &[ContractEvent],
    ) -> anyhow::Result<u64> {
        let mut conn = self.pool.get()?;
        process_events(&mut conn, Utc::now(), contract, events)?;
        Ok(events.len() as u64)
    }
}

pub fn process_events(
    conn: &mut DbConn,
    now: DateTime<Utc>,
    identity_registry_address: &ContractAddress,
    events: &[ContractEvent],
) -> anyhow::Result<()> {
    for event in events {
        let parsed_event = event.parse::<Event>()?;
        debug!("Event: {}/{}", identity_registry_address.index, identity_registry_address.subindex);
        debug!("{:#?}", parsed_event);
        match parsed_event {
            Event::AgentAdded(AgentUpdatedEvent {
                agent,
            }) => {
                db::insert_agent(conn, db::Agent::new(agent, now, identity_registry_address))?;
            }
            Event::AgentRemoved(AgentUpdatedEvent {
                agent,
            }) => {
                db::remove_agent(conn, identity_registry_address, &agent)?;
            }
            Event::IdentityRegistered(IdentityUpdatedEvent {
                address,
            }) => {
                let identity = db::Identity::new(&address, now, identity_registry_address);
                db::insert_identity(conn, identity)?;
            }
            Event::IdentityRemoved(IdentityUpdatedEvent {
                address,
            }) => {
                db::remove_identity(conn, &address)?;
            }
            Event::IssuerAdded(IssuerUpdatedEvent {
                issuer,
            }) => {
                db::insert_issuer(conn, db::Issuer::new(&issuer, now, identity_registry_address))?;
            }
            Event::IssuerRemoved(IssuerUpdatedEvent {
                issuer,
            }) => {
                db::remove_issuer(conn, identity_registry_address, &issuer)?;
            }
        }
    }

    Ok(())
}
