use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use tracing::instrument;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::{user_affiliates, users};

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub cognito_user_id:           String,
    pub email:                     String,
    pub account_address:           Option<String>,
    pub desired_investment_amount: Option<i32>,
    pub affiliate_commission:      Decimal,
}

impl User {
    pub fn account_address(&self) -> Option<AccountAddress> {
        self.account_address
            .as_ref()
            .map(|s| s.parse().expect("Failed to parse account address"))
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

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::user_affiliates)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAffiliate {
    pub id: i32,
    pub cognito_user_id: String,
    pub affiliate_account_address: String,
}

impl UserAffiliate {
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

diesel::table! {
    affiliate_rewards_view (forest_project_id) {
        investment_record_id -> Uuid,
        forest_project_id -> Uuid,
        fund_contract_address -> Numeric,
        investor -> Text,
        currency_amount -> Numeric,
        investor_email -> Text,
        affiliate_account_address -> Text,
        affiliate_email -> Text,
        affiliate_commission -> Numeric,
        reward_amount -> Numeric,
        claim_id -> Nullable<Uuid>,
        claimed_reward_amount -> Nullable<Numeric>,
        remaining_reward_amount -> Nullable<Numeric>,
        investment_time -> Timestamp,
    }
}

#[derive(
    Object, Selectable, Queryable, Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone,
)]
#[diesel(table_name = affiliate_rewards_view)]
#[diesel(primary_key(investment_record_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AffiliateReward {
    pub investment_record_id:      uuid::Uuid,
    pub forest_project_id:         uuid::Uuid,
    pub fund_contract_address:     Decimal,
    pub investor:                  String,
    pub currency_amount:           Decimal,
    pub investor_email:            String,
    pub affiliate_account_address: String,
    pub affiliate_email:           String,
    pub affiliate_commission:      Decimal,
    pub reward_amount:             Decimal,
    pub claim_id:                  Option<uuid::Uuid>,
    pub claimed_reward_amount:     Option<Decimal>,
    pub remaining_reward_amount:   Option<Decimal>,
    pub investment_time:           chrono::NaiveDateTime,
}

impl AffiliateReward {
    pub fn list_by_affiliate(
        conn: &mut DbConn,
        affiliate_account_address: &str,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<AffiliateReward>, i64)> {
        let rewards = affiliate_rewards_view::table
            .filter(affiliate_rewards_view::affiliate_account_address.eq(affiliate_account_address))
            .limit(page_size)
            .offset(page * page_size)
            .order_by(affiliate_rewards_view::investment_time.desc())
            .get_results(conn)?;
        let count: i64 = affiliate_rewards_view::table
            .filter(affiliate_rewards_view::affiliate_account_address.eq(affiliate_account_address))
            .count()
            .get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((rewards, page_count))
    }

    pub fn find(
        conn: &mut DbConn,
        investment_record_id: &uuid::Uuid,
    ) -> DbResult<Option<AffiliateReward>> {
        affiliate_rewards_view::table
            .filter(affiliate_rewards_view::investment_record_id.eq(investment_record_id))
            .select(AffiliateReward::as_select())
            .first(conn)
            .optional()
    }
}
