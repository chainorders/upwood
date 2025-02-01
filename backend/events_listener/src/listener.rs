use std::collections::BTreeMap;
use std::time::Duration;

use concordium_rust_sdk::base::smart_contracts::OwnedReceiveName;
use concordium_rust_sdk::common::types::Amount;
use concordium_rust_sdk::types::smart_contracts::{
    ContractEvent, ModuleReference, OwnedContractName,
};
use concordium_rust_sdk::types::{
    AbsoluteBlockHeight, AccountTransactionEffects, Address, BlockItemSummary,
    BlockItemSummaryDetails, ContractAddress, ContractInitializedEvent, ContractTraceElement,
    InstanceUpdatedEvent,
};
use concordium_rust_sdk::v2;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use futures::TryStreamExt;
use rust_decimal::Decimal;
use shared::db::txn_listener::ListenerBlock;
use shared::db_shared::DbConn;
use tracing::{debug, info, instrument, warn};

use crate::processors::cis2_utils::ContractAddressToDecimal;
use crate::processors::{ProcessorError, Processors};

#[derive(Debug)]
pub struct ParsedBlock {
    pub block:        ListenerBlock,
    pub transactions: Vec<ParsedTxn>,
}

#[derive(Clone, Debug)]
pub struct ParsedTxn {
    pub index:          u64,
    pub hash:           Vec<u8>,
    pub sender:         String,
    pub contract_calls: Vec<ContractCall>,
}

#[derive(Clone, Debug)]
pub struct ContractCall {
    pub call_type: ContractCallType,
    pub contract:  Decimal,
}

impl ContractCall {
    pub fn parse_effects(effects: Vec<ContractTraceElement>) -> Vec<Self> {
        let mut res = Vec::with_capacity(effects.len());
        let mut collected_events: BTreeMap<ContractAddress, Vec<ContractEvent>> = BTreeMap::new();
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
                        contract:  data.address.to_decimal(),
                        call_type: ContractCallType::create_update(data, interrupt_events),
                    });
                }
                ContractTraceElement::Upgraded { address, from, to } => {
                    res.push(ContractCall {
                        contract:  address.to_decimal(),
                        call_type: ContractCallType::Upgraded { from, to },
                    });
                }
                _ => {}
            }
        }

        res
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ContractCallTypeUpdate {
    pub sender:       Address,
    pub amount:       Amount,
    pub events:       Vec<ContractEvent>,
    pub receive_name: OwnedReceiveName,
}

#[derive(Clone, Debug)]
pub struct ContractCallTypeInit {
    pub module_ref:    ModuleReference,
    pub contract_name: OwnedContractName,
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
            ListenerError::ProcessorError(ProcessorError::DatabasePoolError {
                source: _,
                backtrace: _,
            }) => true,
            ListenerError::ProcessorError(_) => false,
        }
    }
}

/// `TransactionsListener` is a struct that listens to transactions from a
/// Concordium node and processes them. It maintains a connection to the node
/// and a MongoDB database, and uses a set of processors to process the
/// transactions.
pub struct Listener {
    processors:           Processors, // Processors to process the transactions
    database:             Pool<ConnectionManager<PgConnection>>, // postgres pool
    default_block_height: AbsoluteBlockHeight, // Default block height to start from
    concordium_client:    v2::Client,
}

impl Listener {
    pub fn new(
        client: v2::Client,
        pool: Pool<ConnectionManager<PgConnection>>,
        processors: Processors,
        default_block_height: AbsoluteBlockHeight,
    ) -> Self {
        Self {
            concordium_client: client,
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
    pub async fn listen(&mut self) -> Result<(), ListenerError> {
        let mut conn = self.database.get()?;
        let block_height = get_block_height_or(&mut conn, self.default_block_height).await?;
        info!("Starting from block {}", block_height.height);

        let mut finalized_block_stream = self
            .concordium_client
            .get_finalized_blocks_from(block_height)
            .await?;

        loop {
            while let Ok((error, finalized_blocks)) = finalized_block_stream
                .next_chunk_timeout(100, Duration::from_millis(5000))
                .await
            {
                debug!("Received chunk of {} blocks", finalized_blocks.len());
                for block in &finalized_blocks {
                    let block = self
                        .concordium_client
                        .get_block_info(block.height)
                        .await?
                        .response;
                    if block.transaction_count.eq(&0u64) {
                        debug!("Block {block:?} has no transactions");
                        continue;
                    }

                    let txns = self
                        .concordium_client
                        .get_block_transaction_events(block.block_hash)
                        .await?
                        .response
                        .try_collect::<Vec<_>>()
                        .await?
                        .into_iter()
                        .filter_map(parse_block_item_summary)
                        .collect();
                    let block = ParsedBlock {
                        block:        ListenerBlock {
                            block_hash:      block.block_hash.bytes.into(),
                            block_height:    block.block_height.height.into(),
                            block_slot_time: block.block_slot_time.naive_utc(),
                        },
                        transactions: txns,
                    };

                    self.processors.process_block(&mut conn, &block).await?;
                }
                info!("Processed chunk of {} blocks", finalized_blocks.len());

                if error {
                    return Err(ListenerError::FinalizedBlockStreamEnded);
                }
            }

            warn!("Finalized block stream timed out, retrying");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
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
    let block_height = shared::db::txn_listener::ListenerBlock::find_last_height(conn)?
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
                index:          index.index,
                hash:           hash.bytes.into(),
                sender:         at.sender.to_string(),
                contract_calls: vec![ContractCall {
                    contract:  data.address.to_decimal(),
                    call_type: ContractCallType::create_init(data),
                }],
            }),
            AccountTransactionEffects::ContractUpdateIssued { effects } => Some(ParsedTxn {
                index:          index.index,
                hash:           hash.bytes.into(),
                sender:         at.sender.to_string(),
                contract_calls: ContractCall::parse_effects(effects),
            }),
            _ => None,
        }
    } else {
        None
    }
}
