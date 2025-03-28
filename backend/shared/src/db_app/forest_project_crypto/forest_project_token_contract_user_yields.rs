use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_token_contract_user_yields)]
#[diesel(primary_key(
    forest_project_id,
    token_contract_address,
    cognito_user_id,
    yielder_contract_address,
    yield_token_id,
    yield_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectTokenContractUserYields {
    pub forest_project_id:        Uuid,
    pub token_contract_address:   Decimal,
    pub token_symbol:             String,
    pub token_decimals:           i32,
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub yield_token_id:           Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_token_symbol:       String,
    pub yield_token_decimals:     i32,
    pub yield_amount:             Decimal,
}

impl ForestProjectTokenContractUserYields {
    pub fn list_by_forest_project_ids(
        conn: &mut DbConn,
        user_id: &str,
        project_ids: &[Uuid],
        yielder_address: Decimal,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_token_contract_user_yields::dsl::*;

        let records = forest_project_token_contract_user_yields
            .filter(cognito_user_id.eq(user_id))
            .filter(forest_project_id.eq_any(project_ids))
            .filter(yielder_contract_address.eq(yielder_address))
            .load::<Self>(conn)?;

        Ok(records)
    }
}
