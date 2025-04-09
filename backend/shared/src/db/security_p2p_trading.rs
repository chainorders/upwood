use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    security_p2p_exchange_records, security_p2p_trading_contracts, security_p2p_trading_markets,
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
    Deserialize,
    Clone,
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
    Deserialize,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = security_p2p_trading_markets)]
#[diesel(primary_key(contract_address, token_id, token_contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Market {
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub liquidity_provider: String,
    pub buy_rate_numerator: Decimal,
    pub buy_rate_denominator: Decimal,
    pub sell_rate_numerator: Decimal,
    pub sell_rate_denominator: Decimal,
    pub total_sell_token_amount: Decimal,
    pub total_sell_currency_amount: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl Market {
    #[instrument(skip_all)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_contract_addresses: Option<&[Decimal]>,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let query = security_p2p_trading_markets::table
            .filter(security_p2p_trading_markets::contract_address.eq(contract_address))
            .into_boxed();
        let count_query = security_p2p_trading_markets::table
            .filter(security_p2p_trading_markets::contract_address.eq(contract_address))
            .into_boxed();
        let (query, count_query) = match token_contract_addresses {
            Some(token_contract_addresses) => {
                let query = query.filter(
                    security_p2p_trading_markets::token_contract_address
                        .eq_any(token_contract_addresses),
                );
                let count_query = count_query.filter(
                    security_p2p_trading_markets::token_contract_address
                        .eq_any(token_contract_addresses),
                );
                (query, count_query)
            }
            None => (query, count_query),
        };

        let markets = query
            .limit(page_size)
            .offset(page * page_size)
            .load::<Market>(conn)?;
        let total_count: i64 = count_query.count().get_result(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((markets, page_count))
    }

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
        token_contract_address: Decimal,
        token_id: Decimal,
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
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    Debug,
    PartialEq,
    Object,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(table_name = crate::schema::security_p2p_trading_traders)]
#[diesel(primary_key(contract_address, token_id, token_contract_address, trader))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trader {
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub trader: String,
    pub token_in_amount: Decimal,
    pub token_out_amount: Decimal,
    pub currency_in_amount: Decimal,
    pub currency_out_amount: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl Trader {
    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(crate::schema::security_p2p_trading_traders::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_id: Decimal,
        token_contract_address: Decimal,
        trader: String,
    ) -> DbResult<Option<Self>> {
        let trader = crate::schema::security_p2p_trading_traders::table
            .filter(
                crate::schema::security_p2p_trading_traders::contract_address.eq(contract_address),
            )
            .filter(crate::schema::security_p2p_trading_traders::token_id.eq(token_id))
            .filter(
                crate::schema::security_p2p_trading_traders::token_contract_address
                    .eq(token_contract_address),
            )
            .filter(crate::schema::security_p2p_trading_traders::trader.eq(trader))
            .first(conn)
            .optional()?;
        Ok(trader)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::update(crate::schema::security_p2p_trading_traders::table)
            .filter(
                crate::schema::security_p2p_trading_traders::contract_address
                    .eq(self.contract_address),
            )
            .filter(crate::schema::security_p2p_trading_traders::token_id.eq(self.token_id))
            .filter(
                crate::schema::security_p2p_trading_traders::token_contract_address
                    .eq(self.token_contract_address),
            )
            .filter(crate::schema::security_p2p_trading_traders::trader.eq(&self.trader))
            .set(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let trader = diesel::insert_into(crate::schema::security_p2p_trading_traders::table)
            .values(self)
            .on_conflict((
                crate::schema::security_p2p_trading_traders::contract_address,
                crate::schema::security_p2p_trading_traders::token_id,
                crate::schema::security_p2p_trading_traders::token_contract_address,
                crate::schema::security_p2p_trading_traders::trader,
            ))
            .do_update()
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(trader)
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
#[diesel(table_name = security_p2p_exchange_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExchangeRecord {
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub seller: String,
    pub buyer: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub rate: Decimal,
    pub create_time: NaiveDateTime,
}

impl ExchangeRecord {
    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_p2p_exchange_records::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let records = security_p2p_exchange_records::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(records)
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(skip_all)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_contract_address: Option<Decimal>,
        token_id: Option<Decimal>,
        buyer: Option<&str>,
        seller: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<ExchangeRecord>, i64)> {
        let mut query = security_p2p_exchange_records::table
            .filter(security_p2p_exchange_records::contract_address.eq(contract_address))
            .into_boxed();
        let mut count_query = security_p2p_exchange_records::table
            .filter(security_p2p_exchange_records::contract_address.eq(contract_address))
            .into_boxed();
        if let Some(token_contract_address) = token_contract_address {
            query = query.filter(
                security_p2p_exchange_records::token_contract_address.eq(token_contract_address),
            );
            count_query = count_query.filter(
                security_p2p_exchange_records::token_contract_address.eq(token_contract_address),
            );
        }
        if let Some(token_id) = token_id {
            query = query.filter(security_p2p_exchange_records::token_id.eq(token_id));
            count_query = count_query.filter(security_p2p_exchange_records::token_id.eq(token_id));
        }
        if let Some(buyer) = buyer {
            query = query.filter(security_p2p_exchange_records::buyer.eq(buyer));
            count_query = count_query.filter(security_p2p_exchange_records::buyer.eq(buyer));
        }
        if let Some(seller) = seller {
            query = query.filter(security_p2p_exchange_records::seller.eq(seller));
            count_query = count_query.filter(security_p2p_exchange_records::seller.eq(seller));
        }
        query = query.order_by(security_p2p_exchange_records::create_time.desc());
        count_query = count_query.order_by(security_p2p_exchange_records::create_time.desc());

        let records = query
            .limit(page_size)
            .offset(page * page_size)
            .load::<ExchangeRecord>(conn)?;
        let total_count: i64 = count_query.count().get_result(conn)?;
        let page_count = std::cmp::max((total_count as f64 / page_size as f64).ceil() as i64, 1);

        Ok((records, page_count))
    }
}
