use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::db_shared::DbConn;

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::token_metadatas)]
#[diesel(primary_key(contract_address, token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
pub struct TokenMetadata {
    pub contract_address: Decimal,
    pub token_id:         Decimal,
    pub symbol:           Option<String>,
    pub decimals:         Option<i32>,
}

impl TokenMetadata {
    pub fn update(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::token_metadatas::dsl::*;
        diesel::update(
            token_metadatas
                .filter(contract_address.eq(self.contract_address))
                .filter(token_id.eq(self.token_id)),
        )
        .set(self)
        .get_result(conn)
    }

    pub fn create(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::token_metadatas::dsl::*;
        diesel::insert_into(token_metadatas)
            .values(self)
            .get_result(conn)
    }

    pub fn find(
        conn: &mut DbConn,
        contract_addr: Decimal,
        metadata_token_id: Decimal,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::token_metadatas::dsl::*;
        token_metadatas
            .filter(contract_address.eq(contract_addr))
            .filter(token_id.eq(metadata_token_id))
            .first(conn)
            .optional()
    }

    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::token_metadatas::dsl::*;

        let total_count = token_metadatas.count().get_result::<i64>(conn)?;

        let records = token_metadatas
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }

    pub fn delete(
        conn: &mut DbConn,
        contract_addr: Decimal,
        metadata_token_id: Decimal,
    ) -> QueryResult<usize> {
        use crate::schema::token_metadatas::dsl::*;
        diesel::delete(
            token_metadatas
                .filter(contract_address.eq(contract_addr))
                .filter(token_id.eq(metadata_token_id)),
        )
        .execute(conn)
    }
}
