//! This module contains the transaction processor for the Concordium RWA
//! backend. It includes the definition of the database module, as well as the
//! modules for the RWA identity registry, RWA market, RWA security NFT, and RWA
//! security SFT. It also defines the listener and API configuration struct, as
//! well as the contracts API configuration struct. The module provides
//! functions to run the contracts API server and listener, as well as to
//! generate the API client. It also includes helper functions to create the
//! listener, server routes, and service for the contracts API.

mod cis2_security;
mod security_sft_single;
use diesel::Connection;
use rust_decimal::Decimal;
use tracing::{debug, info, instrument, warn};
pub mod cis2_utils;
mod identity_registry;
mod nft_multi_rewarded;
mod security_mint_fund;
mod security_p2p_trading;
mod security_sft_rewards;

use std::collections::BTreeMap;

use chrono::{NaiveDateTime, Utc};
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName};
use concordium_rust_sdk::types::ContractAddress;
use shared::db::txn_listener::{
    CallType, ListenerContract, ListenerContractCallInsert, ListenerTransaction, ProcessorType,
};
use shared::db_shared::DbConn;

use crate::listener::{ContractCallType, ParsedBlock, ParsedTxn};

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("R2D2 pool Database error: {0}")]
    DatabasePoolError(#[from] r2d2::Error),
    #[error("Events Parse Error: {0}")]
    EventsParseError(#[from] concordium_rust_sdk::base::contracts_common::ParseError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}

pub type ProcessorFnType = fn(
    &mut DbConn,
    now: NaiveDateTime,
    &ContractAddress,
    &[ContractEvent],
) -> Result<(), ProcessorError>;

#[derive(Clone)]
pub struct Processors {
    pub processors_types: BTreeMap<(ModuleReference, OwnedContractName), ProcessorType>,
    pub processors:       BTreeMap<ProcessorType, ProcessorFnType>,
}

impl Processors {
    pub fn new() -> Self {
        let mut processors = Self {
            processors_types: BTreeMap::new(),
            processors:       BTreeMap::new(),
        };

        processors.insert(
            security_sft_rewards::module_ref(),
            security_sft_rewards::contract_name(),
            ProcessorType::SecuritySftRewards,
            security_sft_rewards::process_events as ProcessorFnType,
        );
        processors.insert(
            security_sft_single::module_ref(),
            security_sft_single::contract_name(),
            ProcessorType::SecuritySftSingle,
            security_sft_single::process_events as ProcessorFnType,
        );
        processors.insert(
            identity_registry::module_ref(),
            identity_registry::contract_name(),
            ProcessorType::IdentityRegistry,
            identity_registry::process_events as ProcessorFnType,
        );
        processors.insert(
            nft_multi_rewarded::module_ref(),
            nft_multi_rewarded::contract_name(),
            ProcessorType::NftMultiRewarded,
            nft_multi_rewarded::process_events as ProcessorFnType,
        );
        processors.insert(
            security_mint_fund::module_ref(),
            security_mint_fund::contract_name(),
            ProcessorType::SecurityMintFund,
            security_mint_fund::process_events as ProcessorFnType,
        );

        processors
    }

    fn insert(
        &mut self,
        module_ref: ModuleReference,
        contract_name: OwnedContractName,
        processor_type: ProcessorType,
        processor: ProcessorFnType,
    ) {
        self.processors_types
            .insert((module_ref, contract_name), processor_type);
        self.processors.insert(processor_type, processor);
    }

    pub fn find_type(
        &self,
        module_ref: &ModuleReference,
        contract_name: &OwnedContractName,
    ) -> Option<ProcessorType> {
        self.processors_types
            .get(&(*module_ref, contract_name.clone()))
            .copied()
    }

    pub fn find_by_type(&self, processor_type: &ProcessorType) -> Option<&ProcessorFnType> {
        self.processors.get(processor_type)
    }

    #[instrument(skip_all)]
    pub async fn process_block(
        &self,
        conn: &mut DbConn,
        ParsedBlock {
            block,
            transactions,
        }: &ParsedBlock,
        admin_account: &str,
    ) -> Result<(), ProcessorError> {
        let res = conn.transaction(|conn| {
            for txn in transactions.iter() {
                let is_txn_processed =
                    self.process_txn(conn, block.block_slot_time, txn, admin_account)?;

                if is_txn_processed {
                    ListenerTransaction {
                        block_hash:        block.block_hash.clone(),
                        block_height:      block.block_height,
                        block_slot_time:   block.block_slot_time,
                        transaction_index: txn.index.into(),
                        transaction_hash:  hex::encode(&txn.hash),
                    }
                    .insert(conn)?;
                }
            }

            // Update the last processed block in the database
            let id = block.insert(conn)?;
            Result::<_, ProcessorError>::Ok(id)
        })?;

        if res.is_none() {
            warn!("block {} already processed", block.block_height);
        }

        let lag = Utc::now() - block.block_slot_time.and_utc();
        info!(
            "Processed block {}, lag: days:{}, hours:{}, mins: {}",
            block.block_height,
            lag.num_days(),
            lag.num_hours(),
            lag.num_minutes(),
        );
        Ok(())
    }

    fn process_txn(
        &self,
        conn: &mut DbConn,
        block_slot_time: NaiveDateTime,
        txn: &ParsedTxn,
        admin_account: &str,
    ) -> Result<bool, ProcessorError> {
        let mut is_any_processed = false;
        for contract_call in &txn.contract_calls {
            let is_processed = match &contract_call.call_type {
                ContractCallType::Init(init) => {
                    // If the contract is owned by the owner, then we process the init call
                    if admin_account.eq(&txn.sender) {
                        if let Some(processor_type) =
                            self.find_type(&init.module_ref, &init.contract_name)
                        {
                            let contract = ListenerContract::new(
                                contract_call.contract,
                                &init.module_ref,
                                &txn.sender,
                                &init.contract_name,
                                processor_type,
                                block_slot_time,
                            )
                            .insert(conn)?;

                            let contract_call = ListenerContractCallInsert {
                                call_type:        CallType::Init,
                                ccd_amount:       init.amount.micro_ccd.into(),
                                contract_address: contract.contract_address,
                                entrypoint_name:  &init.contract_name.to_string(),
                                events_count:     init.events.len() as i32,
                                instigator:       &txn.sender.to_string(),
                                sender:           &txn.sender.to_string(),
                                transaction_hash: txn.hash.clone(),
                                created_at:       block_slot_time,
                            }
                            .insert(conn)?;
                            Some((contract, contract_call, Some(&init.events)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                ContractCallType::Update(update) => {
                    let contract = ListenerContract::find(conn, contract_call.contract)?;
                    match contract {
                        Some(contract) => {
                            let contract_call = ListenerContractCallInsert {
                                call_type:        CallType::Update,
                                ccd_amount:       update.amount.micro_ccd.into(),
                                entrypoint_name:  &update.receive_name.to_string(),
                                events_count:     update.events.len() as i32,
                                contract_address: contract_call.contract,
                                instigator:       &txn.sender.to_string(),
                                sender:           &update.sender.to_string(),
                                transaction_hash: txn.hash.clone(),
                                created_at:       block_slot_time,
                            }
                            .insert(conn)?;
                            Some((contract, contract_call, Some(&update.events)))
                        }
                        None => None,
                    }
                }
                ContractCallType::Upgraded { to, .. } => {
                    let contract = ListenerContract::find(conn, contract_call.contract)?;
                    match contract {
                        Some(contract) => {
                            let contract_call = ListenerContractCallInsert {
                                call_type:        CallType::Upgraded,
                                ccd_amount:       Decimal::ZERO,
                                entrypoint_name:  "",
                                events_count:     0,
                                contract_address: contract_call.contract,
                                instigator:       &txn.sender.to_string(),
                                sender:           &txn.sender.to_string(),
                                transaction_hash: txn.hash.clone(),
                                created_at:       block_slot_time,
                            }
                            .insert(conn)?;
                            Some((contract.update_module_ref(conn, to)?, contract_call, None))
                        }
                        None => None,
                    }
                }
            };

            match is_processed {
                Some((contract, contract_call, events)) => {
                    match self.find_by_type(&contract.processor_type) {
                        Some(processor) => {
                            let events_length = if let Some(events) = events {
                                processor(
                                    conn,
                                    block_slot_time,
                                    &contract.contract_address(),
                                    events,
                                )?;
                                events.len()
                            } else {
                                0
                            };
                            contract_call.update_processed(conn)?;
                            is_any_processed = true;
                            info!(
                                "Processed contract call contract: {}, sender: {}, events count: \
                                 {}",
                                contract.contract_address(),
                                contract_call.sender,
                                events_length
                            );
                        }
                        None => warn!(
                            "No processor found for contract: {} & type: {}",
                            contract.contract_address(),
                            contract.processor_type
                        ),
                    }
                }
                None => debug!(
                    "Contract call not processed: contract: {}, sender: {}",
                    contract_call.contract, txn.sender
                ),
            }
        }

        Ok(is_any_processed)
    }
}

impl Default for Processors {
    fn default() -> Self { Self::new() }
}
