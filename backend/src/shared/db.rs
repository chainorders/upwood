use async_trait::async_trait;
use bson::{doc, to_document, Document};
use concordium_rust_sdk::{
    cis2::{TokenAmount, TokenId},
    smart_contracts::common::AccountAddress,
    types::{Address, ContractAddress},
};
use mongodb::options::UpdateModifications;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A wrapper around `ContractAddress` that can be used in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbContractAddress(pub ContractAddress);

/// A wrapper around `AccountAddress` that can be used in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbAccountAddress(pub AccountAddress);

/// A wrapper around `Address` that can be used in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbAddress(pub Address);

/// A wrapper around `TokenAmount` that can be used in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbTokenAmount(pub TokenAmount);

impl DbTokenAmount {
    pub fn zero() -> Self { Self(TokenAmount(BigUint::zero())) }

    pub fn add_assign(&mut self, other: DbTokenAmount) { self.0 += other.0; }

    pub fn sub_assign(&mut self, other: DbTokenAmount) { self.0 -= other.0; }
}

/// A wrapper around `TokenId` that can be used in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbTokenId(pub TokenId);
impl From<TokenId> for DbTokenId {
    fn from(value: TokenId) -> Self { Self(value) }
}

/// A trait for Generic Mongodb Collection operations.
#[async_trait]
pub trait ICollection {
    type ItemType: Serialize + Send + Sync + Deserialize<'static> + DeserializeOwned + Unpin;

    fn collection(&self) -> &mongodb::Collection<Self::ItemType>;

    /// Insert a single item into the collection.
    /// # Arguments
    /// * `item` - The item to insert.
    /// # Returns
    /// * `Result` - Ok(()) if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn insert_one(&self, item: Self::ItemType) -> anyhow::Result<()> {
        let collection = self.collection();
        let options = mongodb::options::InsertOneOptions::builder().build();
        collection.insert_one(item, options).await?;
        Ok(())
    }

    /// Delete a single item from the collection.
    /// # Arguments
    /// * `key` - The key to delete.
    /// # Returns
    /// * `Result` - Ok(()) if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn delete_one(&self, key: Document) -> anyhow::Result<()> {
        let collection = self.collection();
        let options = mongodb::options::DeleteOptions::builder().build();
        collection.delete_one(key, options).await?;
        Ok(())
    }

    /// Update a single item in the collection.
    /// # Arguments
    /// * `key` - The key to update.
    /// * `update` - The update to apply.
    /// # Returns
    /// * `Result` - Ok(()) if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn update_one(&self, key: Document, update: Self::ItemType) -> anyhow::Result<()> {
        let collection = self.collection();
        let options = mongodb::options::UpdateOptions::builder().build();
        collection
            .update_one(
                key,
                doc! {
                    "$set": to_document(&update)?
                },
                options,
            )
            .await?;
        Ok(())
    }

    /// Update many items in the collection.
    /// # Arguments
    /// * `query` - The query to match.
    /// * `update` - The update to apply.
    /// # Returns
    /// * `Result` - Ok(()) if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn update_many(
        &self,
        query: Document,
        update: impl Into<UpdateModifications> + Send + Sync,
    ) -> anyhow::Result<()> {
        let collection = self.collection();
        let options = mongodb::options::UpdateOptions::builder().build();
        collection.update_many(query, update, options).await?;
        Ok(())
    }

    /// Find a single item in the collection.
    /// # Arguments
    /// * `key` - The key to find.
    /// # Returns
    /// * `Result` - Ok(Some(item)) if the operation was successful and the item
    ///   was found.
    /// * `Result` - Ok(None) if the operation was successful and the item was
    ///   not found.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn find_one(&self, key: Document) -> anyhow::Result<Option<Self::ItemType>> {
        let collection = self.collection();
        let options = mongodb::options::FindOneOptions::builder().build();
        let result = collection.find_one(key, options).await?;
        Ok(result)
    }

    /// Upsert a single item in the collection.
    /// # Arguments
    /// * `key` - The key to upsert.
    /// * `update` - The update to apply.
    /// # Returns
    /// * `Result` - Ok(()) if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn upsert_one<TFn: Fn(Option<Self::ItemType>) -> Self::ItemType + Send + Sync>(
        &self,
        key: Document,
        update: TFn,
    ) -> anyhow::Result<()> {
        let item = self.find_one(key.to_owned()).await?;
        match item {
            Some(item) => self.update_one(key, update(Some(item))).await,
            None => self.insert_one(update(None)).await,
        }
    }

    /// Find items in the collection. Matching `query`.
    /// # Arguments
    /// * `query` - The query to match.
    /// * `skip` - The number of items to skip.
    /// * `take` - The number of items to take.
    /// # Returns
    /// * `Result` - A cursor to the items if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn find(
        &self,
        query: Document,
        skip: u64,
        take: i64,
    ) -> anyhow::Result<mongodb::Cursor<Self::ItemType>> {
        let collection = self.collection();
        let options = mongodb::options::FindOptions::builder().skip(skip).limit(take).build();
        let cursor = collection.find(query, options).await?;
        Ok(cursor)
    }

    /// Count items in the collection. Matching `query`.
    /// # Arguments
    /// * `query` - The query to match.
    /// # Returns
    /// * `Result` - The number of items if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn count(&self, query: Document) -> anyhow::Result<u64> {
        let collection = self.collection();
        let options = mongodb::options::CountOptions::builder().build();
        let count = collection.count_documents(query, options).await?;
        Ok(count)
    }
}

pub struct Collection<T>(mongodb::Collection<T>);
impl<T> Collection<T> {
    pub fn new(collection: mongodb::Collection<T>) -> Self { Self(collection) }
}
impl<T> ICollection for Collection<T>
where
    T: Serialize + Send + Sync + DeserializeOwned + Unpin,
{
    type ItemType = T;

    fn collection(&self) -> &mongodb::Collection<Self::ItemType> { &self.0 }
}
impl<T> From<mongodb::Collection<T>> for Collection<T> {
    fn from(collection: mongodb::Collection<T>) -> Self { Self::new(collection) }
}
