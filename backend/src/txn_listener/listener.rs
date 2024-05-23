use crate::txn_listener::db::DatabaseClient;
use anyhow::Ok;
use async_trait::async_trait;
use concordium_rust_sdk::{
    types::{
        smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
        AbsoluteBlockHeight, ContractAddress,
    },
    v2::{self, FinalizedBlockInfo},
};
use futures::{future::join_all, StreamExt, TryFutureExt};
use std::{collections::BTreeMap, ops::AddAssign};

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
    ) -> anyhow::Result<u64>;
}

/// `TransactionsListener` is a struct that listens to transactions from a
/// Concordium node and processes them. It maintains a connection to the node
/// and a MongoDB database, and uses a set of processors to process the
/// transactions.
pub struct TransactionsListener {
    database:             DatabaseClient, // Client to interact with the MongoDB database
    processors:           Vec<Box<dyn EventsProcessor>>, // Processors to process transactions
    default_block_height: AbsoluteBlockHeight, // Default block height to start from
    client:               v2::Client,
}

impl TransactionsListener {
    /// Constructs a new `TransactionsListener`.
    /// # Returns
    ///
    /// * A new `TransactionsListener`.
    pub async fn new(
        client: v2::Client,
        database: DatabaseClient,
        processors: Vec<Box<dyn EventsProcessor>>,
        default_block_height: AbsoluteBlockHeight,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            client,
            database,
            processors,
            default_block_height,
        })
    }

    /// Starts listening to transactions from the Concordium node.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    pub async fn listen(mut self) -> anyhow::Result<()> {
        let block_height = self.get_block_height().await?;
        let mut finalized_block_stream =
            self.client.get_finalized_blocks_from(block_height).await?;

        while let Some(block) = finalized_block_stream.next().await {
            self.process_block(&block).await?;
            log::debug!("Processed block {}", block.height.height);
        }

        anyhow::bail!("Finalized block stream ended unexpectedly")
    }

    async fn get_block_height(&mut self) -> Result<AbsoluteBlockHeight, anyhow::Error> {
        let block_height = self
            .database
            .get_last_processed_block()
            .map_ok(|db_block| match db_block {
                Some(db_block) => AbsoluteBlockHeight::next(AbsoluteBlockHeight {
                    height: db_block.block_height,
                }),
                None => self.default_block_height,
            })
            .await?;
        Ok(block_height)
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
    async fn process_block(&mut self, block: &FinalizedBlockInfo) -> anyhow::Result<u64> {
        let mut b_events_count = 0u64;
        let mut summaries = self
            .client
            .get_block_transaction_events(block.block_hash)
            .await
            .expect("block not found")
            .response;

        while let Some(summary) =
            summaries.next().await.transpose().expect("error getting block item summary")
        {
            let events_count = self.process_block_item_summary(&summary).await?;
            b_events_count += events_count;
        }

        log::info!("Processed block {}, events: {}", block.height.height, b_events_count);
        if b_events_count > 0 {
            self.database
                .update_last_processed_block(block)
                .await
                .expect("error updating block in db");
        }
        Ok(b_events_count)
    }

    async fn process_block_item_summary(
        &mut self,
        summary: &concordium_rust_sdk::types::BlockItemSummary,
    ) -> Result<u64, anyhow::Error> {
        let events_count = if let Some(init) = summary.contract_init() {
            self.process_contract_init(init).await?;
            self.process_contract_events(init.address, init.events.to_vec()).await?
        } else if let Some(update) = summary.contract_update_logs() {
            let updates = update.into_iter().fold(BTreeMap::new(), |mut map, update| {
                map.entry(update.0)
                    .and_modify(|e: &mut Vec<_>| e.extend(update.1.to_vec()))
                    .or_insert(update.1.to_vec());
                map
            });
            let events_count = self.process_events(updates).await?;
            log::info!("Processed block item: {}, events: {}", summary.hash, events_count);
            events_count
        } else {
            0
        };
        Ok(events_count)
    }

    async fn process_events(
        &mut self,
        updates: BTreeMap<ContractAddress, Vec<ContractEvent>>,
    ) -> Result<u64, anyhow::Error> {
        let mut events_count = 0u64;
        let processor_futures = updates
            .into_iter()
            .map(|(contract_address, events)| {
                self.process_contract_events(contract_address, events)
            })
            .collect::<Vec<_>>();
        let events_count_vec: Vec<Result<u64, anyhow::Error>> = join_all(processor_futures).await;
        for count in events_count_vec {
            events_count.add_assign(count?);
        }
        Ok(events_count)
    }

    async fn process_contract_events(
        &self,
        contract_address: ContractAddress,
        events: Vec<ContractEvent>,
    ) -> anyhow::Result<u64> {
        let contract = self.database.find_contract(&contract_address).await?;
        if let Some((origin_ref, init_name)) = contract {
            if let Some(processor) = self.find_processor(&origin_ref, &init_name) {
                processor.process_events(&contract_address, events.as_slice()).await
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
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

    async fn process_contract_init(
        &self,
        init: &concordium_rust_sdk::types::ContractInitializedEvent,
    ) -> anyhow::Result<()> {
        let contract = self.database.find_contract(&init.address).await?.is_none();
        if contract {
            let processor = self.find_processor(&init.origin_ref, &init.init_name).is_some();
            if processor {
                self.database
                    .add_contract(&init.address, &init.origin_ref, &init.init_name)
                    .await?;
                log::info!(
                    "Listening to contract {}-{}-{}",
                    init.origin_ref,
                    init.init_name,
                    init.address
                );
            }
        }

        Ok(())
    }
}
