use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::security_token_contract_type::SecurityTokenContractType;
use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_funds_affiliate_reward_records)]
#[diesel(primary_key(investment_record_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectFundsAffiliateRewardRecord {
    pub investment_record_id: Uuid,
    pub fund_contract_address: Decimal,
    pub investment_token_contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub fund_type: SecurityTokenContractType,
    pub forest_project_id: Uuid,
    pub is_default: Option<bool>,
    pub investor_cognito_user_id: String,
    pub investor_account_address: String,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub investment_token_symbol: String,
    pub investment_token_decimals: i32,
    pub currency_token_symbol: String,
    pub currency_token_decimals: i32,
    pub claim_id: Option<Uuid>,
    pub claims_contract_address: Option<Decimal>,
    pub reward_amount: Decimal,
    pub remaining_reward_amount: Decimal,
    pub affiliate_cognito_user_id: String,
    pub affiliate_commission: Decimal,
}

impl ForestProjectFundsAffiliateRewardRecord {
    pub fn find(conn: &mut DbConn, investment_recrd_id: Uuid) -> QueryResult<Option<Self>> {
        use crate::schema_manual::forest_project_funds_affiliate_reward_records::dsl::*;
        forest_project_funds_affiliate_reward_records
            .filter(investment_record_id.eq(investment_recrd_id))
            .first(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        affiliate_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_funds_affiliate_reward_records::dsl::*;

        let total_count = forest_project_funds_affiliate_reward_records
            .filter(affiliate_cognito_user_id.eq(affiliate_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_funds_affiliate_reward_records
            .filter(affiliate_cognito_user_id.eq(affiliate_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;
        let pages_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((records, pages_count))
    }
}
