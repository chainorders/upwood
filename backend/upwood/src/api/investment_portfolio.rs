use chrono::{DateTime, Months, NaiveDateTime, Utc};
use concordium_rust_sdk::id::types::AccountAddress;
use diesel::dsl::select;
use diesel::QueryResult;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::db::cis2_security::Token;
use shared::db_app::portfolio::ForestProjectUserInvestmentAmount;
use shared::db_shared::{DbConn, DbPool};
use shared::schema_manual::{
    user_currency_value_for_forest_project_owned_tokens_at, user_exchange_profits,
    user_fund_profits,
};
use tracing::error;

use super::{ensure_account_registered, BearerAuthorization, JsonResult, SystemContractsConfig};
use crate::api::{ApiTags, Error};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(
        path = "/portfolio/aggregate",
        method = "get",
        tag = "ApiTags::InvestmentPortfolio"
    )]
    async fn portfolio_aggreagte(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
        Query(now): Query<Option<NaiveDateTime>>,
    ) -> JsonResult<InvestmentPortfolioUserAggregate> {
        let account = ensure_account_registered(&claims)?;
        let conn = &mut db_pool.get()?;
        let now = now.unwrap_or(Utc::now().naive_utc());
        let ret = InvestmentPortfolioUserAggregate::generate(
            conn,
            &claims.sub,
            &account,
            contracts.carbon_credit_contract_index,
            contracts.carbon_credit_token_id,
            now,
        );
        let ret = match ret {
            Ok(ret) => ret,
            Err(error) => {
                error!(
                    "Error while calculating investment portfolio aggregate: {:?}",
                    error
                );
                return Err(Error::InternalServer(PlainText(
                    "Error while calculating investment portfolio aggregate".to_string(),
                )));
            }
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
        Path(months): Path<u32>,
        Query(now): Query<Option<NaiveDateTime>>,
    ) -> JsonResult<Vec<PortfolioValue>> {
        let conn = &mut db_pool.get()?;
        let now = now.unwrap_or(Utc::now().naive_utc());
        let mut ret = Vec::new();
        for i in 0..months {
            let time = now.checked_sub_months(Months::new(i)).unwrap();
            let value = InvestmentPortfolioUserAggregate::calculate_portfolio_value(
                conn,
                &claims.sub,
                time,
            )?;
            ret.push(PortfolioValue {
                portfolio_value: value,
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
    /// Current portfolio value - Portfolio value at the beginning of the month - Amount invested in the month + Amount withdrawn in the month
    pub monthly_return:                 Decimal,
    /// (Current portfolio value - Amount withdrawn) / Total amount invested
    pub return_on_investment:           Decimal,
    /// The total amount of carbon credit tokens burned
    pub carbon_tons_offset:             Decimal,
}

use diesel::prelude::*;
impl InvestmentPortfolioUserAggregate {
    pub fn generate(
        conn: &mut DbConn,
        cognito_user_id: &str,
        account: &AccountAddress,
        carbon_credit_contract_index: Decimal,
        carbon_credit_token_id: Decimal,
        now: NaiveDateTime,
    ) -> QueryResult<Self> {
        let start_of_year = now.checked_sub_months(Months::new(12)).unwrap();
        let start_of_month = now.checked_sub_months(Months::new(1)).unwrap();

        let invested_amounts =
            ForestProjectUserInvestmentAmount::find_by_cognito_user_id(conn, cognito_user_id)?;
        let current_portfolio_value = Self::calculate_portfolio_value(conn, cognito_user_id, now)?;

        // yearly calculations
        let portfolio_value_at_start_of_year =
            Self::calculate_portfolio_value(conn, cognito_user_id, start_of_year)?;
        let exchange_profit_year = select(user_exchange_profits(
            cognito_user_id.to_string(),
            start_of_year,
            now,
        ))
        .get_result::<Decimal>(conn)?;
        let fund_profit_year = select(user_fund_profits(
            cognito_user_id.to_string(),
            start_of_year,
            now,
        ))
        .get_result::<Decimal>(conn)?;
        let yearly_return = current_portfolio_value - portfolio_value_at_start_of_year
            + exchange_profit_year
            + fund_profit_year;

        // monthly calculations
        let portfolio_value_at_start_of_month =
            Self::calculate_portfolio_value(conn, cognito_user_id, start_of_month)?;
        let exchange_profit_month = select(user_exchange_profits(
            cognito_user_id.to_string(),
            start_of_month,
            now,
        ))
        .get_result::<Decimal>(conn)?;
        let fund_profit_month = select(user_fund_profits(
            cognito_user_id.to_string(),
            start_of_month,
            now,
        ))
        .get_result::<Decimal>(conn)?;
        let monthly_return = current_portfolio_value - portfolio_value_at_start_of_month
            + exchange_profit_month
            + fund_profit_month;

        let locked_invested_value = invested_amounts
            .clone()
            .map(|a| a.total_currency_amount_locked)
            .unwrap_or(Decimal::ZERO);
        let invested_value = invested_amounts
            .map(|a| a.total_currency_amount_invested)
            .unwrap_or(Decimal::ZERO);
        let exchange_profits_total = select(user_exchange_profits(
            cognito_user_id.to_string(),
            DateTime::UNIX_EPOCH.naive_utc(),
            now,
        ))
        .get_result::<Decimal>(conn)?;
        let fund_profits_total = select(user_fund_profits(
            cognito_user_id.to_string(),
            DateTime::UNIX_EPOCH.naive_utc(),
            now,
        ))
        .get_result::<Decimal>(conn)?;
        let return_on_investment = if invested_value.is_zero() {
            Decimal::ZERO
        } else {
            ((current_portfolio_value + exchange_profits_total + fund_profits_total
                - invested_value)
                / invested_value)
                * Decimal::from(100)
        };

        let carbon_tons_offset = Token::total_burned(
            conn,
            &account.to_string(),
            carbon_credit_contract_index,
            carbon_credit_token_id,
            now,
        )?;

        Ok(InvestmentPortfolioUserAggregate {
            locked_mint_fund_euro_e_amount: locked_invested_value,
            current_portfolio_value,
            yearly_return,
            monthly_return,
            invested_value,
            return_on_investment,
            carbon_tons_offset,
        })
    }

    fn calculate_portfolio_value(
        conn: &mut DbConn,
        cognito_user_id: &str,
        now: NaiveDateTime,
    ) -> QueryResult<Decimal> {
        select(user_currency_value_for_forest_project_owned_tokens_at(
            cognito_user_id.to_string(),
            now,
        ))
        .get_result::<Decimal>(conn)
    }
}

#[derive(serde::Serialize, Object)]
pub struct PortfolioValue {
    pub portfolio_value: Decimal,
    pub at:              NaiveDateTime,
}
