use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::users;

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    AsChangeset,
    Object,
    serde::Serialize,
    serde::Deserialize,
    Clone,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
pub struct User {
    pub cognito_user_id:           String,
    pub email:                     String,
    pub account_address:           String,
    pub first_name:                String,
    pub last_name:                 String,
    pub nationality:               String,
    pub desired_investment_amount: Option<i32>,
    pub affiliate_commission:      Decimal,
    pub affiliate_account_address: Option<String>,
}

impl User {
    pub fn account_address(&self) -> AccountAddress {
        self.account_address
            .parse()
            .expect("Failed to parse account address")
    }

    pub fn find(conn: &mut DbConn, cognito_user_id: &str) -> DbResult<Option<User>> {
        users::table
            .filter(users::cognito_user_id.eq(cognito_user_id))
            .select(User::as_select())
            .first(conn)
            .optional()
    }

    pub fn find_by_email(conn: &mut DbConn, email: &str) -> DbResult<Option<User>> {
        users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(conn)
            .optional()
    }

    pub fn find_by_account_address(
        conn: &mut DbConn,
        account_address: &AccountAddress,
    ) -> DbResult<Option<User>> {
        users::table
            .filter(users::account_address.eq(account_address.to_string()))
            .select(User::as_select())
            .first(conn)
            .optional()
    }

    #[instrument(skip(conn))]
    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<(Vec<User>, i64)> {
        let query = users::table.select(User::as_select());
        let users = query
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;
        let count: i64 = query.count().get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((users, page_count))
    }

    #[instrument(skip(conn))]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<User> {
        diesel::insert_into(users::table)
            .values(self)
            .on_conflict(users::cognito_user_id)
            .do_update()
            .set(self)
            .returning(User::as_returning())
            .get_result(conn)
    }

    #[instrument(skip(conn))]
    pub fn delete(conn: &mut DbConn, cognito_user_id: &str) -> DbResult<usize> {
        diesel::delete(users::table.filter(users::cognito_user_id.eq(cognito_user_id)))
            .execute(conn)
    }

    #[instrument(skip(conn))]
    pub fn update_account_address(
        conn: &mut DbConn,
        cognito_user_id: &str,
        account_address: &concordium_rust_sdk::id::types::AccountAddress,
    ) -> DbResult<User> {
        diesel::update(users::table.filter(users::cognito_user_id.eq(cognito_user_id)))
            .set(users::account_address.eq(account_address.to_string()))
            .returning(User::as_returning())
            .get_result(conn)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    AsChangeset,
    Object,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = crate::schema::user_registration_requests)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
pub struct UserRegistrationRequest {
    pub id: Uuid,
    pub email: String,
    pub affiliate_account_address: Option<String>,
    pub is_accepted: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl UserRegistrationRequest {
    #[instrument(skip(conn))]
    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<(Vec<Self>, i64)> {
        use crate::schema::user_registration_requests::dsl::*;
        let query = user_registration_requests.select(Self::as_select());
        let requests = query
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;
        let count: i64 = query.count().get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((requests, page_count))
    }

    pub fn find(conn: &mut DbConn, request_id: Uuid) -> DbResult<Option<Self>> {
        use crate::schema::user_registration_requests::dsl::*;
        user_registration_requests
            .filter(id.eq(request_id))
            .first(conn)
            .optional()
    }

    pub fn find_by_email(conn: &mut DbConn, user_email: &str) -> DbResult<Option<Self>> {
        use crate::schema::user_registration_requests::dsl::*;
        user_registration_requests
            .filter(email.eq(user_email))
            .first(conn)
            .optional()
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<Self> {
        use crate::schema::user_registration_requests::dsl::*;
        diesel::insert_into(user_registration_requests)
            .values(self)
            .get_result(conn)
    }

    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        use crate::schema::user_registration_requests::dsl::*;
        diesel::update(user_registration_requests.filter(id.eq(self.id)))
            .set(self)
            .get_result(conn)
    }

    pub fn delete(conn: &mut DbConn, request_id: Uuid) -> DbResult<usize> {
        use crate::schema::user_registration_requests::dsl::*;
        diesel::delete(user_registration_requests.filter(id.eq(request_id))).execute(conn)
    }
}
