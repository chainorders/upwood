use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::db_shared::DbConn;

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_investor)]
#[diesel(primary_key(cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectInvestor {
    pub cognito_user_id:                String,
    pub total_currency_amount_locked:   Decimal,
    pub total_currency_amount_invested: Decimal,
}

impl ForestProjectInvestor {
    pub fn find_by_cognito_user_id(
        conn: &mut DbConn,
        user_id: &str,
    ) -> QueryResult<Option<ForestProjectInvestor>> {
        use crate::schema_manual::forest_project_investor::dsl::*;
        let amount = forest_project_investor
            .select(ForestProjectInvestor::as_select())
            .filter(cognito_user_id.eq(user_id))
            .first(conn)
            .optional()?;
        Ok(amount)
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_trader)]
#[diesel(primary_key(cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectTrader {
    pub cognito_user_id:           String,
    pub total_currency_in_amount:  Decimal,
    pub total_currency_out_amount: Decimal,
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_user_investment_amounts)]
#[diesel(primary_key(cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUserInvestmentAmount {
    pub cognito_user_id:                String,
    pub total_currency_amount_locked:   Decimal,
    pub total_currency_amount_invested: Decimal,
}
impl ForestProjectUserInvestmentAmount {
    pub fn find_by_cognito_user_id(
        conn: &mut DbConn,
        user_id: &str,
    ) -> QueryResult<Option<ForestProjectUserInvestmentAmount>> {
        use crate::schema_manual::forest_project_user_investment_amounts::dsl::*;
        let amount = forest_project_user_investment_amounts
            .select(ForestProjectUserInvestmentAmount::as_select())
            .filter(cognito_user_id.eq(user_id))
            .first(conn)
            .optional()?;
        Ok(amount)
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::user_transactions)]
#[diesel(primary_key(transaction_hash))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserTransaction {
    pub transaction_hash:  String,
    pub forest_project_id: uuid::Uuid,
    pub currency_amount:   rust_decimal::Decimal,
    pub cognito_user_id:   String,
    pub transaction_type:  String,
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
        let count = user_transactions
            .filter(cognito_user_id.eq(user_id))
            .count()
            .get_result(conn)?;
        Ok((transactions, count))
    }
}
