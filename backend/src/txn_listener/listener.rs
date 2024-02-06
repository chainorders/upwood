use anyhow::Ok;
use async_trait::async_trait;
use concordium_rust_sdk::{
    types::{
        hashes::BlockHash,
        smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
        transactions::BlockItem,
        AbsoluteBlockHeight, BlockItemSummary, ContractAddress,
    },
    v2::FinalizedBlockInfo,
};
use futures::StreamExt;
use log::info;

use crate::txn_listener::db::DatabaseClient;

/// `EventsProcessor` is a trait that defines the necessary methods for
/// processing events of a specific contract. It is designed to be implemented
/// by structs that handle the logic for processing events of a specific
/// contract.
#[async_trait]
pub trait EventsProcessor: Send + Sync {
    /// Returns the name of the contract this processor is responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `OwnedContractName` of the contract.
    fn contract_name(&self) -> &OwnedContractName;

    /// Returns the module reference of the contract this processor is
    /// responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `ModuleReference` of the contract.
    fn module_ref(&self) -> &ModuleReference;

    /// Checks if this processor is responsible for the given contract.
    ///
    /// # Arguments
    ///
    /// * `module_ref` - A reference to the `ModuleReference` of the contract to
    ///   check.
    /// * `contract_name` - A reference to the `OwnedContractName` of the
    ///   contract to check.
    ///
    /// # Returns
    ///
    /// * `true` if this processor is responsible for the given contract,
    ///   `false` otherwise.
    fn matches(&self, module_ref: &ModuleReference, contract_name: &OwnedContractName) -> bool {
        module_ref.eq(self.module_ref()) && contract_name.eq(self.contract_name())
    }

    /// Processes the events of a contract.
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
    ) -> anyhow::Result<()>;
}

/// `TransactionsListener` is a struct that listens to transactions from a
/// Concordium node and processes them. It maintains a connection to the node
/// and a MongoDB database, and uses a set of processors to process the
/// transactions.
pub struct TransactionsListener {
    node:       concordium_rust_sdk::v2::Client, // Client to interact with the Concordium node
    database:   DatabaseClient,                  // Client to interact with the MongoDB database
    processors: Vec<Box<dyn EventsProcessor>>,   // Set of processors to process the transactions
}

impl TransactionsListener {
    /// Constructs a new `TransactionsListener`.
    ///
    /// # Arguments
    ///
    /// * `concordium_node_uri` - URI of the Concordium node.
    /// * `mongodb_uri` - URI of the MongoDB database.
    ///
    /// # Returns
    ///
    /// * A new `TransactionsListener`.
    pub async fn new(
        concordium_client: concordium_rust_sdk::v2::Client,
        mongo_client: mongodb::Client,
        processors: Vec<Box<dyn EventsProcessor>>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            node: concordium_client,
            database: DatabaseClient {
                client: mongo_client,
            },
            processors,
        })
    }

    /// Starts listening to transactions from the Concordium node.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    pub async fn listen(&mut self, starting_block_hash: Option<BlockHash>) -> anyhow::Result<()> {
        let starting_block_height = match starting_block_hash {
            Some(hash) => {
                let block_info = self.node.get_block_info(hash).await?;
                block_info.response.block_height
            }
            None => {
                let consensus_info = self.node.get_consensus_info().await?;
                consensus_info.best_block_height
            }
        };

        let last_processed_block = self.database.get_last_processed_block().await?;
        let next_block_height = match last_processed_block {
            Some(block) => AbsoluteBlockHeight {
                height: block.block_height,
            }
            .next(),
            None => starting_block_height,
        };

        info!("Starting from block {}", next_block_height.height);

        let mut finalized_block_stream =
            self.node.get_finalized_blocks_from(next_block_height).await?;
        while let Some(block) = finalized_block_stream.next().await {
            log::info!("Processing block {}", block.height.height);
            self.process_block(&block).await?;
            self.database.update_last_processed_block(&block).await?;
            log::trace!("Processed block {}", block.height.height)
        }

        Ok(())
    }

    /// Processes a block of transactions.
    ///
    /// # Arguments
    ///
    /// * `block` - The block of transactions to be processed.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    async fn process_block(&mut self, block: &FinalizedBlockInfo) -> anyhow::Result<()> {
        let mut block_items_stream = self.node.get_block_items(block.block_hash).await?;
        while let Some(block_item) = block_items_stream.response.next().await {
            let block_item = &block_item?;
            match block_item {
                BlockItem::AccountTransaction(t) => {
                    let block_item_hash = block_item.hash();
                    let block_item_status =
                        self.node.get_block_item_status(&block_item_hash).await?;
                    // Can unwrap because we know the block item exists and is finalized
                    let (_, summary) = block_item_status.is_finalized().unwrap();
                    self.process_block_item(summary).await?;
                }
                _ => continue,
            }
        }

        Ok(())
    }

    /// Processes a block item.
    ///
    /// # Arguments
    ///
    /// * `summary` - The summary of the block item to be processed.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    async fn process_block_item(&self, summary: &BlockItemSummary) -> anyhow::Result<()> {
        if let Some(init) = summary.contract_init() {
            if let Some(processor) = self.find_processor(&init.origin_ref, &init.init_name) {
                processor.process_events(&init.address, &init.events).await?;
                self.database.add_contract(init.address, &init.origin_ref, &init.init_name).await?;
                log::info!(
                    "Started Listening to {:?} at address: {:?}",
                    processor.contract_name(),
                    init.address
                )
            }
        } else if let Some(mut update) = summary.contract_update_logs() {
            for (contract_address, events) in update.by_ref() {
                if let Some((module_ref, contract_name)) =
                    self.database.find_contract(contract_address).await?
                {
                    if let Some(processor) = self.find_processor(&module_ref, &contract_name) {
                        processor.process_events(&contract_address, events).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Finds a processor that matches the given origin reference and contract
    /// name.
    ///
    /// # Arguments
    ///
    /// * `origin_ref` - The origin reference to match.
    /// * `init_name` - The contract name to match.
    ///
    /// # Returns
    ///
    /// * An Option containing a reference to the matching processor, or None if
    ///   no match was found.
    fn find_processor(
        &self,
        origin_ref: &ModuleReference,
        init_name: &OwnedContractName,
    ) -> Option<&dyn EventsProcessor> {
        let processor =
            self.processors.iter().find(|processor| processor.matches(origin_ref, init_name));

        processor.map(|processor| processor.as_ref())
    }
}
