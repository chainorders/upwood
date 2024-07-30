//! The API for the RWA Security SFT module.
//!
//! This module contains the API for the RWA Security SFT module, which allows
//! users to retrieve paged lists of tokens, holders, and deposited tokens for
//! a specific RWA Security SFT contract. It interacts with the
//! `IRwaSecuritySftDb` trait to fetch data from the database.
use super::cis2_api;
use crate::shared::{
    api::{Error, PagedRequest, PagedResponse, PAGE_SIZE},
    db::DbPool,
};
use concordium_rust_sdk::{
    cis2,
    types::{Address, ContractAddress},
};
use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi};

/// The API for the RWA Security SFT module.
pub struct RwaSecuritySftApi;

#[OpenApi]
impl RwaSecuritySftApi {
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
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(page): Path<i64>,
    ) -> Result<Json<PagedResponse<cis2_api::Token>>, Error> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let mut conn = pool.get()?;
        let res = cis2_api::tokens(&mut conn, PagedRequest {
            data: cis2_address,
            page,
            page_size: PAGE_SIZE,
        })?;
        Ok(res)
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
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(address): Path<String>,
        Path(page): Path<i64>,
    ) -> Result<Json<PagedResponse<cis2_api::TokenHolder>>, Error> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let holder_address: Address = address.parse()?;
        let mut conn = pool.get()?;
        cis2_api::holders(&mut conn, PagedRequest {
            data: (cis2_address, holder_address),
            page_size: PAGE_SIZE,
            page,
        })
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
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(token_id): Path<String>,
        Path(page): Path<i64>,
    ) -> Result<Json<PagedResponse<cis2_api::TokenHolder>>, Error> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let token_id: cis2::TokenId = token_id.parse()?;
        let mut conn = pool.get()?;
        cis2_api::holders_of(&mut conn, PagedRequest {
            data: (cis2_address, token_id),
            page_size: PAGE_SIZE,
            page,
        })
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
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(owner): Path<String>,
        Path(page): Path<i64>,
    ) -> Result<Json<PagedResponse<cis2_api::Cis2Deposit>>, Error> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let owner: Address = owner.parse()?;
        let mut conn = pool.get()?;
        cis2_api::deposits_for_address(&mut conn, PagedRequest {
            data: (cis2_address, owner),
            page_size: PAGE_SIZE,
            page,
        })
    }
}
