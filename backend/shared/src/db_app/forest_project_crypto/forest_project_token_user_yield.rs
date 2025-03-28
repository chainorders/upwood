use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_token_user_yields)]
#[diesel(primary_key(
    forest_project_id,
    token_id,
    token_contract_address,
    holder_address,
    yielder_contract_address,
    yield_token_id,
    yield_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectTokenUserYield {
    pub forest_project_id:        Uuid,
    pub token_id:                 Decimal,
    pub token_contract_address:   Decimal,
    pub holder_address:           String,
    pub token_balance:            Decimal,
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub yield_token_id:           Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_amount:             Decimal,
    pub max_token_id:             Decimal,
    pub token_symbol:             String,
    pub token_decimals:           i32,
    pub yield_token_symbol:       String,
    pub yield_token_decimals:     i32,
}

impl ForestProjectTokenUserYield {
    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        yielder_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_token_user_yields::dsl::*;

        let total_count = forest_project_token_user_yields
            .filter(cognito_user_id.eq(user_id))
            .filter(yielder_contract_address.eq(yielder_address))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_token_user_yields
            .filter(cognito_user_id.eq(user_id))
            .filter(yielder_contract_address.eq(yielder_address))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}
