use chrono::{DateTime, Months, Utc};
use concordium_rust_sdk::id::types::AccountAddress;
use diesel::{sql_query, QueryResult};
use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use rust_decimal::Decimal;
use shared::db::cis2_security::Token;
use shared::db_app::forest_project::ForestProjectState;
use shared::db_shared::{DbConn, DbPool};
use shared::schema;

use super::{ensure_account_registered, BearerAuthorization, JsonResult, SystemContractsConfig};
use crate::api::ApiTags;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(
        path = "/portfolio/aggregate",
        method = "get",
        tag = "ApiTags::InvestmentPortfolio"
    )]
    async fn get_aggregate(
        &self,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        BearerAuthorization(claims): BearerAuthorization,
    ) -> JsonResult<InvestmentPortfolioUserAggregate> {
        let account = ensure_account_registered(&claims)?;
        let conn = &mut db_pool.get()?;
        let now = Utc::now();
        let ret = InvestmentPortfolioUserAggregate::generate(
            conn,
            &account,
            contracts.euro_e_contract_index,
            contracts.euro_e_token_id,
            contracts.carbon_credit_contract_index,
            contracts.carbon_credit_token_id,
            now,
        )?;
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
    ) -> JsonResult<Vec<PortfolioValue>> {
        let account = ensure_account_registered(&claims)?;
        let conn = &mut db_pool.get()?;
        let now = Utc::now();
        let mut ret = Vec::new();
        for i in 0..months {
            let time = now.checked_sub_months(Months::new(i)).unwrap();
            let value =
                InvestmentPortfolioUserAggregate::calculate_portfolio_value(conn, &account, time)?;
            ret.push(PortfolioValue {
                portfolio_value: value.currency,
                at:              time,
            });
        }
        Ok(Json(ret))
    }
}

#[derive(serde::Serialize, Object)]
pub struct InvestmentPortfolioUserAggregate {
    /// The amount of locked euros in the all the mint fund contracts
    pub locked_euro_e_amount:    Decimal,
    /// Sum Of(Balance of each Forest Project Token * the current price of the token)
    pub current_portfolio_value: Decimal,
    /// Current portfolio value - Portfolio value at the beginning of the year - Amount invested in the year + Amount withdrawn in the year
    pub yearly_return:           Decimal,
    /// Current portfolio value - Portfolio value at the beginning of the month - Amount invested in the month + Amount withdrawn in the month
    pub monthly_return:          Decimal,
    /// (Current portfolio value - Amount withdrawn) / Total amount invested
    pub return_on_investment:    Decimal,
    /// The total amount of carbon credit tokens burned
    pub carbon_tons_offset:      Decimal,
}

use diesel::prelude::*;
impl InvestmentPortfolioUserAggregate {
    pub fn generate(
        conn: &mut DbConn,
        account: &AccountAddress,
        euro_e_contract_index: Decimal,
        euro_e_token_id: Decimal,
        carbon_credit_contract_index: Decimal,
        carbon_credit_token_id: Decimal,
        now: chrono::DateTime<chrono::Utc>,
    ) -> QueryResult<Self> {
        let locked_euro_e_amount = Self::calculate_locked_euro_e_amounts(
            conn,
            account,
            euro_e_contract_index,
            euro_e_token_id,
            now,
        )?
        .currency;
        let current_portfolio_value = Self::calculate_portfolio_value(conn, account, now)?.currency;
        let portfolio_value_at_start_of_year = Self::calculate_portfolio_value(
            conn,
            account,
            now.checked_sub_months(Months::new(12)).unwrap(),
        )?
        .currency;
        let yearly_return = current_portfolio_value - portfolio_value_at_start_of_year;
        let portfolio_value_at_start_of_month = Self::calculate_portfolio_value(
            conn,
            account,
            now.checked_sub_months(Months::new(1)).unwrap(),
        )?;
        let monthly_return = current_portfolio_value - portfolio_value_at_start_of_month.currency;
        let invested_amount_mint_fund = Self::calculate_euro_e_invested_amount_mint_funds(
            conn,
            account,
            euro_e_contract_index,
            euro_e_token_id,
            now,
        )?;
        let p2p_trade_buy_amount = Self::calculate_p2p_trade_buy_amount(
            conn,
            account,
            euro_e_contract_index,
            euro_e_token_id,
            now,
        )?;
        let invested_value = invested_amount_mint_fund.currency + p2p_trade_buy_amount.currency;
        let return_on_investment = if invested_value.is_zero() {
            Decimal::ZERO
        } else {
            (current_portfolio_value - invested_value) / invested_value
        };
        let carbon_tons_offset = Token::total_burned(
            conn,
            &account.to_string(),
            carbon_credit_contract_index,
            carbon_credit_token_id,
            now.naive_utc(),
        )?;

        Ok(InvestmentPortfolioUserAggregate {
            locked_euro_e_amount,
            current_portfolio_value,
            yearly_return,
            monthly_return,
            return_on_investment,
            carbon_tons_offset,
        })
    }

    fn calculate_portfolio_value(
        conn: &mut DbConn,
        account: &AccountAddress,
        now: chrono::DateTime<chrono::Utc>,
    ) -> QueryResult<CurrencyAmount> {
        let account = account.to_string();
        #[rustfmt::skip]
        let amounts = sql_query("
        select
            sum(currency) as currency
        from
            (
                select distinct
                    on (project_id) project_id,
                    first_value(un_frozen_balance * price) over w as currency
                from
                    (
                        select
                            forest_projects.id as project_id,
                            bu.create_time as update_time,
                            bu.un_frozen_balance,
                            fpp.price,
                            fpp.created_at as price_time
                        from
                            forest_projects
                            join cis2_token_holder_balance_updates as bu on forest_projects.contract_address = bu.cis2_address
                            and bu.token_id = 0
                            and bu.holder_address = ?
                            and bu.create_time > ?
                            and bu.create_time <= ?
                            join forest_project_prices as fpp on forest_projects.id = fpp.project_id
                            and fpp.created_at <= bu.create_time
                        where
                            forest_projects.state = ?
                    ) as t
                window
                    w as (
                        partition by
                            project_id
                        order by
                            update_time desc,
                            price_time desc
                    )
            ) t2
        ");

        let amounts = amounts
            .bind::<diesel::sql_types::VarChar, _>(account)
            .bind::<diesel::sql_types::Timestamp, _>(chrono::DateTime::UNIX_EPOCH.naive_utc())
            .bind::<diesel::sql_types::Timestamp, _>(now.naive_utc())
            .bind::<schema::sql_types::ForestProjectState, _>(ForestProjectState::Listed)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }

    fn calculate_locked_euro_e_amounts(
        conn: &mut DbConn,
        account: &AccountAddress,
        euro_e_contract_index: Decimal,
        euro_e_token_id: Decimal,
        now: chrono::DateTime<chrono::Utc>,
    ) -> QueryResult<CurrencyAmount> {
        let account = account.to_string();
        #[rustfmt::skip]
        let amounts = sql_query(
            "select
                sum(currency_amount_balance) as currency
            from
                (
                    select distinct
                        on (project_id) project_id,
                        first_value(currency_amount_balance) over w as currency_amount_balance
                    from
                        (
                            select
                                forest_projects.id as project_id,
                                security_mint_fund_investment_records.token_amount_balance,
                                security_mint_fund_investment_records.currency_amount_balance,
                                security_mint_fund_investment_records.create_time
                            from
                                forest_projects
                                join security_mint_fund_contracts on forest_projects.mint_fund_contract_address = security_mint_fund_contracts.contract_address
                                join security_mint_fund_investment_records on security_mint_fund_contracts.contract_address = security_mint_fund_investment_records.contract_address
                            where
                                forest_projects.state = ?
                                and security_mint_fund_investment_records.investor = ?
                                and security_mint_fund_investment_records.create_time <= ?
                                and security_mint_fund_investment_records.create_time > ?
                                and security_mint_fund_contracts.currency_token_contract_address = ?
                                and security_mint_fund_contracts.currency_token_id = ?
                        ) as t
                    window
                        w as (
                            partition by
                                project_id
                            order by
                                create_time desc
                        )
                ) as t2"
        );

        let amounts = amounts
            .bind::<schema::sql_types::ForestProjectState, _>(ForestProjectState::Listed)
            .bind::<diesel::sql_types::VarChar, _>(account)
            .bind::<diesel::sql_types::Timestamp, _>(now.naive_utc())
            .bind::<diesel::sql_types::Timestamp, _>(chrono::DateTime::UNIX_EPOCH.naive_utc())
            .bind::<diesel::sql_types::Numeric, _>(euro_e_contract_index)
            .bind::<diesel::sql_types::Numeric, _>(euro_e_token_id)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }

    fn calculate_euro_e_invested_amount_mint_funds(
        conn: &mut DbConn,
        account: &AccountAddress,
        euro_e_contract_index: Decimal,
        euro_e_token_id: Decimal,
        now: chrono::DateTime<chrono::Utc>,
    ) -> QueryResult<CurrencyAmount> {
        let account = account.to_string();
        #[rustfmt::skip]
        let amounts = sql_query("
        select
            sum(currency) as currency
        from
            (
                select distinct
                    on (project_id) project_id,
                    first_value(currency_amount_balance) over w as currency
                from
                    (
                        select
                            forest_projects.id as project_id,
                            ir.create_time,
                            ir.currency_amount_balance
                        from
                            forest_projects
                            join security_mint_fund_contracts as funds on forest_projects.mint_fund_contract_address = funds.contract_address
                            and funds.currency_token_contract_address = ?
                            and funds.currency_token_id = ?
                            join security_mint_fund_investment_records as ir on funds.contract_address = ir.contract_address
                            and ir.investor = ?
                            and ir.create_time > ?
                            and ir.create_time <= ?
                            and ir.investment_record_type = 2 -- claimed records
                        where
                            forest_projects.state = ?
                    ) as t
                window
                    w as (
                        partition by
                            project_id
                        order by
                            create_time desc
                    )
            ) t2;"
        );
        let amounts = amounts
            .bind::<diesel::sql_types::Numeric, _>(euro_e_contract_index)
            .bind::<diesel::sql_types::Numeric, _>(euro_e_token_id)
            .bind::<diesel::sql_types::VarChar, _>(account)
            .bind::<diesel::sql_types::Timestamp, _>(chrono::DateTime::UNIX_EPOCH.naive_utc())
            .bind::<diesel::sql_types::Timestamp, _>(now.naive_utc())
            .bind::<schema::sql_types::ForestProjectState, _>(ForestProjectState::Listed)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }

    fn calculate_p2p_trade_buy_amount(
        conn: &mut DbConn,
        account: &AccountAddress,
        euro_e_contract_index: Decimal,
        euro_e_token_id: Decimal,
        now: chrono::DateTime<chrono::Utc>,
    ) -> QueryResult<CurrencyAmount> {
        let account = account.to_string();
        #[rustfmt::skip]
        let amounts = sql_query("
        select
            sum(security_p2p_trading_trades.currency_amount) as currency
        from
            forest_projects
            join security_p2p_trading_contracts on forest_projects.p2p_trade_contract_address = security_p2p_trading_contracts.contract_address
            and security_p2p_trading_contracts.currency_token_contract_address = ?
            and security_p2p_trading_contracts.currency_token_id = ?
            join security_p2p_trading_trades on security_p2p_trading_contracts.contract_address = security_p2p_trading_trades.contract_address
            and security_p2p_trading_trades.buyer_address = ?
            and security_p2p_trading_trades.create_time > ?
            and security_p2p_trading_trades.create_time <= ?
        where
            forest_projects.state = ?"
        );
        let amounts = amounts
            .bind::<diesel::sql_types::Numeric, _>(euro_e_contract_index)
            .bind::<diesel::sql_types::Numeric, _>(euro_e_token_id)
            .bind::<diesel::sql_types::VarChar, _>(account)
            .bind::<diesel::sql_types::Timestamp, _>(chrono::DateTime::UNIX_EPOCH.naive_utc())
            .bind::<diesel::sql_types::Timestamp, _>(now.naive_utc())
            .bind::<schema::sql_types::ForestProjectState, _>(ForestProjectState::Listed)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }
}

#[derive(QueryableByName)]
pub struct CurrencyAmount {
    #[diesel(sql_type = diesel::sql_types::Numeric)]
    currency: Decimal,
}

#[derive(serde::Serialize, Object)]
pub struct PortfolioValue {
    pub portfolio_value: Decimal,
    pub at:              DateTime<Utc>,
}
