use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    security_mint_fund_contracts, security_mint_fund_investment_records,
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
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_mint_fund_contracts::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> DbResult<Option<Self>> {
        let contract = security_mint_fund_contracts::table
            .filter(security_mint_fund_contracts::contract_address.eq(contract_address))
            .first(conn)
            .optional()?;
        Ok(contract)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::update(security_mint_fund_contracts::table)
            .filter(security_mint_fund_contracts::contract_address.eq(self.contract_address))
            .set(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn delete(conn: &mut DbConn, contract_address: Decimal) -> DbResult<()> {
        diesel::delete(security_mint_fund_contracts::table)
            .filter(security_mint_fund_contracts::contract_address.eq(contract_address))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let contracts = security_mint_fund_contracts::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(contracts)
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
#[diesel(table_name = security_mint_funds)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityMintFund {
    pub id: Decimal,
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub receiver_address: Option<String>,
    pub rate: Decimal,
    pub fund_state: SecurityMintFundState,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl SecurityMintFund {
    #[instrument(skip_all)]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let fund = diesel::insert_into(security_mint_funds::table)
            .values(self)
            .on_conflict(security_mint_funds::id)
            .do_update()
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(fund)
    }

    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let fund = diesel::insert_into(security_mint_funds::table)
            .values(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(fund)
    }

    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, id: Decimal) -> DbResult<Option<Self>> {
        let fund = security_mint_funds::table
            .filter(security_mint_funds::id.eq(id))
            .first(conn)
            .optional()?;
        Ok(fund)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let updated_fund = diesel::update(security_mint_funds::table)
            .filter(security_mint_funds::id.eq(self.id))
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(updated_fund)
    }

    #[instrument(skip_all)]
    pub fn delete(conn: &mut DbConn, id: Decimal) -> DbResult<()> {
        diesel::delete(security_mint_funds::table)
            .filter(security_mint_funds::id.eq(id))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let funds = security_mint_funds::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(funds)
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
#[diesel(table_name = security_mint_fund_investors)]
#[diesel(primary_key(contract_address, fund_id, investor))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Investor {
    pub contract_address: Decimal,
    pub fund_id:          Decimal,
    pub investor:         String,
    pub currency_amount:  Decimal,
    pub token_amount:     Decimal,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl Investor {
    #[instrument(skip_all)]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let investor = diesel::insert_into(security_mint_fund_investors::table)
            .values(self)
            .on_conflict((
                security_mint_fund_investors::contract_address,
                security_mint_fund_investors::fund_id,
                security_mint_fund_investors::investor,
            ))
            .do_update()
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(investor)
    }

    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_mint_fund_investors::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        fund_id: Decimal,
        investor: &str,
    ) -> DbResult<Option<Self>> {
        let investor_record = security_mint_fund_investors::table
            .filter(security_mint_fund_investors::contract_address.eq(contract_address))
            .filter(security_mint_fund_investors::fund_id.eq(fund_id))
            .filter(security_mint_fund_investors::investor.eq(investor))
            .first(conn)
            .optional()?;
        Ok(investor_record)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let investor = diesel::update(security_mint_fund_investors::table)
            .filter(security_mint_fund_investors::contract_address.eq(self.contract_address))
            .filter(security_mint_fund_investors::fund_id.eq(self.fund_id))
            .filter(security_mint_fund_investors::investor.eq(&self.investor))
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(investor)
    }

    #[instrument(skip_all)]
    pub fn delete(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::delete(security_mint_fund_investors::table)
            .filter(security_mint_fund_investors::contract_address.eq(self.contract_address))
            .filter(security_mint_fund_investors::fund_id.eq(self.fund_id))
            .filter(security_mint_fund_investors::investor.eq(&self.investor))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let investors = security_mint_fund_investors::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(investors)
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
    pub fund_id: Decimal,
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
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_mint_fund_investment_records::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    pub fn last_before(
        conn: &mut DbConn,
        contract: Decimal,
        investor: &str,
        create_time: NaiveDateTime,
    ) -> DbResult<Option<Self>> {
        let record = security_mint_fund_investment_records::table
            .filter(security_mint_fund_investment_records::contract_address.eq(contract))
            .filter(security_mint_fund_investment_records::investor.eq(investor))
            .filter(security_mint_fund_investment_records::create_time.lt(create_time))
            .order_by(security_mint_fund_investment_records::create_time.desc())
            .first(conn)
            .optional()?;
        Ok(record)
    }
}

#[derive(
    diesel_derive_enum::DbEnum, Debug, PartialEq, Enum, serde::Serialize, serde::Deserialize,
)]
#[ExistingTypePath = "crate::schema::sql_types::SecurityMintFundInvestmentRecordType"]
pub enum InvestmentRecordType {
    Invested,
    Cancelled,
    Claimed,
}
