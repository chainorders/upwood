use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use shared::db::DbConn;
use tracing::instrument;

use super::DbResult;
use crate::schema::users;

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub cognito_user_id:           String,
    pub email:                     String,
    pub account_address:           Option<String>,
    pub desired_investment_amount: Option<i32>,
}

impl User {
    pub fn account_address(&self) -> Option<AccountAddress> {
        self.account_address
            .as_ref()
            .map(|s| s.parse().expect("Failed to parse account address"))
    }
}

#[instrument(skip(conn))]
pub fn find_user_by_cognito_user_id(
    conn: &mut DbConn,
    cognito_user_id: &str,
) -> DbResult<Option<User>> {
    users::table
        .filter(users::cognito_user_id.eq(cognito_user_id))
        .select(User::as_select())
        .first(conn)
        .optional()
}

#[instrument(skip(conn))]
pub fn find_user_by_email(conn: &mut DbConn, email: &str) -> DbResult<Option<User>> {
    users::table
        .filter(users::email.eq(email))
        .select(User::as_select())
        .first(conn)
        .optional()
}

#[instrument(skip(conn))]
pub fn find_user_by_account_address(
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
pub fn list_users(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<Vec<User>> {
    users::table
        .select(User::as_select())
        .limit(page_size)
        .offset(page * page_size)
        .load(conn)
}

#[instrument(skip(conn))]
pub fn insert(conn: &mut DbConn, user: &User) -> DbResult<User> {
    diesel::insert_into(users::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(conn)
}

#[instrument(skip(conn))]
pub fn delete_by_cognito_user_id(conn: &mut DbConn, cognito_user_id: &str) -> DbResult<usize> {
    diesel::delete(users::table.filter(users::cognito_user_id.eq(cognito_user_id))).execute(conn)
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
