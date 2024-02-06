use std::str::FromStr;

use bson::{doc, serde_helpers::u64_as_f64};
use concordium_rust_sdk::{
    types::{
        smart_contracts::{ModuleReference, OwnedContractName},
        ContractAddress,
    },
    v2::FinalizedBlockInfo,
};
use mongodb::{
    options::{FindOneOptions, InsertOneOptions},
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DbProcessedBlock {
    block_hash:       String,
    #[serde(with = "u64_as_f64")]
    pub block_height: u64,
}

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
    pub fn new(
        module_ref: &ModuleReference,
        contract_name: &OwnedContractName,
        address: &ContractAddress,
    ) -> Self {
        Self {
            module_ref:       module_ref.to_string(),
            contract_name:    contract_name.to_string(),
            address_index:    address.index,
            address_subindex: address.subindex,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseClient {
    pub client: mongodb::Client,
}

impl DatabaseClient {
    pub fn database(&self) -> mongodb::Database { self.client.database("concordium") }

    pub fn processed_blocks(&self) -> mongodb::Collection<DbProcessedBlock> {
        self.database().collection::<DbProcessedBlock>("processed_blocks")
    }

    pub fn contracts(&self) -> mongodb::Collection<DbContract> {
        self.database().collection::<DbContract>("contracts")
    }

    pub async fn get_last_processed_block(&self) -> anyhow::Result<Option<DbProcessedBlock>> {
        let collection = self.processed_blocks();
        let result = collection
            .find_one(None, FindOneOptions::builder().sort(doc! { "block_height": -1 }).build())
            .await?;

        Ok(result)
    }

    pub async fn update_last_processed_block(
        &mut self,
        block: &FinalizedBlockInfo,
    ) -> anyhow::Result<InsertOneResult> {
        let collection = self.processed_blocks();
        let result = collection
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

    pub async fn add_contract(
        &self,
        address: concordium_rust_sdk::types::ContractAddress,
        origin_ref: &ModuleReference,
        init_name: &OwnedContractName,
    ) -> anyhow::Result<InsertOneResult> {
        let result = self
            .contracts()
            .insert_one(
                DbContract::new(origin_ref, init_name, &address),
                InsertOneOptions::builder().build(),
            )
            .await?;

        Ok(result)
    }

    pub async fn find_contract(
        &self,
        contract_address: ContractAddress,
    ) -> anyhow::Result<Option<(ModuleReference, OwnedContractName)>> {
        let result = self
            .contracts()
            .find_one(
                doc! {
                    "address_index": contract_address.index as f64,
                    "address_subindex": contract_address.subindex as f64,
                },
                None,
            )
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
