use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
// use security_mint_fund::AnyTokenUId;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    security_p2p_trading_contracts, security_p2p_trading_markets, security_p2p_trading_sell_records,
};

/// Represents a contract in the security P2P trading system.
#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    AsChangeset,
    Serialize,
)]
#[diesel(table_name = security_p2p_trading_contracts)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct P2PTradeContract {
    pub contract_address:                Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_token_id:               Decimal,
    pub total_sell_currency_amount:      Decimal,
    pub create_time:                     NaiveDateTime,
}

impl P2PTradeContract {
    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_p2p_trading_contracts::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> DbResult<Option<Self>> {
        let contract = security_p2p_trading_contracts::table
            .filter(security_p2p_trading_contracts::contract_address.eq(contract_address))
            .first(conn)
            .optional()?;
        Ok(contract)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::update(security_p2p_trading_contracts::table)
            .filter(security_p2p_trading_contracts::contract_address.eq(self.contract_address))
            .set(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn delete(conn: &mut DbConn, contract_address: Decimal) -> DbResult<()> {
        diesel::delete(security_p2p_trading_contracts::table)
            .filter(security_p2p_trading_contracts::contract_address.eq(contract_address))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let contracts = security_p2p_trading_contracts::table
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
#[diesel(table_name = security_p2p_trading_markets)]
#[diesel(primary_key(contract_address, token_id, token_contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Market {
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub buyer: String,
    pub rate: Decimal,
    pub total_sell_token_amount: Decimal,
    pub total_sell_currency_amount: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl Market {
    #[instrument(skip_all)]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let market = diesel::insert_into(security_p2p_trading_markets::table)
            .values(self)
            .on_conflict((
                security_p2p_trading_markets::contract_address,
                security_p2p_trading_markets::token_id,
                security_p2p_trading_markets::token_contract_address,
            ))
            .do_update()
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(market)
    }

    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let market = diesel::insert_into(security_p2p_trading_markets::table)
            .values(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(market)
    }

    #[instrument(skip_all)]
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_id: Decimal,
        token_contract_address: Decimal,
    ) -> DbResult<Option<Self>> {
        let market = security_p2p_trading_markets::table
            .filter(security_p2p_trading_markets::contract_address.eq(contract_address))
            .filter(security_p2p_trading_markets::token_id.eq(token_id))
            .filter(security_p2p_trading_markets::token_contract_address.eq(token_contract_address))
            .first(conn)
            .optional()?;
        Ok(market)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let updated_market = diesel::update(security_p2p_trading_markets::table)
            .filter(security_p2p_trading_markets::contract_address.eq(self.contract_address))
            .filter(security_p2p_trading_markets::token_id.eq(self.token_id))
            .filter(
                security_p2p_trading_markets::token_contract_address
                    .eq(self.token_contract_address),
            )
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(updated_market)
    }

    #[instrument(skip_all)]
    pub fn delete(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_id: Decimal,
        token_contract_address: Decimal,
    ) -> DbResult<()> {
        diesel::delete(security_p2p_trading_markets::table)
            .filter(security_p2p_trading_markets::contract_address.eq(contract_address))
            .filter(security_p2p_trading_markets::token_id.eq(token_id))
            .filter(security_p2p_trading_markets::token_contract_address.eq(token_contract_address))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let markets = security_p2p_trading_markets::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(markets)
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
#[diesel(table_name = security_p2p_trading_sell_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SellRecord {
    pub id:                     Uuid,
    pub block_height:           Decimal,
    pub txn_index:              Decimal,
    pub contract_address:       Decimal,
    pub token_id:               Decimal,
    pub token_contract_address: Decimal,
    pub seller:                 String,
    pub currency_amount:        Decimal,
    pub token_amount:           Decimal,
    pub rate:                   Decimal,
    pub create_time:            NaiveDateTime,
}

impl SellRecord {
    #[instrument(skip_all, fields(seller = %self.seller))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_p2p_trading_sell_records::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<Self>> {
        let record = security_p2p_trading_sell_records::table
            .filter(security_p2p_trading_sell_records::id.eq(id))
            .first(conn)
            .optional()?;
        Ok(record)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let updated_record = diesel::update(security_p2p_trading_sell_records::table)
            .filter(security_p2p_trading_sell_records::id.eq(self.id))
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(updated_record)
    }

    #[instrument(skip_all)]
    pub fn delete(conn: &mut DbConn, id: Uuid) -> DbResult<()> {
        diesel::delete(security_p2p_trading_sell_records::table)
            .filter(security_p2p_trading_sell_records::id.eq(id))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let records = security_p2p_trading_sell_records::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(records)
    }
}
