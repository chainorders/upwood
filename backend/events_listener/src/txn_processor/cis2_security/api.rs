use concordium_rust_sdk::cis2;
use concordium_rust_sdk::types::{Address, ContractAddress};
use shared::api::{ApiAddress, ApiResult, PagedResponse, PAGE_SIZE};
use shared::db::DbPool;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

use super::db;

#[derive(Object)]
pub struct Token {
    pub token_id:          String,
    pub is_paused:         bool,
    pub metadata_url:      String,
    pub metadata_url_hash: String,
    pub supply:            String,
}

impl From<db::Token> for Token {
    fn from(value: db::Token) -> Self {
        Token {
            is_paused:         value.is_paused,
            metadata_url:      value.metadata_url,
            metadata_url_hash: value.metadata_hash.map(hex::encode).unwrap_or_default(),
            supply:            value.supply.to_string(),
            token_id:          value.token_id,
        }
    }
}

#[derive(Object)]
pub struct TokenHolder {
    pub token_id:       String,
    pub address:        ApiAddress,
    pub balance:        String,
    pub frozen_balance: String,
}

impl From<db::TokenHolder> for TokenHolder {
    fn from(token_holder: db::TokenHolder) -> Self {
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

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/security-cis2/:cis2_address/tokens", method = "get")]
    pub async fn tokens(
        &self,
        Data(pool): Data<&DbPool>,
        Path(cis2_address): Path<String>,
        Query(page): Query<i64>,
    ) -> ApiResult<PagedResponse<Token>> {
        let cis2_address: ContractAddress = cis2_address.parse()?;
        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_tokens_for_contract(&mut conn, &cis2_address, PAGE_SIZE, page)?;
        let tokens = tokens.into_iter().map(|t| t.into()).collect();
        let res = PagedResponse {
            data: tokens,
            page,
            page_count,
        };

        ApiResult::Ok(Json(res))
    }

    #[oai(
        path = "/security-cis2/:cis2_address/holder/:address/tokens",
        method = "get"
    )]
    pub async fn tokens_of_holder(
        self,
        Data(pool): Data<&DbPool>,
        Path(cis2_address): Path<String>,
        Path(address): Path<String>,
        Query(page): Query<i64>,
    ) -> ApiResult<PagedResponse<TokenHolder>> {
        let cis2_address: ContractAddress = cis2_address.parse()?;
        let holder_address: Address = address.parse()?;
        let mut conn = pool.get()?;

        let (tokens, page_count) =
            db::list_tokens_by_holder(&mut conn, &cis2_address, &holder_address, PAGE_SIZE, page)?;
        let tokens = tokens.into_iter().map(|t| t.into()).collect();
        let res = PagedResponse {
            data: tokens,
            page,
            page_count,
        };

        ApiResult::Ok(Json(res))
    }

    #[oai(
        path = "/security-cis2/:cis2_address/token/:token_id/holders",
        method = "get"
    )]
    pub async fn holders_of_token(
        &self,
        Data(pool): Data<&DbPool>,
        Path(cis2_address): Path<String>,
        Path(token_id): Path<String>,
        Query(page): Query<i64>,
    ) -> ApiResult<PagedResponse<TokenHolder>> {
        let cis2_address: ContractAddress = cis2_address.parse()?;
        let token_id: cis2::TokenId = token_id.parse()?;
        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_holders_by_token(&mut conn, &cis2_address, &token_id, PAGE_SIZE, page)?;

        let tokens = tokens.into_iter().map(|t| t.into()).collect();
        let res = PagedResponse {
            data: tokens,
            page,
            page_count,
        };
        ApiResult::Ok(Json(res))
    }
    // todo copy contract functions for the api, ex balanceOf
}
