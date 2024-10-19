use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use shared::db::DbConn;
use tracing::instrument;

use super::DbResult;
use crate::schema::users;

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, AsChangeset)]
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
pub fn find_account_address(
    conn: &mut DbConn,
    cognito_user_id: &str,
) -> DbResult<Option<AccountAddress>> {
    users::table
        .filter(users::cognito_user_id.eq(cognito_user_id))
        .select(users::account_address)
        .first::<Option<String>>(conn)
        .map(|s| s.map(|s| s.parse().expect("Failed to parse account address")))
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
pub fn list_users(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<(Vec<User>, i64)> {
    let query = users::table.select(User::as_select());
    let users = query
        .limit(page_size)
        .offset(page * page_size)
        .get_results(conn)?;
    let count: i64 = query.count().get_result(conn)?;
    let page_count = (count + page_size - 1) / page_size;
    Ok((users, page_count))
}

#[instrument(skip(conn))]
pub fn upsert(conn: &mut DbConn, user: &User) -> DbResult<User> {
    diesel::insert_into(users::table)
        .values(user)
        .on_conflict(users::cognito_user_id)
        .do_update()
        .set(user)
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

pub mod affiliation {
    use concordium_rust_sdk::id::types::AccountAddress;
    use diesel::prelude::*;
    use shared::db::DbConn;
    use tracing::instrument;

    use crate::db::DbResult;
    use crate::schema::user_affiliates;

    #[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable)]
    #[diesel(table_name = crate::schema::user_affiliates)]
    #[diesel(primary_key(id))]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct UserAffiliate {
        pub id: i32,
        pub cognito_user_id: String,
        pub affiliate_account_address: String,
    }

    #[instrument(skip(conn))]
    pub fn insert(
        conn: &mut DbConn,
        user_id: &str,
        affiliate_account_address: &AccountAddress,
    ) -> DbResult<UserAffiliate> {
        let res = diesel::insert_into(user_affiliates::table)
            .values((
                user_affiliates::cognito_user_id.eq(user_id),
                user_affiliates::affiliate_account_address
                    .eq(affiliate_account_address.to_string()),
            ))
            .returning(UserAffiliate::as_returning())
            .get_result(conn)?;
        Ok(res)
    }
}
