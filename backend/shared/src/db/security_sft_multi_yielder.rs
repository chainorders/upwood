use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::cis2_security::Token;
use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    security_sft_multi_yielder_treasuries, security_sft_multi_yielder_yeild_distributions,
    security_sft_multi_yielder_yields,
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
    std::cmp::Eq,
    std::hash::Hash,
)]
#[ExistingTypePath = "crate::schema::sql_types::SecuritySftMultiYielderYieldType"]
pub enum YieldType {
    Quantity,
    #[db_rename = "simple_intrest"]
    SimpleIntrest,
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    AsChangeset,
    Object,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = security_sft_multi_yielder_yields)]
#[diesel(primary_key(
    contract_address,
    token_contract_address,
    token_id,
    yield_contract_address,
    yield_token_id,
    yield_type
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Yield {
    pub contract_address:       Decimal,
    pub token_contract_address: Decimal,
    pub token_id:               Decimal,
    pub yield_contract_address: Decimal,
    pub yield_token_id:         Decimal,
    pub yield_rate_numerator:   Decimal,
    pub yield_rate_denominator: Decimal,
    pub yield_type:             YieldType,
    pub create_time:            NaiveDateTime,
}

impl Yield {
    pub fn list_yielded_tokens(
        conn: &mut DbConn,
        contract: Decimal,
        token_contract_address_: Decimal,
        page: i64,
        page_size: i64,
    ) -> DbResult<(
        Vec<Token>,
        i64, // page count
    )> {
        use crate::schema::security_sft_multi_yielder_yields::dsl::*;

        let query = security_sft_multi_yielder_yields
            .inner_join(
                crate::schema::cis2_tokens::table.on(token_contract_address
                    .eq(crate::schema::cis2_tokens::cis2_address)
                    .and(token_id.eq(crate::schema::cis2_tokens::token_id))),
            )
            .filter(contract_address.eq(contract))
            .filter(token_contract_address.eq(token_contract_address_))
            .into_boxed();
        let count_query = security_sft_multi_yielder_yields
            .inner_join(
                crate::schema::cis2_tokens::table.on(token_contract_address
                    .eq(crate::schema::cis2_tokens::cis2_address)
                    .and(token_id.eq(crate::schema::cis2_tokens::token_id))),
            )
            .filter(contract_address.eq(contract))
            .filter(token_contract_address.eq(token_contract_address_))
            .into_boxed();

        let tokens = query
            .select(crate::schema::cis2_tokens::all_columns)
            .distinct()
            .order_by(token_id.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load::<Token>(conn)?;

        let count = count_query.count().get_result::<i64>(conn)?;
        let page_count = if count == 0 {
            0
        } else {
            (count as f64 / page_size as f64).ceil() as i64
        };
        Ok((tokens, page_count))
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(skip_all)]
    pub fn list(
        conn: &mut DbConn,
        contract: Decimal,
        token_contract_address_: Option<Decimal>,
        token_id_: Option<Decimal>,
        yielded_token_contract_address_: Option<Decimal>,
        yielded_token_id_: Option<Decimal>,
        yield_type_: Option<YieldType>,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64 /* page count */)> {
        use crate::schema::security_sft_multi_yielder_yields::dsl::*;

        let mut query = security_sft_multi_yielder_yields
            .filter(contract_address.eq(contract))
            .into_boxed();
        let mut count_query = security_sft_multi_yielder_yields
            .filter(contract_address.eq(contract))
            .into_boxed();
        if let Some(token_contract_address_) = token_contract_address_ {
            query = query.filter(token_contract_address.eq(token_contract_address_));
            count_query = count_query.filter(token_contract_address.eq(token_contract_address_));
        }
        if let Some(token_id_) = token_id_ {
            query = query.filter(token_id.eq(token_id_));
            count_query = count_query.filter(token_id.eq(token_id_));
        }
        if let Some(yielded_token_contract_address_) = yielded_token_contract_address_ {
            query = query.filter(yield_contract_address.eq(yielded_token_contract_address_));
            count_query =
                count_query.filter(yield_contract_address.eq(yielded_token_contract_address_));
        }
        if let Some(yielded_token_id_) = yielded_token_id_ {
            query = query.filter(yield_token_id.eq(yielded_token_id_));
            count_query = count_query.filter(yield_token_id.eq(yielded_token_id_));
        }
        if let Some(yield_type_) = yield_type_ {
            query = query.filter(yield_type.eq(yield_type_));
            count_query = count_query.filter(yield_type.eq(yield_type_));
        }
        let yields = query
            .order_by(token_contract_address.desc())
            .order_by(token_id.desc())
            .order_by(yield_contract_address.desc())
            .order_by(yield_token_id.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;
        let count = count_query.count().get_result::<i64>(conn)?;
        let page_count = if count == 0 {
            0
        } else {
            (count as f64 / page_size as f64).ceil() as i64
        };
        Ok((yields, page_count))
    }

    #[instrument(skip_all)]
    pub fn find_batch(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_contract_address: Decimal,
        from_token_id: Decimal,
        to_token_id: Decimal,
    ) -> DbResult<Vec<Self>> {
        let yields = security_sft_multi_yielder_yields::table
            .filter(security_sft_multi_yielder_yields::contract_address.eq(contract_address))
            .filter(
                security_sft_multi_yielder_yields::token_contract_address
                    .eq(token_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::token_id.gt(from_token_id))
            .filter(security_sft_multi_yielder_yields::token_id.le(to_token_id))
            .load(conn)?;
        Ok(yields)
    }

    #[instrument(skip_all)]
    pub fn delete_batch(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_contract_address: Decimal,
        token_id: Decimal,
    ) -> DbResult<()> {
        diesel::delete(security_sft_multi_yielder_yields::table)
            .filter(security_sft_multi_yielder_yields::contract_address.eq(contract_address))
            .filter(
                security_sft_multi_yielder_yields::token_contract_address
                    .eq(token_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::token_id.eq(token_id))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn insert_batch(conn: &mut DbConn, yields: &[Self]) -> DbResult<()> {
        diesel::insert_into(security_sft_multi_yielder_yields::table)
            .values(yields)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_sft_multi_yielder_yields::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_contract_address: Decimal,
        token_id: Decimal,
        yield_contract_address: Decimal,
        yield_token_id: Decimal,
    ) -> DbResult<Option<Self>> {
        let yield_record = security_sft_multi_yielder_yields::table
            .filter(security_sft_multi_yielder_yields::contract_address.eq(contract_address))
            .filter(
                security_sft_multi_yielder_yields::token_contract_address
                    .eq(token_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::token_id.eq(token_id))
            .filter(
                security_sft_multi_yielder_yields::yield_contract_address
                    .eq(yield_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::yield_token_id.eq(yield_token_id))
            .first(conn)
            .optional()?;
        Ok(yield_record)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let updated_yield = diesel::update(security_sft_multi_yielder_yields::table)
            .filter(security_sft_multi_yielder_yields::contract_address.eq(self.contract_address))
            .filter(
                security_sft_multi_yielder_yields::token_contract_address
                    .eq(self.token_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::token_id.eq(self.token_id))
            .filter(
                security_sft_multi_yielder_yields::yield_contract_address
                    .eq(self.yield_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::yield_token_id.eq(self.yield_token_id))
            .set(self)
            .returning(Self::as_returning())
            .get_result(conn)?;
        Ok(updated_yield)
    }

    #[instrument(skip_all)]
    pub fn delete(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_contract_address: Decimal,
        token_id: Decimal,
        yield_contract_address: Decimal,
        yield_token_id: Decimal,
    ) -> DbResult<()> {
        diesel::delete(security_sft_multi_yielder_yields::table)
            .filter(security_sft_multi_yielder_yields::contract_address.eq(contract_address))
            .filter(
                security_sft_multi_yielder_yields::token_contract_address
                    .eq(token_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::token_id.eq(token_id))
            .filter(
                security_sft_multi_yielder_yields::yield_contract_address
                    .eq(yield_contract_address),
            )
            .filter(security_sft_multi_yielder_yields::yield_token_id.eq(yield_token_id))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let yields = security_sft_multi_yielder_yields::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(yields)
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
#[diesel(table_name = security_sft_multi_yielder_yeild_distributions)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YieldDistribution {
    pub id:                     uuid::Uuid,
    pub contract_address:       Decimal,
    pub token_contract_address: Decimal,
    pub from_token_version:     Decimal,
    pub to_token_version:       Decimal,
    pub token_amount:           Decimal,
    pub yield_contract_address: Decimal,
    pub yield_token_id:         Decimal,
    pub yield_amount:           Decimal,
    pub to_address:             String,
    pub create_time:            NaiveDateTime,
}

impl YieldDistribution {
    #[instrument(skip_all)]
    pub fn delete_batch(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_contract_address: Decimal,
    ) -> DbResult<()> {
        diesel::delete(security_sft_multi_yielder_yeild_distributions::table)
            .filter(
                security_sft_multi_yielder_yeild_distributions::contract_address
                    .eq(contract_address),
            )
            .filter(
                security_sft_multi_yielder_yeild_distributions::token_contract_address
                    .eq(token_contract_address),
            )
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn insert_batch(conn: &mut DbConn, distributions: &[Self]) -> DbResult<()> {
        diesel::insert_into(security_sft_multi_yielder_yeild_distributions::table)
            .values(distributions)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_sft_multi_yielder_yeild_distributions::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, id: uuid::Uuid) -> DbResult<Option<Self>> {
        let distribution = security_sft_multi_yielder_yeild_distributions::table
            .filter(security_sft_multi_yielder_yeild_distributions::id.eq(id))
            .first(conn)
            .optional()?;
        Ok(distribution)
    }

    #[instrument(skip_all)]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let updated_distribution =
            diesel::update(security_sft_multi_yielder_yeild_distributions::table)
                .filter(security_sft_multi_yielder_yeild_distributions::id.eq(self.id))
                .set(self)
                .returning(Self::as_returning())
                .get_result(conn)?;
        Ok(updated_distribution)
    }

    #[instrument(skip_all)]
    pub fn delete(conn: &mut DbConn, id: uuid::Uuid) -> DbResult<()> {
        diesel::delete(security_sft_multi_yielder_yeild_distributions::table)
            .filter(security_sft_multi_yielder_yeild_distributions::id.eq(id))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn list_all(conn: &mut DbConn, limit: i64, offset: i64) -> DbResult<Vec<Self>> {
        let distributions = security_sft_multi_yielder_yeild_distributions::table
            .limit(limit)
            .offset(offset)
            .load(conn)?;
        Ok(distributions)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    AsChangeset,
    Object,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = security_sft_multi_yielder_treasuries)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Treasury {
    pub contract_address: Decimal,
    pub treasury_address: String,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl Treasury {
    #[instrument(skip_all)]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(security_sft_multi_yielder_treasuries::table)
            .values(self)
            .on_conflict(security_sft_multi_yielder_treasuries::contract_address)
            .do_update()
            .set(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> DbResult<Option<Self>> {
        let treasury = security_sft_multi_yielder_treasuries::table
            .filter(security_sft_multi_yielder_treasuries::contract_address.eq(contract_address))
            .first(conn)
            .optional()?;
        Ok(treasury)
    }
}
