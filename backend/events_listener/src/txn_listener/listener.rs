use std::collections::BTreeMap;
use std::time::Duration;

use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::hashes::{BlockHash, TransactionHash};
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
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, PgConnection};
use futures::TryStreamExt;
use rust_decimal::Decimal;
use shared::db::DbConn;
use tracing::{debug, info, instrument, trace, warn};

use super::db::{self, ListenerContract, ListenerContractCallInsert, ListenerTransaction};
use crate::txn_listener::db::{CallType, ListenerConfigInsert};
use crate::txn_processor::cis2_utils::ContractAddressToDecimal;

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("R2D2 pool Database error: {0}")]
    DatabasePoolError(#[from] r2d2::Error),
    #[error("Events Parse Error: {0}")]
    EventsParseError(#[from] concordium_rust_sdk::base::contracts_common::ParseError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}

#[derive(Clone)]
pub struct Processors {
    pub processors_types: BTreeMap<(ModuleReference, OwnedContractName), db::ProcessorType>,
    pub processors:       BTreeMap<db::ProcessorType, ProcessorFnType>,
}

impl Processors {
    pub fn new() -> Self {
        Self {
            processors_types: BTreeMap::new(),
            processors:       BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        module_ref: ModuleReference,
        contract_name: OwnedContractName,
        processor_type: db::ProcessorType,
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
    ) -> Option<db::ProcessorType> {
        self.processors_types
            .get(&(*module_ref, contract_name.clone()))
            .copied()
    }

    pub fn find_by_type(&self, processor_type: &db::ProcessorType) -> Option<&ProcessorFnType> {
        self.processors.get(processor_type)
    }
}

impl Default for Processors {
    fn default() -> Self { Self::new() }
}

pub struct ParsedBlock {
    pub block_slot_time: DateTime<Utc>,
    pub block_height:    AbsoluteBlockHeight,
    pub block_hash:      BlockHash,
    pub transactions:    Vec<ParsedTxn>,
}

pub struct ParsedTxn {
    pub index:          TransactionIndex,
    pub hash:           TransactionHash,
    pub sender:         AccountAddress,
    pub contract_calls: Vec<ContractCall>,
}

pub struct ContractCall {
    pub call_type: ContractCallType,
    pub contract:  ContractAddress,
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

pub enum ContractCallType {
    Init(ContractCallTypeInit),
    Update(ContractCallTypeUpdate),
    Upgraded {
        from: ModuleReference,
        to:   ModuleReference,
    },
}

impl ContractCallType {
    /// Creates a new `Init` variant of `ContractCallType` from a `ContractInitializedEvent`.
    ///
    /// This function takes a `ContractInitializedEvent` and constructs an `Init` variant of `ContractCallType`
    /// with the relevant information from the event, such as the module reference, contract name, amount, and events.
    pub fn create_init(init: ContractInitializedEvent) -> Self {
        Self::Init(ContractCallTypeInit {
            module_ref:    init.origin_ref,
            contract_name: init.init_name,
            amount:        init.amount,
            events:        init.events,
        })
    }

    /// Creates a new `Update` variant of `ContractCallType` from an `InstanceUpdatedEvent`.
    ///
    /// This function takes an `InstanceUpdatedEvent` and constructs an `Update` variant of `ContractCallType`
    /// with the relevant information from the event, such as the sender, amount, receive name, and events.
    /// If `interrupt_events` is provided, it will be concatenated with the events from the `InstanceUpdatedEvent`.
    pub fn create_update(
        event: InstanceUpdatedEvent,
        interrupt_events: Option<Vec<ContractEvent>>,
    ) -> Self {
        let events = interrupt_events
            .map(|events| [events, event.events.clone()].concat())
            .unwrap_or(event.events);
        Self::Update(ContractCallTypeUpdate {
            sender: event.instigator,
            amount: event.amount,
            receive_name: event.receive_name,
            events,
        })
    }
}

pub struct ContractCallTypeUpdate {
    pub sender:       Address,
    pub amount:       Amount,
    pub events:       Vec<ContractEvent>,
    pub receive_name: OwnedReceiveName,
}

pub struct ContractCallTypeInit {
    pub module_ref:    ModuleReference,
    pub contract_name: OwnedContractName,
    pub amount:        Amount,
    pub events:        Vec<ContractEvent>,
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

impl ListenerError {
    pub fn is_retryable(&self) -> bool {
        match self {
            ListenerError::DatabaseError(_) => false,
            ListenerError::FinalizedBlockTimeout => true,
            ListenerError::FinalizedBlockStreamEnded => true,
            ListenerError::QueryError(_) => true,
            ListenerError::DatabasePoolError(_) => true,
            ListenerError::GrpcError(_) => true,
            ListenerError::ProcessorError(ProcessorError::EventsParseError(_)) => false,
            ListenerError::ProcessorError(ProcessorError::DatabaseError(_)) => false,
            ListenerError::ProcessorError(ProcessorError::DatabasePoolError(_)) => true,
        }
    }
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
#[derive(Clone)]
pub struct ListenerConfig {
    account:              AccountAddress, // Account address to listen to
    processors:           Processors,     // Processors to process the transactions
    database:             Pool<ConnectionManager<PgConnection>>, // postgres pool
    default_block_height: AbsoluteBlockHeight, // Default block height to start from
    concordium_client:    v2::Client,
}

impl ListenerConfig {
    pub fn new(
        client: v2::Client,
        pool: Pool<ConnectionManager<PgConnection>>,
        account: AccountAddress,
        processors: Processors,
        default_block_height: AbsoluteBlockHeight,
    ) -> Self {
        Self {
            account,
            concordium_client: client,
            database: pool,
            processors,
            default_block_height,
        }
    }
}

/// Starts listening to transactions from the Concordium node.
///
/// # Returns
///
/// * A Result indicating the success or failure of the operation.
#[instrument(skip_all)]
pub async fn listen(mut config: ListenerConfig) -> Result<(), ListenerError> {
    let mut conn = config.database.get()?;
    let block_height = get_block_height_or(&mut conn, config.default_block_height).await?;
    info!("Starting from block {}", block_height.height);

    let mut finalized_block_stream = config
        .concordium_client
        .get_finalized_blocks_from(block_height)
        .await?;

    loop {
        let (error, finalized_blocks) = finalized_block_stream
            .next_chunk_timeout(1000, Duration::from_millis(10000))
            .await
            .map_err(|_| ListenerError::FinalizedBlockTimeout)?;
        for block in &finalized_blocks {
            let block = config
                .concordium_client
                .get_block_info(block.height)
                .await?
                .response;
            if block.transaction_count.eq(&0u64) {
                trace!("Block {block:?} has no transactions");
                continue;
            }

            let block: ParsedBlock = parse_block(&mut config.concordium_client, &block).await?;
            process_block(&mut conn, &block, &config.account, &config.processors).await?;
        }

        debug!("Processed chunk of {} blocks", finalized_blocks.len());
        if error {
            return Err(ListenerError::FinalizedBlockStreamEnded);
        }
    }
}

#[instrument(skip_all)]
async fn parse_block(
    client: &mut v2::Client,
    block: &BlockInfo,
) -> Result<ParsedBlock, ListenerError> {
    let txns: Vec<ParsedTxn> = client
        .get_block_transaction_events(block.block_hash)
        .await?
        .response
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .filter_map(parse_block_item_summary)
        .collect();

    Ok(ParsedBlock {
        block_slot_time: block.block_slot_time,
        block_height:    block.block_height,
        block_hash:      block.block_hash,
        transactions:    txns,
    })
}

/// Processes a block of transactions, extracting contract calls, processing them, and updating the last processed block in the database.
///
/// # Arguments
/// * `client` - A mutable reference to the Concordium client used to fetch block information.
/// * `conn` - A mutable reference to the database connection.
/// * `block` - A reference to the block information.
/// * `contract_owner` - A reference to the contract owner's account address.
/// * `processors` - A reference to the map of contract processors.
///
/// # Returns
/// A `Result` indicating the success or failure of the operation.
#[instrument(skip_all)]
pub async fn process_block(
    conn: &mut DbConn,
    block: &ParsedBlock,
    contract_owner: &AccountAddress,
    processors: &Processors,
) -> Result<(), ListenerError> {
    let res = conn.transaction(|conn| {
        for txn in block.transactions.iter() {
            let is_txn_processed =
                process_txn(conn, block.block_slot_time, txn, contract_owner, processors)?;

            if is_txn_processed {
                ListenerTransaction {
                    block_hash:        block.block_hash.bytes.into(),
                    block_height:      block.block_height.height.into(),
                    transaction_index: txn.index.index.into(),
                    block_slot_time:   block.block_slot_time.naive_utc(),
                    transaction_hash:  txn.hash.to_string(),
                }
                .insert(conn)?;
            }
        }

        // Update the last processed block in the database
        let id = ListenerConfigInsert {
            last_block_hash:      block.block_hash.bytes.into(),
            last_block_height:    block.block_height.height.into(),
            last_block_slot_time: block.block_slot_time.naive_utc(),
        }
        .insert(conn)?;
        Result::<_, ListenerError>::Ok(id)
    })?;

    if res.is_none() {
        warn!("block {} already processed", block.block_height.height);
    }

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

fn process_txn(
    conn: &mut DbConn,
    block_slot_time: DateTime<Utc>,
    txn: &ParsedTxn,
    contract_owner: &AccountAddress,
    processors: &Processors,
) -> Result<bool, ListenerError> {
    let mut is_any_processed = false;
    for contract_call in &txn.contract_calls {
        let is_processed = match &contract_call.call_type {
            ContractCallType::Init(init) => {
                // If the contract is owned by the owner, then we process the init call
                if contract_owner.eq(&txn.sender) {
                    if let Some(processor_type) =
                        processors.find_type(&init.module_ref, &init.contract_name)
                    {
                        let contract = ListenerContract::new(
                            contract_call.contract.to_decimal(),
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
                            transaction_hash: txn.hash.bytes.into(),
                            created_at:       block_slot_time.naive_utc(),
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
                let contract = ListenerContract::find(conn, contract_call.contract.to_decimal())?;
                match contract {
                    Some(contract) => {
                        let contract_call = ListenerContractCallInsert {
                            call_type:        CallType::Update,
                            ccd_amount:       update.amount.micro_ccd.into(),
                            entrypoint_name:  &update.receive_name.to_string(),
                            events_count:     update.events.len() as i32,
                            contract_address: contract_call.contract.to_decimal(),
                            instigator:       &txn.sender.to_string(),
                            sender:           &update.sender.to_string(),
                            transaction_hash: txn.hash.bytes.into(),
                            created_at:       block_slot_time.naive_utc(),
                        }
                        .insert(conn)?;
                        Some((contract, contract_call, Some(&update.events)))
                    }
                    None => None,
                }
            }
            ContractCallType::Upgraded { to, .. } => {
                let contract = ListenerContract::find(conn, contract_call.contract.to_decimal())?;
                match contract {
                    Some(contract) => {
                        let contract_call = ListenerContractCallInsert {
                            call_type:        CallType::Upgraded,
                            ccd_amount:       Decimal::ZERO,
                            entrypoint_name:  "",
                            events_count:     0,
                            contract_address: contract_call.contract.to_decimal(),
                            instigator:       &txn.sender.to_string(),
                            sender:           &txn.sender.to_string(),
                            transaction_hash: txn.hash.bytes.into(),
                            created_at:       block_slot_time.naive_utc(),
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
                match processors.find_by_type(&contract.processor_type) {
                    Some(processor) => {
                        let events_length = if let Some(events) = events {
                            processor(conn, block_slot_time, &contract.contract_address(), events)?;
                            events.len()
                        } else {
                            0
                        };
                        contract_call.update_processed(conn)?;
                        is_any_processed = true;
                        info!(
                            "Processed contract call contract: {}, sender: {}, events count: {}",
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
    let block_height = db::ListenerConfig::find_last(conn)?
        .map(|b| b.next())
        .unwrap_or(default_block_height);

    Ok(block_height)
}

/// Parses a `BlockItemSummary` and returns an optional vector of `ContractCall` instances.
///
/// If the `BlockItemSummary` represents an `AccountTransaction` with either a `ContractInitialized` or
/// `ContractUpdateIssued` effect, this function will parse the details and return a vector of
/// `ContractCall` instances. Otherwise, it will return `None`.
///
/// # Arguments
///
/// * `summary` - The `BlockItemSummary` to parse.
///
/// # Returns
///
/// An optional vector of `ContractCall` instances, or `None` if the `BlockItemSummary` does not
/// represent a relevant transaction.
#[instrument(skip_all, fields(txn_hash = %summary.hash))]
fn parse_block_item_summary(summary: BlockItemSummary) -> Option<ParsedTxn> {
    let BlockItemSummary {
        details,
        index,
        hash,
        ..
    } = summary;
    if let BlockItemSummaryDetails::AccountTransaction(at) = details {
        match at.effects {
            AccountTransactionEffects::ContractInitialized { data } => Some(ParsedTxn {
                index,
                hash,
                sender: at.sender,
                contract_calls: vec![ContractCall {
                    contract:  data.address,
                    call_type: ContractCallType::create_init(data),
                }],
            }),
            AccountTransactionEffects::ContractUpdateIssued { effects } => {
                let mut res = Vec::with_capacity(effects.len());
                let mut collected_events: BTreeMap<ContractAddress, Vec<ContractEvent>> =
                    BTreeMap::new();
                for effect in effects {
                    match effect {
                        ContractTraceElement::Interrupted { address, events } => {
                            // If we have events for this address, add them to the interrupt events and continue
                            collected_events
                                .entry(address)
                                .and_modify(|e| e.extend(events.clone().into_iter()))
                                .or_insert(events);
                        }
                        ContractTraceElement::Updated { data } => {
                            // after interrupt the contract is resumed and updated.
                            // So we can add the interrupt events to the update events
                            let interrupt_events = collected_events.remove(&data.address);
                            res.push(ContractCall {
                                contract:  data.address,
                                call_type: ContractCallType::create_update(data, interrupt_events),
                            });
                        }
                        ContractTraceElement::Upgraded { address, from, to } => {
                            res.push(ContractCall {
                                contract:  address,
                                call_type: ContractCallType::Upgraded { from, to },
                            });
                        }
                        _ => {}
                    }
                }
                Some(ParsedTxn {
                    index,
                    hash,
                    sender: at.sender,
                    contract_calls: res,
                })
            }
            _ => None,
        }
    } else {
        None
    }
}
