use chrono::NaiveDateTime;
use diesel::pg::upsert::excluded;
use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::forest_project_crypto::prelude::ForestProjectUserBalanceAgg;
use crate::db_shared::{DbConn, DbResult};
use crate::schema::{self, forest_projects};

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
#[diesel(treat_none_as_null = true)]
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
    pub image_small_url: String,
    pub shares_available: i32,
    pub property_media_header: String,
    pub property_media_footer: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub offering_doc_title: Option<String>,
    pub offering_doc_header: Option<String>,
    pub offering_doc_img_url: Option<String>,
    pub offering_doc_footer: Option<String>,
    pub financial_projection_title: Option<String>,
    pub financial_projection_header: Option<String>,
    pub financial_projection_img_url: Option<String>,
    pub financial_projection_footer: Option<String>,
    pub geo_title: Option<String>,
    pub geo_header: Option<String>,
    pub geo_img_url: Option<String>,
    pub geo_footer: Option<String>,
}

impl ForestProject {
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProject> {
        diesel::insert_into(schema::forest_projects::table)
            .values(self)
            .returning(ForestProject::as_returning())
            .get_result(conn)
    }

    pub fn update(&self, conn: &mut DbConn) -> DbResult<ForestProject> {
        diesel::update(schema::forest_projects::table)
            .filter(schema::forest_projects::id.eq(self.id))
            .set(self)
            .returning(ForestProject::as_returning())
            .get_result(conn)
    }

    pub fn find(conn: &mut DbConn, project_id: uuid::Uuid) -> DbResult<Option<Self>> {
        schema::forest_projects::table
            .filter(schema::forest_projects::id.eq(project_id))
            .select(ForestProject::as_select())
            .first::<Self>(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        project_ids: Option<&[Uuid]>,
        states: Option<&[ForestProjectState]>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let mut query = schema::forest_projects::table.into_boxed();
        let mut count_query = schema::forest_projects::table.into_boxed();

        if let Some(ids) = project_ids {
            query = query.filter(schema::forest_projects::id.eq_any(ids));
            count_query = count_query.filter(schema::forest_projects::id.eq_any(ids));
        }

        if let Some(states) = states {
            query = query.filter(schema::forest_projects::state.eq_any(states));
            count_query = count_query.filter(schema::forest_projects::state.eq_any(states));
        }

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

    pub fn list_ids(
        conn: &mut DbConn,
        states: Option<&[ForestProjectState]>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Uuid>, i64)> {
        let mut query = schema::forest_projects::table.into_boxed();
        let mut count_query = schema::forest_projects::table.into_boxed();

        if let Some(states) = states {
            query = query.filter(schema::forest_projects::state.eq_any(states));
            count_query = count_query.filter(schema::forest_projects::state.eq_any(states));
        }

        let projects = query
            .select(schema::forest_projects::id)
            .order(schema::forest_projects::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results::<Uuid>(conn)?;

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
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
        diesel::insert_into(schema::forest_project_property_media::table)
            .values(self)
            .returning(ForestProjectMedia::as_returning())
            .get_result(conn)
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
    Bond,
    Archived,
}

impl std::fmt::Display for ForestProjectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForestProjectState::Draft => write!(f, "Draft"),
            ForestProjectState::Active => write!(f, "Active"),
            ForestProjectState::Funded => write!(f, "Funded"),
            ForestProjectState::Bond => write!(f, "Bond"),
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
    pub price: Decimal,
    pub price_at: NaiveDateTime,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
}

impl ForestProjectPrice {
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProjectPrice> {
        diesel::insert_into(schema::forest_project_prices::table)
            .values(self)
            .returning(ForestProjectPrice::as_returning())
            .get_result(conn)
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

    pub fn list_by_forest_project_ids(
        conn: &mut DbConn,
        project_ids: &[Uuid],
        currency_token_id: Decimal,
        currency_token_contract_address: Decimal,
    ) -> DbResult<Vec<Self>> {
        let prices = schema::forest_project_prices::table
            .filter(
                schema::forest_project_prices::project_id
                    .eq_any(project_ids)
                    .and(schema::forest_project_prices::currency_token_id.eq(currency_token_id))
                    .and(
                        schema::forest_project_prices::currency_token_contract_address
                            .eq(currency_token_contract_address),
                    ),
            )
            .select(ForestProjectPrice::as_select())
            .order_by((
                schema::forest_project_prices::project_id,
                schema::forest_project_prices::price_at.desc(),
            ))
            .distinct_on(schema::forest_project_prices::project_id)
            .get_results(conn)?;

        Ok(prices)
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
    pub project_id:      uuid::Uuid,
    pub cognito_user_id: String,
    pub user_account:    String,
    pub user_signature:  String,
    pub created_at:      NaiveDateTime,
    pub updated_at:      NaiveDateTime,
}

impl LegalContractUserSignature {
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<LegalContractUserSignature> {
        diesel::insert_into(schema::forest_project_legal_contract_user_signatures::table)
            .values(self)
            .returning(LegalContractUserSignature::as_returning())
            .get_result(conn)
    }

    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<LegalContractUserSignature> {
        use crate::schema::forest_project_legal_contract_user_signatures::dsl::*;
        diesel::insert_into(forest_project_legal_contract_user_signatures)
            .values(self)
            .on_conflict((project_id, cognito_user_id))
            .do_update()
            .set((
                user_account.eq(excluded(user_account)),
                user_signature.eq(excluded(user_signature)),
                updated_at.eq(excluded(updated_at)),
            ))
            .returning(LegalContractUserSignature::as_returning())
            .get_result(conn)
    }

    pub fn find(conn: &mut DbConn, id: Uuid, user_id: &str) -> DbResult<Option<Self>> {
        use crate::schema::forest_project_legal_contract_user_signatures::dsl::*;
        forest_project_legal_contract_user_signatures
            .filter(project_id.eq(id).and(cognito_user_id.eq(user_id)))
            .first::<Self>(conn)
            .optional()
    }

    pub fn list_for_user(
        conn: &mut DbConn,
        project_ids: Option<&[Uuid]>,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<(Uuid, bool)>, i64)> {
        use crate::schema::forest_project_legal_contract_user_signatures::dsl::*;

        let mut query = forest_project_legal_contract_user_signatures
            .filter(cognito_user_id.eq(user_id))
            .into_boxed();
        let mut count_query = forest_project_legal_contract_user_signatures
            .filter(cognito_user_id.eq(user_id))
            .into_boxed();

        if let Some(ids) = project_ids {
            query = query.filter(project_id.eq_any(ids));
            count_query = count_query.filter(project_id.eq_any(ids));
        }

        let results = query
            .select((
                project_id,
                diesel::dsl::sql::<diesel::sql_types::Bool>("true"),
            ))
            .order(created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results::<(Uuid, bool)>(conn)?;

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((results, page_count))
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
    AsChangeset,
)]
#[diesel(table_name = crate::schema::forest_project_legal_contracts)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LegalContract {
    pub project_id: uuid::Uuid,
    pub name:       String,
    pub tag:        String,
    pub text_url:   String,
    pub edoc_url:   String,
    pub pdf_url:    String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl LegalContract {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<Self>> {
        use crate::schema::forest_project_legal_contracts::dsl::*;
        forest_project_legal_contracts
            .filter(project_id.eq(id))
            .first::<Self>(conn)
            .optional()
    }

    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<(Vec<Self>, i64)> {
        use crate::schema::forest_project_legal_contracts::dsl::*;

        let contracts = forest_project_legal_contracts
            .select(LegalContract::as_select())
            .order(created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = forest_project_legal_contracts
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((contracts, page_count))
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<LegalContract> {
        diesel::insert_into(schema::forest_project_legal_contracts::table)
            .values(self)
            .returning(LegalContract::as_returning())
            .get_result(conn)
    }

    pub fn update(&self, conn: &mut DbConn) -> DbResult<LegalContract> {
        diesel::update(schema::forest_project_legal_contracts::table)
            .filter(schema::forest_project_legal_contracts::project_id.eq(self.project_id))
            .set(self)
            .returning(LegalContract::as_returning())
            .get_result(conn)
    }
}

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Queryable)]
pub struct LegalContractUserDbModel {
    pub project_id:      uuid::Uuid,
    pub name:            String,
    pub tag:             String,
    pub text_url:        String,
    pub edoc_url:        String,
    pub pdf_url:         String,
    pub created_at:      NaiveDateTime,
    pub cognito_user_id: String,
    pub signed_date:     NaiveDateTime,
}

impl<DB> Selectable<DB> for LegalContractUserDbModel
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        schema::forest_project_legal_contracts::project_id,
        schema::forest_project_legal_contracts::name,
        schema::forest_project_legal_contracts::tag,
        schema::forest_project_legal_contracts::text_url,
        schema::forest_project_legal_contracts::edoc_url,
        schema::forest_project_legal_contracts::pdf_url,
        schema::forest_project_legal_contracts::created_at,
        schema::forest_project_legal_contract_user_signatures::cognito_user_id,
        schema::forest_project_legal_contract_user_signatures::updated_at,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            schema::forest_project_legal_contracts::project_id,
            schema::forest_project_legal_contracts::name,
            schema::forest_project_legal_contracts::tag,
            schema::forest_project_legal_contracts::text_url,
            schema::forest_project_legal_contracts::edoc_url,
            schema::forest_project_legal_contracts::pdf_url,
            schema::forest_project_legal_contracts::created_at,
            schema::forest_project_legal_contract_user_signatures::cognito_user_id,
            schema::forest_project_legal_contract_user_signatures::updated_at,
        )
    }
}

impl LegalContractUserDbModel {
    pub fn find(conn: &mut DbConn, project_id: Uuid, user_id: &str) -> DbResult<Option<Self>> {
        use crate::schema::forest_project_legal_contract_user_signatures::dsl as signatures_dsl;
        use crate::schema::forest_project_legal_contracts::dsl as contracts_dsl;

        let result = contracts_dsl::forest_project_legal_contracts
            .inner_join(
                signatures_dsl::forest_project_legal_contract_user_signatures
                    .on(contracts_dsl::project_id.eq(signatures_dsl::project_id)),
            )
            .select(LegalContractUserDbModel::as_select())
            .filter(contracts_dsl::project_id.eq(project_id))
            .filter(signatures_dsl::cognito_user_id.eq(user_id))
            .first(conn)
            .optional()?;

        Ok(result)
    }

    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        forest_project_states: &[ForestProjectState],
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        use crate::schema::forest_project_legal_contract_user_signatures::dsl as signatures_dsl;
        use crate::schema::forest_project_legal_contracts::dsl as contracts_dsl;

        let results = contracts_dsl::forest_project_legal_contracts
            .inner_join(
                signatures_dsl::forest_project_legal_contract_user_signatures
                    .on(contracts_dsl::project_id.eq(signatures_dsl::project_id)),
            )
            .inner_join(
                forest_projects::table.on(contracts_dsl::project_id.eq(forest_projects::id)),
            )
            .filter(schema::forest_projects::state.eq_any(forest_project_states))
            .filter(signatures_dsl::cognito_user_id.eq(user_id))
            .select(LegalContractUserDbModel::as_select())
            .order(contracts_dsl::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load::<LegalContractUserDbModel>(conn)?;

        let total_count = contracts_dsl::forest_project_legal_contracts
            .inner_join(
                signatures_dsl::forest_project_legal_contract_user_signatures
                    .on(contracts_dsl::project_id.eq(signatures_dsl::project_id)),
            )
            .count()
            .get_result::<i64>(conn)?;

        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((results, page_count))
    }
}

#[derive(Object, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegalContractUserModel {
    pub project_id:         uuid::Uuid,
    pub name:               String,
    pub tag:                String,
    pub text_url:           String,
    pub edoc_url:           String,
    pub pdf_url:            String,
    pub created_at:         NaiveDateTime,
    pub cognito_user_id:    String,
    pub signed_date:        NaiveDateTime,
    pub user_token_balance: Option<Decimal>,
}

impl LegalContractUserModel {
    pub fn from_db_model(
        db_model: &LegalContractUserDbModel,
        user_token_balance: Option<Decimal>,
    ) -> Self {
        Self {
            project_id: db_model.project_id,
            name: db_model.name.clone(),
            tag: db_model.tag.clone(),
            text_url: db_model.text_url.clone(),
            edoc_url: db_model.edoc_url.clone(),
            pdf_url: db_model.pdf_url.clone(),
            created_at: db_model.created_at,
            cognito_user_id: db_model.cognito_user_id.clone(),
            signed_date: db_model.signed_date,
            user_token_balance,
        }
    }

    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        account_address: &str,
        forest_project_states: &[ForestProjectState],
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let (models, page_count) =
            LegalContractUserDbModel::list(conn, user_id, forest_project_states, page, page_size)?;
        let project_ids = models
            .iter()
            .map(|model| model.project_id)
            .collect::<Vec<_>>();
        let user_token_balances =
            ForestProjectUserBalanceAgg::list_by_user_id_and_forest_project_ids(
                conn,
                account_address,
                &project_ids,
            )?;
        let user_token_balance_map = user_token_balances
            .into_iter()
            .map(|balance| (balance.forest_project_id, balance.total_balance))
            .collect::<std::collections::HashMap<_, _>>();
        let results = models
            .into_iter()
            .map(|model| {
                let user_token_balance = user_token_balance_map.get(&model.project_id).cloned();
                LegalContractUserModel::from_db_model(&model, user_token_balance)
            })
            .collect::<Vec<_>>();
        Ok((results, page_count))
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: Uuid,
        user_id: &str,
        account_address: &str,
    ) -> DbResult<Option<LegalContractUserModel>> {
        let db_model = LegalContractUserDbModel::find(conn, project_id, user_id)?;
        if let Some(db_model) = db_model {
            let user_token_balance =
                ForestProjectUserBalanceAgg::list_by_user_id_and_forest_project_ids(
                    conn,
                    account_address,
                    &[db_model.project_id],
                )?
                .into_iter()
                .find(|balance| balance.forest_project_id == db_model.project_id)
                .map(|balance| balance.total_balance);
            Ok(Some(LegalContractUserModel::from_db_model(
                &db_model,
                user_token_balance,
            )))
        } else {
            Ok(None)
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
        schema::forest_project_notifications::table
            .filter(
                schema::forest_project_notifications::project_id
                    .eq(project_id)
                    .and(schema::forest_project_notifications::cognito_user_id.eq(cognito_user_id)),
            )
            .first(conn)
            .optional()
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<Notification> {
        diesel::insert_into(schema::forest_project_notifications::table)
            .values(self)
            .returning(Notification::as_returning())
            .get_result(conn)
    }

    pub fn list_for_user(
        conn: &mut DbConn,
        project_ids: Option<&[Uuid]>,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<(Uuid, bool)>, i64)> {
        use crate::schema::forest_project_notifications::dsl::*;

        let mut query = forest_project_notifications
            .filter(cognito_user_id.eq(user_id))
            .into_boxed();
        let mut count_query = forest_project_notifications
            .filter(cognito_user_id.eq(user_id))
            .into_boxed();

        if let Some(ids) = project_ids {
            query = query.filter(project_id.eq_any(ids));
            count_query = count_query.filter(project_id.eq_any(ids));
        }

        let results = query
            .select((
                project_id,
                diesel::dsl::sql::<diesel::sql_types::Bool>("true"),
            ))
            .order(created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results::<(Uuid, bool)>(conn)?;

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((results, page_count))
    }
}
