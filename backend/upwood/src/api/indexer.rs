use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use rust_decimal::Decimal;
use shared::db::cis2_security::Token;
use shared::db::security_mint_fund::SecurityMintFund;
use shared::db::security_p2p_trading::Market;
use shared::db::security_sft_multi_yielder::Yield;
use shared::db::txn_listener::{ListenerBlock, ListenerContract};
use shared::db_app::forest_project_crypto::TokenMetadata;
use shared::db_shared::DbPool;

use super::{
    ensure_is_admin, ApiTags, BearerAuthorization, Error, SystemContractsConfig, PAGE_SIZE,
};
use crate::api::JsonResult;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(
        path = "/admin/indexer/block/latest",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_block_latest(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
    ) -> JsonResult<Option<ListenerBlock>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let block = ListenerBlock::find_last(&mut conn)?;
        Ok(Json(block))
    }

    #[oai(
        path = "/admin/indexer/contract/:contract_address/exists",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_contract_exists(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
    ) -> JsonResult<bool> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        ListenerContract::find(&mut conn, contract_address)?
            .map_or(Ok(Json(false)), |_| Ok(Json(true)))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/token/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<Vec<Token>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let page = page.unwrap_or(0);
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let (tokens, _) = Token::list(&mut conn, contract_address, page, page_size)?;
        Ok(Json(tokens))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/token/:token_id",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
    ) -> JsonResult<Token> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Token::find(&mut conn, contract_address, token_id)?
            .ok_or_else(|| Error::NotFound(PlainText("Token not found".to_string())))
            .map(Json)
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/token/:token_id/market",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token_market(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
    ) -> JsonResult<Market> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Market::find(
            &mut conn,
            contracts.trading_contract_index,
            contract_address,
            token_id,
        )?
        .ok_or_else(|| Error::NotFound(PlainText("Market not found".to_string())))
        .map(Json)
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/market/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_market_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
    ) -> JsonResult<Vec<Market>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let markets = Market::list(
            &mut conn,
            contracts.trading_contract_index,
            contract_address,
            0,
            i64::MAX,
        )?;
        Ok(Json(markets))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/token/:token_id/fund",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token_fund(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
    ) -> JsonResult<SecurityMintFund> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        SecurityMintFund::find(
            &mut conn,
            contracts.mint_funds_contract_index,
            token_id,
            contract_address,
        )?
        .ok_or_else(|| Error::NotFound(PlainText("Fund not found".to_string())))
        .map(Json)
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/fund/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_fund_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
    ) -> JsonResult<Vec<SecurityMintFund>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let funds = SecurityMintFund::list(
            &mut conn,
            contracts.mint_funds_contract_index,
            contract_address,
            0,
            i64::MAX,
        )?;
        Ok(Json(funds))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/token/:token_id/yields/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token_yields(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
    ) -> JsonResult<Vec<YieldApiModel>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let yields = Yield::list_for_token(
            &mut conn,
            contracts.yielder_contract_index,
            contract_address,
            token_id,
        )?
        .into_iter()
        .map(|(yield_, yield_token_metadata)| YieldApiModel {
            yield_,
            yield_token_metadata,
        })
        .collect();
        Ok(Json(yields))
    }
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct YieldApiModel {
    pub yield_:               Yield,
    pub yield_token_metadata: Option<TokenMetadata>,
}
