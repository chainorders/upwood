//! This module contains the API implementation for the RWA security NFT.
//! The `RwaSecurityNftApi` struct provides methods to retrieve paged lists of
//! tokens and holders for a specific RWA security NFT contract. It interacts
//! with the `IRwaSecurityNftDb` trait to fetch data from the database.
//! The API endpoints are defined using the `poem_openapi` and `poem` crates,
//! and the responses are serialized as JSON using the `Json` type.

use super::db;
use crate::shared::{
    api::{ApiAddress, Error, PagedResponse, PAGE_SIZE},
    db::DbPool,
};
use concordium_rust_sdk::{
    cis2,
    types::{Address, ContractAddress},
};
use itertools::Itertools;
use poem::web::Data;
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

impl From<db::SecurityCis2Token> for ApiNftToken {
    fn from(value: db::SecurityCis2Token) -> Self {
        ApiNftToken {
            is_paused:         value.is_paused,
            metadata_url:      value.metadata_url,
            metadata_url_hash: value.metadata_hash.map(hex::encode).unwrap_or_default(),
            supply:            value.supply.to_string(),
            token_id:          value.token_id,
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

impl From<db::SecurityCis2TokenHolder> for ApiNftHolder {
    fn from(token_holder: db::SecurityCis2TokenHolder) -> Self {
        let address: ApiAddress = token_holder
            .holder_address
            .parse::<Address>()
            .expect("Error parsing holder address to address")
            .into();

        Self {
            token_id: token_holder.token_id,
            address,
            balance: token_holder.balance.to_string(),
            frozen_balance: token_holder.frozen_balance.to_string(),
        }
    }
}

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
    ) -> Result<Json<PagedResponse<ApiNftToken>>, Error> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_tokens_for_contract(&mut conn, &cis2_address, PAGE_SIZE as i64, page)?;
        let tokens: Vec<ApiNftToken> = tokens.into_iter().map(|t| t.into()).collect_vec();
        let res = PagedResponse {
            data:       tokens,
            page:       page as u64,
            page_count: page_count as u64,
        };
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
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(address): Path<String>,
        Path(page): Path<i64>,
    ) -> Result<Json<PagedResponse<ApiNftHolder>>, Error> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let holder_address: Address = address.parse()?;

        let mut conn = pool.get()?;
        let (tokens, page_count) = db::list_tokens_by_holder(
            &mut conn,
            &cis2_address,
            &holder_address,
            PAGE_SIZE as i64,
            page,
        )?;

        let tokens: Vec<ApiNftHolder> = tokens.into_iter().map(|t| t.into()).collect_vec();
        let res = PagedResponse {
            data:       tokens,
            page:       page as u64,
            page_count: page_count as u64,
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
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(token_id): Path<String>,
        Path(page): Path<i64>,
    ) -> Result<Json<PagedResponse<ApiNftHolder>>, Error> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let token_id: cis2::TokenId = token_id.parse()?;

        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_holders_by_token(&mut conn, &cis2_address, &token_id, PAGE_SIZE as i64, page)?;

        let tokens: Vec<ApiNftHolder> = tokens.into_iter().map(|t| t.into()).collect_vec();
        let res = PagedResponse {
            data:       tokens,
            page:       page as u64,
            page_count: page_count as u64,
        };
        Ok(Json(res))
    }
}
