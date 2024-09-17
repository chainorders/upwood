use std::ops::{Add, Sub};

use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_cis2::TokenAmountU64;
use concordium_protocols::rate::Rate;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::{Address, ContractAddress};
use concordium_rwa_backend_shared::db::{token_amount_u64_to_sql, DbConn, DbResult};
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use diesel::sql_types::Integer;
use security_mint_fund::{AnyTokenUId, FundState};
use tracing::instrument;

use crate::schema::{
    security_mint_fund_contracts, security_mint_fund_investment_records,
    security_mint_fund_investors,
};

#[repr(i32)]
#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy, PartialEq)]
#[diesel(sql_type = Integer)]
pub enum SecurityMintFundState {
    Open    = 0,
    Success = 1,
    Fail    = 2,
}

impl From<FundState> for SecurityMintFundState {
    fn from(value: FundState) -> Self {
        match value {
            FundState::Open => SecurityMintFundState::Open,
            FundState::Success(_) => SecurityMintFundState::Success,
            FundState::Fail => SecurityMintFundState::Fail,
        }
    }
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

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = security_mint_fund_contracts)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityMintFundContract {
    pub contract_address: String,
    pub token_contract_address: String,
    pub token_id: String,
    pub investment_token_contract_address: String,
    pub investment_token_id: String,
    pub currency_token_contract_address: String,
    pub currency_token_id: String,
    pub rate_numerator: i64,
    pub rate_denominator: i64,
    pub fund_state: SecurityMintFundState,
    pub currency_amount: BigDecimal,
    pub token_amount: BigDecimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl SecurityMintFundContract {
    pub fn new(
        contract: &ContractAddress,
        token: AnyTokenUId,
        investment_token: AnyTokenUId,
        currency_token: AnyTokenUId,
        rate: Rate,
        fund_state: FundState,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address: contract.to_string(),
            token_contract_address: token.contract.to_string(),
            token_id: token.id.to_string(),
            investment_token_contract_address: investment_token.contract.to_string(),
            investment_token_id: investment_token.id.to_string(),
            currency_token_contract_address: currency_token.contract.to_string(),
            currency_token_id: currency_token.id.to_string(),
            rate_numerator: rate.numerator as i64,
            rate_denominator: rate.denominator as i64,
            fund_state: fund_state.into(),
            currency_amount: BigDecimal::from(0),
            token_amount: BigDecimal::from(0),
            create_time: now.naive_utc(),
            update_time: now.naive_utc(),
        }
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = security_mint_fund_investors)]
#[diesel(primary_key(contract_address, investor))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityMintFundInvestor {
    pub contract_address: String,
    pub investor:         String,
    pub currency_amount:  BigDecimal,
    pub token_amount:     BigDecimal,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl SecurityMintFundInvestor {
    pub fn new(
        contract: &ContractAddress,
        investor: &AccountAddress,
        currency_amount: TokenAmountU64,
        token_amount: TokenAmountU64,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address: contract.to_string(),
            investor:         investor.to_string(),
            currency_amount:  token_amount_u64_to_sql(&currency_amount),
            token_amount:     token_amount_u64_to_sql(&token_amount),
            create_time:      now.naive_utc(),
            update_time:      now.naive_utc(),
        }
    }
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = security_mint_fund_investment_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityMintFundInvestmentRecord {
    pub id:                     i64,
    pub contract_address:       String,
    pub investor:               String,
    pub currency_amount:        Option<BigDecimal>,
    pub token_amount:           Option<BigDecimal>,
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
pub struct SecurityMintFundInvestmentRecordInsert {
    pub contract_address:       String,
    pub investor:               String,
    pub currency_amount:        Option<BigDecimal>,
    pub token_amount:           Option<BigDecimal>,
    pub investment_record_type: InvestmentRecordType,
    pub create_time:            NaiveDateTime,
}
impl SecurityMintFundInvestmentRecordInsert {
    pub fn new(
        contract: &ContractAddress,
        investor: &Address,
        currency_amount: Option<&TokenAmountU64>,
        security_amount: Option<&TokenAmountU64>,
        record_type: InvestmentRecordType,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address:       contract.to_string(),
            investor:               investor.to_string(),
            currency_amount:        currency_amount.map(token_amount_u64_to_sql),
            token_amount:           security_amount.map(token_amount_u64_to_sql),
            investment_record_type: record_type,
            create_time:            now.naive_utc(),
        }
    }
}

#[instrument(skip_all)]
pub fn insert_fund(conn: &mut DbConn, fund: SecurityMintFundContract) -> DbResult<()> {
    diesel::insert_into(security_mint_fund_contracts::table)
        .values(fund)
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all)]
pub fn update_fund_add_investment_amount(
    conn: &mut DbConn,
    contract: &ContractAddress,
    currency_amount: &TokenAmountU64,
    security_amount: &TokenAmountU64,
    now: DateTime<Utc>,
) -> DbResult<()> {
    let currency_amount = token_amount_u64_to_sql(currency_amount);
    let security_amount = token_amount_u64_to_sql(security_amount);
    diesel::update(security_mint_fund_contracts::table)
        .filter(security_mint_fund_contracts::contract_address.eq(contract.to_string()))
        .set((
            security_mint_fund_contracts::currency_amount.eq(currency_amount),
            security_mint_fund_contracts::token_amount.eq(security_amount),
            security_mint_fund_contracts::update_time.eq(now.naive_utc()),
        ))
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all)]
pub fn update_fund_sub_investment_amount(
    conn: &mut DbConn,
    contract: &ContractAddress,
    currency_amount: &TokenAmountU64,
    security_amount: &TokenAmountU64,
    now: DateTime<Utc>,
) -> DbResult<()> {
    let currency_amount = token_amount_u64_to_sql(currency_amount);
    let security_amount = token_amount_u64_to_sql(security_amount);
    diesel::update(security_mint_fund_contracts::table)
        .filter(security_mint_fund_contracts::contract_address.eq(contract.to_string()))
        .set((
            security_mint_fund_contracts::currency_amount
                .eq(security_mint_fund_contracts::currency_amount.sub(currency_amount)),
            security_mint_fund_contracts::token_amount
                .eq(security_mint_fund_contracts::token_amount.sub(security_amount)),
            security_mint_fund_contracts::update_time.eq(now.naive_utc()),
        ))
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all)]
pub fn update_fund_sub_token_amount(
    conn: &mut DbConn,
    contract: &ContractAddress,
    security_amount: &TokenAmountU64,
    now: DateTime<Utc>,
) -> DbResult<()> {
    let security_amount = token_amount_u64_to_sql(security_amount);
    diesel::update(security_mint_fund_contracts::table)
        .filter(security_mint_fund_contracts::contract_address.eq(contract.to_string()))
        .set((
            security_mint_fund_contracts::token_amount
                .eq(security_mint_fund_contracts::token_amount.sub(security_amount)),
            security_mint_fund_contracts::update_time.eq(now.naive_utc()),
        ))
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all)]
pub fn update_fund_sub_currency_amount(
    conn: &mut DbConn,
    contract: &ContractAddress,
    currency_amount: &TokenAmountU64,
    now: DateTime<Utc>,
) -> DbResult<()> {
    let currency_amount = token_amount_u64_to_sql(currency_amount);
    diesel::update(security_mint_fund_contracts::table)
        .filter(security_mint_fund_contracts::contract_address.eq(contract.to_string()))
        .set((
            security_mint_fund_contracts::currency_amount
                .eq(security_mint_fund_contracts::currency_amount.sub(currency_amount)),
            security_mint_fund_contracts::update_time.eq(now.naive_utc()),
        ))
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all)]
pub fn update_fund_state(
    conn: &mut DbConn,
    contract: &ContractAddress,
    fund_state: FundState,
    now: DateTime<Utc>,
) -> DbResult<()> {
    diesel::update(security_mint_fund_contracts::table)
        .filter(security_mint_fund_contracts::contract_address.eq(contract.to_string()))
        .set((
            security_mint_fund_contracts::fund_state.eq::<SecurityMintFundState>(fund_state.into()),
            security_mint_fund_contracts::update_time.eq(now.naive_utc()),
        ))
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all, fields(investor = %record.investor))]
pub fn insert_investment_record(
    conn: &mut DbConn,
    record: SecurityMintFundInvestmentRecordInsert,
) -> DbResult<()> {
    diesel::insert_into(security_mint_fund_investment_records::table)
        .values(record)
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all, fields(investor = %investor.investor))]
pub fn insert_investor_or_update_add_investment(
    conn: &mut DbConn,
    investor: SecurityMintFundInvestor,
) -> DbResult<()> {
    diesel::insert_into(security_mint_fund_investors::table)
        .values(investor.clone())
        .on_conflict((
            security_mint_fund_investors::contract_address,
            security_mint_fund_investors::investor,
        ))
        .do_update()
        .set((
            security_mint_fund_investors::currency_amount
                .eq(security_mint_fund_investors::currency_amount.add(investor.currency_amount)),
            security_mint_fund_investors::token_amount
                .eq(security_mint_fund_investors::token_amount.add(investor.token_amount)),
            security_mint_fund_investors::update_time.eq(investor.update_time),
        ))
        .execute(conn)?;
    Ok(())
}

#[instrument(skip_all, fields(investor = %investor))]
pub fn update_investor_sub_investment(
    conn: &mut DbConn,
    contract: &ContractAddress,
    investor: &AccountAddress,
    currency_amount: &TokenAmountU64,
    security_amount: &TokenAmountU64,
    now: DateTime<Utc>,
) -> DbResult<()> {
    let currency_amount = token_amount_u64_to_sql(currency_amount);
    let security_amount = token_amount_u64_to_sql(security_amount);
    diesel::update(security_mint_fund_investors::table)
        .filter(security_mint_fund_investors::contract_address.eq(contract.to_string()))
        .filter(security_mint_fund_investors::investor.eq(investor.to_string()))
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
pub fn update_investor_sub_investment_token_amount(
    conn: &mut DbConn,
    contract: &ContractAddress,
    investor: &AccountAddress,
    security_amount: &TokenAmountU64,
    now: DateTime<Utc>,
) -> DbResult<()> {
    let security_amount = token_amount_u64_to_sql(security_amount);
    diesel::update(security_mint_fund_investors::table)
        .filter(security_mint_fund_investors::contract_address.eq(contract.to_string()))
        .filter(security_mint_fund_investors::investor.eq(investor.to_string()))
        .set((
            security_mint_fund_investors::token_amount
                .eq(security_mint_fund_investors::token_amount.sub(security_amount)),
            security_mint_fund_investors::update_time.eq(now.naive_utc()),
        ))
        .execute(conn)?;
    Ok(())
}
