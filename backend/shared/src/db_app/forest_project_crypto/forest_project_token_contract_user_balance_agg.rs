use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::security_token_contract_type::SecurityTokenContractType;
use crate::db_app::forest_project::ForestProjectState;
use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_token_contract_user_balance_agg)]
#[diesel(primary_key(forest_project_id, cognito_user_id, contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectTokenContractUserBalanceAgg {
    pub forest_project_id:    Uuid,
    pub forest_project_state: ForestProjectState,
    pub forest_project_name:  String,
    pub cognito_user_id:      String,
    pub contract_address:     Decimal,
    pub contract_type:        SecurityTokenContractType,
    pub token_symbol:         String,
    pub token_decimals:       i32,
    pub total_balance:        Decimal,
    pub un_frozen_balance:    Decimal,
}

impl ForestProjectTokenContractUserBalanceAgg {
    pub fn list_by_user_id(
        conn: &mut DbConn,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_token_contract_user_balance_agg::dsl::*;

        let total_count = forest_project_token_contract_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(total_balance.gt(Decimal::ZERO))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_token_contract_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(total_balance.gt(Decimal::ZERO))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((records, page_count))
    }
}
