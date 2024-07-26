use std::ops::{Add, Sub};

use crate::{
    schema::token_market::{
        self, market_contract_index, market_contract_sub_index, token_contract_index,
        token_contract_sub_index, token_listed_amount, token_unlisted_amount,
    },
    shared::db::{self, address_to_sql_string, token_amount_to_sql, DbResult},
};
use bigdecimal::BigDecimal;
use concordium_rust_sdk::{
    cis2,
    id::types::AccountAddress,
    types::{Address, ContractAddress},
};
use diesel::prelude::*;
use log::debug;
use num_traits::Zero;

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug)]
#[diesel(table_name = token_market)]
#[diesel(primary_key(
    market_contract_index,
    market_contract_sub_index,
    token_contract_index,
    token_contract_sub_index,
    token_id,
    token_owner
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MarketToken {
    pub market_contract_index:     BigDecimal,
    pub market_contract_sub_index: BigDecimal,
    pub token_contract_index:      BigDecimal,
    pub token_contract_sub_index:  BigDecimal,
    pub token_id:                  BigDecimal,
    pub token_owner:               String,
    pub token_listed_amount:       BigDecimal,
    pub token_unlisted_amount:     BigDecimal,
}

impl MarketToken {
    pub fn new(
        market_contract: ContractAddress,
        token_contract: ContractAddress,
        token_id: cis2::TokenId,
        token_owner: AccountAddress,
        listed_amount: cis2::TokenAmount,
        unlisted_amount: cis2::TokenAmount,
    ) -> Self {
        Self {
            market_contract_index:     market_contract.index.into(),
            market_contract_sub_index: market_contract.subindex.into(),
            token_contract_index:      token_contract.index.into(),
            token_contract_sub_index:  token_contract.subindex.into(),
            token_id:                  db::token_id_to_sql(&token_id),
            token_owner:               db::address_to_sql_string(&Address::Account(token_owner)),
            token_listed_amount:       db::token_amount_to_sql(&listed_amount),
            token_unlisted_amount:     db::token_amount_to_sql(&unlisted_amount),
        }
    }
}

pub fn insert_or_inc_unlisted_supply(conn: &mut db::DbConn, token: &MarketToken) -> DbResult<()> {
    let updated_count = diesel::insert_into(token_market::table)
        .values(token)
        .on_conflict((
            market_contract_index,
            market_contract_sub_index,
            token_contract_index,
            token_contract_sub_index,
            token_market::token_id,
            token_market::token_owner,
        ))
        .do_update()
        .set(
            token_unlisted_amount
                .eq(token_unlisted_amount.add(token.token_unlisted_amount.clone())),
        )
        .execute(conn)?;
    assert_eq!(updated_count, 1, "insert_or_inc_unlisted_supply: Updated row count should be 1");

    Ok(())
}

pub fn update_dec_unlisted_supply(
    conn: &mut db::DbConn,
    market_contract: &ContractAddress,
    token_contract: &ContractAddress,
    token_id: &cis2::TokenId,
    token_owner: AccountAddress,
    amount_delta: cis2::TokenAmount,
) -> DbResult<()> {
    let update_filter = token_market::market_contract_index
        .eq::<BigDecimal>(market_contract.index.into())
        .and(market_contract_sub_index.eq::<BigDecimal>(market_contract.subindex.into()))
        .and(token_contract_index.eq::<BigDecimal>(token_contract.index.into()))
        .and(token_contract_sub_index.eq::<BigDecimal>(token_contract.subindex.into()))
        .and(token_market::token_id.eq::<BigDecimal>(db::token_id_to_sql(token_id)))
        .and(
            token_market::token_owner.eq(db::address_to_sql_string(&Address::Account(token_owner))),
        );
    let delete_filter = update_filter.clone().and(
        token_market::token_listed_amount
            .add(token_market::token_unlisted_amount)
            .eq(BigDecimal::zero()),
    );
    let update_query =
        token_unlisted_amount.eq(token_unlisted_amount.sub(token_amount_to_sql(&amount_delta)));

    conn.transaction(|conn| {
        let updated_count = diesel::update(token_market::table)
            .filter(update_filter)
            .set(update_query)
            .execute(conn)?;
        let deleted_rows_count =
            diesel::delete(token_market::table).filter(delete_filter).execute(conn)?;

        assert_eq!(updated_count, 1, "update_dec_unlisted_supply: Updated row count should be 1");
        debug!("update_dec_listed_supply deleted {} row(s)", deleted_rows_count);
        Ok(())
    })
}

pub fn update_dec_listed_supply(
    conn: &mut db::DbConn,
    market_contract: &ContractAddress,
    token_contract: &ContractAddress,
    token_id: &cis2::TokenId,
    token_owner: AccountAddress,
    amount_delta: cis2::TokenAmount,
) -> DbResult<()> {
    let update_filter = token_market::market_contract_index
        .eq::<BigDecimal>(market_contract.index.into())
        .and(market_contract_sub_index.eq::<BigDecimal>(market_contract.subindex.into()))
        .and(token_contract_index.eq::<BigDecimal>(token_contract.index.into()))
        .and(token_contract_sub_index.eq::<BigDecimal>(token_contract.subindex.into()))
        .and(token_market::token_id.eq::<BigDecimal>(db::token_id_to_sql(token_id)))
        .and(
            token_market::token_owner.eq(db::address_to_sql_string(&Address::Account(token_owner))),
        );
    let delete_filter = update_filter.clone().and(
        token_market::token_listed_amount
            .add(token_market::token_unlisted_amount)
            .eq(BigDecimal::zero()),
    );
    let update_query =
        token_listed_amount.eq(token_listed_amount.sub(token_amount_to_sql(&amount_delta)));

    conn.transaction(|conn| {
        let updated_count = diesel::update(token_market::table)
            .filter(update_filter.clone())
            .set(update_query)
            .execute(conn)?;
        let deleted_rows_count =
            diesel::delete(token_market::table).filter(delete_filter).execute(conn)?;

        assert_eq!(updated_count, 1, "update_dec_listed_supply: Updated row count should be 1");
        debug!("update_dec_listed_supply deleted {} row(s)", deleted_rows_count);
        Ok(())
    })
}

pub fn update_unlisted_to_listed_supply(
    conn: &mut db::DbConn,
    market_contract: &ContractAddress,
    token_contract: &ContractAddress,
    token_id: &cis2::TokenId,
    token_owner: AccountAddress,
    amount_delta: cis2::TokenAmount,
) -> DbResult<()> {
    let update_filter = token_market::market_contract_index
        .eq::<BigDecimal>(market_contract.index.into())
        .and(market_contract_sub_index.eq::<BigDecimal>(market_contract.subindex.into()))
        .and(token_contract_index.eq::<BigDecimal>(token_contract.index.into()))
        .and(token_contract_sub_index.eq::<BigDecimal>(token_contract.subindex.into()))
        .and(token_market::token_id.eq::<BigDecimal>(db::token_id_to_sql(token_id)))
        .and(
            token_market::token_owner.eq(db::address_to_sql_string(&Address::Account(token_owner))),
        );
    let update_query = (
        token_unlisted_amount.eq(token_unlisted_amount.sub(token_amount_to_sql(&amount_delta))),
        token_listed_amount.eq(token_listed_amount.add(token_amount_to_sql(&amount_delta))),
    );

    let updated_count = diesel::update(token_market::table)
        .filter(update_filter)
        .set(update_query)
        .execute(conn)?;
    assert_eq!(updated_count, 1, "update_unlisted_to_listed_supply: Updated row count should be 1");

    Ok(())
}

pub fn update_listed_all_to_unlisted_supply(
    conn: &mut db::DbConn,
    market_contract: &ContractAddress,
    token_contract: &ContractAddress,
    token_id: &cis2::TokenId,
    token_owner: AccountAddress,
) -> DbResult<()> {
    let update_filter = token_market::market_contract_index
        .eq::<BigDecimal>(market_contract.index.into())
        .and(market_contract_sub_index.eq::<BigDecimal>(market_contract.subindex.into()))
        .and(token_contract_index.eq::<BigDecimal>(token_contract.index.into()))
        .and(token_contract_sub_index.eq::<BigDecimal>(token_contract.subindex.into()))
        .and(token_market::token_id.eq::<BigDecimal>(db::token_id_to_sql(token_id)))
        .and(
            token_market::token_owner.eq(db::address_to_sql_string(&Address::Account(token_owner))),
        );
    let update_query = (
        token_unlisted_amount.eq(token_unlisted_amount.add(token_listed_amount)),
        token_listed_amount.eq(BigDecimal::zero()),
    );
    let updated_count = diesel::update(token_market::table)
        .filter(update_filter)
        .set(update_query)
        .execute(conn)?;
    assert_eq!(
        updated_count, 1,
        "update_listed_all_to_unlisted_supply: Updated row count should be 1"
    );

    Ok(())
}

pub fn list_tokens(
    conn: &mut db::DbConn,
    market_contract: &ContractAddress,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<MarketToken>, i64)> {
    let filter = token_market::market_contract_index
        .eq::<BigDecimal>(market_contract.index.into())
        .and(market_contract_sub_index.eq::<BigDecimal>(market_contract.subindex.into()));
    let tokens: Vec<MarketToken> = token_market::table
        .filter(filter.clone())
        .order((token_market::token_contract_index, token_market::token_id))
        .offset(page * page_size)
        .limit(page_size)
        .select(MarketToken::as_select())
        .get_results(conn)?;
    let count_total: i64 = token_market::table.filter(filter).count().get_result(conn)?;
    let page_count = (count_total + page_size - 1) / page_size;

    Ok((tokens, page_count))
}

pub fn list_tokens_by_owner(
    conn: &mut db::DbConn,
    market_contract: &ContractAddress,
    owner: AccountAddress,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<MarketToken>, i64)> {
    let filter = token_market::market_contract_index
        .eq::<BigDecimal>(market_contract.index.into())
        .and(market_contract_sub_index.eq::<BigDecimal>(market_contract.subindex.into()))
        .and(token_market::token_owner.eq(address_to_sql_string(&Address::Account(owner))));
    let tokens: Vec<MarketToken> = token_market::table
        .filter(filter.clone())
        .order((token_market::token_contract_index, token_market::token_id))
        .offset(page * page_size)
        .limit(page_size)
        .select(MarketToken::as_select())
        .get_results(conn)?;
    let count_total: i64 = token_market::table.filter(filter).count().get_result(conn)?;
    let page_count = (count_total + page_size - 1) / page_size;

    Ok((tokens, page_count))
}
