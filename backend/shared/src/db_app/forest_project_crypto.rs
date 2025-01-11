use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::forest_project::ForestProjectState;
use crate::{db::security_mint_fund::InvestmentRecordType, db_shared::DbConn};

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_funds_affiliate_reward_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectFundsAffiliateRewardRecord {
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub investor: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub investment_time: chrono::NaiveDateTime,
    pub forest_project_id: Uuid,
    pub mint_fund_type: String,
    pub claim_id: Option<Uuid>,
    pub reward_amount: Decimal,
    pub remaining_reward_amount: Decimal,
    pub affiliate_cognito_user_id: String,
    pub investor_cognito_user_id: String,
    pub investor_account_address: String,
}

impl ForestProjectFundsAffiliateRewardRecord {
    pub fn find(conn: &mut DbConn, investment_record_id: Uuid) -> QueryResult<Option<Self>> {
        use crate::schema_manual::forest_project_funds_affiliate_reward_records::dsl::*;
        forest_project_funds_affiliate_reward_records
            .filter(id.eq(investment_record_id))
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

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::active_forest_project_users)]
#[diesel(primary_key(id, cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActiveForestProjectUser {
    pub id: Uuid,
    pub name: String,
    pub label: String,
    pub desc_short: String,
    pub desc_long: String,
    pub area: String,
    pub carbon_credits: i32,
    pub roi_percent: f32,
    pub state: ForestProjectState,
    pub image_small_url: String,
    pub image_large_url: String,
    pub geo_spatial_url: Option<String>,
    pub shares_available: i32,
    pub offering_doc_link: Option<String>,
    pub property_media_header: String,
    pub property_media_footer: String,
    pub latest_price: Decimal,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub investment_token_contract_address: Option<Decimal>,
    pub total_supply: Decimal,
    pub fund_contract_address: Option<Decimal>,
    pub fund_token_id: Option<Decimal>,
    pub fund_token_contract_address: Option<Decimal>,
    pub fund_investment_token_id: Option<Decimal>,
    pub fund_rate_numerator: Option<Decimal>,
    pub fund_rate_denominator: Option<Decimal>,
    pub notification_id: Option<Uuid>,
    pub cognito_user_id: Option<String>,
    pub has_signed_contract: bool,
}

impl ActiveForestProjectUser {
    pub fn find(conn: &mut DbConn, user_id: Uuid, cognito_id: &str) -> QueryResult<Option<Self>> {
        use crate::schema_manual::active_forest_project_users::dsl::*;
        active_forest_project_users
            .filter(id.eq(user_id))
            .filter(cognito_user_id.eq(cognito_id))
            .first(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        cognito_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::active_forest_project_users::dsl::*;

        let total_count = active_forest_project_users
            .filter(cognito_user_id.eq(cognito_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = active_forest_project_users
            .filter(cognito_user_id.eq(cognito_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::funded_forest_project_users)]
#[diesel(primary_key(id, cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FundedForestProjectUser {
    pub id: Uuid,
    pub name: String,
    pub label: String,
    pub desc_short: String,
    pub desc_long: String,
    pub area: String,
    pub carbon_credits: i32,
    pub roi_percent: f32,
    pub state: ForestProjectState,
    pub image_small_url: String,
    pub image_large_url: String,
    pub geo_spatial_url: Option<String>,
    pub shares_available: i32,
    pub offering_doc_link: Option<String>,
    pub property_media_header: String,
    pub property_media_footer: String,
    pub latest_price: Decimal,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub investment_token_contract_address: Option<Decimal>,
    pub total_supply: Decimal,
    pub market_contract_address: Option<Decimal>,
    pub market_token_id: Option<Decimal>,
    pub market_liquidity_provider: Option<String>,
    pub market_buy_rate_numerator: Option<Decimal>,
    pub market_buy_rate_denominator: Option<Decimal>,
    pub notification_id: Option<Uuid>,
    pub cognito_user_id: Option<String>,
    pub has_signed_contract: bool,
}

impl FundedForestProjectUser {
    pub fn find(conn: &mut DbConn, user_id: Uuid, cognito_id: &str) -> QueryResult<Option<Self>> {
        use crate::schema_manual::funded_forest_project_users::dsl::*;
        funded_forest_project_users
            .filter(id.eq(user_id))
            .filter(cognito_user_id.eq(cognito_id))
            .first(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        cognito_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::funded_forest_project_users::dsl::*;

        let total_count = funded_forest_project_users
            .filter(cognito_user_id.eq(cognito_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = funded_forest_project_users
            .filter(cognito_user_id.eq(cognito_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_projects_owned_by_user)]
#[diesel(primary_key(id, cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectOwned {
    pub id: Uuid,
    pub name: String,
    pub label: String,
    pub desc_short: String,
    pub desc_long: String,
    pub area: String,
    pub carbon_credits: i32,
    pub roi_percent: f32,
    pub state: ForestProjectState,
    pub image_small_url: String,
    pub image_large_url: String,
    pub geo_spatial_url: Option<String>,
    pub shares_available: i32,
    pub offering_doc_link: Option<String>,
    pub property_media_header: String,
    pub property_media_footer: String,
    pub latest_price: Decimal,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub total_supply: Decimal,
    pub property_contract_address: Option<Decimal>,
    pub property_token_id: Option<Decimal>,
    pub market_contract_address: Option<Decimal>,
    pub market_liquidity_provider: Option<String>,
    pub market_buy_rate_numerator: Option<Decimal>,
    pub market_buy_rate_denominator: Option<Decimal>,
    pub bond_contract_address: Option<Decimal>,
    pub bond_token_id: Option<Decimal>,
    pub bond_fund_contract_address: Option<Decimal>,
    pub bond_fund_rate_numerator: Option<Decimal>,
    pub bond_fund_rate_denominator: Option<Decimal>,
    pub cognito_user_id: String,
    pub account_address: String,
    pub total_balance: Decimal,
}

impl ForestProjectOwned {
    pub fn find(
        conn: &mut DbConn,
        project_id: Uuid,
        user_id: &str,
        fund_address: Decimal,
        market_address: Decimal,
    ) -> QueryResult<Option<Self>> {
        use crate::schema_manual::forest_projects_owned_by_user::dsl::*;
        forest_projects_owned_by_user
            .filter(id.eq(project_id))
            .filter(cognito_user_id.eq(user_id))
            .filter(
                bond_fund_contract_address
                    .is_null()
                    .or(bond_fund_contract_address.eq(fund_address)),
            )
            .filter(
                market_contract_address
                    .is_null()
                    .or(market_contract_address.eq(market_address)),
            )
            .first(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        fund_address: Decimal,
        market_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_projects_owned_by_user::dsl::*;

        let total_count = forest_projects_owned_by_user
            .filter(cognito_user_id.eq(user_id))
            .filter(
                bond_fund_contract_address
                    .is_null()
                    .or(bond_fund_contract_address.eq(fund_address)),
            )
            .filter(
                market_contract_address
                    .is_null()
                    .or(market_contract_address.eq(market_address)),
            )
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_projects_owned_by_user
            .filter(cognito_user_id.eq(user_id))
            .filter(
                bond_fund_contract_address
                    .is_null()
                    .or(bond_fund_contract_address.eq(fund_address)),
            )
            .filter(
                market_contract_address
                    .is_null()
                    .or(market_contract_address.eq(market_address)),
            )
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_user_yields_for_each_owned_token)]
#[diesel(primary_key(
    forest_project_id,
    token_id,
    token_contract_address,
    holder_address,
    yielder_contract_address,
    yield_token_id,
    yield_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUserYieldsForEachOwnedToken {
    pub forest_project_id:        Uuid,
    pub token_id:                 Decimal,
    pub token_contract_address:   Decimal,
    pub holder_address:           String,
    pub token_balance:            Decimal,
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub yield_token_id:           Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_amount:             Decimal,
    pub max_token_id:             Decimal,
}

impl ForestProjectUserYieldsForEachOwnedToken {
    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        yielder_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_user_yields_for_each_owned_token::dsl::*;

        let total_count = forest_project_user_yields_for_each_owned_token
            .filter(cognito_user_id.eq(user_id))
            .filter(yielder_contract_address.eq(yielder_address))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_user_yields_for_each_owned_token
            .filter(cognito_user_id.eq(user_id))
            .filter(yielder_contract_address.eq(yielder_address))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_user_yields_aggregate)]
#[diesel(primary_key(
    cognito_user_id,
    yielder_contract_address,
    yield_token_id,
    yield_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUserYieldsAggregate {
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub yield_token_id:           Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_amount:             Decimal,
}

impl ForestProjectUserYieldsAggregate {
    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        yielder_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_user_yields_aggregate::dsl::*;

        let query = forest_project_user_yields_aggregate
            .filter(cognito_user_id.eq(user_id))
            .filter(
                yielder_contract_address
                    .is_null()
                    .or(yielder_contract_address.eq(yielder_address)),
            );

        let total_count = forest_project_user_yields_aggregate
            .filter(cognito_user_id.eq(user_id))
            .filter(
                yielder_contract_address
                    .is_null()
                    .or(yielder_contract_address.eq(yielder_address)),
            )
            .count()
            .get_result::<i64>(conn)?;

        let records = query
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_fund_investor)]
#[diesel(primary_key(fund_contract_address, investor_cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectFundInvestor {
    pub forest_project_id:        Uuid,
    pub fund_contract_address:    Decimal,
    pub investor_account_address: String,
    pub investment_token_amount:  Decimal,
    pub investor_cognito_user_id: String,
    pub investor_email:           String,
}

impl ForestProjectFundInvestor {
    pub fn list(
        conn: &mut DbConn,
        project_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_fund_investor::dsl::*;

        let total_count = forest_project_fund_investor
            .filter(forest_project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_fund_investor
            .filter(forest_project_id.eq(project_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    diesel_derive_enum::DbEnum,
    Debug,
    PartialEq,
    Enum,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
)]
#[ExistingTypePath = "crate::schema::sql_types::ForestProjectSecurityTokenContractType"]
#[DbValueStyle = "snake_case"]
pub enum SecurityTokenContractType {
    Property,
    Bond,
    PropertyPreSale,
    BondPreSale,
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_funds_investment_records)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectFundsInvestmentRecord {
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub investor: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub currency_amount_balance: Decimal,
    pub token_amount_balance: Decimal,
    pub investment_record_type: InvestmentRecordType,
    pub create_time: chrono::NaiveDateTime,
    pub forest_project_id: Uuid,
    pub fund_type: SecurityTokenContractType,
    pub is_default: bool,
    pub investor_cognito_user_id: String,
}

impl ForestProjectFundsInvestmentRecord {
    pub fn list(
        conn: &mut DbConn,
        contract_addr: Decimal,
        investor_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_funds_investment_records::dsl::*;

        let total_count = forest_project_funds_investment_records
            .filter(contract_address.eq(contract_addr))
            .filter(investor_cognito_user_id.eq(investor_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_funds_investment_records
            .filter(contract_address.eq(contract_addr))
            .filter(investor_cognito_user_id.eq(investor_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}
