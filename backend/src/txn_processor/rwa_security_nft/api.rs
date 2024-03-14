//! This module contains the API implementation for the RWA security NFT.
//! The `RwaSecurityNftApi` struct provides methods to retrieve paged lists of
//! tokens and holders for a specific RWA security NFT contract. It interacts
//! with the `IRwaSecurityNftDb` trait to fetch data from the database.
//! The API endpoints are defined using the `poem_openapi` and `poem` crates,
//! and the responses are serialized as JSON using the `Json` type.

use super::db::{DbToken, IRwaSecurityNftDb, TokenHolder};
use crate::shared::{
    api::{ApiAddress, Error, PagedResponse, PAGE_SIZE},
    db::{DbAddress, DbTokenAmount, ICollection},
};
use bson::{doc, to_bson, Document};
use concordium_rust_sdk::types::ContractAddress;
use futures::TryStreamExt;
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};

/// The `ApiNftToken` struct represents a token in the RWA security NFT,
/// containing information such as the token ID, whether it is paused, the
/// metadata URL, the metadata URL hash, and the supply.
#[derive(Object)]
pub struct ApiNftToken {
    pub token_id:          String,
    pub is_paused:         bool,
    pub metadata_url:      String,
    pub metadata_url_hash: String,
    pub supply:            String,
}

impl From<DbToken> for ApiNftToken {
    fn from(db_token: DbToken) -> Self {
        Self {
            token_id:          db_token.token_id.0.into(),
            is_paused:         db_token.is_paused,
            metadata_url:      db_token.metadata_url.unwrap_or_default(),
            metadata_url_hash: db_token.metadata_url_hash.unwrap_or_default(),
            supply:            db_token.supply.0.to_string(),
        }
    }
}

/// A struct representing holder of a RWA security NFT.
#[derive(Object)]
pub struct ApiNftHolder {
    pub token_id:       String,
    pub address:        ApiAddress,
    pub balance:        String,
    pub frozen_balance: String,
}

impl From<TokenHolder> for ApiNftHolder {
    fn from(token_holder: TokenHolder) -> Self {
        Self {
            token_id:       token_holder.token_id.0.into(),
            address:        token_holder.address.into(),
            balance:        token_holder.balance.0.to_string(),
            frozen_balance: token_holder.frozen_balance.0.to_string(),
        }
    }
}

/// The RWA security NFT API.
pub struct RwaSecurityNftApi<TDb: IRwaSecurityNftDb> {
    pub db: TDb,
}

/// API implementation for the RWA security NFT.
#[OpenApi]
impl<TDb: IRwaSecurityNftDb + Sync + Send + 'static> RwaSecurityNftApi<TDb> {
    /// Get the list of tokens for a specific RWA security NFT contract.
    ///
    /// # Parameters
    /// - `index`: The index of the RWA security NFT contract.
    /// - `subindex`: The subindex of the RWA security NFT contract.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A list of tokens for the RWA security NFT contract.
    #[oai(path = "/rwa-security-nft/:index/:subindex/tokens/:page", method = "get")]
    pub async fn tokens(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<ApiNftToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "supply": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_token_response(query, contract, page).await?;
        Ok(Json(res))
    }

    /// Get the list of holders for a specific RWA security NFT contract.
    ///
    /// # Parameters
    /// - `index`: The index of the RWA security NFT contract.
    /// - `subindex`: The subindex of the RWA security NFT contract.
    /// - `address`: The address of the holder.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A list of holders for the RWA security NFT contract.
    #[oai(path = "/rwa-security-nft/:index/:subindex/holders/:address/:page", method = "get")]
    pub async fn holders(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(address): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<ApiNftHolder>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let address: DbAddress = DbAddress(address.parse()?);
        let query = doc! {
            "address": to_bson(&address)?,
            "balance": {
                "$ne": "0",
            }
        };
        let coll = self.db.holders(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<TokenHolder> = cursor.try_collect().await?;
        let data: Vec<ApiNftHolder> = data.into_iter().map(|holder| holder.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;
        let res = PagedResponse {
            page_count,
            page: 0,
            data,
        };

        Ok(Json(res))
    }

    /// Get the list of holders of a specific token for a RWA security NFT
    /// contract.
    ///
    /// # Parameters
    /// - `index`: The index of the RWA security NFT contract.
    /// - `subindex`: The subindex of the RWA security NFT contract.
    /// - `token_id`: The ID of the token.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A list of holders of the specified token for the RWA security NFT
    /// contract.
    #[oai(path = "/rwa-security-nft/:index/:subindex/holdersOf/:token_id/:page", method = "get")]
    pub async fn holders_of(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(token_id): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<ApiNftHolder>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "token_id": token_id,
        };
        let coll = self.db.holders(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<TokenHolder> = cursor.try_collect().await?;
        let data: Vec<ApiNftHolder> = data.into_iter().map(|holder| holder.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;
        let res = PagedResponse {
            page_count,
            page: 0,
            data,
        };

        Ok(Json(res))
    }

    /// Convert the query result to a paged token response.
    ///
    /// # Parameters
    /// - `query`: The query to filter the tokens.
    /// - `contract`: The contract address of the RWA security NFT contract.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A paged response containing the filtered tokens.
    pub async fn to_paged_token_response(
        &self,
        query: Document,
        contract: ContractAddress,
        page: u64,
    ) -> anyhow::Result<PagedResponse<ApiNftToken>> {
        let coll = self.db.tokens(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<DbToken> = cursor.try_collect().await?;
        let data: Vec<ApiNftToken> = data.into_iter().map(|token| token.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;

        Ok(PagedResponse {
            page_count,
            page,
            data,
        })
    }
}
