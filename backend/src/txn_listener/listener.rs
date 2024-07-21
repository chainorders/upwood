use super::db::*;
use anyhow::Ok;
use async_trait::async_trait;
use concordium_rust_sdk::{
    types::{
        smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
        AbsoluteBlockHeight, ContractAddress,
    },
    v2::{self, FinalizedBlockInfo},
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use futures::{future::join_all, StreamExt};
use std::{collections::BTreeMap, ops::AddAssign, sync::Arc};
use tokio::sync::RwLock;

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
        &mut self,
        contract: &ContractAddress,
        events: &[ContractEvent],
    ) -> anyhow::Result<u64>;
}

/// `TransactionsListener` is a struct that listens to transactions from a
/// Concordium node and processes them. It maintains a connection to the node
/// and a MongoDB database, and uses a set of processors to process the
/// transactions.
pub struct TransactionsListener {
    database:             Pool<ConnectionManager<PgConnection>>, /* popstgres pool */
    processors:           Vec<Arc<RwLock<dyn EventsProcessor>>>, /* Processors to process
                                                                  * transactions */
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
        pool: Pool<ConnectionManager<PgConnection>>,
        processors: Vec<Arc<RwLock<dyn EventsProcessor>>>,
        default_block_height: AbsoluteBlockHeight,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            client,
            database: pool,
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

    async fn get_block_height(&self) -> Result<AbsoluteBlockHeight, anyhow::Error> {
        let mut conn = self.database.get()?;
        let block_height = listener_config::get_last_processed_block(&mut conn)
            .await?
            .unwrap_or(self.default_block_height);

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
        let mut conn = self.database.get()?;
        if b_events_count > 0 {
            listener_config::update_last_processed_block(&mut conn, block)
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
            let mut updates = BTreeMap::new();
            updates.entry(init.address).or_insert(init.events.clone());
            let events_count = self.process_events(updates).await?;
            log::info!("Processed block item: {}, events: {}", summary.hash, events_count);
            events_count
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
        let contract_updates: std::collections::btree_map::IntoIter<
            ContractAddress,
            Vec<ContractEvent>,
        > = updates.into_iter();
        let mut processor_futures = vec![];
        for (contract_address, events) in contract_updates {
            processor_futures.push(self.process_contract_events(contract_address, events));
        }
        let events_count_vec = join_all(processor_futures).await;
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
        let mut conn = self.database.get()?;
        let db_contract = listener_contracts::find_contract(&mut conn, &contract_address).await?;
        let processor = match db_contract {
            Some((module_ref, contract_name)) => {
                match self.find_processor(&module_ref, &contract_name).await? {
                    Some(processor) => processor.clone(),
                    None => return Ok(0),
                }
            }
            None => return Ok(0),
        };

        async move { processor.write().await.process_events(&contract_address, &events).await }
            .await
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
    async fn find_processor(
        &self,
        origin_ref: &ModuleReference,
        init_name: &OwnedContractName,
    ) -> Result<Option<Arc<RwLock<dyn EventsProcessor>>>, anyhow::Error> {
        for processor in self.processors.iter() {
            let matches = processor.read().await.matches(origin_ref, init_name);
            match matches {
                true => return Ok(Some(processor.clone())),
                false => continue,
            }
        }

        Ok(None)
    }

    async fn process_contract_init(
        &mut self,
        init: &concordium_rust_sdk::types::ContractInitializedEvent,
    ) -> anyhow::Result<()> {
        let processor_exists =
            self.find_processor(&init.origin_ref, &init.init_name).await?.is_some();
        if processor_exists {
            let mut conn = self.database.get()?;
            listener_contracts::add_contract(
                &mut conn,
                &init.address,
                &init.origin_ref,
                &init.init_name,
            )
            .await?;
            log::info!(
                "Listening to contract {}-{}-{}",
                init.origin_ref,
                init.init_name,
                init.address
            );
        }

        Ok(())
    }
}
