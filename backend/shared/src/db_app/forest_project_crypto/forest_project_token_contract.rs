use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::security_token_contract_type::SecurityTokenContractType;
use crate::db_shared::DbConn;

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = crate::schema::forest_project_token_contracts)]
#[diesel(belongs_to(crate::db_app::forest_project::ForestProject, foreign_key = forest_project_id))]
#[diesel(primary_key(forest_project_id, contract_type))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
pub struct ForestProjectTokenContract {
    pub contract_address:  Decimal,
    pub forest_project_id: Uuid,
    pub contract_type:     SecurityTokenContractType,
    pub fund_token_id:     Option<Decimal>,
    pub market_token_id:   Option<Decimal>,
    pub symbol:            String,
    pub decimals:          i32,
    pub metadata_url:      String,
    pub metadata_hash:     Option<String>,
    pub created_at:        chrono::NaiveDateTime,
    pub updated_at:        chrono::NaiveDateTime,
}

impl ForestProjectTokenContract {
    pub fn update(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        diesel::update(
            forest_project_token_contracts.filter(contract_address.eq(self.contract_address)),
        )
        .set(self)
        .get_result(conn)
    }

    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        diesel::insert_into(forest_project_token_contracts)
            .values(self)
            .get_result(conn)
    }

    pub fn delete(
        conn: &mut DbConn,
        project_id: Uuid,
        r#type: SecurityTokenContractType,
    ) -> QueryResult<usize> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        diesel::delete(
            forest_project_token_contracts
                .filter(forest_project_id.eq(project_id))
                .filter(contract_type.eq(r#type)),
        )
        .execute(conn)
    }

    pub fn find_by_type(
        conn: &mut DbConn,
        project_id: Uuid,
        token_contract_type: SecurityTokenContractType,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        forest_project_token_contracts
            .filter(forest_project_id.eq(project_id))
            .filter(contract_type.eq(token_contract_type))
            .first(conn)
            .optional()
    }

    pub fn find(conn: &mut DbConn, contract_addr: Decimal) -> QueryResult<Option<Self>> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        forest_project_token_contracts
            .filter(contract_address.eq(contract_addr))
            .first(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        project_ids: Option<&[Uuid]>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::forest_project_token_contracts::dsl::*;

        let mut query = forest_project_token_contracts.into_boxed();
        let mut total_count_query = forest_project_token_contracts.into_boxed();

        if let Some(project_ids) = project_ids {
            query = query.filter(forest_project_id.eq_any(project_ids));
            total_count_query = total_count_query.filter(forest_project_id.eq_any(project_ids));
        }

        let total_count = total_count_query.count().get_result::<i64>(conn)?;
        let records = query
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;
        let pages_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((records, pages_count))
    }
}
