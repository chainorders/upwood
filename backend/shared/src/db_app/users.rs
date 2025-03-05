use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::db::identity_registry::Identity;
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

#[derive(Object, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct UserKYCModel {
    pub cognito_user_id:           String,
    pub email:                     String,
    pub account_address:           String,
    pub first_name:                String,
    pub last_name:                 String,
    pub nationality:               String,
    pub desired_investment_amount: Option<i32>,
    pub affiliate_commission:      Decimal,
    pub affiliate_account_address: Option<String>,
    pub kyc_verified:              bool,
}

impl UserKYCModel {
    pub fn new(user: User, kyc_verified: bool) -> Self {
        UserKYCModel {
            cognito_user_id: user.cognito_user_id,
            email: user.email,
            account_address: user.account_address,
            first_name: user.first_name,
            last_name: user.last_name,
            affiliate_account_address: user.affiliate_account_address,
            affiliate_commission: user.affiliate_commission,
            desired_investment_amount: user.desired_investment_amount,
            nationality: user.nationality,
            kyc_verified,
        }
    }

    pub fn account_address(&self) -> AccountAddress {
        self.account_address
            .parse()
            .expect("Failed to parse account address")
    }

    pub fn find_by_account_address(
        conn: &mut DbConn,
        identity_registry_contract_index: Decimal,
        account_address_: &str,
    ) -> DbResult<Option<Self>> {
        use crate::schema::identity_registry_identities::dsl::*;
        use crate::schema::users::dsl::*;

        let res = users
            .left_join(identity_registry_identities.on(account_address.eq(identity_address)))
            .select((User::as_select(), Option::<Identity>::as_select()))
            .filter(account_address.eq(account_address_))
            .filter(identity_registry_address.eq(identity_registry_contract_index))
            .first(conn)
            .optional()?
            .map(|(user, identity)| UserKYCModel::new(user, identity.is_some()));
        Ok(res)
    }

    pub fn find(
        conn: &mut DbConn,
        identity_registry_contract_index: Decimal,
        user_id: &str,
    ) -> DbResult<Option<Self>> {
        use crate::schema::identity_registry_identities::dsl::*;
        use crate::schema::users::dsl::*;

        let res = users
            .left_join(identity_registry_identities.on(account_address.eq(identity_address)))
            .select((User::as_select(), Option::<Identity>::as_select()))
            .filter(cognito_user_id.eq(user_id))
            .filter(identity_registry_address.eq(identity_registry_contract_index))
            .first(conn)
            .optional()?
            .map(|(user, identity)| UserKYCModel::new(user, identity.is_some()));
        Ok(res)
    }

    pub fn list(
        conn: &mut DbConn,
        identity_registry_contract_index: Decimal,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        use crate::schema::identity_registry_identities::dsl::*;
        use crate::schema::users::dsl::*;

        let query = users
            .left_join(identity_registry_identities.on(account_address.eq(identity_address)))
            .select((User::as_select(), Option::<Identity>::as_select()))
            .filter(
                identity_registry_address
                    .eq(identity_registry_contract_index)
                    .or(identity_registry_address.is_null()),
            );

        let ret = query
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?
            .into_iter()
            .map(|(user, identity)| UserKYCModel::new(user, identity.is_some()));

        let count: i64 = query.count().get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((ret.collect(), page_count))
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
