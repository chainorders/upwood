use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::db_shared::DbConn;
use crate::schema::{
    self, security_mint_fund_contracts, security_mint_fund_investment_records,
    security_mint_fund_investors, security_mint_funds,
};

#[derive(
    diesel_derive_enum::DbEnum,
    Debug,
    PartialEq,
    Enum,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
)]
#[ExistingTypePath = "crate::schema::sql_types::SecurityMintFundState"]
pub enum SecurityMintFundState {
    Open,
    Success,
    Fail,
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    Serialize,
    Deserialize,
    AsChangeset,
)]
#[diesel(table_name = security_mint_fund_contracts)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityMintFundContract {
    pub contract_address:                Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_token_id:               Decimal,
    pub create_time:                     NaiveDateTime,
}

impl SecurityMintFundContract {
    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<()> {
        diesel::insert_into(security_mint_fund_contracts::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> QueryResult<Option<Self>> {
        let contract = security_mint_fund_contracts::table
            .filter(security_mint_fund_contracts::contract_address.eq(contract_address))
            .first(conn)
            .optional()?;
        Ok(contract)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    Serialize,
    Deserialize,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = security_mint_funds)]
#[diesel(primary_key(
    contract_address,
    investment_token_id,
    investment_token_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
pub struct SecurityMintFund {
    pub contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub receiver_address: Option<String>,
    pub rate_numerator: Decimal,
    pub rate_denominator: Decimal,
    pub fund_state: SecurityMintFundState,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl SecurityMintFund {
    #[instrument(skip_all)]
    pub fn list_by_investment_contracts(
        conn: &mut DbConn,
        contract_address: Decimal,
        investment_contracts: Option<&[Decimal]>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = schema::security_mint_funds::table
            .filter(schema::security_mint_funds::contract_address.eq(contract_address))
            .into_boxed();
        let count_query = schema::security_mint_funds::table
            .filter(schema::security_mint_funds::contract_address.eq(contract_address))
            .into_boxed();
        let (query, count_query) = match investment_contracts {
            Some(investment_contracts) => (
                query.filter(
                    schema::security_mint_funds::investment_token_contract_address
                        .eq_any(investment_contracts),
                ),
                count_query.filter(
                    schema::security_mint_funds::investment_token_contract_address
                        .eq_any(investment_contracts),
                ),
            ),
            None => (query, count_query),
        };

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let funds = query.limit(page_size).offset(page * page_size).load(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((funds, page_count))
    }

    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<Self> {
        let fund = diesel::insert_into(security_mint_funds::table)
            .values(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(fund)
    }

    #[instrument(skip_all)]
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        investment_token_id: Decimal,
        investment_token_contract_address: Decimal,
    ) -> QueryResult<Option<Self>> {
        let fund = security_mint_funds::table
            .filter(security_mint_funds::contract_address.eq(contract_address))
            .filter(security_mint_funds::investment_token_id.eq(investment_token_id))
            .filter(
                security_mint_funds::investment_token_contract_address
                    .eq(investment_token_contract_address),
            )
            .first(conn)
            .optional()?;
        Ok(fund)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> QueryResult<Self> {
        let updated_fund = diesel::update(security_mint_funds::table)
            .filter(security_mint_funds::contract_address.eq(self.contract_address))
            .filter(security_mint_funds::investment_token_id.eq(self.investment_token_id))
            .filter(
                security_mint_funds::investment_token_contract_address
                    .eq(self.investment_token_contract_address),
            )
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(updated_fund)
    }

    #[instrument(skip_all)]
    pub fn delete(
        conn: &mut DbConn,
        contract_address: Decimal,
        investment_token_id: Decimal,
        investment_token_contract_address: Decimal,
    ) -> QueryResult<()> {
        diesel::delete(security_mint_funds::table)
            .filter(security_mint_funds::contract_address.eq(contract_address))
            .filter(security_mint_funds::investment_token_id.eq(investment_token_id))
            .filter(
                security_mint_funds::investment_token_contract_address
                    .eq(investment_token_contract_address),
            )
            .execute(conn)?;
        Ok(())
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    Serialize,
    Deserialize,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = security_mint_fund_investors)]
#[diesel(primary_key(
    contract_address,
    investment_token_id,
    investment_token_contract_address,
    investor
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Investor {
    pub contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub investor: String,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_amount: Decimal,
    pub currency_amount_total: Decimal,
    pub token_amount: Decimal,
    pub token_amount_total: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl Investor {
    #[instrument(skip_all)]
    pub fn upsert(&self, conn: &mut DbConn) -> QueryResult<Self> {
        let investor = diesel::insert_into(security_mint_fund_investors::table)
            .values(self)
            .on_conflict((
                security_mint_fund_investors::contract_address,
                security_mint_fund_investors::investment_token_id,
                security_mint_fund_investors::investment_token_contract_address,
                security_mint_fund_investors::investor,
            ))
            .do_update()
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(investor)
    }

    #[instrument(skip_all)]
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        security_token_id: Decimal,
        security_token_contract_address: Decimal,
        investor: &str,
    ) -> QueryResult<Option<Self>> {
        let investor_record = security_mint_fund_investors::table
            .filter(security_mint_fund_investors::contract_address.eq(contract_address))
            .filter(security_mint_fund_investors::investment_token_id.eq(security_token_id))
            .filter(
                security_mint_fund_investors::investment_token_contract_address
                    .eq(security_token_contract_address),
            )
            .filter(security_mint_fund_investors::investor.eq(investor))
            .first(conn)
            .optional()?;
        Ok(investor_record)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> QueryResult<Self> {
        let investor = diesel::update(security_mint_fund_investors::table)
            .filter(security_mint_fund_investors::contract_address.eq(self.contract_address))
            .filter(security_mint_fund_investors::investment_token_id.eq(self.investment_token_id))
            .filter(
                security_mint_fund_investors::investment_token_contract_address
                    .eq(self.investment_token_contract_address),
            )
            .filter(security_mint_fund_investors::investor.eq(&self.investor))
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(investor)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    Serialize,
    AsChangeset,
)]
#[diesel(table_name = security_mint_fund_investment_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentRecord {
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub investor: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub currency_amount_balance: Decimal,
    pub token_amount_balance: Decimal,
    pub investment_record_type: InvestmentRecordType,
    pub create_time: NaiveDateTime,
}

impl InvestmentRecord {
    #[instrument(skip_all, fields(investor = %self.investor))]
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<()> {
        diesel::insert_into(security_mint_fund_investment_records::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        investment_token_contract: Option<Decimal>,
        investment_token_id: Option<Decimal>,
        investor: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = security_mint_fund_investment_records::table
            .filter(security_mint_fund_investment_records::contract_address.eq(contract_address))
            .into_boxed();
        let count_query = security_mint_fund_investment_records::table
            .filter(security_mint_fund_investment_records::contract_address.eq(contract_address))
            .into_boxed();
        let (query, count_query) = match investment_token_id {
            Some(investment_token_id) => (
                query.filter(
                    security_mint_fund_investment_records::investment_token_id
                        .eq(investment_token_id),
                ),
                count_query.filter(
                    security_mint_fund_investment_records::investment_token_id
                        .eq(investment_token_id),
                ),
            ),
            None => (query, count_query),
        };
        let (query, count_query) = match investment_token_contract {
            Some(investment_token_contract_address) => (
                query.filter(
                    security_mint_fund_investment_records::investment_token_contract_address
                        .eq(investment_token_contract_address),
                ),
                count_query.filter(
                    security_mint_fund_investment_records::investment_token_contract_address
                        .eq(investment_token_contract_address),
                ),
            ),
            None => (query, count_query),
        };
        let (query, count_query) = match investor {
            Some(investor) => (
                query.filter(security_mint_fund_investment_records::investor.eq(investor)),
                count_query.filter(security_mint_fund_investment_records::investor.eq(investor)),
            ),
            None => (query, count_query),
        };

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let investment_records = query
            .order_by(security_mint_fund_investment_records::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((investment_records, page_count))
    }
}

#[derive(
    diesel_derive_enum::DbEnum,
    Debug,
    PartialEq,
    Enum,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Eq,
)]
#[ExistingTypePath = "crate::schema::sql_types::SecurityMintFundInvestmentRecordType"]
pub enum InvestmentRecordType {
    Invested,
    Cancelled,
    Claimed,
}
