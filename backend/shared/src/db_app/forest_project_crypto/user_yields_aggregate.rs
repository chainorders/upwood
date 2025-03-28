use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::db_shared::DbConn;
#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::user_yields_aggregate)]
#[diesel(primary_key(
    cognito_user_id,
    yielder_contract_address,
    yield_token_id,
    yield_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserYieldsAggregate {
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub yield_token_id:           Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_amount:             Decimal,
    // TODO: Remove these fields
    pub yield_token_symbol:       String,
    pub yield_token_decimals:     i32,
}

impl UserYieldsAggregate {
    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        yielder_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::user_yields_aggregate::dsl::*;

        let query = user_yields_aggregate
            .filter(cognito_user_id.eq(user_id))
            .filter(
                yielder_contract_address
                    .is_null()
                    .or(yielder_contract_address.eq(yielder_address)),
            );

        let total_count = user_yields_aggregate
            .filter(cognito_user_id.eq(user_id))
            .filter(
                yielder_contract_address
                    .is_null()
                    .or(yielder_contract_address.eq(yielder_address)),
            )
            .count()
            .get_result::<i64>(conn)?;

        let records = query
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}
