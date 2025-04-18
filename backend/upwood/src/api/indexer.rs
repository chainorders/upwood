use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use rust_decimal::Decimal;
use shared::api::PagedResponse;
use shared::db::cis2_security::{Agent, Token, TokenHolderBalanceUpdateType};
use shared::db::security_mint_fund::{InvestmentRecord, SecurityMintFund};
use shared::db::security_p2p_trading::{ExchangeRecord, Market};
use shared::db::security_sft_multi_yielder::{Treasury, Yield, YieldType};
use shared::db::txn_listener::{ListenerBlock, ListenerContract};
use shared::db_app::tokens::{TokenContract, TokenHolderUser, TokenHolderUserBalanceUpdate};
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
        path = "/admin/indexer/cis2/:contract_address",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
    ) -> JsonResult<Option<TokenContract>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let contract = TokenContract::find(&mut conn, contract_address)?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/indexer/cis2/:contract_address/agent/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_agents(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<Agent>> {
        ensure_is_admin(&claims)?;
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let mut conn = db_pool.get()?;
        let (agents, page_count) = Agent::list(&mut conn, contract_address, page, page_size)?;
        Ok(Json(PagedResponse::new(agents, page, page_count)))
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
    ) -> JsonResult<PagedResponse<Token>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let page = page.unwrap_or(0);
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let (tokens, page_count) = Token::list(&mut conn, contract_address, page, page_size)?;
        Ok(Json(PagedResponse::new(tokens, page, page_count)))
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
        path = "/admin/indexer/cis2/:contract_address/market",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_market(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_address): Path<Decimal>,
        Query(token_id): Query<Option<Decimal>>,
    ) -> JsonResult<Option<Market>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let market = Market::find(
            &mut conn,
            contracts.trading_contract_index,
            contract_address,
        )?;
        let market = match token_id {
            None => market,
            Some(token_id) => market.filter(|m| m.token_id.eq(&Some(token_id))),
        };
        Ok(Json(market))
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
        path = "/admin/indexer/cis2/fund/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_fund_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(investment_token_contract_address): Query<Decimal>,
    ) -> JsonResult<Vec<SecurityMintFund>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (funds, _) = SecurityMintFund::list_by_investment_contracts(
            &mut conn,
            contracts.mint_funds_contract_index,
            Some(&[investment_token_contract_address]),
            0,
            i64::MAX,
        )?;
        Ok(Json(funds))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/cis2/fund/investment-records/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_fund_investment_records(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(investment_token_contract): Query<Option<Decimal>>,
        Query(investment_token_id): Query<Option<Decimal>>,
        Query(investor): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<InvestmentRecord>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let (records, page_count) = InvestmentRecord::list(
            &mut conn,
            contracts.mint_funds_contract_index,
            investment_token_contract,
            investment_token_id,
            investor.as_deref(),
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: records,
            page_count,
            page,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/cis2/fund/market-records/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_market_trading_records(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(token_contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(buyer): Query<Option<String>>,
        Query(seller): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<ExchangeRecord>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let (records, page_count) = ExchangeRecord::list(
            &mut conn,
            contracts.trading_contract_index,
            token_contract_address,
            token_id,
            buyer.as_deref(),
            seller.as_deref(),
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: records,
            page_count,
            page,
        }))
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

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/yield/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_yield_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
        Query(token_contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(yield_token_contract_address): Query<Option<Decimal>>,
        Query(yield_token_id): Query<Option<Decimal>>,
        Query(yield_type): Query<Option<YieldType>>,
    ) -> JsonResult<PagedResponse<Yield>> {
        ensure_is_admin(&claims)?;
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let mut conn = db_pool.get()?;
        let (yields, page_count) = Yield::list(
            &mut conn,
            contracts.yielder_contract_index,
            token_contract_address,
            token_id,
            yield_token_contract_address,
            yield_token_id,
            yield_type,
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: yields,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/indexer/yield/:token_contract_address/tokens",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_yield_tokens(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Path(token_contract_address): Path<Decimal>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<Token>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (tokens, page_count) = Yield::list_yielded_tokens(
            &mut conn,
            contracts.yielder_contract_index,
            token_contract_address,
            page,
            page_size.unwrap_or(PAGE_SIZE),
        )?;
        Ok(Json(PagedResponse {
            data: tokens,
            page_count,
            page,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/cis2/holder/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis2_token_holder_list(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(holder_address): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<TokenHolderUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (holders, page_count) = TokenHolderUser::list(
            &mut conn,
            contract_address,
            token_id,
            holder_address.as_deref(),
            page,
            page_size.unwrap_or(PAGE_SIZE),
        )?;
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
    ) -> JsonResult<Option<TokenHolderUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let holder = TokenHolderUser::find(&mut conn, contract_address, token_id, &holder_address)?;
        Ok(Json(holder))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/cis2/balance-updates/list",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_cis_balance_updates(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(holder_address): Query<Option<String>>,
        Query(update_type): Query<Option<TokenHolderBalanceUpdateType>>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<TokenHolderUserBalanceUpdate>> {
        ensure_is_admin(&claims)?;

        let mut conn = db_pool.get()?;
        let page_size = page_size.unwrap_or(PAGE_SIZE);
        let (updates, page_count) = TokenHolderUserBalanceUpdate::list(
            &mut conn,
            contract_address,
            token_id,
            holder_address.as_deref(),
            update_type,
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: updates,
            page_count,
            page,
        }))
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
