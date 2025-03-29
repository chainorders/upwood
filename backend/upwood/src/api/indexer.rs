use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use rust_decimal::Decimal;
use shared::api::PagedResponse;
use shared::db::cis2_security::{Agent, Token, TokenHolder};
use shared::db::security_mint_fund::SecurityMintFund;
use shared::db::security_p2p_trading::Market;
use shared::db::security_sft_multi_yielder::{Treasury, Yield};
use shared::db::txn_listener::{ListenerBlock, ListenerContract};
use shared::db_shared::DbPool;

use super::{ensure_is_admin, ApiTags, BearerAuthorization, SystemContractsConfig, PAGE_SIZE};
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
    ) -> JsonResult<Option<Token>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Ok(Json(Token::find(&mut conn, contract_address, token_id)?))
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
    ) -> JsonResult<Option<Market>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Ok(Json(Market::find(
            &mut conn,
            contracts.trading_contract_index,
            contract_address,
            token_id,
        )?))
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
        let (markets, _) = Market::list(
            &mut conn,
            contracts.trading_contract_index,
            Some(&[contract_address]),
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
    ) -> JsonResult<Option<SecurityMintFund>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Ok(Json(SecurityMintFund::find(
            &mut conn,
            contracts.mint_funds_contract_index,
            token_id,
            contract_address,
        )?))
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
        let (funds, _) = SecurityMintFund::list(
            &mut conn,
            contracts.mint_funds_contract_index,
            Some(&[contract_address]),
            0,
            i64::MAX,
        )?;
        Ok(Json(funds))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/agent/:agent_address",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_agent(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Path(agent_address): Path<String>,
        /// Whether the agent_address is a contract or not
        Query(is_contract): Query<bool>,
    ) -> JsonResult<Option<Agent>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let agent_address = if is_contract {
            format!("<{},0>", agent_address)
        } else {
            agent_address
        };

        let agent = Agent::find(&mut conn, contract_address, &agent_address)?;
        Ok(Json(agent))
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
    ) -> JsonResult<Vec<Yield>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let yields = Yield::list_for_token(
            &mut conn,
            contracts.yielder_contract_index,
            contract_address,
            token_id,
        )?;
        Ok(Json(yields))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/token/:token_id/holder/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token_holder_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<TokenHolder>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let page = page.unwrap_or(0);
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let (holders, page_count) =
            TokenHolder::list(&mut conn, contract_address, token_id, page, page_size)?;
        Ok(Json(PagedResponse {
            data: holders,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/token/:token_id/holder/:holder_address",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token_holder(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
        Path(holder_address): Path<String>,
    ) -> JsonResult<Option<TokenHolder>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let holder = TokenHolder::find(&mut conn, contract_address, token_id, &holder_address)?;
        Ok(Json(holder))
    }

    #[oai(
        path = "/admin/indexer/yielder/:contract_address/treasury",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_yielder_treasury(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
    ) -> JsonResult<Option<Treasury>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let treasury = Treasury::find(&mut conn, contract_address)?;
        Ok(Json(treasury))
    }
}
