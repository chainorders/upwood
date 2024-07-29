use super::db;
use crate::{
    shared::db::{DbConn, DbPool},
    txn_listener::EventsProcessor,
    txn_processor::rwa_security_nft::db::{
        SecurityCis2Operator, SecurityCis2RecoveryRecord, SecurityCis2Token,
        SecurityCis2TokenHolder,
    },
};
use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use concordium_cis2::{
    BurnEvent, Cis2Event, MintEvent, OperatorUpdate, TokenMetadataEvent, TransferEvent,
    UpdateOperatorEvent,
};
use concordium_rust_sdk::{
    base::contracts_common::{Cursor, Deserial, Serial},
    cis2::{self, TokenAmount},
    types::{
        smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
        ContractAddress,
    },
};
use concordium_rwa_security_nft::event::{
    AgentUpdatedEvent, ComplianceAdded, Event, IdentityRegistryAdded, Paused, RecoverEvent,
    TokenFrozen,
};
use diesel::Connection;
use log::debug;
use num_bigint::BigUint;
use num_traits::Zero;

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
            }) => {
                db::insert_agent(conn, db::Agent::new(agent, now, cis2_address))?;
            }
            Event::AgentRemoved(AgentUpdatedEvent {
                agent,
            }) => {
                db::remove_agent(conn, cis2_address, &agent)?;
            }
            Event::ComplianceAdded(ComplianceAdded(compliance_contract)) => {
                db::upsert_compliance(
                    conn,
                    &db::SecurityCis2ContractCompliance::new(cis2_address, &compliance_contract),
                )?;
            }
            Event::IdentityRegistryAdded(IdentityRegistryAdded(identity_registry_contract)) => {
                db::upsert_identity_registry(
                    conn,
                    &db::SecurityCis2ContractIdentityRegistry::new(
                        cis2_address,
                        &identity_registry_contract,
                    ),
                )?;
            }
            Event::Paused(Paused {
                token_id,
            }) => {
                let token_id: cis2::TokenId = token_id.to_string().parse()?;
                db::update_token_paused(conn, cis2_address, &token_id, true)?;
            }
            Event::UnPaused(Paused {
                token_id,
            }) => {
                let token_id: cis2::TokenId = token_id.to_string().parse()?;
                db::update_token_paused(conn, cis2_address, &token_id, false)?;
            }
            Event::Recovered(RecoverEvent {
                lost_account,
                new_account,
            }) => {
                let updated_rows = conn.transaction(|conn| {
                    db::insert_recovery_record(
                        conn,
                        &SecurityCis2RecoveryRecord::new(cis2_address, &lost_account, &new_account),
                    )?;
                    db::update_replace_holder(conn, cis2_address, &lost_account, &new_account)
                })?;
                debug!("account recovery, {} token ids updated", updated_rows);
            }
            Event::TokenFrozen(TokenFrozen {
                address,
                amount,
                token_id,
            }) => {
                let token_id: cis2::TokenId = token_id.to_string().parse()?;
                let amount: cis2::TokenAmount = to_cis2_token_amount(amount)?;
                db::update_balance_frozen(conn, cis2_address, &token_id, &address, &amount, true)?;
            }
            Event::TokenUnFrozen(TokenFrozen {
                address,
                amount,
                token_id,
            }) => {
                let token_id: cis2::TokenId = token_id.to_string().parse()?;
                let amount: cis2::TokenAmount = to_cis2_token_amount(amount)?;
                db::update_balance_frozen(conn, cis2_address, &token_id, &address, &amount, false)?;
            }
            Event::Cis2(e) => match e {
                Cis2Event::Mint(MintEvent {
                    token_id,
                    owner,
                    amount,
                }) => {
                    let token_id = token_id.to_string().parse()?;
                    let token_amount = to_cis2_token_amount(amount)?;
                    conn.transaction(|conn| {
                        db::insert_holder_or_add_balance(
                            conn,
                            &SecurityCis2TokenHolder::new(
                                cis2_address,
                                &token_id,
                                &owner,
                                &to_cis2_token_amount(amount)?,
                                &cis2::TokenAmount(BigUint::zero()),
                                now,
                            ),
                        )?;
                        db::update_supply(conn, cis2_address, &token_id, &token_amount, true)?;
                        Ok(())
                    })?;
                }
                Cis2Event::TokenMetadata(TokenMetadataEvent {
                    token_id,
                    metadata_url,
                }) => {
                    let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
                    db::insert_token_or_update_metadata(
                        conn,
                        &SecurityCis2Token::new(
                            cis2_address,
                            &token_id,
                            false,
                            metadata_url.url,
                            metadata_url.hash,
                            &TokenAmount(BigUint::zero()),
                            now,
                        ),
                    )?;
                }
                Cis2Event::Burn(BurnEvent {
                    token_id,
                    owner,
                    amount,
                }) => {
                    let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
                    let token_amount = to_cis2_token_amount(amount)?;
                    conn.transaction(|conn| {
                        db::update_sub_balance(
                            conn,
                            cis2_address,
                            &token_id,
                            &owner,
                            &token_amount,
                        )?;
                        db::update_supply(conn, cis2_address, &token_id, &token_amount, false)?;
                        Ok(())
                    })?;
                }
                Cis2Event::Transfer(TransferEvent {
                    token_id,
                    from,
                    to,
                    amount,
                }) => {
                    let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
                    let token_amount = to_cis2_token_amount(amount)?;
                    conn.transaction(|conn| {
                        db::update_sub_balance(
                            conn,
                            cis2_address,
                            &token_id,
                            &from,
                            &token_amount,
                        )?;
                        db::insert_holder_or_add_balance(
                            conn,
                            &SecurityCis2TokenHolder::new(
                                cis2_address,
                                &token_id,
                                &to,
                                &to_cis2_token_amount(amount)?,
                                &cis2::TokenAmount(BigUint::zero()),
                                now,
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                Cis2Event::UpdateOperator(UpdateOperatorEvent {
                    owner,
                    operator,
                    update,
                }) => {
                    let record = SecurityCis2Operator::new(cis2_address, &owner, &operator);
                    match update {
                        OperatorUpdate::Add => db::insert_operator(conn, &record)?,
                        OperatorUpdate::Remove => db::delete_operator(conn, &record)?,
                    }
                }
            },
        }
    }

    Ok(())
}

fn to_cis2_token_amount(
    amount: concordium_cis2::TokenAmountU8,
) -> Result<TokenAmount, anyhow::Error> {
    let mut bytes = vec![];
    amount
        .serial(&mut bytes)
        .map_err(|_| anyhow::Error::msg("error serializing amount to bytes"))?;
    let mut cursor: Cursor<_> = Cursor::new(bytes);
    Ok(cis2::TokenAmount::deserial(&mut cursor)?)
}
