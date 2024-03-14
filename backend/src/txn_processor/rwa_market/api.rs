//! This module contains the API implementation for the RWA market.
//!
//! The `RwaMarketApi` struct provides methods to retrieve paged lists of tokens that are listed, unlisted, or deposited in the RWA market.
//! It interacts with the `IRwaMarketDb` trait to fetch data from the database.
//!
//! The API endpoints are defined using the `poem_openapi` and `poem` crates, and the responses are serialized as JSON using the `Json` type.
//!
//! The `MarketToken` struct represents a token listed in the RWA market, containing information such as the token contract address, token ID, owner, deposited amount, listed amount, and unlisted amount.
//!
//! The `RwaMarketApi` struct has the following methods:
//! - `listed`: Retrieves a paged list of tokens that are listed in the RWA market.
//! - `unlisted`: Retrieves a paged list of tokens that are unlisted in the RWA market for a specific owner.
//! - `deposited`: Retrieves a paged list of tokens that are deposited in the RWA market for a specific owner.
//!
//! The `to_paged_response` method is a helper method used by the above methods to convert the query result into a paged response.
use crate::shared::{
    api::{ApiContractAddress, Error, PagedResponse, PAGE_SIZE},
    db::{DbAccountAddress, DbTokenAmount, ICollection},
};
use bson::{doc, to_bson, Document};
use concordium_rust_sdk::types::ContractAddress;
use futures::TryStreamExt;
use poem::Result;
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};

use super::db::{DbDepositedToken, IRwaMarketDb};

#[derive(Object)]
pub struct MarketToken {
    pub token_contract:   ApiContractAddress,
    pub token_id:         String,
    pub owner:            String,
    pub deposited_amount: String,
    pub listed_amount:    String,
    pub unlisted_amount:  String,
}

impl From<DbDepositedToken> for MarketToken {
    fn from(db_deposited_token: DbDepositedToken) -> Self {
        Self {
            token_contract:   db_deposited_token.token_contract.into(),
            token_id:         db_deposited_token.token_id.0.into(),
            owner:            db_deposited_token.owner.0.to_string(),
            deposited_amount: db_deposited_token.deposited_amount.0.to_string(),
            listed_amount:    db_deposited_token.listed_amount.0.to_string(),
            unlisted_amount:  db_deposited_token.unlisted_amount.0.to_string(),
        }
    }
}

/// Represents the RWA Market API.
pub struct RwaMarketApi<TDb: IRwaMarketDb> {
    pub db: TDb,
}

/// API implementation for the RWA market.
#[OpenApi]
impl<TDb: IRwaMarketDb + Sync + Send + 'static> RwaMarketApi<TDb> {
    /// Retrieves a paged list of tokens that are listed in the RWA market.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the contract.
    /// - `subindex`: The subindex of the contract.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    ///
    /// A `PagedResponse` containing a list of `MarketToken` objects.
    #[oai(path = "/rwa-market/:index/:subindex/listed/:page", method = "get")]
    pub async fn listed(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<MarketToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "listed_amount": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_response(query, contract, page).await?;
        Ok(Json(res))
    }

    /// Retrieves a paged list of tokens that are unlisted in the RWA market for a specific owner.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the contract.
    /// - `subindex`: The subindex of the contract.
    /// - `owner`: The owner of the tokens.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    ///
    /// A `PagedResponse` containing a list of `MarketToken` objects.
    #[oai(path = "/rwa-market/:index/:subindex/unlisted/:owner/:page", method = "get")]
    pub async fn unlisted(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(owner): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<MarketToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "owner": to_bson(&DbAccountAddress(owner.parse()?))?,
            "unlisted_amount": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_response(query, contract, page).await?;
        Ok(Json(res))
    }

    /// Retrieves a paged list of tokens that are deposited in the RWA market for a specific owner.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the contract.
    /// - `subindex`: The subindex of the contract.
    /// - `owner`: The owner of the tokens.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    ///
    /// A `PagedResponse` containing a list of `MarketToken` objects.
    #[oai(path = "/rwa-market/:index/:subindex/deposited/:owner/:page", method = "get")]
    pub async fn deposited(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(owner): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<MarketToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "owner": to_bson(&DbAccountAddress(owner.parse()?))?,
            "deposited_amount": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_response(query, contract, page).await?;
        Ok(Json(res))
    }

    /// Converts the query result into a paged response.
    ///
    /// # Parameters
    ///
    /// - `query`: The query document.
    /// - `contract`: The contract address.
    /// - `page`: The page number.
    ///
    /// # Returns
    ///
    /// A `PagedResponse` containing a list of `MarketToken` objects.
    pub async fn to_paged_response(
        &self,
        query: Document,
        contract: ContractAddress,
        page: u64,
    ) -> anyhow::Result<PagedResponse<MarketToken>> {
        let coll = self.db.deposited_tokens(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<DbDepositedToken> = cursor.try_collect().await?;
        let data: Vec<MarketToken> = data.into_iter().map(|token| token.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;

        Ok(PagedResponse {
            page_count,
            page,
            data,
        })
    }
}
