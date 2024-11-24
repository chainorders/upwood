use std::ops::{Add, Sub};

use chrono::NaiveDateTime;
// use concordium_protocols::rate::Rate;
use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
// use security_mint_fund::AnyTokenUId;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    security_p2p_trading_contracts, security_p2p_trading_deposits, security_p2p_trading_records,
    security_p2p_trading_trades,
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
    pub contract_address: Decimal,
    pub token_contract_address: Decimal,
    pub token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    /// The total amount of tokens available for buying in this contract.
    pub token_amount: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl P2PTradeContract {
    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, contract: Decimal) -> DbResult<Option<Self>> {
        let contract = security_p2p_trading_contracts::table
            .filter(security_p2p_trading_contracts::contract_address.eq(contract))
            .first::<Self>(conn)?;
        Ok(Some(contract))
    }

    /// Inserts a new contract into the `security_p2p_trading_contracts` table.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `contract` - The `Contract` to be inserted.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_p2p_trading_contracts::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    /// Updates the contract's token amount by adding the specified amount.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `contract` - The contract address to update.
    /// * `amount` - The amount to add to the contract's token amount.
    /// * `now` - The current timestamp to update the contract's update time.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn add_amount(
        conn: &mut DbConn,
        contract: Decimal,
        amount: Decimal,
        now: NaiveDateTime,
    ) -> DbResult<()> {
        diesel::update(security_p2p_trading_contracts::table)
            .filter(security_p2p_trading_contracts::contract_address.eq(contract))
            .set((
                security_p2p_trading_contracts::token_amount
                    .eq(security_p2p_trading_contracts::token_amount.add(amount)),
                security_p2p_trading_contracts::update_time.eq(now),
            ))
            .execute(conn)?;
        Ok(())
    }

    /// Updates the contract's token amount by subtracting the specified amount.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `contract` - The contract address to update.
    /// * `amount` - The amount to subtract from the contract's token amount.
    /// * `now` - The current timestamp to update the contract's update time.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn sub_amount(
        conn: &mut DbConn,
        contract: Decimal,
        amount: Decimal,
        now: NaiveDateTime,
    ) -> DbResult<()> {
        diesel::update(security_p2p_trading_contracts::table)
            .filter(security_p2p_trading_contracts::contract_address.eq(contract))
            .set((
                security_p2p_trading_contracts::token_amount
                    .eq(security_p2p_trading_contracts::token_amount.sub(amount)),
                security_p2p_trading_contracts::update_time.eq(now),
            ))
            .execute(conn)?;
        Ok(())
    }
}

/// Represents a deposit Or alternatively a Sell Position in the security P2P trading system.
#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Object, Serialize)]
#[diesel(table_name = security_p2p_trading_deposits)]
#[diesel(primary_key(contract_address, trader_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Seller {
    pub contract_address: Decimal,
    pub trader_address:   String,
    pub token_amount:     Decimal,
    pub rate:             Decimal,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl Seller {
    /// Inserts a new deposit record or updates an existing one by adding the specified amount.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `deposit` - The deposit record to insert or update.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all, fields(trader_address = %self.trader_address))]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let trader = diesel::insert_into(security_p2p_trading_deposits::table)
            .values(self)
            .on_conflict((
                security_p2p_trading_deposits::contract_address,
                security_p2p_trading_deposits::trader_address,
            ))
            .do_update()
            .set((
                security_p2p_trading_deposits::token_amount
                    .eq(security_p2p_trading_deposits::token_amount.add(self.token_amount)),
                security_p2p_trading_deposits::update_time.eq(self.update_time),
            ))
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(trader)
    }

    /// Updates a deposit record by subtracting the specified amount.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `contract` - The contract address of the deposit to update.
    /// * `from` - The account address of the trader to update.
    /// * `amount` - The amount to subtract from the deposit.
    /// * `now` - The current timestamp to update the deposit's update time.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn sub_amount(
        conn: &mut DbConn,
        contract: Decimal,
        from: &AccountAddress,
        amount: Decimal,
        now: NaiveDateTime,
    ) -> DbResult<Self> {
        let trader = diesel::update(security_p2p_trading_deposits::table)
            .filter(security_p2p_trading_deposits::contract_address.eq(contract))
            .filter(security_p2p_trading_deposits::trader_address.eq(from.to_string()))
            .set((
                security_p2p_trading_deposits::token_amount
                    .eq(security_p2p_trading_deposits::token_amount.sub(amount)),
                security_p2p_trading_deposits::update_time.eq(now),
            ))
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(trader)
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::SecurityP2pTradingRecordType"]
pub enum TradingRecordType {
    Sell,
    SellCancel,
    Exchange,
}

/// Represents a trading record in the security P2P trading system.
/// This struct contains information about a specific trade, including the contract address,
/// trader address, type of trade (sell, sell cancel, etc.), the amount of tokens traded,
/// and any additional metadata associated with the trade.
#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable)]
#[diesel(table_name = security_p2p_trading_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradingRecord {
    pub id: Uuid,
    pub contract_address: Decimal,
    pub trader_address: String,
    pub record_type: TradingRecordType,
    pub token_amount: Decimal,
    pub currency_amount: Decimal,
    pub token_amount_balance: Decimal,
    pub currency_amount_balance: Decimal,
    pub create_time: NaiveDateTime,
}

impl TradingRecord {
    /// Inserts a new trading record into the `security_p2p_trading_records` table.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `record` - The trading record to insert.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_p2p_trading_records::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    pub fn last_before(
        conn: &mut DbConn,
        contract: Decimal,
        trader: &AccountAddress,
        time: NaiveDateTime,
    ) -> DbResult<Option<Self>> {
        let record = security_p2p_trading_records::table
            .filter(security_p2p_trading_records::contract_address.eq(contract))
            .filter(security_p2p_trading_records::trader_address.eq(trader.to_string()))
            .filter(security_p2p_trading_records::create_time.lt(time))
            .order_by(security_p2p_trading_records::create_time.desc())
            .first::<Self>(conn)
            .optional()?;
        Ok(record)
    }
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable)]
#[diesel(table_name = security_p2p_trading_trades)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trade {
    pub id:               Uuid,
    pub contract_address: Decimal,
    pub seller_address:   String,
    pub buyer_address:    String,
    pub token_amount:     Decimal,
    pub currency_amount:  Decimal,
    pub rate:             Decimal,
    pub create_time:      NaiveDateTime,
}

impl Trade {
    /// Inserts a new trade into the `security_p2p_trading_trades` table.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `trade` - The trade to insert.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_p2p_trading_trades::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}
