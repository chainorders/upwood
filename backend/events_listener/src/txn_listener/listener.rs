use std::collections::BTreeMap;
use std::time::Duration;

use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::hashes::TransactionHash;
use concordium_rust_sdk::base::smart_contracts::OwnedReceiveName;
use concordium_rust_sdk::common::types::Amount;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::queries::BlockInfo;
use concordium_rust_sdk::types::smart_contracts::{
    ContractEvent, ModuleReference, OwnedContractName,
};
use concordium_rust_sdk::types::{
    AbsoluteBlockHeight, AccountTransactionEffects, Address, BlockItemSummary,
    BlockItemSummaryDetails, ContractAddress, ContractInitializedEvent, ContractTraceElement,
    InstanceUpdatedEvent, TransactionIndex,
};
use concordium_rust_sdk::v2;
use concordium_rwa_backend_shared::db::DbConn;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, PgConnection};
use futures::TryStreamExt;
use tracing::{info, instrument, trace, warn};

use super::db::{self, ListenerContractCallInsert, ListenerTransaction};
use crate::txn_listener::db::CallType;

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("R2D2 pool Database error: {0}")]
    DatabasePoolError(#[from] r2d2::Error),
    #[error("Events Parse Error: {0}")]
    EventsParseError(#[from] concordium_rust_sdk::base::contracts_common::ParseError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}

pub struct ContractCallTxn {
    pub index:  TransactionIndex,
    pub hash:   TransactionHash,
    pub sender: AccountAddress,
}

impl ContractCallTxn {
    pub fn new(index: TransactionIndex, hash: TransactionHash, sender: AccountAddress) -> Self {
        Self {
            index,
            hash,
            sender,
        }
    }
}

pub struct ContractCall {
    call_type: ContractCallType,
    contract:  ContractAddress,
    txn:       ContractCallTxn,
}

pub struct Update {
    pub sender:       Address,
    pub amount:       Amount,
    pub events:       Vec<ContractEvent>,
    pub receive_name: OwnedReceiveName,
}

pub struct Init {
    pub module_ref:    ModuleReference,
    pub contract_name: OwnedContractName,
    pub amount:        Amount,
    pub events:        Vec<ContractEvent>,
}

pub enum ContractCallType {
    Init(Init),
    Update(Update),
    Upgraded {
        from: ModuleReference,
        to:   ModuleReference,
    },
}

impl ContractCallType {
    pub fn create_init(init: ContractInitializedEvent) -> Self {
        Self::Init(Init {
            module_ref:    init.origin_ref,
            contract_name: init.init_name,
            amount:        init.amount,
            events:        init.events,
        })
    }

    pub fn create_update(
        event: InstanceUpdatedEvent,
        interrupt_events: Option<Vec<ContractEvent>>,
    ) -> Self {
        let events = interrupt_events
            .map(|events| [events, event.events.clone()].concat())
            .unwrap_or(event.events);
        Self::Update(Update {
            sender: event.instigator,
            amount: event.amount,
            receive_name: event.receive_name,
            events,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ListenerError {
    #[error("R2D2 pool Database error: {0}")]
    DatabasePoolError(#[from] r2d2::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Concordium node query error: {0}")]
    QueryError(#[from] concordium_rust_sdk::endpoints::QueryError),
    #[error("Processor error: {0}")]
    ProcessorError(#[from] ProcessorError),
    #[error("Grpc error: {0}")]
    GrpcError(#[from] concordium_rust_sdk::v2::Status),
    #[error("Finalized Block Timeout. Try increasing timeout and retrying")]
    FinalizedBlockTimeout,
    #[error("Finalized Block Stream ended")]
    FinalizedBlockStreamEnded,
}

pub type ProcessorFnType = fn(
    &mut DbConn,
    now: DateTime<Utc>,
    &ContractAddress,
    &[ContractEvent],
) -> Result<(), ProcessorError>;

/// `TransactionsListener` is a struct that listens to transactions from a
/// Concordium node and processes them. It maintains a connection to the node
/// and a MongoDB database, and uses a set of processors to process the
/// transactions.
pub struct TransactionsListener {
    account:              AccountAddress, // Account address to listen to
    processors:           BTreeMap<(ModuleReference, OwnedContractName), ProcessorFnType>,
    database:             Pool<ConnectionManager<PgConnection>>, // postgres pool
    default_block_height: AbsoluteBlockHeight, // Default block height to start from
    client:               v2::Client,
}

impl TransactionsListener {
    /// Constructs a new `TransactionsListener`.
    /// # Returns
    ///
    /// * A new `TransactionsListener`.
    pub fn new(
        client: v2::Client,
        pool: Pool<ConnectionManager<PgConnection>>,
        account: AccountAddress,
        processors: BTreeMap<(ModuleReference, OwnedContractName), ProcessorFnType>,
        default_block_height: AbsoluteBlockHeight,
    ) -> Self {
        Self {
            account,
            client,
            database: pool,
            processors,
            default_block_height,
        }
    }

    /// Starts listening to transactions from the Concordium node.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    #[instrument(skip_all)]
    pub async fn listen(mut self) -> Result<(), ListenerError> {
        let mut conn = self.database.get()?;
        let block_height = get_block_height_or(&mut conn, self.default_block_height).await?;
        info!("Starting from block {}", block_height.height);

        let mut finalized_block_stream =
            self.client.get_finalized_blocks_from(block_height).await?;

        loop {
            let (error, finalized_blocks) = finalized_block_stream
                .next_chunk_timeout(1000, Duration::from_millis(500))
                .await
                .map_err(|_| ListenerError::FinalizedBlockTimeout)?;
            for block in &finalized_blocks {
                let block = self.client.get_block_info(block.height).await?.response;
                if block.transaction_count.eq(&0u64) {
                    trace!("Block {block:?} has no transactions");
                    continue;
                }

                process_block(
                    &mut self.client,
                    &mut conn,
                    &block,
                    &self.account,
                    &self.processors,
                )
                .await?;
            }

            info!("Processed chunk of {} blocks", finalized_blocks.len());
            if error {
                return Err(ListenerError::FinalizedBlockStreamEnded);
            }
        }
    }
}

#[instrument(skip_all)]
async fn process_block(
    client: &mut v2::Client,
    conn: &mut DbConn,
    block: &BlockInfo,
    contract_owner: &AccountAddress,
    processors: &BTreeMap<(ModuleReference, OwnedContractName), ProcessorFnType>,
) -> Result<(), ListenerError> {
    let contract_calls = client
        .get_block_transaction_events(block.block_hash)
        .await?
        .response
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .filter_map(parse_block_item_summary)
        .flatten()
        .collect();
    conn.transaction(|conn| {
        process_contract_calls(contract_owner, processors, conn, block, contract_calls)?;
        db::update_last_processed_block(conn, block).map_err(ListenerError::DatabaseError)
    })?;
    let lag = Utc::now() - block.block_slot_time;
    info!(
        "Processed block {}, lag: days:{}, hours:{}, mins: {}",
        block.block_height.height,
        lag.num_days(),
        lag.num_hours(),
        lag.num_minutes(),
    );
    Ok(())
}

#[instrument(skip_all)]
fn process_contract_calls(
    contract_owner: &AccountAddress,
    processors: &BTreeMap<(ModuleReference, OwnedContractName), ProcessorFnType>,
    conn: &mut DbConn,
    block: &BlockInfo,
    contract_calls: Vec<ContractCall>,
) -> Result<(), ListenerError> {
    for contract_call in contract_calls {
        let is_processed = match &contract_call.call_type {
            ContractCallType::Init(init) => {
                if contract_owner.eq(&contract_call.txn.sender) {
                    processors
                        .get(&(init.module_ref, init.contract_name.clone()))
                        .map(|process| {
                            db::add_contract(
                                conn,
                                &contract_call.contract,
                                &init.module_ref,
                                &init.contract_name,
                                &contract_call.txn.sender,
                            )?;
                            process(
                                conn,
                                block.block_slot_time,
                                &contract_call.contract,
                                &init.events,
                            )
                        })
                        .transpose()?
                        .is_some()
                } else {
                    false
                }
            }
            ContractCallType::Update(update) => db::find_contract(conn, &contract_call.contract)?
                .and_then(|(module_ref, contract_name)| {
                    processors.get(&(module_ref, contract_name))
                })
                .map(|process| {
                    process(
                        conn,
                        block.block_slot_time,
                        &contract_call.contract,
                        &update.events,
                    )
                })
                .transpose()?
                .is_some(),
            ContractCallType::Upgraded { to, .. } => {
                db::find_contract(conn, &contract_call.contract)?
                    .map(|_| db::update_contract(conn, &contract_call.contract, to))
                    .transpose()?
                    .is_some()
            }
        };

        if is_processed {
            let (call_type, entrypoint_name, amount, sender, events_count) =
                match &contract_call.call_type {
                    ContractCallType::Init(init) => (
                        CallType::Init,
                        &init.contract_name.to_string(),
                        init.amount,
                        Address::Account(contract_call.txn.sender),
                        init.events.len() as i32,
                    ),
                    ContractCallType::Update(update) => (
                        CallType::Update,
                        &update.receive_name.to_string(),
                        update.amount,
                        update.sender,
                        update.events.len() as i32,
                    ),
                    ContractCallType::Upgraded { .. } => (
                        CallType::Upgraded,
                        &"".to_string(),
                        Amount::zero(),
                        Address::Account(contract_call.txn.sender),
                        0i32,
                    ),
                };
            db::upsert_transaction(
                conn,
                ListenerTransaction::new(block, contract_call.txn.hash, contract_call.txn.index),
            )?;
            db::add_contract_call(conn, ListenerContractCallInsert {
                call_type,
                ccd_amount: amount.micro_ccd.into(),
                entrypoint_name,
                events_count,
                index: contract_call.contract.index.into(),
                sub_index: contract_call.contract.subindex.into(),
                instigator: &contract_call.txn.sender.to_string(),
                sender: &sender.to_string(),
                transaction_hash: contract_call.txn.hash.bytes.into(),
            })?;
            info!(
                "Processed contract call txn: {}, contract: {}, sender: {}, events count: {}",
                contract_call.txn.hash, contract_call.contract, sender, events_count
            );
        }
    }

    Ok(())
}
/// Gets the last processed block height from the database, or the default block height if no
/// last processed block is found.
///
/// # Returns
///
/// * A `Result` containing the last processed block height, or a `ListenerError` if there was
///   an error retrieving the block height from the database.
async fn get_block_height_or(
    conn: &mut DbConn,
    default_block_height: AbsoluteBlockHeight,
) -> Result<AbsoluteBlockHeight, ListenerError> {
    let block_height = db::get_last_processed_block(conn)?
        .map(|b| b.next())
        .unwrap_or(default_block_height);

    Ok(block_height)
}

fn parse_block_item_summary(summary: BlockItemSummary) -> Option<Vec<ContractCall>> {
    let BlockItemSummary {
        details,
        index,
        hash,
        ..
    } = summary;
    if let BlockItemSummaryDetails::AccountTransaction(at) = details {
        match at.effects {
            AccountTransactionEffects::ContractInitialized { data } => Some(vec![ContractCall {
                txn:       ContractCallTxn::new(index, hash, at.sender),
                contract:  data.address,
                call_type: ContractCallType::create_init(data),
            }]),
            AccountTransactionEffects::ContractUpdateIssued { effects } => {
                let mut res = Vec::with_capacity(effects.len());
                let mut collected_events: BTreeMap<ContractAddress, Vec<ContractEvent>> =
                    BTreeMap::new();
                for effect in effects {
                    match effect {
                        ContractTraceElement::Interrupted { address, events } => {
                            collected_events
                                .entry(address)
                                .and_modify(|e| e.extend(events.clone().into_iter()))
                                .or_insert(events);
                        }
                        ContractTraceElement::Updated { data } => {
                            let interrupt_events = collected_events.remove(&data.address);
                            res.push(ContractCall {
                                txn:       ContractCallTxn::new(index, hash, at.sender),
                                contract:  data.address,
                                call_type: ContractCallType::create_update(data, interrupt_events),
                            });
                        }
                        ContractTraceElement::Upgraded { address, from, to } => {
                            res.push(ContractCall {
                                txn:       ContractCallTxn::new(index, hash, at.sender),
                                contract:  address,
                                call_type: ContractCallType::Upgraded { from, to },
                            });
                        }
                        _ => {}
                    }
                }
                Some(res)
            }
            _ => None,
        }
    } else {
        None
    }
}
