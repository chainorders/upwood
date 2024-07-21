use super::db::RwaIdentityRegistryDb;
use crate::{
    shared::db::{DbAddress, DbContractAddress, ICollection},
    txn_listener::EventsProcessor,
};
use async_trait::async_trait;
use bson::to_document;
use concordium_rust_sdk::types::{
    smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
    ContractAddress,
};
use concordium_rwa_identity_registry::event::Event;

/// `RwaIdentityRegistryProcessor` is a struct that processes events for the
/// rwa-identity-registry contract. It maintains a connection to a MongoDB
/// database and contains the module reference and name of the contract.
pub struct RwaIdentityRegistryProcessor {
    /// Module reference of the contract.
    pub module_ref:    ModuleReference,
    pub contract_name: OwnedContractName,
    pub client:        mongodb::Client,
}

impl RwaIdentityRegistryProcessor {
    pub fn database(&self, contract: &ContractAddress) -> RwaIdentityRegistryDb {
        let db = self
            .client
            .database(&format!("{}-{}-{}", self.contract_name, contract.index, contract.subindex));

        RwaIdentityRegistryDb::init(db)
    }
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
        let mut process_events_count = 0u64;
        let mut db = self.database(contract);
        for event in events {
            let parsed_event = event.parse::<Event>()?;
            log::debug!("Event: {}/{} {:?}", contract.index, contract.subindex, parsed_event);

            match parsed_event {
                Event::AgentAdded(e) => {
                    db.agents.insert_one(DbAddress(e.agent)).await?;
                    process_events_count += 1;
                }
                Event::AgentRemoved(e) => {
                    db.agents.delete_one(to_document(&DbAddress(e.agent))?).await?;
                    process_events_count += 1;
                }
                Event::IdentityRegistered(e) => {
                    db.identities.insert_one(DbAddress(e.address)).await?;
                    process_events_count += 1;
                }
                Event::IdentityRemoved(e) => {
                    db.identities.delete_one(to_document(&DbAddress(e.address))?).await?;
                    process_events_count += 1;
                }
                Event::IssuerAdded(e) => {
                    db.issuers.insert_one(DbContractAddress(e.issuer)).await?;
                    process_events_count += 1;
                }
                Event::IssuerRemoved(e) => {
                    db.issuers.delete_one(to_document(&DbContractAddress(e.issuer))?).await?;
                    process_events_count += 1;
                }
            }
        }

        Ok(process_events_count)
    }
}
