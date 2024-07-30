//! This module contains the API implementation for the RWA security NFT.
//! The `RwaSecurityNftApi` struct provides methods to retrieve paged lists of
//! tokens and holders for a specific RWA security NFT contract. It interacts
//! with the `IRwaSecurityNftDb` trait to fetch data from the database.
//! The API endpoints are defined using the `poem_openapi` and `poem` crates,
//! and the responses are serialized as JSON using the `Json` type.
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

/// The RWA security NFT API.
pub struct RwaSecurityNftApi;

/// API implementation for the RWA security NFT.
#[OpenApi]
impl RwaSecurityNftApi {
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
}
