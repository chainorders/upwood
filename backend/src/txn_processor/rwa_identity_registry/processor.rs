use super::db::{self};
use crate::{
    shared::db::{DbConn, DbPool},
    txn_listener::EventsProcessor,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::{
    smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
    ContractAddress,
};
use concordium_rwa_identity_registry::event::Event;
use log::{debug, info};

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
        let count = process_events(&mut conn, Utc::now(), contract, events)?;
        Ok(count as u64)
    }
}

pub fn process_events(
    conn: &mut DbConn,
    now: DateTime<Utc>,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> anyhow::Result<usize> {
    let mut process_events_count: usize = 0;
    for event in events {
        let parsed_event = event.parse::<Event>()?;
        debug!("Event: {}/{}", contract.index, contract.subindex);
        debug!("{:#?}", parsed_event);

        match parsed_event {
            Event::AgentAdded(e) => {
                let agent = db::Agent::new(e.agent, now, contract);
                db::insert_agent(conn, agent)?;
                info!("Identity Registry agent added: {}", e.agent);
                process_events_count += 1;
            }
            Event::AgentRemoved(e) => {
                db::remove_agent(conn, &e.agent)?;
                info!("Identity Registry agent removed: {}", e.agent);
                process_events_count += 1;
            }
            Event::IdentityRegistered(e) => {
                let identity = db::Identity::new(e.address, now, contract);
                db::insert_identity(conn, identity)?;
                info!("Identity Registry identity added: {}", e.address);
                process_events_count += 1;
            }
            Event::IdentityRemoved(e) => {
                db::remove_identity(conn, &e.address)?;
                info!("Identity Registry identity removed: {}", e.address);
                process_events_count += 1;
            }
            Event::IssuerAdded(e) => {
                let issuer = db::Issuer::new(e.issuer, now, contract);
                db::insert_issuer(conn, issuer)?;
                info!("Identity Registry issuer added: {}", e.issuer);
                process_events_count += 1;
            }
            Event::IssuerRemoved(e) => {
                db::remove_issuer(conn, e.issuer)?;
                info!("Identity Registry issuer removed: {}", e.issuer);
                process_events_count += 1;
            }
        }
    }

    Ok(process_events_count)
}
