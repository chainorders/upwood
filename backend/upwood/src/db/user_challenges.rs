use chrono::Duration;
use concordium_rust_sdk::constants;
use concordium_rust_sdk::id::types::AccountAddress;
use diesel::prelude::*;
use shared::db::DbConn;

use crate::db::DbResult;
use crate::schema::user_challenges;

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::user_challenges)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserChallenge {
    pub id:              i32,
    pub user_id:         String,
    pub challenge:       Vec<u8>,
    pub account_address: String,
    pub created_at:      chrono::NaiveDateTime,
}

impl UserChallenge {
    pub fn challenge(&self) -> [u8; constants::SHA256] {
        let mut challenge = [0u8; constants::SHA256];
        challenge.copy_from_slice(&self.challenge);
        challenge
    }

    pub fn account_address(&self) -> AccountAddress { self.account_address.parse().unwrap() }
}

#[derive(Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::user_challenges)]
pub struct UserChallengeInsert {
    pub user_id:         String,
    pub challenge:       Vec<u8>,
    pub account_address: String,
}

impl UserChallengeInsert {
    pub fn new(
        user_id: String,
        challenge: [u8; constants::SHA256],
        account_address: AccountAddress,
    ) -> Self {
        Self {
            user_id,
            challenge: challenge.to_vec(),
            account_address: account_address.to_string(),
        }
    }
}

pub fn insert(conn: &mut DbConn, db_challenge: &UserChallengeInsert) -> DbResult<UserChallenge> {
    let user = diesel::insert_into(user_challenges::table)
        .values(db_challenge)
        .returning(UserChallenge::as_returning())
        .get_result::<UserChallenge>(conn)?;
    Ok(user)
}

pub fn find_by_user_id(
    conn: &mut DbConn,
    user_id: &str,
    now: chrono::DateTime<chrono::Utc>,
    expiry: Duration,
) -> DbResult<Option<UserChallenge>> {
    let user = user_challenges::table
        .filter(
            user_challenges::user_id
                .eq(user_id)
                .and(user_challenges::created_at.gt(now - expiry)),
        )
        .order_by(user_challenges::created_at.desc())
        .first::<UserChallenge>(conn)
        .optional()?;
    Ok(user)
}

pub fn delete_by_user_id(conn: &mut DbConn, user_id: &str) -> DbResult<usize> {
    let count = diesel::delete(user_challenges::table.filter(user_challenges::user_id.eq(user_id)))
        .execute(conn)?;
    Ok(count)
}
