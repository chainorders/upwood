use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::affiliate_claims)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AffiliateClaim {
    pub forest_project_id: Uuid,
    pub contract_address: Decimal,
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub token_contract_address: Decimal,
    pub token_id: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub account_address: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub create_time: chrono::NaiveDateTime,
    pub user_cognito_user_id: String,
    pub user_email: String,
    pub affiliate_account_address: String,
    pub affiliate_cognito_user_id: String,
    pub affiliate_commission: Decimal,
    pub affiliate_reward: Decimal,
    pub claim_nonce: Option<Decimal>,
    pub claim_amount: Option<Decimal>,
    pub affiliate_remaining_reward: Decimal,
}

impl AffiliateClaim {
    pub fn list(
        conn: &mut DbConn,
        affiliate_cognito_user_id_: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64 /* Page Count */)> {
        use crate::schema_manual::affiliate_claims::dsl::*;

        let ret = affiliate_claims
            .filter(affiliate_cognito_user_id.eq(affiliate_cognito_user_id_))
            .order_by(create_time.desc())
            .offset(page * page_size)
            .limit(page_size)
            .load::<Self>(conn)?;

        let count = affiliate_claims
            .filter(affiliate_cognito_user_id.eq(affiliate_cognito_user_id_))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        let page_count = if page_count == 0 { 1 } else { page_count };

        Ok((ret, page_count))
    }

    pub fn find(conn: &mut DbConn, id_: Uuid) -> QueryResult<Option<Self>> {
        use crate::schema_manual::affiliate_claims::dsl::*;

        let ret = affiliate_claims
            .filter(id.eq(id_))
            .first::<Self>(conn)
            .optional()?;

        Ok(ret)
    }
}
