use concordium_rust_sdk::types::{Address, ContractAddress};
use shared::api::{ApiResult, PagedResponse};
use shared::db::DbPool;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;

use super::db;
use crate::txn_processor::cis2_security;
const PAGE_SIZE: i64 = 20;

#[derive(Clone)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(
        path = "/nft_multi_rewarded/:contract/holder/:address/nonce",
        method = "get"
    )]
    pub async fn nonce(
        &self,
        Path(contract): Path<String>,
        Path(address): Path<String>,
        Data(pool): Data<&DbPool>,
    ) -> ApiResult<i64> {
        let contract: ContractAddress = contract.parse()?;
        let address: Address = address.parse()?;
        let mut conn = pool.get()?;
        let res = db::find_address_nonce(&mut conn, &contract, &address)?.unwrap_or(0);
        ApiResult::Ok(Json(res))
    }

    #[oai(
        path = "/nft_multi_rewarded/:contract/holders/metadata_url/:metadata_url",
        method = "get"
    )]
    pub async fn holders_by_token_metadata_url(
        &self,
        Path(contract): Path<String>,
        Path(metadata_url): Path<String>,
        Query(page): Query<i64>,
        Data(pool): Data<&DbPool>,
    ) -> ApiResult<PagedResponse<cis2_security::api::TokenHolder>> {
        let contract: ContractAddress = contract.parse()?;
        let mut conn = pool.get()?;
        let (holders, count) = cis2_security::db::list_holders_by_token_metadata_url(
            &mut conn,
            &contract,
            &metadata_url,
            PAGE_SIZE,
            page,
        )?;
        let res = PagedResponse {
            data: holders.into_iter().map(|h| h.into()).collect(),
            page,
            page_count: count,
        };
        ApiResult::Ok(Json(res))
    }
}
