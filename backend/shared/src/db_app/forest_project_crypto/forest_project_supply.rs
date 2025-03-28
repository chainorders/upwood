use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_app::forest_project::ForestProjectState;
use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_supply)]
#[diesel(primary_key(forest_project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectSupply {
    pub forest_project_id:    Uuid,
    pub forest_project_state: ForestProjectState,
    pub supply:               Option<Decimal>,
    pub symbol:               String,
    pub decimals:             i32,
}

impl ForestProjectSupply {
    pub fn find_by_forest_project_id(
        conn: &mut DbConn,
        project_id: Uuid,
    ) -> QueryResult<Option<Self>> {
        use crate::schema_manual::forest_project_supply::dsl::*;
        forest_project_supply
            .filter(forest_project_id.eq(project_id))
            .first(conn)
            .optional()
    }

    pub fn list_by_forest_project_state(
        conn: &mut DbConn,
        state: ForestProjectState,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_supply::dsl::*;

        let total_count = forest_project_supply
            .filter(forest_project_state.eq(state))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_supply
            .filter(forest_project_state.eq(state))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((records, page_count))
    }

    pub fn list_by_forest_project_ids(
        conn: &mut DbConn,
        project_ids: &[Uuid],
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_supply::dsl::*;

        let records = forest_project_supply
            .filter(forest_project_id.eq_any(project_ids))
            .load::<Self>(conn)?;

        Ok(records)
    }

    pub fn list_by_forest_project_id(
        conn: &mut DbConn,
        project_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_supply::dsl::*;

        let total_count = forest_project_supply
            .filter(forest_project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_supply
            .filter(forest_project_id.eq(project_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((records, page_count))
    }
}
