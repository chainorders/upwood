use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_user_investment_amounts)]
#[diesel(primary_key(cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUserInvestmentAmount {
    pub cognito_user_id:                 String,
    pub currency_token_id:               Decimal,
    pub currency_token_contract_address: Decimal,
    pub total_currency_amount_locked:    Decimal,
    pub total_currency_amount_invested:  Decimal,
}
impl ForestProjectUserInvestmentAmount {
    pub fn find_by_cognito_user_id(
        conn: &mut DbConn,
        user_id: &str,
        curr_token_id: Decimal,
        curr_token_contract_address: Decimal,
    ) -> QueryResult<Option<ForestProjectUserInvestmentAmount>> {
        use crate::schema_manual::forest_project_user_investment_amounts::dsl::*;
        let result = forest_project_user_investment_amounts
            .filter(cognito_user_id.eq(user_id))
            .filter(currency_token_id.eq(curr_token_id))
            .filter(currency_token_contract_address.eq(curr_token_contract_address))
            .first::<ForestProjectUserInvestmentAmount>(conn)
            .optional()?;
        Ok(result)
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::user_transactions)]
#[diesel(primary_key(transaction_hash))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserTransaction {
    pub transaction_hash:                String,
    pub block_height:                    Decimal,
    pub forest_project_id:               uuid::Uuid,
    pub currency_token_id:               Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_amount:                 Decimal,
    pub currency_token_symbol:           String,
    pub currency_token_decimals:         i32,
    pub cognito_user_id:                 String,
    pub transaction_type:                String,
    pub account_address:                 String,
}

impl UserTransaction {
    pub fn list_by_cognito_user_id(
        conn: &mut DbConn,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<UserTransaction>, i64)> {
        use crate::schema_manual::user_transactions::dsl::*;
        let transactions = user_transactions
            .filter(cognito_user_id.eq(user_id))
            .limit(page_size)
            .offset(page_size * page)
            .load::<UserTransaction>(conn)?;
        let total_count: i64 = user_transactions
            .filter(cognito_user_id.eq(user_id))
            .count()
            .get_result(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((transactions, page_count))
    }
}
