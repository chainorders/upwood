//! The API for the RWA Security SFT module.
//!
//! This module contains the API for the RWA Security SFT module, which allows
//! users to retrieve paged lists of tokens, holders, and deposited tokens for
//! a specific RWA Security SFT contract. It interacts with the
//! `IRwaSecuritySftDb` trait to fetch data from the database.
use super::db::{DbDepositedToken, DbToken, RwaSecuritySftDb, TokenHolder};
use crate::shared::{
    api::{ApiAddress, ApiContractAddress, Error, PagedResponse, PAGE_SIZE},
    db::{DbAccountAddress, DbAddress, ICollection},
};
use bson::{doc, to_bson, Document};
use concordium_rust_sdk::types::ContractAddress;
use futures::TryStreamExt;
use log::debug;
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};

#[derive(Object, Debug)]
pub struct ApiSftToken {
    pub token_id:          String,
    pub is_paused:         bool,
    pub metadata_url:      String,
    pub metadata_url_hash: String,
    pub supply:            String,
}

impl From<DbToken> for ApiSftToken {
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

#[derive(Object)]
pub struct ApiSftHolder {
    pub token_id:       String,
    pub address:        ApiAddress,
    pub balance:        String,
    pub frozen_balance: String,
}

impl From<TokenHolder> for ApiSftHolder {
    fn from(token_holder: TokenHolder) -> Self {
        Self {
            token_id:       token_holder.token_id.0.into(),
            address:        token_holder.address.into(),
            balance:        token_holder.balance.0.to_string(),
            frozen_balance: token_holder.frozen_balance.0.to_string(),
        }
    }
}

#[derive(Object)]
pub struct ApiDepositedToken {
    pub token_contract:   ApiContractAddress,
    pub token_id:         String,
    pub owner:            String,
    pub deposited_amount: String,
    pub locked_amount:    String,
    pub un_locked_amount: String,
}

impl From<DbDepositedToken> for ApiDepositedToken {
    fn from(db_deposited_token: DbDepositedToken) -> Self {
        Self {
            token_contract:   db_deposited_token.token_contract.into(),
            token_id:         db_deposited_token.token_id.0.into(),
            owner:            db_deposited_token.owner.0.to_string(),
            deposited_amount: db_deposited_token.deposited_amount.0.to_string(),
            locked_amount:    db_deposited_token.locked_amount.0.to_string(),
            un_locked_amount: db_deposited_token.un_locked_amount.0.to_string(),
        }
    }
}

/// The API for the RWA Security SFT module.
pub struct RwaSecuritySftApi {
    pub client:        mongodb::Client,
    pub contract_name: String,
}

#[OpenApi]
impl RwaSecuritySftApi {
    pub fn db(&self, contract: &ContractAddress) -> RwaSecuritySftDb {
        let db = self
            .client
            .database(&format!("{}-{}-{}", self.contract_name, contract.index, contract.subindex));

        RwaSecuritySftDb::init(db)
    }

    /// Get all tokens for a specific contract
    ///
    /// # Parameters
    /// - `index`: The index of the contract.
    /// - `subindex`: The subindex of the contract.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A list of tokens for the contract.
    #[oai(path = "/rwa-security-sft/:index/:subindex/tokens/:page", method = "get")]
    pub async fn tokens(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<ApiSftToken>>, Error> {
        let contract = &ContractAddress {
            index,
            subindex,
        };
        let query = doc! {};
        let res = self.to_paged_token_response(query, contract, page).await?;
        debug!("tokens contract: {:?}, res: {:?}", contract, res.data);
        Ok(Json(res))
    }

    /// Get all holders.
    ///
    /// # Parameters
    /// - `index`: The index of the contract.
    /// - `subindex`: The subindex of the contract.
    /// - `address`: The address of the holder.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A list of holders for the token.
    #[oai(path = "/rwa-security-sft/:index/:subindex/holders/:address/:page", method = "get")]
    pub async fn holders(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(address): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<ApiSftHolder>>, Error> {
        let contract = &ContractAddress {
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
        let coll = self.db(contract).holders;
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<TokenHolder> = cursor.try_collect().await?;
        let data: Vec<ApiSftHolder> = data.into_iter().map(|holder| holder.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;
        let res = PagedResponse {
            page_count,
            page: 0,
            data,
        };

        Ok(Json(res))
    }

    /// Get all holders of a specific token
    ///
    /// # Parameters
    /// - `index`: The index of the contract.
    /// - `subindex`: The subindex of the contract.
    /// - `token_id`: The ID of the token.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A list of holders of the token.
    #[oai(path = "/rwa-security-sft/:index/:subindex/holdersOf/:token_id/:page", method = "get")]
    pub async fn holders_of(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(token_id): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<ApiSftHolder>>, Error> {
        let contract = &ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "token_id": token_id,
        };
        let coll = self.db(contract).holders;
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<TokenHolder> = cursor.try_collect().await?;
        let data: Vec<ApiSftHolder> = data.into_iter().map(|holder| holder.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;
        let res = PagedResponse {
            page_count,
            page: 0,
            data,
        };

        Ok(Json(res))
    }

    /// Get all deposited tokens for a specific owner
    ///
    /// # Parameters
    /// - `index`: The index of the contract.
    /// - `subindex`: The subindex of the contract.
    /// - `owner`: The address of the owner.
    /// - `page`: The page number of the results.
    ///
    /// # Returns
    /// A list of deposited tokens for the owner.
    #[oai(path = "/rwa-security-sft/:index/:subindex/deposited/:owner/:page", method = "get")]
    pub async fn deposited(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(owner): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<ApiDepositedToken>>, Error> {
        let contract = &ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "owner": to_bson(&DbAccountAddress(owner.parse()?))?,
            "deposited_amount": {
                "$ne": "0",
            }
        };
        let res = self.to_paged_deposited_token_response(query, contract, page).await?;
        Ok(Json(res))
    }

    async fn to_paged_token_response(
        &self,
        query: Document,
        contract: &ContractAddress,
        page: u64,
    ) -> anyhow::Result<PagedResponse<ApiSftToken>> {
        let coll = &self.db(contract).tokens;
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<DbToken> = cursor.try_collect().await?;
        let data: Vec<ApiSftToken> = data.into_iter().map(|token| token.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;

        Ok(PagedResponse {
            page_count,
            page,
            data,
        })
    }

    async fn to_paged_deposited_token_response(
        &self,
        query: Document,
        contract: &ContractAddress,
        page: u64,
    ) -> anyhow::Result<PagedResponse<ApiDepositedToken>> {
        let coll = &self.db(contract).deposited_tokens;
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<DbDepositedToken> = cursor.try_collect().await?;
        let data: Vec<ApiDepositedToken> = data.into_iter().map(|token| token.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;

        Ok(PagedResponse {
            page_count,
            page,
            data,
        })
    }
}
