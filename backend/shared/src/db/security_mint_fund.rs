use std::ops::{Add, Sub};

use chrono::{DateTime, NaiveDateTime, Utc};
// use concordium_protocols::rate::Rate;
use concordium_rust_sdk::id::types::AccountAddress;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use diesel::sql_types::Integer;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::Serialize;
use tracing::instrument;

use super::cis2_security::Token;
use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    security_mint_fund_contracts, security_mint_fund_investment_records,
    security_mint_fund_investors,
};

#[repr(i32)]
#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy, PartialEq, Enum, Serialize)]
#[diesel(sql_type = Integer)]
pub enum SecurityMintFundState {
    Open    = 0,
    Success = 1,
    Fail    = 2,
}

impl FromSql<Integer, diesel::pg::Pg> for SecurityMintFundState {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        match value {
            0 => Ok(SecurityMintFundState::Open),
            1 => Ok(SecurityMintFundState::Success),
            2 => Ok(SecurityMintFundState::Fail),
            _ => Err(format!("Unknown call type: {}", value).into()),
        }
    }
}

impl ToSql<Integer, diesel::pg::Pg> for SecurityMintFundState {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let v = *self as i32;
        <i32 as ToSql<Integer, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Object, Serialize)]
#[diesel(table_name = security_mint_fund_contracts)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityMintFundContract {
    pub contract_address: Decimal,
    pub token_contract_address: Decimal,
    pub token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub rate: Decimal,
    pub fund_state: SecurityMintFundState,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
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
    pub fn find(conn: &mut DbConn, contract: Decimal) -> DbResult<Option<Self>> {
        let result = security_mint_fund_contracts::table
            .filter(security_mint_fund_contracts::contract_address.eq(contract))
            .first(conn)
            .optional()?;
        Ok(result)
    }

    #[instrument(skip_all)]
    pub fn update_state(
        conn: &mut DbConn,
        contract: Decimal,
        fund_state: SecurityMintFundState,
        block_slot_time: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_mint_fund_contracts::table)
            .filter(security_mint_fund_contracts::contract_address.eq(contract))
            .set((
                security_mint_fund_contracts::fund_state.eq(fund_state),
                security_mint_fund_contracts::update_time.eq(block_slot_time.naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn token(&self, conn: &mut DbConn) -> DbResult<Option<Token>> {
        let token = Token::find(conn, self.token_contract_address, self.token_id)?;
        Ok(token)
    }

    pub fn rate(conn: &mut DbConn, contract_address: Decimal) -> DbResult<Decimal> {
        let rate = security_mint_fund_contracts::table
            .filter(security_mint_fund_contracts::contract_address.eq(contract_address))
            .select(security_mint_fund_contracts::rate)
            .first(conn)?;
        Ok(rate)
    }

    #[instrument(skip_all)]
    pub fn add_investment_amount(
        conn: &mut DbConn,
        contract: Decimal,
        currency_amount: Decimal,
        security_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        {
            diesel::update(security_mint_fund_contracts::table)
                .filter(security_mint_fund_contracts::contract_address.eq(contract))
                .set((
                    security_mint_fund_contracts::currency_amount.eq(currency_amount),
                    security_mint_fund_contracts::token_amount.eq(security_amount),
                    security_mint_fund_contracts::update_time.eq(now.naive_utc()),
                ))
                .execute(conn)?;
        }
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn sub_investment_amount(
        conn: &mut DbConn,
        contract: Decimal,
        currency_amount: Decimal,
        security_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        {
            diesel::update(security_mint_fund_contracts::table)
                .filter(security_mint_fund_contracts::contract_address.eq(contract))
                .set((
                    security_mint_fund_contracts::currency_amount
                        .eq(security_mint_fund_contracts::currency_amount.sub(currency_amount)),
                    security_mint_fund_contracts::token_amount
                        .eq(security_mint_fund_contracts::token_amount.sub(security_amount)),
                    security_mint_fund_contracts::update_time.eq(now.naive_utc()),
                ))
                .execute(conn)?;
        }
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn sub_currency_amount(
        conn: &mut DbConn,
        contract: Decimal,
        currency_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_mint_fund_contracts::table)
            .filter(security_mint_fund_contracts::contract_address.eq(contract))
            .set((
                security_mint_fund_contracts::currency_amount
                    .eq(security_mint_fund_contracts::currency_amount.sub(currency_amount)),
                security_mint_fund_contracts::update_time.eq(now.naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn sub_token_amount(
        conn: &mut DbConn,
        contract: Decimal,
        security_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_mint_fund_contracts::table)
            .filter(security_mint_fund_contracts::contract_address.eq(contract))
            .set((
                security_mint_fund_contracts::token_amount
                    .eq(security_mint_fund_contracts::token_amount.sub(security_amount)),
                security_mint_fund_contracts::update_time.eq(now.naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = security_mint_fund_investors)]
#[diesel(primary_key(contract_address, investor))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Investor {
    pub contract_address: Decimal,
    pub investor:         String,
    pub currency_amount:  Decimal,
    pub token_amount:     Decimal,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl Investor {
    pub fn new(
        contract: Decimal,
        investor: &AccountAddress,
        currency_amount: Decimal,
        token_amount: Decimal,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address: contract,
            investor: investor.to_string(),
            currency_amount,
            token_amount,
            create_time: now.naive_utc(),
            update_time: now.naive_utc(),
        }
    }

    #[instrument(skip_all, fields(investor = %self.investor))]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_mint_fund_investors::table)
            .values(self)
            .on_conflict((
                security_mint_fund_investors::contract_address,
                security_mint_fund_investors::investor,
            ))
            .do_update()
            .set((
                security_mint_fund_investors::currency_amount
                    .eq(security_mint_fund_investors::currency_amount.add(&self.currency_amount)),
                security_mint_fund_investors::token_amount
                    .eq(security_mint_fund_investors::token_amount.add(&self.token_amount)),
                security_mint_fund_investors::update_time.eq(&self.update_time),
            ))
            .execute(conn)?;

        InvestmentRecordInsert {
            contract_address:       self.contract_address,
            investor:               self.investor.clone(),
            currency_amount:        Some(self.currency_amount),
            token_amount:           Some(self.token_amount),
            investment_record_type: InvestmentRecordType::Invested,
            create_time:            self.create_time,
        }
        .insert(conn)?;
        Ok(())
    }

    fn sub_token_amount(
        conn: &mut DbConn,
        contract: Decimal,
        investor: &str,
        security_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_mint_fund_investors::table)
            .filter(security_mint_fund_investors::contract_address.eq(contract))
            .filter(security_mint_fund_investors::investor.eq(investor))
            .set((
                security_mint_fund_investors::token_amount
                    .eq(security_mint_fund_investors::token_amount.sub(security_amount)),
                security_mint_fund_investors::update_time.eq(now.naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }

    fn sub_investment_amount(
        conn: &mut DbConn,
        contract: Decimal,
        investor: &str,
        currency_amount: Decimal,
        security_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_mint_fund_investors::table)
            .filter(security_mint_fund_investors::contract_address.eq(contract))
            .filter(security_mint_fund_investors::investor.eq(investor))
            .set((
                security_mint_fund_investors::currency_amount
                    .eq(security_mint_fund_investors::currency_amount.sub(currency_amount)),
                security_mint_fund_investors::token_amount
                    .eq(security_mint_fund_investors::token_amount.sub(security_amount)),
                security_mint_fund_investors::update_time.eq(now.naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all, fields(investor = %investor))]
    pub fn cancel_investment(
        conn: &mut DbConn,
        contract: Decimal,
        investor: &AccountAddress,
        currency_amount: Decimal,
        security_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        Self::sub_investment_amount(
            conn,
            contract,
            &investor.to_string(),
            currency_amount,
            security_amount,
            now,
        )?;
        InvestmentRecordInsert {
            contract_address:       contract,
            investor:               investor.to_string(),
            currency_amount:        Some(currency_amount),
            token_amount:           Some(security_amount),
            investment_record_type: InvestmentRecordType::Cancelled,
            create_time:            now.naive_utc(),
        }
        .insert(conn)?;
        Ok(())
    }

    #[instrument(skip_all, fields(investor = %investor))]
    pub fn claim_investment(
        conn: &mut DbConn,
        contract: Decimal,
        investor: &AccountAddress,
        security_amount: Decimal,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        let investor = investor.to_string();

        Self::sub_token_amount(conn, contract, &investor, security_amount, now)?;
        InvestmentRecordInsert {
            contract_address:       contract,
            investor:               investor.to_string(),
            currency_amount:        None,
            token_amount:           Some(security_amount),
            investment_record_type: InvestmentRecordType::Claimed,
            create_time:            now.naive_utc(),
        }
        .insert(conn)?;
        Ok(())
    }
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = security_mint_fund_investment_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentRecord {
    pub id:                     i64,
    pub contract_address:       Decimal,
    pub investor:               String,
    pub currency_amount:        Option<Decimal>,
    pub token_amount:           Option<Decimal>,
    pub investment_record_type: InvestmentRecordType,
    pub create_time:            NaiveDateTime,
}

#[repr(i32)]
#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy, PartialEq)]
#[diesel(sql_type = Integer)]
pub enum InvestmentRecordType {
    Invested  = 0,
    Cancelled = 1,
    Claimed   = 2,
    Disbursed = 3,
}

impl FromSql<Integer, diesel::pg::Pg> for InvestmentRecordType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        match value {
            0 => Ok(InvestmentRecordType::Invested),
            1 => Ok(InvestmentRecordType::Cancelled),
            2 => Ok(InvestmentRecordType::Claimed),
            3 => Ok(InvestmentRecordType::Disbursed),
            _ => Err(format!("Unknown call type: {}", value).into()),
        }
    }
}

impl ToSql<Integer, diesel::pg::Pg> for InvestmentRecordType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let v = *self as i32;
        <i32 as ToSql<Integer, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = security_mint_fund_investment_records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentRecordInsert {
    pub contract_address:       Decimal,
    pub investor:               String,
    pub currency_amount:        Option<Decimal>,
    pub token_amount:           Option<Decimal>,
    pub investment_record_type: InvestmentRecordType,
    pub create_time:            NaiveDateTime,
}
impl InvestmentRecordInsert {
    pub fn new(
        contract: Decimal,
        investor: &AccountAddress,
        currency_amount: Option<Decimal>,
        security_amount: Option<Decimal>,
        record_type: InvestmentRecordType,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address: contract,
            investor: investor.to_string(),
            currency_amount,
            token_amount: security_amount,
            investment_record_type: record_type,
            create_time: now.naive_utc(),
        }
    }

    #[instrument(skip_all, fields(investor = %self.investor))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_mint_fund_investment_records::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}
