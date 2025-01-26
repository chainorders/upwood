use std::{cmp, hash};

use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::forest_project::ForestProjectState;
use crate::db::security_mint_fund::{InvestmentRecordType, SecurityMintFundState};
use crate::db_app::forest_project::ForestProject;
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

        Ok((records, total_count))
    }
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::forest_project_token_contracts)]
#[diesel(belongs_to(ForestProject, foreign_key = forest_project_id))]
#[diesel(primary_key(forest_project_id, contract_type))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectTokenContract {
    pub contract_address:  Decimal,
    pub forest_project_id: Uuid,
    pub contract_type:     SecurityTokenContractType,
    pub fund_token_id:     Option<Decimal>,
    pub market_token_id:   Option<Decimal>,
    pub symbol:            Option<String>,
    pub decimals:          Option<i32>,
    pub created_at:        chrono::NaiveDateTime,
    pub updated_at:        chrono::NaiveDateTime,
}

impl ForestProjectTokenContract {
    pub fn update(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        diesel::update(
            forest_project_token_contracts
                .filter(forest_project_id.eq(self.forest_project_id))
                .filter(contract_type.eq(self.contract_type)),
        )
        .set(self)
        .get_result(conn)
    }

    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        diesel::insert_into(forest_project_token_contracts)
            .values(self)
            .get_result(conn)
    }

    pub fn delete(
        conn: &mut DbConn,
        project_id: Uuid,
        r#type: SecurityTokenContractType,
    ) -> QueryResult<usize> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        diesel::delete(
            forest_project_token_contracts
                .filter(forest_project_id.eq(project_id))
                .filter(contract_type.eq(r#type)),
        )
        .execute(conn)
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: Uuid,
        token_contract_type: SecurityTokenContractType,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::forest_project_token_contracts::dsl::*;
        forest_project_token_contracts
            .filter(forest_project_id.eq(project_id))
            .filter(contract_type.eq(token_contract_type))
            .first(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        project_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::forest_project_token_contracts::dsl::*;

        let total_count = forest_project_token_contracts
            .filter(forest_project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_token_contracts
            .filter(forest_project_id.eq(project_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}
#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_token_user_yields)]
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
pub struct ForestProjectTokenUserYield {
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
    pub token_symbol:             String,
    pub token_decimals:           i32,
    pub yield_token_symbol:       String,
    pub yield_token_decimals:     i32,
}

impl ForestProjectTokenUserYield {
    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        yielder_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_token_user_yields::dsl::*;

        let total_count = forest_project_token_user_yields
            .filter(cognito_user_id.eq(user_id))
            .filter(yielder_contract_address.eq(yielder_address))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_token_user_yields
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
#[diesel(table_name = crate::schema_manual::user_yields_aggregate)]
#[diesel(primary_key(
    cognito_user_id,
    yielder_contract_address,
    yield_token_id,
    yield_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserYieldsAggregate {
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub yield_token_id:           Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_amount:             Decimal,
    pub yield_token_symbol:       String,
    pub yield_token_decimals:     i32,
}

impl UserYieldsAggregate {
    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        yielder_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::user_yields_aggregate::dsl::*;

        let query = user_yields_aggregate
            .filter(cognito_user_id.eq(user_id))
            .filter(
                yielder_contract_address
                    .is_null()
                    .or(yielder_contract_address.eq(yielder_address)),
            );

        let total_count = user_yields_aggregate
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
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    QueryableByName,
)]
#[diesel(table_name = crate::schema_manual::forest_project_fund_investor)]
#[diesel(primary_key(forest_project_id, fund_contract_address, investor_cognito_user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectFundInvestor {
    pub forest_project_id: Uuid,
    pub fund_contract_address: Decimal,
    pub fund_token_id: Decimal,
    pub fund_token_contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub fund_type: SecurityTokenContractType,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_token_symbol: String,
    pub currency_token_decimals: i32,
    pub investment_token_symbol: String,
    pub investment_token_decimals: i32,
    pub fund_token_symbol: String,
    pub fund_token_decimals: i32,
    pub investor_account_address: String,
    pub investment_token_amount: Decimal,
    pub investment_currency_amount: Decimal,
    pub investor_cognito_user_id: String,
    pub investor_email: String,
}

impl ForestProjectFundInvestor {
    pub fn list(
        conn: &mut DbConn,
        fund_contract_addr: Decimal,
        project_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_fund_investor::dsl::*;

        let total_count = forest_project_fund_investor
            .filter(forest_project_id.eq(project_id))
            .filter(fund_contract_address.eq(fund_contract_addr))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_fund_investor
            .filter(forest_project_id.eq(project_id))
            .filter(fund_contract_address.eq(fund_contract_addr))
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
    cmp::Eq,
    hash::Hash,
)]
#[ExistingTypePath = "crate::schema::sql_types::ForestProjectSecurityTokenContractType"]
#[DbValueStyle = "snake_case"]
pub enum SecurityTokenContractType {
    Property,
    Bond,
    PropertyPreSale,
    BondPreSale,
}

impl std::fmt::Display for SecurityTokenContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityTokenContractType::Property => write!(f, "Property"),
            SecurityTokenContractType::Bond => write!(f, "Bond"),
            SecurityTokenContractType::PropertyPreSale => write!(f, "PropertyPreSale"),
            SecurityTokenContractType::BondPreSale => write!(f, "BondPreSale"),
        }
    }
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
    pub investment_token_symbol: String,
    pub investment_token_decimals: i32,
    pub currency_token_symbol: String,
    pub currency_token_decimals: i32,
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

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_supply)]
#[diesel(primary_key(forest_project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectSupply {
    pub forest_project_id:    Uuid,
    pub forest_project_state: ForestProjectState,
    pub supply:               Option<Decimal>,
    pub symbol:               String,
    pub decimals:             i32,
}

impl ForestProjectSupply {
    pub fn find_by_forest_project_id(
        conn: &mut DbConn,
        project_id: Uuid,
    ) -> QueryResult<Option<Self>> {
        use crate::schema_manual::forest_project_supply::dsl::*;
        forest_project_supply
            .filter(forest_project_id.eq(project_id))
            .first(conn)
            .optional()
    }

    pub fn list_by_forest_project_state(
        conn: &mut DbConn,
        state: ForestProjectState,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_supply::dsl::*;

        let total_count = forest_project_supply
            .filter(forest_project_state.eq(state))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_supply
            .filter(forest_project_state.eq(state))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }

    pub fn list_by_forest_project_ids(
        conn: &mut DbConn,
        project_ids: &[Uuid],
        page: i64,
        page_size: i64,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_supply::dsl::*;

        let records = forest_project_supply
            .filter(forest_project_id.eq_any(project_ids))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok(records)
    }

    pub fn list_by_forest_project_id(
        conn: &mut DbConn,
        project_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_supply::dsl::*;

        let total_count = forest_project_supply
            .filter(forest_project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_supply
            .filter(forest_project_id.eq(project_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_current_token_fund_markets)]
#[diesel(primary_key(forest_project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectCurrentTokenFundMarkets {
    pub forest_project_id:            Uuid,
    pub forest_project_state:         ForestProjectState,
    pub token_contract_address:       Decimal,
    pub token_id:                     Option<Decimal>,
    pub token_contract_type:          SecurityTokenContractType,
    pub market_token_id:              Option<Decimal>,
    pub token_symbol:                 String,
    pub token_decimals:               i32,
    pub fund_contract_address:        Option<Decimal>,
    pub fund_rate_numerator:          Option<Decimal>,
    pub fund_rate_denominator:        Option<Decimal>,
    pub fund_state:                   Option<SecurityMintFundState>,
    pub fund_token_contract_address:  Option<Decimal>,
    pub fund_token_id:                Option<Decimal>,
    pub market_contract_address:      Option<Decimal>,
    pub market_sell_rate_numerator:   Option<Decimal>,
    pub market_sell_rate_denominator: Option<Decimal>,
    pub market_buy_rate_numerator:    Option<Decimal>,
    pub market_buy_rate_denominator:  Option<Decimal>,
    pub market_liquidity_provider:    Option<String>,
}

impl ForestProjectCurrentTokenFundMarkets {
    pub fn list_by_forest_project_state(
        conn: &mut DbConn,
        state: ForestProjectState,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_current_token_fund_markets::dsl::*;

        let total_count = forest_project_current_token_fund_markets
            .filter(forest_project_state.eq(state))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_current_token_fund_markets
            .filter(forest_project_state.eq(state))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }

    pub fn list_by_forest_project_ids(
        conn: &mut DbConn,
        project_ids: &[Uuid],
        page: i64,
        page_size: i64,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_current_token_fund_markets::dsl::*;

        let records = forest_project_current_token_fund_markets
            .filter(forest_project_id.eq_any(project_ids))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok(records)
    }

    pub fn list_by_forest_project_id(
        conn: &mut DbConn,
        project_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_current_token_fund_markets::dsl::*;

        let total_count = forest_project_current_token_fund_markets
            .filter(forest_project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_current_token_fund_markets
            .filter(forest_project_id.eq(project_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_user_balance_agg)]
#[diesel(primary_key(cognito_user_id, forest_project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUserBalanceAgg {
    pub cognito_user_id:   String,
    pub forest_project_id: Uuid,
    pub total_balance:     Decimal,
}

impl ForestProjectUserBalanceAgg {
    pub fn find(conn: &mut DbConn, user_id: &str, project_id: Uuid) -> QueryResult<Option<Self>> {
        use crate::schema_manual::forest_project_user_balance_agg::dsl::*;
        forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(forest_project_id.eq(project_id))
            .first(conn)
            .optional()
    }

    pub fn list_by_user_id_and_forest_project_ids(
        conn: &mut DbConn,
        user_id: &str,
        project_ids: &[Uuid],
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_user_balance_agg::dsl::*;

        forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(forest_project_id.eq_any(project_ids))
            .load::<Self>(conn)
    }

    pub fn list_by_user_id(
        conn: &mut DbConn,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_user_balance_agg::dsl::*;

        let total_count = forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(total_balance.gt(Decimal::ZERO))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    Serialize,
    AsChangeset,
)]
#[diesel(table_name = crate::schema_manual::forest_project_user_yield_distributions)]
#[diesel(primary_key(yield_distribution_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUserYieldDistribution {
    pub forest_project_id:        Uuid,
    pub forest_project_name:      String,
    pub token_contract_type:      SecurityTokenContractType,
    pub cognito_user_id:          String,
    pub yield_distribution_id:    Uuid,
    pub yielder_contract_address: Decimal,
    pub token_contract_address:   Decimal,
    pub from_token_version:       Decimal,
    pub to_token_version:         Decimal,
    pub token_amount:             Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_token_id:           Decimal,
    pub yield_amount:             Decimal,
    pub yield_token_symbol:       String,
    pub yield_token_decimals:     i32,
    pub token_symbol:             String,
    pub token_decimals:           i32,
    pub to_address:               String,
    pub create_time:              chrono::NaiveDateTime,
}
impl ForestProjectUserYieldDistribution {
    pub fn list_by_user_id(
        conn: &mut DbConn,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_user_yield_distributions::dsl::*;

        let total_count = forest_project_user_yield_distributions
            .filter(cognito_user_id.eq(user_id))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_user_yield_distributions
            .filter(cognito_user_id.eq(user_id))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::token_metadatas)]
#[diesel(primary_key(contract_address, token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenMetadata {
    pub contract_address: Decimal,
    pub token_id:         Decimal,
    pub symbol:           Option<String>,
    pub decimals:         Option<i32>,
}

impl TokenMetadata {
    pub fn update(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::token_metadatas::dsl::*;
        diesel::update(
            token_metadatas
                .filter(contract_address.eq(self.contract_address))
                .filter(token_id.eq(self.token_id)),
        )
        .set(self)
        .get_result(conn)
    }

    pub fn create(&self, conn: &mut DbConn) -> QueryResult<Self> {
        use crate::schema::token_metadatas::dsl::*;
        diesel::insert_into(token_metadatas)
            .values(self)
            .get_result(conn)
    }

    pub fn find(
        conn: &mut DbConn,
        contract_addr: Decimal,
        metadata_token_id: Decimal,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::token_metadatas::dsl::*;
        token_metadatas
            .filter(contract_address.eq(contract_addr))
            .filter(token_id.eq(metadata_token_id))
            .first(conn)
            .optional()
    }

    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::token_metadatas::dsl::*;

        let total_count = token_metadatas.count().get_result::<i64>(conn)?;

        let records = token_metadatas
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }

    pub fn delete(
        conn: &mut DbConn,
        contract_addr: Decimal,
        metadata_token_id: Decimal,
    ) -> QueryResult<usize> {
        use crate::schema::token_metadatas::dsl::*;
        diesel::delete(
            token_metadatas
                .filter(contract_address.eq(contract_addr))
                .filter(token_id.eq(metadata_token_id)),
        )
        .execute(conn)
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_token_contract_user_balance_agg)]
#[diesel(primary_key(forest_project_id, cognito_user_id, contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectTokenContractUserBalanceAgg {
    pub forest_project_id:    Uuid,
    pub forest_project_state: ForestProjectState,
    pub forest_project_name:  String,
    pub cognito_user_id:      String,
    pub contract_address:     Decimal,
    pub contract_type:        SecurityTokenContractType,
    pub token_symbol:         String,
    pub token_decimals:       i32,
    pub total_balance:        Decimal,
    pub un_frozen_balance:    Decimal,
}

impl ForestProjectTokenContractUserBalanceAgg {
    pub fn list_by_user_id(
        conn: &mut DbConn,
        user_id: &str,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema_manual::forest_project_token_contract_user_balance_agg::dsl::*;

        let total_count = forest_project_token_contract_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(total_balance.gt(Decimal::ZERO))
            .count()
            .get_result::<i64>(conn)?;

        let records = forest_project_token_contract_user_balance_agg
            .filter(cognito_user_id.eq(user_id))
            .filter(total_balance.gt(Decimal::ZERO))
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::forest_project_token_contract_user_yields)]
#[diesel(primary_key(
    forest_project_id,
    token_contract_address,
    cognito_user_id,
    yielder_contract_address,
    yield_token_id,
    yield_contract_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectTokenContractUserYields {
    pub forest_project_id:        Uuid,
    pub token_contract_address:   Decimal,
    pub token_symbol:             String,
    pub token_decimals:           i32,
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub yield_token_id:           Decimal,
    pub yield_contract_address:   Decimal,
    pub yield_token_symbol:       String,
    pub yield_token_decimals:     i32,
    pub yield_amount:             Decimal,
}

impl ForestProjectTokenContractUserYields {
    pub fn list_by_forest_project_ids(
        conn: &mut DbConn,
        user_id: &str,
        project_ids: &[Uuid],
        yielder_address: Decimal,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_token_contract_user_yields::dsl::*;

        let records = forest_project_token_contract_user_yields
            .filter(cognito_user_id.eq(user_id))
            .filter(forest_project_id.eq_any(project_ids))
            .filter(yielder_contract_address.eq(yielder_address))
            .load::<Self>(conn)?;

        Ok(records)
    }
}
