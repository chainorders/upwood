use std::io::Write;

use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::dsl::*;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use events_listener::txn_processor::security_mint_fund::db::SecurityMintFundContract;
use events_listener::txn_processor::security_p2p_trading::db::P2PTradeContract;
use events_listener::txn_processor::security_sft_rewards::db::SecuritySftRewardsContract;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::Serialize;
use shared::db::DbConn;
use uuid::Uuid;

use super::DbResult;
use crate::schema;

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
)]
#[diesel(table_name = crate::schema::forest_projects)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProject {
    pub id: uuid::Uuid,
    pub name: String,
    pub label: String,
    pub desc_short: String,
    pub desc_long: String,
    pub area: String,
    pub carbon_credits: i32,
    pub roi_percent: f32,
    pub state: ForestProjectState,
    pub image_large_url: String,
    pub geo_spatial_url: Option<String>,
    pub contract_address: Decimal,
    pub p2p_trade_contract_address: Option<Decimal>,
    pub mint_fund_contract_address: Option<Decimal>,
    pub shares_available: i32,
    pub offering_doc_link: Option<String>,
    pub property_media_header: String,
    pub property_media_footer: String,
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

    pub fn list_by_status(
        conn: &mut DbConn,
        status: ForestProjectState,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let projects = schema::forest_projects::table
            .filter(schema::forest_projects::state.eq(status))
            .select(ForestProject::as_select())
            .order(schema::forest_projects::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = schema::forest_projects::table
            .filter(schema::forest_projects::state.eq(status))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
    }

    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<(Vec<Self>, i64)> {
        let projects = schema::forest_projects::table
            .select(ForestProject::as_select())
            .order(schema::forest_projects::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = schema::forest_projects::table
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
    }

    pub fn tracked_token_id() -> Decimal { SecuritySftRewardsContract::tracked_token_id() }

    pub fn mint_fund(&self, conn: &mut DbConn) -> DbResult<Option<SecurityMintFundContract>> {
        match self.mint_fund_contract_address {
            Some(mint_fund_contract_address) => {
                let contract = SecurityMintFundContract::find(conn, mint_fund_contract_address)?;
                Ok(contract)
            }
            None => Ok(None),
        }
    }

    pub fn p2p_trade(&self, conn: &mut DbConn) -> DbResult<Option<P2PTradeContract>> {
        match self.p2p_trade_contract_address {
            Some(p2p_trade_contract_address) => {
                let contract = P2PTradeContract::find(conn, p2p_trade_contract_address)?;
                Ok(contract)
            }
            None => Ok(None),
        }
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

    pub fn contract(&self, conn: &mut DbConn) -> DbResult<Option<SecuritySftRewardsContract>> {
        SecuritySftRewardsContract::find(conn, self.contract_address)
    }

    pub fn media(
        &self,
        conn: &mut DbConn,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<PropertyMedia>, i64)> {
        PropertyMedia::list(conn, self.id, page, page_size)
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
#[diesel(table_name = crate::schema::forest_project_property_media)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PropertyMedia {
    #[serde(skip_serializing)]
    pub id:         uuid::Uuid,
    pub image_url:  String,
    #[serde(skip_serializing)]
    pub project_id: uuid::Uuid,
}

impl PropertyMedia {
    pub fn list(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let media = schema::forest_project_property_media::table
            .filter(schema::forest_project_property_media::project_id.eq(project_id))
            .select(PropertyMedia::as_select())
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
}

#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy, PartialEq, Serialize)]
#[diesel(sql_type = schema::sql_types::ForestProjectState)]
#[derive(Enum)]
pub enum ForestProjectState {
    Draft,
    Funding,
    Funded,
    Archived,
}

impl FromSql<schema::sql_types::ForestProjectState, diesel::pg::Pg> for ForestProjectState {
    fn from_sql(
        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"draft" => Ok(ForestProjectState::Draft),
            b"funding" => Ok(ForestProjectState::Funding),
            b"funded" => Ok(ForestProjectState::Funded),
            b"archived" => Ok(ForestProjectState::Archived),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<schema::sql_types::ForestProjectState, diesel::pg::Pg> for ForestProjectState {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let s = match self {
            ForestProjectState::Draft => "draft",
            ForestProjectState::Funding => "funding",
            ForestProjectState::Funded => "funded",
            ForestProjectState::Archived => "archived",
        };
        out.write_all(s.as_bytes())?;
        Ok(diesel::serialize::IsNull::No)
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
#[diesel(table_name = crate::schema::forest_project_prices)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id, price_at))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectPrice {
    pub project_id: uuid::Uuid,
    pub price:      Decimal,
    pub price_at:   chrono::NaiveDateTime,
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
        price_at: chrono::NaiveDateTime,
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
        price_at: chrono::NaiveDateTime,
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
    created_at:      chrono::NaiveDateTime,
    updated_at:      chrono::NaiveDateTime,
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
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
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
    pub created_at:      chrono::NaiveDateTime,
    pub updated_at:      chrono::NaiveDateTime,
}

impl Notification {
    pub fn exists(conn: &mut DbConn, project_id: Uuid, cognito_user_id: &str) -> DbResult<bool> {
        select(exists(
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
