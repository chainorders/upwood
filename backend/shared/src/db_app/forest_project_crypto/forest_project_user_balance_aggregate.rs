use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_user_balance_agg)]
#[diesel(primary_key(cognito_user_id, forest_project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUserBalanceAgg {
    pub cognito_user_id:   String,
    pub forest_project_id: Uuid,
    pub total_balance:     Decimal,
}

impl ForestProjectUserBalanceAgg {
    pub fn find(conn: &mut DbConn, user_id: &str, project_id: Uuid) -> QueryResult<Option<Self>> {
        use crate::schema_manual::forest_project_user_balance_agg::dsl::*;
        forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(forest_project_id.eq(project_id))
            .first(conn)
            .optional()
    }

    pub fn list_by_user_id_and_forest_project_ids(
        conn: &mut DbConn,
        user_id: &str,
        project_ids: &[Uuid],
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_user_balance_agg::dsl::*;

        forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(forest_project_id.eq_any(project_ids))
            .load::<Self>(conn)
    }

    pub fn list_by_user_id(
        conn: &mut DbConn,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_user_balance_agg::dsl::*;

        let total_count = forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(total_balance.gt(Decimal::ZERO))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(total_balance.gt(Decimal::ZERO))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((records, page_count))
    }
}
