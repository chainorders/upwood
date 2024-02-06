use async_trait::async_trait;
use bson::{doc, to_document};
use concordium_rust_sdk::types::{
    smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
    ContractAddress,
};
use concordium_rwa_identity_registry::event::Event;

use crate::{
    txn_listener::EventsProcessor,
    txn_processor::db::{DbAddress, DbContractAddress, ICollection},
};

use super::db::IContractDb;

/// `RwaIdentityRegistryProcessor` is a struct that processes events for the
/// rwa-identity-registry contract. It maintains a connection to a MongoDB
/// database and contains the module reference and name of the contract.
pub struct Processor<TDb: IContractDb> {
    /// Client to interact with the MongoDB database.
    pub db:         TDb,
    /// Module reference of the contract.
    pub module_ref: ModuleReference,
}

#[async_trait]
impl<TDb: Sync + Send + IContractDb> EventsProcessor for Processor<TDb> {
    /// Returns the name of the contract this processor is responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `OwnedContractName` of the contract.
    fn contract_name(&self) -> &OwnedContractName { self.db.contract_name() }

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
        &self,
        contract: &ContractAddress,
        events: &[ContractEvent],
    ) -> anyhow::Result<()> {
        for event in events {
            let parsed_event = event.parse::<Event>()?;
            log::info!("Event: {:?}", parsed_event);

            match parsed_event {
                Event::AgentAdded(e) => {
                    self.db.agents(contract).insert_one(DbAddress(e.agent)).await?;
                }
                Event::AgentRemoved(e) => {
                    self.db.agents(contract).delete_one(to_document(&DbAddress(e.agent))?).await?;
                }
                Event::IdentityRegistered(e) => {
                    self.db.identities(contract).insert_one(DbAddress(e.address)).await?;
                }
                Event::IdentityRemoved(e) => {
                    self.db
                        .identities(contract)
                        .delete_one(to_document(&DbAddress(e.address))?)
                        .await?;
                }
                Event::IssuerAdded(e) => {
                    self.db.issuers(contract).insert_one(DbContractAddress(e.issuer)).await?;
                }
                Event::IssuerRemoved(e) => {
                    self.db
                        .issuers(contract)
                        .delete_one(to_document(&DbContractAddress(e.issuer))?)
                        .await?;
                }
            }
        }

        Ok(())
    }
}
