use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_shared::{DbConn, DbResult};
use crate::schema;

pub const TRACKED_TOKEN_ID: Decimal = Decimal::ZERO;

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    AsChangeset,
    Deserialize,
    Clone,
)]
#[diesel(table_name = crate::schema::forest_projects)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProject {
    pub id:                    uuid::Uuid,
    pub name:                  String,
    pub label:                 String,
    pub desc_short:            String,
    pub desc_long:             String,
    pub area:                  String,
    pub carbon_credits:        i32,
    pub roi_percent:           f32,
    pub state:                 ForestProjectState,
    pub image_large_url:       String,
    pub image_small_url:       String,
    pub geo_spatial_url:       Option<String>,
    pub shares_available:      i32,
    pub offering_doc_link:     Option<String>,
    pub property_media_header: String,
    pub property_media_footer: String,
    pub latest_price:          Decimal,
    pub created_at:            NaiveDateTime,
    pub updated_at:            NaiveDateTime,
}

impl ForestProject {
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProject> {
        let project = diesel::insert_into(schema::forest_projects::table)
            .values(self)
            .returning(ForestProject::as_returning())
            .get_result(conn)?;
        Ok(project)
    }

    pub fn update(&self, conn: &mut DbConn) -> DbResult<ForestProject> {
        let project = diesel::update(schema::forest_projects::table)
            .filter(schema::forest_projects::id.eq(self.id))
            .set(self)
            .returning(ForestProject::as_returning())
            .get_result(conn)?;
        Ok(project)
    }

    pub fn find(conn: &mut DbConn, project_id: uuid::Uuid) -> DbResult<Option<Self>> {
        schema::forest_projects::table
            .filter(schema::forest_projects::id.eq(project_id))
            .select(ForestProject::as_select())
            .first::<Self>(conn)
            .optional()
    }

    pub fn list_by_state_optional(
        conn: &mut DbConn,
        state: Option<ForestProjectState>,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let query = schema::forest_projects::table.into_boxed();
        let count_query = schema::forest_projects::table.into_boxed();
        let (query, count_query) = match state {
            Some(state) => (
                query.filter(schema::forest_projects::state.eq(state)),
                count_query.filter(schema::forest_projects::state.eq(state)),
            ),
            None => (query, count_query),
        };

        let projects = query
            .select(ForestProject::as_select())
            .order(schema::forest_projects::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
    }

    pub fn list_by_state(
        conn: &mut DbConn,
        forest_project_state: ForestProjectState,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let projects = schema::forest_projects::table
            .filter(schema::forest_projects::state.eq(forest_project_state))
            .select(ForestProject::as_select())
            .order(schema::forest_projects::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = schema::forest_projects::table
            .filter(schema::forest_projects::state.eq(forest_project_state))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
    }

    pub fn list_by_ids(conn: &mut DbConn, project_ids: &[Uuid]) -> DbResult<Vec<Self>> {
        let projects = schema::forest_projects::table
            .filter(schema::forest_projects::id.eq_any(project_ids))
            .select(ForestProject::as_select())
            .order(schema::forest_projects::created_at.desc())
            .get_results(conn)?;

        Ok(projects)
    }

    pub fn user_notification_exists(
        &self,
        conn: &mut DbConn,
        user_cognito_id: &str,
    ) -> DbResult<bool> {
        Notification::exists(conn, self.id, user_cognito_id)
    }

    pub fn user_notification(
        &self,
        conn: &mut DbConn,
        user_cognito_id: &str,
    ) -> DbResult<Option<Notification>> {
        Notification::find(conn, self.id, user_cognito_id)
    }

    pub fn legal_contract(&self, conn: &mut DbConn) -> DbResult<Option<LegalContract>> {
        LegalContract::find(conn, self.id)
    }

    pub fn user_legal_contract(
        &self,
        conn: &mut DbConn,
        user_cognito_id: &str,
    ) -> DbResult<Option<LegalContractUserSignature>> {
        LegalContractUserSignature::find(conn, self.id, user_cognito_id)
    }

    pub fn media(
        &self,
        conn: &mut DbConn,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<ForestProjectMedia>, i64)> {
        ForestProjectMedia::list(conn, self.id, page, page_size)
    }
}

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
)]
#[diesel(table_name = crate::schema::forest_project_property_media)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectMedia {
    #[serde(skip_serializing)]
    pub id:         uuid::Uuid,
    pub image_url:  String,
    #[serde(skip_serializing)]
    pub project_id: uuid::Uuid,
}

impl ForestProjectMedia {
    pub fn list(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let media = schema::forest_project_property_media::table
            .filter(schema::forest_project_property_media::project_id.eq(project_id))
            .select(ForestProjectMedia::as_select())
            .order(schema::forest_project_property_media::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;
        let total_count = schema::forest_project_property_media::table
            .filter(schema::forest_project_property_media::project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((media, page_count))
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProjectMedia> {
        let media = diesel::insert_into(schema::forest_project_property_media::table)
            .values(self)
            .returning(ForestProjectMedia::as_returning())
            .get_result(conn)?;
        Ok(media)
    }

    pub fn delete(conn: &mut DbConn, id: uuid::Uuid) -> DbResult<ForestProjectMedia> {
        diesel::delete(schema::forest_project_property_media::table.find(id))
            .returning(ForestProjectMedia::as_returning())
            .get_result(conn)
    }

    pub fn delete_self(&self, conn: &mut DbConn) -> DbResult<ForestProjectMedia> {
        ForestProjectMedia::delete(conn, self.id)
    }

    pub fn find(conn: &mut DbConn, id: uuid::Uuid) -> DbResult<Option<Self>> {
        schema::forest_project_property_media::table
            .filter(schema::forest_project_property_media::id.eq(id))
            .select(ForestProjectMedia::as_select())
            .first::<Self>(conn)
            .optional()
    }
}

#[derive(
    diesel_derive_enum::DbEnum,
    Debug,
    PartialEq,
    Enum,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
)]
#[ExistingTypePath = "crate::schema::sql_types::ForestProjectState"]
#[DbValueStyle = "snake_case"]
pub enum ForestProjectState {
    Draft,
    Active,
    Funded,
    Archived,
}

impl std::fmt::Display for ForestProjectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForestProjectState::Draft => write!(f, "Draft"),
            ForestProjectState::Active => write!(f, "Active"),
            ForestProjectState::Funded => write!(f, "Funded"),
            ForestProjectState::Archived => write!(f, "Archived"),
        }
    }
}

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
)]
#[diesel(table_name = crate::schema::forest_project_prices)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id, price_at))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectPrice {
    pub project_id: uuid::Uuid,
    pub price:      Decimal,
    pub price_at:   NaiveDateTime,
}

impl ForestProjectPrice {
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProjectPrice> {
        let price = diesel::insert_into(schema::forest_project_prices::table)
            .values(self)
            .returning(ForestProjectPrice::as_returning())
            .get_result(conn)?;
        Ok(price)
    }

    pub fn latest(conn: &mut DbConn, project_id: uuid::Uuid) -> DbResult<Option<Self>> {
        schema::forest_project_prices::table
            .filter(schema::forest_project_prices::project_id.eq(project_id))
            .select(ForestProjectPrice::as_select())
            .order(schema::forest_project_prices::price_at.desc())
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        price_at: NaiveDateTime,
    ) -> DbResult<Option<Self>> {
        schema::forest_project_prices::table
            .filter(
                schema::forest_project_prices::project_id
                    .eq(project_id)
                    .and(schema::forest_project_prices::price_at.eq(price_at)),
            )
            .select(ForestProjectPrice::as_select())
            .first::<Self>(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let prices = schema::forest_project_prices::table
            .filter(schema::forest_project_prices::project_id.eq(project_id))
            .select(ForestProjectPrice::as_select())
            .order(schema::forest_project_prices::price_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = schema::forest_project_prices::table
            .filter(schema::forest_project_prices::project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((prices, page_count))
    }

    pub fn delete(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        price_at: NaiveDateTime,
    ) -> DbResult<usize> {
        diesel::delete(
            schema::forest_project_prices::table.filter(
                schema::forest_project_prices::project_id
                    .eq(project_id)
                    .and(schema::forest_project_prices::price_at.eq(price_at)),
            ),
        )
        .execute(conn)
    }
}

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
)]
#[diesel(table_name = crate::schema::forest_project_legal_contract_user_signatures)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LegalContractUserSignature {
    project_id:      uuid::Uuid,
    cognito_user_id: String,
    user_account:    String,
    user_signature:  String,
    created_at:      NaiveDateTime,
    updated_at:      NaiveDateTime,
}

impl LegalContractUserSignature {
    pub fn find(conn: &mut DbConn, id: Uuid, user_id: &str) -> DbResult<Option<Self>> {
        use crate::schema::forest_project_legal_contract_user_signatures::dsl::*;
        forest_project_legal_contract_user_signatures
            .filter(project_id.eq(id).and(cognito_user_id.eq(user_id)))
            .first::<Self>(conn)
            .optional()
    }
}

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
)]
#[diesel(table_name = crate::schema::forest_project_legal_contracts)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LegalContract {
    project_id: uuid::Uuid,
    text_url:   String,
    edoc_url:   String,
    pdf_url:    String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl LegalContract {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<Self>> {
        use crate::schema::forest_project_legal_contracts::dsl::*;
        let contract = forest_project_legal_contracts
            .filter(project_id.eq(id))
            .first::<Self>(conn)?;
        Ok(Some(contract))
    }
}

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
)]
#[diesel(table_name = crate::schema::forest_project_notifications)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification {
    pub id:              uuid::Uuid,
    pub project_id:      uuid::Uuid,
    pub cognito_user_id: String,
    pub created_at:      NaiveDateTime,
    pub updated_at:      NaiveDateTime,
}

impl Notification {
    pub fn exists(conn: &mut DbConn, project_id: Uuid, cognito_user_id: &str) -> DbResult<bool> {
        diesel::select(diesel::dsl::exists(
            schema::forest_project_notifications::table.filter(
                schema::forest_project_notifications::project_id
                    .eq(project_id)
                    .and(schema::forest_project_notifications::cognito_user_id.eq(cognito_user_id)),
            ),
        ))
        .get_result(conn)
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: Uuid,
        cognito_user_id: &str,
    ) -> DbResult<Option<Self>> {
        let res = schema::forest_project_notifications::table
            .filter(
                schema::forest_project_notifications::project_id
                    .eq(project_id)
                    .and(schema::forest_project_notifications::cognito_user_id.eq(cognito_user_id)),
            )
            .first(conn)
            .optional()?;

        Ok(res)
    }
}
