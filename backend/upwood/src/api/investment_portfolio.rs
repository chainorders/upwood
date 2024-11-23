use chrono::{DateTime, Months, NaiveDateTime, Utc};
use concordium_rust_sdk::id::types::AccountAddress;
use diesel::sql_types::{Integer, Nullable, Numeric, Timestamp, VarChar};
use diesel::{sql_query, QueryResult};
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::db::cis2_security::Token;
use shared::db::security_mint_fund::InvestmentRecordType;
use shared::db_app::forest_project::ForestProjectState;
use shared::db_shared::{DbConn, DbPool};
use shared::schema;
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
            &account,
            contracts.euro_e_contract_index,
            contracts.euro_e_token_id,
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
        let account = ensure_account_registered(&claims)?;
        let conn = &mut db_pool.get()?;
        let now = now.unwrap_or(Utc::now().naive_utc());
        let mut ret = Vec::new();
        for i in 0..months {
            let time = now.checked_sub_months(Months::new(i)).unwrap();
            let value =
                InvestmentPortfolioUserAggregate::calculate_portfolio_value(conn, &account, time)?;
            ret.push(PortfolioValue {
                portfolio_value: value.currency.unwrap_or(Decimal::ZERO),
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
    /// Sum Of(Balance of each Forest Project Token * the current price of the token)
    pub current_portfolio_value:        Decimal,
    /// Current portfolio value - Portfolio value at the beginning of the year - Amount invested in the year + Amount withdrawn in the year
    pub yearly_return:                  Decimal,
    /// Current portfolio value - Portfolio value at the beginning of the month - Amount invested in the month + Amount withdrawn in the month
    pub monthly_return:                 Decimal,
    /// Sum of the amount invested in the mint funds and the amount bought in the P2P trading
    pub invested_value:                 Decimal,
    /// (Current portfolio value - Amount withdrawn) / Total amount invested
    pub return_on_investment:           Decimal,
    /// The total amount of carbon credit tokens burned
    pub carbon_tons_offset:             Decimal,
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
        now: NaiveDateTime,
    ) -> QueryResult<Self> {
        let start_of_year = now.checked_sub_months(Months::new(12)).unwrap();
        let start_of_month = now.checked_sub_months(Months::new(1)).unwrap();

        let locked_mint_fund_euro_e_amount = Self::calculate_mint_fund_locked_euro_e_amounts(
            conn,
            account,
            euro_e_contract_index,
            euro_e_token_id,
            now,
        )?
        .currency
        .unwrap_or(Decimal::ZERO);
        let current_portfolio_value = Self::calculate_portfolio_value(conn, account, now)?
            .currency
            .unwrap_or(Decimal::ZERO);
        let portfolio_value_at_start_of_year =
            Self::calculate_portfolio_value(conn, account, start_of_year)?
                .currency
                .unwrap_or(Decimal::ZERO);
        let outgoing_amount_year =
            Self::calculate_outgoing_amount(conn, account, start_of_year, now)?;
        let yearly_return =
            current_portfolio_value - portfolio_value_at_start_of_year + outgoing_amount_year;
        let portfolio_value_at_start_of_month =
            Self::calculate_portfolio_value(conn, account, start_of_month)?
                .currency
                .unwrap_or(Decimal::ZERO);
        let outgoing_amount_month =
            Self::calculate_outgoing_amount(conn, account, start_of_month, now)?;
        let monthly_return =
            current_portfolio_value - portfolio_value_at_start_of_month + outgoing_amount_month;
        let invested_amount_mint_fund = Self::calculate_euro_e_invested_amount_mint_funds(
            conn,
            account,
            euro_e_contract_index,
            euro_e_token_id,
            now,
        )?
        .currency
        .unwrap_or(Decimal::ZERO);
        let p2p_trade_buy_amount = Self::calculate_p2p_trade_buy_amount(
            conn,
            account,
            euro_e_contract_index,
            euro_e_token_id,
            now,
        )?
        .currency
        .unwrap_or(Decimal::ZERO);
        let total_outgoing_amount =
            Self::calculate_outgoing_amount(conn, account, DateTime::UNIX_EPOCH.naive_utc(), now)?;
        let invested_value = invested_amount_mint_fund + p2p_trade_buy_amount;
        let return_on_investment = if invested_value.is_zero() {
            Decimal::ZERO
        } else {
            ((current_portfolio_value + total_outgoing_amount - invested_value) / invested_value)
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
            locked_mint_fund_euro_e_amount,
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
        account: &AccountAddress,
        now: NaiveDateTime,
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
                            fpp.price_at as price_time
                        from
                            forest_projects
                            join cis2_token_holder_balance_updates as bu on forest_projects.contract_address = bu.cis2_address
                            and bu.token_id = 0
                            and bu.holder_address = $1
                            and bu.create_time > $2
                            and bu.create_time <= $3
                            join forest_project_prices as fpp on forest_projects.id = fpp.project_id
                            and fpp.price_at <= $3
                        where
                            forest_projects.state = $4
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
            .bind::<VarChar, _>(account)
            .bind::<Timestamp, _>(chrono::DateTime::UNIX_EPOCH.naive_utc())
            .bind::<Timestamp, _>(now)
            .bind::<schema::sql_types::ForestProjectState, _>(ForestProjectState::Listed)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }

    fn calculate_mint_fund_locked_euro_e_amounts(
        conn: &mut DbConn,
        account: &AccountAddress,
        euro_e_contract_index: Decimal,
        euro_e_token_id: Decimal,
        now: NaiveDateTime,
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
                                forest_projects.state = $1
                                and security_mint_fund_investment_records.investor = $2
                                and security_mint_fund_investment_records.create_time <= $3
                                and security_mint_fund_investment_records.create_time > $4
                                and security_mint_fund_contracts.currency_token_contract_address = $5
                                and security_mint_fund_contracts.currency_token_id = $6
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
            .bind::<VarChar, _>(account)
            .bind::<Timestamp, _>(now)
            .bind::<Timestamp, _>(chrono::DateTime::UNIX_EPOCH.naive_utc())
            .bind::<Numeric, _>(euro_e_contract_index)
            .bind::<Numeric, _>(euro_e_token_id)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }

    fn calculate_euro_e_invested_amount_mint_funds(
        conn: &mut DbConn,
        account: &AccountAddress,
        euro_e_contract_index: Decimal,
        euro_e_token_id: Decimal,
        now: NaiveDateTime,
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
                    first_value(currency_amount) over w as currency
                from
                    (
                        select
                            forest_projects.id as project_id,
                            ir.create_time,
                            ir.currency_amount
                        from
                            forest_projects
                            join security_mint_fund_contracts as funds on forest_projects.mint_fund_contract_address = funds.contract_address
                            and funds.currency_token_contract_address = $1
                            and funds.currency_token_id = $2
                            join security_mint_fund_investment_records as ir on funds.contract_address = ir.contract_address
                            and ir.investor = $3
                            and ir.create_time > $4
                            and ir.create_time <= $5
                            and ir.investment_record_type = $6
                        where
                            forest_projects.state = $7
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
            .bind::<Numeric, _>(euro_e_contract_index)
            .bind::<Numeric, _>(euro_e_token_id)
            .bind::<VarChar, _>(account)
            .bind::<Timestamp, _>(chrono::DateTime::UNIX_EPOCH.naive_utc())
            .bind::<Timestamp, _>(now)
            .bind::<Integer, _>(InvestmentRecordType::Invested)
            .bind::<schema::sql_types::ForestProjectState, _>(ForestProjectState::Listed)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }

    fn calculate_p2p_trade_buy_amount(
        conn: &mut DbConn,
        account: &AccountAddress,
        euro_e_contract_index: Decimal,
        euro_e_token_id: Decimal,
        now: NaiveDateTime,
    ) -> QueryResult<CurrencyAmount> {
        let account = account.to_string();
        #[rustfmt::skip]
        let amounts = sql_query("
        select
            sum(security_p2p_trading_trades.currency_amount) as currency
        from
            forest_projects
            join security_p2p_trading_contracts on forest_projects.p2p_trade_contract_address = security_p2p_trading_contracts.contract_address
            and security_p2p_trading_contracts.currency_token_contract_address = $1
            and security_p2p_trading_contracts.currency_token_id = $2
            join security_p2p_trading_trades on security_p2p_trading_contracts.contract_address = security_p2p_trading_trades.contract_address
            and security_p2p_trading_trades.buyer_address = $3
            and security_p2p_trading_trades.create_time > $4
            and security_p2p_trading_trades.create_time <= $5
        where
            forest_projects.state = $6"
        );
        let amounts = amounts
            .bind::<Numeric, _>(euro_e_contract_index)
            .bind::<Numeric, _>(euro_e_token_id)
            .bind::<VarChar, _>(account)
            .bind::<Timestamp, _>(DateTime::UNIX_EPOCH.naive_utc())
            .bind::<Timestamp, _>(now)
            .bind::<schema::sql_types::ForestProjectState, _>(ForestProjectState::Listed)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts)
    }

    pub fn calculate_outgoing_amount(
        conn: &mut DbConn,
        account: &AccountAddress,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> QueryResult<Decimal> {
        let account = account.to_string();
        #[rustfmt::skip]
        let amounts = sql_query("
        select
            sum(currency) as currency
        from
            (
                select distinct
                on (project_id, update_time) project_id,
                update_time,
                first_value(amount) over w as amount,
                first_value(price) over w as price,
                first_value(price_time) over w as price_time,
                first_value(price) over w * first_value(amount) over w as currency
            from
                (
                    select
                        projects.id as project_id,
                        bu.amount,
                        bu.create_time as update_time,
                        prices.price,
                        prices.price_at as price_time
                    from
                        forest_projects as projects
                        join cis2_token_holder_balance_updates as bu on projects.contract_address = bu.cis2_address
                        and bu.token_id = 0
                        and (
                            bu.update_type = 'transfer_out'
                            or bu.update_type = 'burn'
                        )
                        and bu.holder_address = $1
                        and bu.create_time > $2
                        and bu.create_time <= $3
                        join forest_project_prices as prices on projects.id = prices.project_id
                        and prices.price_at <= bu.create_time
                    where
                        projects.state = 'listed'
                ) as t
            window
                w as (
                    partition by
                        project_id,
                        update_time
                    order by
                        update_time desc,
                        price_time desc
                )
            ) as t2
        "
        );
        let amounts = amounts
            .bind::<VarChar, _>(account)
            .bind::<Timestamp, _>(start)
            .bind::<Timestamp, _>(end)
            .get_result::<CurrencyAmount>(conn)?;
        Ok(amounts.currency.unwrap_or(Decimal::ZERO))
    }
}

#[derive(QueryableByName)]
pub struct CurrencyAmount {
    #[diesel(sql_type = Nullable<Numeric>)]
    currency: Option<Decimal>,
}

#[derive(serde::Serialize, Object)]
pub struct PortfolioValue {
    pub portfolio_value: Decimal,
    pub at:              NaiveDateTime,
}
