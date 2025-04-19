use poem::web::Data;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use rust_decimal::Decimal;
use shared::api::PagedResponse;
use shared::db::cis2_security::{Agent, Token, TokenHolderBalanceUpdateType};
use shared::db::security_mint_fund::{InvestmentRecord, SecurityMintFund};
use shared::db::security_p2p_trading::{ExchangeRecord, Market};
use shared::db::security_sft_multi_yielder::{Treasury, Yield, YieldType};
use shared::db::txn_listener::{ListenerBlock, ListenerContract};
use shared::db_app::forest_project_crypto::prelude::SecurityTokenContractType;
use shared::db_app::tokens::{
    ForestProjectContract, InvestorUser, TokenContract, TokenHolderUser,
    TokenHolderUserBalanceUpdate, TraderUser, UserYieldDistribution,
};
use shared::db_shared::DbPool;
use uuid::Uuid;

use super::{ensure_is_admin, ApiTags, BearerAuthorization, SystemContractsConfig};
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
        path = "/admin/indexer/contract-exists",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_contract_exists(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
    ) -> JsonResult<bool> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        ListenerContract::find(&mut conn, contract_address)?
            .map_or(Ok(Json(false)), |_| Ok(Json(true)))
    }

    #[oai(
        path = "/admin/indexer/token-contract",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_token_contract(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
    ) -> JsonResult<Option<TokenContract>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let contract = TokenContract::find(&mut conn, contract_address)?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/indexer/fp-token-contracts",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_fp_token_contracts(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(project_id): Query<Option<Uuid>>,
        Query(contract_type): Query<Option<SecurityTokenContractType>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectContract>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (contract, page_count) =
            ForestProjectContract::list(&mut conn, project_id, contract_type, page, page_size)?;
        Ok(Json(PagedResponse::new(contract, page, page_count)))
    }

    #[oai(
        path = "/admin/indexer/fp-token-contract",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_fp_token_contract(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
    ) -> JsonResult<Option<ForestProjectContract>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let contract = ForestProjectContract::find(&mut conn, contract_address)?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/indexer/agents",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_agents(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<Agent>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (agents, page_count) = Agent::list(&mut conn, contract_address, page, page_size)?;
        Ok(Json(PagedResponse::new(agents, page, page_count)))
    }

    #[oai(
        path = "/admin/indexer/tokens",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_tokens(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Option<Decimal>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<Token>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (tokens, page_count) = Token::list(&mut conn, contract_address, page, page_size)?;
        Ok(Json(PagedResponse::new(tokens, page, page_count)))
    }

    #[oai(
        path = "/admin/indexer/token",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_token(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
        Query(token_id): Query<Decimal>,
    ) -> JsonResult<Option<Token>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Ok(Json(Token::find(&mut conn, contract_address, token_id)?))
    }

    #[oai(
        path = "/admin/indexer/market",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_market(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
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

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/markets",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_markets(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<Market>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (markets, page_count) = Market::list(
            &mut conn,
            contracts.trading_contract_index,
            contract_address.map(|c| vec![c]),
            token_id,
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse::new(markets, page, page_count)))
    }

    #[oai(path = "/admin/indexer/fund", method = "get", tag = "ApiTags::Indexer")]
    pub async fn admin_indexer_fund(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
        Query(token_id): Query<Decimal>,
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
        path = "/admin/indexer/funds",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_funds(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(investment_token_contract_address): Query<Decimal>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<SecurityMintFund>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (funds, page_count) = SecurityMintFund::list_by_investment_contracts(
            &mut conn,
            contracts.mint_funds_contract_index,
            Some(&[investment_token_contract_address]),
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse::new(funds, page, page_count)))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/investment-records",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_investment_records(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(investment_token_contract): Query<Option<Decimal>>,
        Query(investment_token_id): Query<Option<Decimal>>,
        Query(investor): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<InvestmentRecord>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
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
        path = "/admin/indexer/exchange-records",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_exchange_records(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(token_contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(buyer): Query<Option<String>>,
        Query(seller): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<ExchangeRecord>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
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
        path = "/admin/indexer/agent",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_agent(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
        Query(agent_address): Query<String>,
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
        path = "/admin/indexer/yields",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_yields(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(token_contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(yielded_token_contract_address): Query<Option<Decimal>>,
        Query(yielded_token_id): Query<Option<Decimal>>,
        Query(yield_type): Query<Option<YieldType>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<Yield>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (yields, page_count) = Yield::list(
            &mut conn,
            contracts.yielder_contract_index,
            token_contract_address,
            token_id,
            yielded_token_contract_address,
            yielded_token_id,
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

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/yield-distributions",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_yield_distributions(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(forest_project_id): Query<Option<Uuid>>,
        Query(token_contract_address): Query<Option<Decimal>>,
        Query(to_address): Query<Option<String>>,
        Query(yielded_token_contract_address): Query<Option<Decimal>>,
        Query(yielded_token_id): Query<Option<Decimal>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<UserYieldDistribution>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (distributions, page_count) = UserYieldDistribution::list(
            &mut conn,
            contracts.yielder_contract_index,
            forest_project_id,
            token_contract_address,
            to_address.as_deref(),
            yielded_token_contract_address,
            yielded_token_id,
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: distributions,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/indexer/yielded-tokens",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_yielded_tokens(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(token_contract_address): Query<Decimal>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<Token>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (tokens, page_count) = Yield::list_yielded_tokens(
            &mut conn,
            contracts.yielder_contract_index,
            token_contract_address,
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: tokens,
            page_count,
            page,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/holders",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_holders(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(forest_project_id): Query<Option<Uuid>>,
        Query(contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(holder_address): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<TokenHolderUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (holders, page_count) = TokenHolderUser::list(
            &mut conn,
            forest_project_id,
            contract_address,
            token_id,
            holder_address.as_deref(),
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: holders,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/indexer/holder",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_holder(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(contract_address): Query<Decimal>,
        Query(token_id): Query<Decimal>,
        Query(holder_address): Query<String>,
    ) -> JsonResult<Option<TokenHolderUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let holder = TokenHolderUser::find(&mut conn, contract_address, token_id, &holder_address)?;
        Ok(Json(holder))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/balance-updates",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_balance_updates(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(forest_project_id): Query<Option<Uuid>>,
        Query(contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(holder_address): Query<Option<String>>,
        Query(update_type): Query<Option<TokenHolderBalanceUpdateType>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<TokenHolderUserBalanceUpdate>> {
        ensure_is_admin(&claims)?;

        let mut conn = db_pool.get()?;
        let (updates, page_count) = TokenHolderUserBalanceUpdate::list(
            &mut conn,
            forest_project_id,
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
        path = "/admin/indexer/treasury",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_treasury(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<Option<Treasury>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let treasury = Treasury::find(&mut conn, contracts.yielder_contract_index)?;
        Ok(Json(treasury))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/investors",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_investors(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(forest_project_id): Query<Option<Uuid>>,
        Query(investment_contract_address): Query<Option<Decimal>>,
        Query(investment_token_id): Query<Option<Decimal>>,
        Query(investor): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<InvestorUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (investors, page_count) = InvestorUser::list(
            &mut conn,
            contracts.mint_funds_contract_index,
            forest_project_id,
            investment_contract_address,
            investment_token_id,
            investor.as_deref(),
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: investors,
            page_count,
            page,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/admin/indexer/traders",
        method = "get",
        tag = "ApiTags::Indexer"
    )]
    pub async fn admin_indexer_traders(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(forest_project_id): Query<Option<Uuid>>,
        Query(token_contract_address): Query<Option<Decimal>>,
        Query(token_id): Query<Option<Decimal>>,
        Query(trader): Query<Option<String>>,
        Query(page): Query<i64>,
        Query(page_size): Query<i64>,
    ) -> JsonResult<PagedResponse<TraderUser>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (traders, page_count) = TraderUser::list(
            &mut conn,
            contracts.trading_contract_index,
            forest_project_id,
            token_contract_address,
            token_id,
            trader.as_deref(),
            page,
            page_size,
        )?;
        Ok(Json(PagedResponse {
            data: traders,
            page_count,
            page,
        }))
    }
}
