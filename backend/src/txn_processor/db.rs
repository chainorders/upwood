use async_trait::async_trait;
use bson::{doc, to_document, Document};
use concordium_rust_sdk::{
    cis2::{TokenAmount, TokenId},
    smart_contracts::common::AccountAddress,
    types::{smart_contracts::OwnedContractName, Address, ContractAddress},
};
use mongodb::options::UpdateModifications;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbContractAddress(pub ContractAddress);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbAccountAddress(pub AccountAddress);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbAddress(pub Address);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbTokenAmount(pub TokenAmount);

impl DbTokenAmount {
    pub fn zero() -> Self { Self(TokenAmount(BigUint::zero())) }

    pub fn add_assign(&mut self, other: DbTokenAmount) { self.0 += other.0; }

    pub fn sub_assign(&mut self, other: DbTokenAmount) { self.0 -= other.0; }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbTokenId(pub TokenId);
impl From<TokenId> for DbTokenId {
    fn from(value: TokenId) -> Self { Self(value) }
}

pub trait IDb {
    fn client(&self) -> &mongodb::Client;
    fn contract_name(&self) -> &OwnedContractName;

    fn database(&self, contract: &ContractAddress) -> mongodb::Database {
        self.client().database(&format!(
            "{}-{}-{}",
            self.contract_name(),
            contract.index,
            contract.subindex
        ))
    }
}

#[derive(Clone)]
pub struct ContractDb {
    pub client: mongodb::Client,
    pub contract_name: OwnedContractName,
}
impl IDb for ContractDb {
    fn client(&self) -> &mongodb::Client { &self.client }
    fn contract_name(&self) -> &OwnedContractName {
        &self.contract_name
    }
}

#[async_trait]
pub trait ICollection {
    type ItemType: Serialize + Send + Sync + Deserialize<'static> + DeserializeOwned + Unpin;

    fn collection(&self) -> &mongodb::Collection<Self::ItemType>;

    async fn insert_one(&self, item: Self::ItemType) -> anyhow::Result<()> {
        let collection = self.collection();
        let options = mongodb::options::InsertOneOptions::builder().build();
        collection.insert_one(item, options).await?;
        Ok(())
    }

    async fn delete_one(&self, key: Document) -> anyhow::Result<()> {
        let collection = self.collection();
        let options = mongodb::options::DeleteOptions::builder().build();
        collection.delete_one(key, options).await?;
        Ok(())
    }

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

    async fn find_one(&self, key: Document) -> anyhow::Result<Option<Self::ItemType>> {
        let collection = self.collection();
        let options = mongodb::options::FindOneOptions::builder().build();
        let result = collection.find_one(key, options).await?;
        Ok(result)
    }

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

    async fn find(&self, query: Document, skip: u64, take: i64) -> anyhow::Result<mongodb::Cursor<Self::ItemType>> {
        let collection = self.collection();
        let options = mongodb::options::FindOptions::builder()
            .skip(skip)
            .limit(take)
            .build();
        let cursor = collection.find(query, options).await?;
        Ok(cursor)
    }

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
