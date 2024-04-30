use bson::{doc, serde_helpers::u64_as_f64};
use concordium_rust_sdk::{
    types::{
        smart_contracts::{ModuleReference, OwnedContractName},
        ContractAddress,
    },
    v2::FinalizedBlockInfo,
};
use mongodb::{
    options::{
        CreateCollectionOptions, CreateIndexOptions, FindOneOptions, IndexOptions, InsertOneOptions,
    },
    results::InsertOneResult,
    IndexModel,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents a processed block in the database.
#[derive(Serialize, Deserialize)]
pub struct DbProcessedBlock {
    block_hash:       String,
    #[serde(with = "u64_as_f64")]
    pub block_height: u64,
}

/// Represents a contract in the database.
#[derive(Serialize, Deserialize)]
pub struct DbContract {
    pub module_ref:    String,
    pub contract_name: String,

    #[serde(with = "u64_as_f64")]
    pub address_index: u64,

    #[serde(with = "u64_as_f64")]
    pub address_subindex: u64,
}

impl DbContract {
    pub fn mongodb_index_key() -> bson::Document {
        doc! {
            "address_index": 1,
            "address_subindex": 1,
        }
    }

    pub fn mongodb_find_one_query(address: &ContractAddress) -> bson::Document {
        doc! {
            "address_index": address.index as f64,
            "address_subindex": address.subindex as f64,
        }
    }
}

const DATABASE_NAME: &str = "concordium";
const CONTRACTS_COLLECTION: &str = "contracts";
const PROCESSED_BLOCKS_COLLECTION: &str = "processed_blocks";

/// Represents a client for interacting with the database.
pub struct DatabaseClient {
    contracts:        mongodb::Collection<DbContract>,
    processed_blocks: mongodb::Collection<DbProcessedBlock>,
}

impl DatabaseClient {
    /// Creates a new `DatabaseClient` instance.
    pub async fn init(client: mongodb::Client) -> anyhow::Result<Self> {
        let coll_names = client.database(DATABASE_NAME).list_collection_names(None).await?;
        if !coll_names.contains(&PROCESSED_BLOCKS_COLLECTION.to_owned()) {
            client
                .database(DATABASE_NAME)
                .create_collection(
                    PROCESSED_BLOCKS_COLLECTION,
                    CreateCollectionOptions::builder().capped(true).size(1000).build(),
                )
                .await?;
            let processed_blocks = client
                .database(DATABASE_NAME)
                .collection::<DbProcessedBlock>(PROCESSED_BLOCKS_COLLECTION);
            processed_blocks
                .create_index(
                    IndexModel::builder()
                        .keys(doc! { "block_height": -1 })
                        .options(IndexOptions::builder().unique(true).build())
                        .build(),
                    Some(CreateIndexOptions::builder().build()),
                )
                .await?;
        }

        if !coll_names.contains(&CONTRACTS_COLLECTION.to_owned()) {
            client.database(DATABASE_NAME).create_collection(CONTRACTS_COLLECTION, None).await?;
            let contracts =
                client.database(DATABASE_NAME).collection::<DbContract>(CONTRACTS_COLLECTION);
            contracts
                .create_index(
                    IndexModel::builder()
                        .keys(DbContract::mongodb_index_key())
                        .options(IndexOptions::builder().unique(true).build())
                        .build(),
                    Some(CreateIndexOptions::builder().build()),
                )
                .await?;
        }

        Ok(Self {
            contracts:        client
                .database(DATABASE_NAME)
                .collection::<DbContract>(CONTRACTS_COLLECTION),
            processed_blocks: client
                .database(DATABASE_NAME)
                .collection::<DbProcessedBlock>(PROCESSED_BLOCKS_COLLECTION),
        })
    }

    /// Retrieves the last processed block from the database.
    pub async fn get_last_processed_block(&self) -> anyhow::Result<Option<DbProcessedBlock>> {
        let result = self
            .processed_blocks
            .find_one(None, FindOneOptions::builder().sort(doc! { "block_height": -1 }).build())
            .await?;

        Ok(result)
    }

    /// Updates the last processed block in the database.
    pub async fn update_last_processed_block(
        &mut self,
        block: &FinalizedBlockInfo,
    ) -> anyhow::Result<InsertOneResult> {
        let result = self
            .processed_blocks
            .insert_one(
                DbProcessedBlock {
                    block_hash:   block.block_hash.to_string(),
                    block_height: block.height.height,
                },
                InsertOneOptions::builder().build(),
            )
            .await?;

        Ok(result)
    }

    /// Finds a contract in the database based on its address.
    pub async fn find_contract(
        &self,
        contract_address: &ContractAddress,
    ) -> anyhow::Result<Option<(ModuleReference, OwnedContractName)>> {
        let result = self
            .contracts
            .find_one(DbContract::mongodb_find_one_query(contract_address), None)
            .await?;

        match result {
            Some(db_contract) => {
                let module_ref = ModuleReference::from_str(&db_contract.module_ref)?;
                let contract_name = OwnedContractName::new_unchecked(db_contract.contract_name);

                Ok(Some((module_ref, contract_name)))
            }
            None => Ok(None),
        }
    }
}
