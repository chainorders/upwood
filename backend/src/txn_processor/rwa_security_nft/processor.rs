use crate::{
    shared::db::{DbConn, DbPool},
    txn_listener::EventsProcessor,
    txn_processor::cis2_processor,
};
use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::{
    smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
    ContractAddress,
};
use concordium_rwa_security_nft::event::{
    AgentUpdatedEvent, ComplianceAdded, Event, IdentityRegistryAdded, Paused, RecoverEvent,
    TokenFrozen,
};
use log::debug;

pub struct RwaSecurityNftProcessor {
    pub pool:          DbPool,
    /// Module reference of the contract.
    pub module_ref:    ModuleReference,
    /// Name of the contract.
    pub contract_name: OwnedContractName,
}

#[async_trait]
impl EventsProcessor for RwaSecurityNftProcessor {
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

    /// Processes the events of the contract.
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
    cis2_address: &ContractAddress,
    events: &[ContractEvent],
) -> anyhow::Result<()> {
    for event in events {
        let parsed_event = event.parse::<Event>()?;
        debug!("Event: {}/{}", cis2_address.index, cis2_address.subindex);
        debug!("{:#?}", parsed_event);

        match parsed_event {
            Event::AgentAdded(AgentUpdatedEvent {
                agent,
            }) => cis2_processor::agent_added(conn, agent, now, cis2_address)?,
            Event::AgentRemoved(AgentUpdatedEvent {
                agent,
            }) => cis2_processor::agent_removed(conn, cis2_address, agent)?,
            Event::ComplianceAdded(ComplianceAdded(compliance_contract)) => {
                cis2_processor::compliance_updated(conn, cis2_address, compliance_contract)?
            }
            Event::IdentityRegistryAdded(IdentityRegistryAdded(identity_registry_contract)) => {
                cis2_processor::identity_registry_updated(
                    conn,
                    cis2_address,
                    identity_registry_contract,
                )?
            }
            Event::Paused(Paused {
                token_id,
            }) => cis2_processor::token_paused(conn, cis2_address, token_id)?,
            Event::UnPaused(Paused {
                token_id,
            }) => cis2_processor::token_unpaused(conn, cis2_address, token_id)?,
            Event::Recovered(RecoverEvent {
                lost_account,
                new_account,
            }) => cis2_processor::account_recovered(conn, cis2_address, lost_account, new_account)?,
            Event::TokenFrozen(TokenFrozen {
                address,
                amount,
                token_id,
            }) => cis2_processor::token_frozen(conn, cis2_address, token_id, address, amount)?,
            Event::TokenUnFrozen(TokenFrozen {
                address,
                amount,
                token_id,
            }) => cis2_processor::token_un_frozen(conn, cis2_address, token_id, address, amount)?,
            Event::Cis2(e) => cis2_processor::cis2(conn, now, cis2_address, e)?,
        }
    }

    Ok(())
}
