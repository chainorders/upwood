use std::ops::{Add, Sub};

use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_protocols::rate::Rate;
use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use num_traits::Zero;
use poem_openapi::Object;
use rust_decimal::Decimal;
use security_mint_fund::AnyTokenUId;
use serde::Serialize;
use serde_json::to_value;
use shared::db::{DbConn, DbResult};
use tracing::instrument;

use crate::schema::{
    security_p2p_trading_contracts, security_p2p_trading_deposits, security_p2p_trading_records,
};
use crate::txn_processor::cis2_utils::{ContractAddressToDecimal, TokenIdToDecimal};

/// Represents a contract in the security P2P trading system.
#[derive(
    Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Object, AsChangeset,
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
    pub token_amount: Decimal,
    pub rate_numerator: i64,
    pub rate_denominator: i64,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl P2PTradeContract {
    pub fn new(
        contract: Decimal,
        token: &AnyTokenUId,
        currency: &AnyTokenUId,
        rate: &Rate,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address: contract,
            token_contract_address: token.contract.to_decimal(),
            token_id: token.id.to_decimal(),
            currency_token_contract_address: currency.contract.to_decimal(),
            currency_token_id: currency.id.to_decimal(),
            token_amount: Decimal::zero(),
            rate_numerator: rate.numerator as i64,
            rate_denominator: rate.denominator as i64,
            create_time: now.naive_utc(),
            update_time: now.naive_utc(),
        }
    }

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

    /// Updates the contract's rate by setting the new rate and update time.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `contract` - The contract address to update.
    /// * `rate` - The new rate to set for the contract.
    /// * `now` - The current timestamp to update the contract's update time.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn update_rate(
        conn: &mut DbConn,
        contract: Decimal,
        rate: &Rate,
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_p2p_trading_contracts::table)
            .filter(security_p2p_trading_contracts::contract_address.eq(contract))
            .set((
                security_p2p_trading_contracts::rate_numerator.eq(rate.numerator as i64),
                security_p2p_trading_contracts::rate_denominator.eq(rate.denominator as i64),
                security_p2p_trading_contracts::update_time.eq(now.naive_utc()),
            ))
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
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_p2p_trading_contracts::table)
            .filter(security_p2p_trading_contracts::contract_address.eq(contract))
            .set((
                security_p2p_trading_contracts::token_amount
                    .eq(security_p2p_trading_contracts::token_amount.add(amount)),
                security_p2p_trading_contracts::update_time.eq(now.naive_utc()),
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
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_p2p_trading_contracts::table)
            .filter(security_p2p_trading_contracts::contract_address.eq(contract))
            .set((
                security_p2p_trading_contracts::token_amount
                    .eq(security_p2p_trading_contracts::token_amount.sub(amount)),
                security_p2p_trading_contracts::update_time.eq(now.naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn rate(conn: &mut DbConn, contract: Decimal) -> DbResult<f32> {
        let (numerator, denominator) = security_p2p_trading_contracts::table
            .filter(security_p2p_trading_contracts::contract_address.eq(contract))
            .select((
                security_p2p_trading_contracts::rate_numerator,
                security_p2p_trading_contracts::rate_denominator,
            ))
            .first::<(i64, i64)>(conn)?;
        let rate = numerator as f32 / denominator as f32;
        Ok(rate)
    }
}

/// Represents a deposit Or alternatively a Sell Position in the security P2P trading system.
#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = security_p2p_trading_deposits)]
#[diesel(primary_key(contract_address, trader_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Deposit {
    pub contract_address: Decimal,
    pub trader_address:   String,
    pub token_amount:     Decimal,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}
impl Deposit {
    pub(crate) fn new(
        contract: Decimal,
        from: &AccountAddress,
        amount: Decimal,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address: contract,
            trader_address:   from.to_string(),
            token_amount:     amount,
            create_time:      now.naive_utc(),
            update_time:      now.naive_utc(),
        }
    }

    /// Inserts a new deposit record or updates an existing one by adding the specified amount.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `deposit` - The deposit record to insert or update.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all, fields(trader_address = %self.trader_address))]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_p2p_trading_deposits::table)
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
            .execute(conn)?;
        Ok(())
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
        now: DateTime<Utc>,
    ) -> DbResult<()> {
        diesel::update(security_p2p_trading_deposits::table)
            .filter(security_p2p_trading_deposits::contract_address.eq(contract))
            .filter(security_p2p_trading_deposits::trader_address.eq(from.to_string()))
            .set((
                security_p2p_trading_deposits::token_amount
                    .eq(security_p2p_trading_deposits::token_amount.sub(amount)),
                security_p2p_trading_deposits::update_time.eq(now.naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::SecurityP2pTradingRecordType"]
pub enum TradingRecordType {
    Sell,
    SellCancel,
    ExchangeSell,
    ExchangeBuy,
}

/// Represents a trading record in the security P2P trading system.
/// This struct contains information about a specific trade, including the contract address,
/// trader address, type of trade (sell, sell cancel, etc.), the amount of tokens traded,
/// and any additional metadata associated with the trade.
#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = security_p2p_trading_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradingRecord {
    pub id:               i64,
    pub contract_address: Decimal,
    pub trader_address:   String,
    pub record_type:      TradingRecordType,
    pub token_amount:     Decimal,
    pub metadata:         serde_json::Value,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = security_p2p_trading_records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradingRecordInsert {
    pub contract_address: Decimal,
    pub trader_address:   String,
    pub record_type:      TradingRecordType,
    pub token_amount:     Decimal,
    pub metadata:         serde_json::Value,
    pub create_time:      NaiveDateTime,
}
impl TradingRecordInsert {
    pub fn new_sell(
        contract: Decimal,
        from: &AccountAddress,
        amount: Decimal,
        now: DateTime<Utc>,
    ) -> Self {
        TradingRecordInsert {
            contract_address: contract,
            trader_address:   from.to_string(),
            record_type:      TradingRecordType::Sell,
            token_amount:     amount,
            metadata:         to_value(SellTradingRecordMetadata)
                .expect("Failed to serialize metadata"),
            create_time:      now.naive_utc(),
        }
    }

    pub fn new_sell_cancelled(
        contract: Decimal,
        from: &AccountAddress,
        amount: Decimal,
        now: DateTime<Utc>,
    ) -> Self {
        TradingRecordInsert {
            contract_address: contract,
            trader_address:   from.to_string(),
            record_type:      TradingRecordType::SellCancel,
            token_amount:     amount,
            metadata:         to_value(SellCancelTradingRecordMetadata)
                .expect("Failed to serialize metadata"),
            create_time:      now.naive_utc(),
        }
    }

    pub fn new_exchange_sell(
        contract: Decimal,
        seller: &AccountAddress,
        sell_amount: Decimal,
        payer: &AccountAddress,
        pay_amount: Decimal,
        now: DateTime<Utc>,
    ) -> Self {
        TradingRecordInsert {
            contract_address: contract,
            trader_address:   seller.to_string(),
            record_type:      TradingRecordType::ExchangeSell,
            token_amount:     sell_amount,
            metadata:         to_value(ExchangeSellTradingRecordMetadata {
                currency_amount: pay_amount,
                payer_address:   payer.to_string(),
            })
            .expect("Failed to serialize metadata"),
            create_time:      now.naive_utc(),
        }
    }

    pub fn new_exchange_buy(
        contract: Decimal,
        seller: &AccountAddress,
        sell_amount: Decimal,
        payer: &AccountAddress,
        pay_amount: Decimal,
        now: DateTime<Utc>,
    ) -> Self {
        TradingRecordInsert {
            contract_address: contract,
            trader_address:   payer.to_string(),
            record_type:      TradingRecordType::ExchangeBuy,
            token_amount:     sell_amount,
            metadata:         to_value(ExchangeBuyTradingRecordMetadata {
                currency_amount: pay_amount,
                sell_address:    seller.to_string(),
            })
            .expect("Failed to serialize metadata"),
            create_time:      now.naive_utc(),
        }
    }

    /// Inserts a single trading record into the database.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `record` - The trading record to insert.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all, fields(trader_address = %self.trader_address))]
    pub fn insert(self, conn: &mut DbConn) -> DbResult<()> {
        return Self::insert_batch(conn, vec![self]);
    }

    /// Inserts multiple trading records into the database.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `records` - A vector of trading records to insert.
    ///
    /// # Returns
    /// A `DbResult<()>` indicating whether the operation was successful.
    #[instrument(skip_all)]
    pub fn insert_batch(conn: &mut DbConn, records: Vec<Self>) -> DbResult<()> {
        diesel::insert_into(security_p2p_trading_records::table)
            .values(&records)
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct SellTradingRecordMetadata;

#[derive(Serialize)]
pub struct SellCancelTradingRecordMetadata;

#[derive(Serialize)]
pub struct ExchangeSellTradingRecordMetadata {
    currency_amount: Decimal,
    payer_address:   String,
}

#[derive(Serialize)]
pub struct ExchangeBuyTradingRecordMetadata {
    currency_amount: Decimal,
    sell_address:    String,
}
