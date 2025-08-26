use chrono::{Months, NaiveDateTime, Utc};
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::db::cis2_security::Token;
use shared::db_app::forest_project::ForestProjectState;
use shared::db_app::forest_project_crypto::prelude::SecurityTokenContractType;
use shared::db_app::portfolio::{portfolio_value_at, total_invested_value_till};
use shared::db_shared::DbPool;

use super::{ensure_account_registered, BearerAuthorization, JsonResult, SystemContractsConfig};
use crate::api::ApiTags;
const FOREST_PROJECT_STATES: [ForestProjectState; 3] = [
    ForestProjectState::Active,
    ForestProjectState::Bond,
    ForestProjectState::Funded,
];
const TOKEN_CONTRACT_TYPES: [SecurityTokenContractType; 2] = [
    SecurityTokenContractType::Bond,
    SecurityTokenContractType::Property,
];

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(
        path = "/portfolio/value",
        method = "get",
        tag = "ApiTags::InvestmentPortfolio"
    )]
    async fn portfolio_value(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(at): Query<NaiveDateTime>,
    ) -> JsonResult<Decimal> {
        let conn = &mut db_pool.get()?;
        let account = ensure_account_registered(&claims)?.to_string();
        let (un_frozen, _) = portfolio_value_at(
            conn,
            &account,
            contracts.euro_e_token_id,
            contracts.euro_e_contract_index,
            &FOREST_PROJECT_STATES,
            &TOKEN_CONTRACT_TYPES,
            at,
        )?;
        Ok(Json(un_frozen))
    }

    #[oai(
        path = "/portfolio/aggregate",
        method = "get",
        tag = "ApiTags::InvestmentPortfolio"
    )]
    async fn portfolio_aggregate(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(now): Query<Option<NaiveDateTime>>,
    ) -> JsonResult<InvestmentPortfolioUserAggregate> {
        let account = ensure_account_registered(&claims)?.to_string();
        let conn = &mut db_pool.get()?;
        let now = now.unwrap_or(Utc::now().naive_utc());
        let year_ago = now
            .checked_sub_months(Months::new(12))
            .expect("Failed to calculate year ago date");
        let (portfolio_value_now, _) = portfolio_value_at(
            conn,
            &account,
            contracts.euro_e_token_id,
            contracts.euro_e_contract_index,
            &FOREST_PROJECT_STATES,
            &TOKEN_CONTRACT_TYPES,
            now,
        )?;
        let (portfolio_value_year_ago, _) = portfolio_value_at(
            conn,
            &account,
            contracts.euro_e_token_id,
            contracts.euro_e_contract_index,
            &FOREST_PROJECT_STATES,
            &TOKEN_CONTRACT_TYPES,
            year_ago,
        )?;
        let yearly_returns = portfolio_value_now - portfolio_value_year_ago;
        let (invested_value, locked_invested_value) = total_invested_value_till(
            conn,
            &account.to_string(),
            contracts.euro_e_token_id,
            contracts.euro_e_contract_index,
            &FOREST_PROJECT_STATES,
            &TOKEN_CONTRACT_TYPES,
            now,
        )?;
        let roi = if invested_value.is_zero() {
            Decimal::ONE_HUNDRED
        } else {
            ((portfolio_value_now - invested_value) / invested_value) * Decimal::from(100)
        }
        .round_dp(2);
        let carbon_tons_offset = Token::total_burned(
            conn,
            &account.to_string(),
            contracts.carbon_credit_contract_index,
            contracts.carbon_credit_token_id,
            now,
        )?;
        let ret = InvestmentPortfolioUserAggregate {
            carbon_tons_offset,
            current_portfolio_value: portfolio_value_now,
            invested_value,
            locked_mint_fund_euro_e_amount: locked_invested_value,
            yearly_return: yearly_returns,
            return_on_investment: roi,
        };
        Ok(Json(ret))
    }

    #[oai(
        path = "/portfolio/value_last_n_months/:months",
        method = "get",
        tag = "ApiTags::InvestmentPortfolio"
    )]
    async fn get_portfolio_value_last_n_months(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
        Path(months): Path<u32>,
        Query(now): Query<Option<NaiveDateTime>>,
    ) -> JsonResult<Vec<PortfolioValue>> {
        let conn = &mut db_pool.get()?;
        let account = ensure_account_registered(&claims)?.to_string();
        let now = now.unwrap_or(Utc::now().naive_utc());
        let mut ret = Vec::new();
        for i in 0..months {
            let time = now.checked_sub_months(Months::new(i)).unwrap();
            let (un_frozen, _) = portfolio_value_at(
                conn,
                &account,
                contracts.euro_e_token_id,
                contracts.euro_e_contract_index,
                &FOREST_PROJECT_STATES,
                &TOKEN_CONTRACT_TYPES,
                time,
            )?;
            ret.push(PortfolioValue {
                portfolio_value: un_frozen,
                at:              time,
            });
        }
        Ok(Json(ret))
    }
}

#[derive(Serialize, Deserialize, Object, Eq, PartialEq, Debug)]
pub struct InvestmentPortfolioUserAggregate {
    /// The amount of locked euros in the all the mint fund contracts
    pub locked_mint_fund_euro_e_amount: Decimal,
    /// Sum of the amount invested in the mint funds and the amount bought in the P2P trading
    pub invested_value:                 Decimal,
    /// Sum Of(Balance of each Forest Project Token * the current price of the token)
    pub current_portfolio_value:        Decimal,
    /// Current portfolio value - Portfolio value at the beginning of the year - Amount invested in the year + Amount withdrawn in the year
    pub yearly_return:                  Decimal,
    /// (Current portfolio value - Amount withdrawn) / Total amount invested
    pub return_on_investment:           Decimal,
    /// The total amount of carbon credit tokens burned
    pub carbon_tons_offset:             Decimal,
}

#[derive(serde::Serialize, serde::Deserialize, Object)]
pub struct PortfolioValue {
    pub portfolio_value: Decimal,
    pub at:              NaiveDateTime,
}
