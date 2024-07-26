use async_trait::async_trait;
use bigdecimal::BigDecimal;
use bson::{doc, to_document, Document};
use concordium_rust_sdk::{
    cis2::{self, TokenAmount, TokenId},
    smart_contracts::common::AccountAddress,
    types::{Address, ContractAddress},
};
use diesel::PgConnection;
use itertools::Itertools;
use mongodb::options::{FindOneOptions, UpdateModifications};
use num_bigint::{BigInt, BigUint};
use num_traits::{identities::Zero, Num};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, diesel::result::Error>;

pub fn address_to_sql_string(addr: &Address) -> String {
    match addr {
        Address::Account(account) => format!("acc:{}", account),
        Address::Contract(contract) => format!("con:{}/{}", contract.index, contract.subindex),
    }
}

pub fn address_from_sql_string(addr: &str) -> anyhow::Result<Address> {
    let prefix = &addr[0..3];
    let addr_str = &addr[4..];
    match prefix {
        "acc" => Ok(Address::Account(addr_str.parse()?)),
        "con" => {
            let parts = addr_str.split('/').collect_vec();
            if parts.len() != 2 {
                Err(anyhow::Error::msg(format!("Error parsing Contract Address: {}", addr_str)))
            } else {
                let index_str = parts[0];
                let sub_index_str = parts[1];
                let index: u64 = index_str.parse().map_err(|_| {
                    anyhow::Error::msg(format!("Error parsing contract index: {}", index_str))
                })?;
                let subindex: u64 = sub_index_str.parse().map_err(|_| {
                    anyhow::Error::msg(format!(
                        "Error parsing contract sub index: {}",
                        sub_index_str
                    ))
                })?;
                Ok(Address::Contract(ContractAddress {
                    index,
                    subindex,
                }))
            }
        }
        _ => Err(anyhow::Error::msg(format!("Error parsing prefix: {}", prefix))),
    }
}

pub fn token_id_to_sql(token_id: &cis2::TokenId) -> BigDecimal {
    BigInt::from_str_radix(&token_id.to_string(), 16)
        .expect("Error converting from Cis2 TokenId to BigInt")
        .into()
}

pub fn token_amount_to_sql(amount: &cis2::TokenAmount) -> BigDecimal {
    BigInt::from_bytes_le(num_bigint::Sign::NoSign, &amount.0.to_bytes_le()).into()
}

#[cfg(test)]
mod test {
    use concordium_rust_sdk::types::{Address, ContractAddress};

    use super::{address_from_sql_string, address_to_sql_string};

    #[test]
    pub fn address_to_sql_conversions() {
        let con_addr = Address::Contract(ContractAddress::new(19, 20));
        let str = address_to_sql_string(&con_addr);
        assert_eq!(str, "con:19/20");
        let con_addr_2 = address_from_sql_string(&str).unwrap();
        assert_eq!(con_addr, con_addr_2);

        let account_addr =
            Address::Account("47fb97YAZtEEYNpaWz3ccrUCwqEnNfm2qQXiUGHEJ52Fiu7AVi".parse().unwrap());
        let str = address_to_sql_string(&account_addr);
        assert_eq!(str, "acc:47fb97YAZtEEYNpaWz3ccrUCwqEnNfm2qQXiUGHEJ52Fiu7AVi");
        let account_addr_2 = address_from_sql_string(&str).unwrap();
        assert_eq!(account_addr_2, account_addr);
    }
}

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
pub trait ICollection: Send + Sync {
    type ItemType: Serialize + Send + Sync + Deserialize<'static> + DeserializeOwned + Unpin;

    fn collection(&self) -> &mongodb::Collection<Self::ItemType>;

    /// Insert a single item into the collection.
    /// # Arguments
    /// * `item` - The item to insert.
    /// # Returns
    /// * `Result` - Ok(()) if the operation was successful.
    /// # Errors
    /// * `anyhow::Error` - If the operation failed.
    async fn insert_one(&mut self, item: Self::ItemType) -> anyhow::Result<()> {
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
    async fn delete_one(&mut self, key: Document) -> anyhow::Result<()> {
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
    async fn update_one(&mut self, key: Document, update: Self::ItemType) -> anyhow::Result<()> {
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
        &mut self,
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
        self.collection().find_one(key, FindOneOptions::builder().build()).await.map_err(Into::into)
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
        &mut self,
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

/// A wrapper around `mongodb::Collection` that implements `ICollection`.
#[derive(Clone)]
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
