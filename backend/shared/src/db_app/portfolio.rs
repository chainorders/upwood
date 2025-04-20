use std::collections::BTreeMap;

use chrono::NaiveDateTime;
use diesel::dsl::sum;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::forest_project::ForestProjectState;
use super::forest_project_crypto::prelude::SecurityTokenContractType;
use crate::db::cis2_security::TokenHolderBalanceUpdate;
use crate::db::security_mint_fund::InvestmentRecordType;
use crate::db_shared::DbConn;
use crate::schema::{
    forest_project_token_contracts, forest_projects, security_mint_fund_investment_records,
    security_p2p_exchange_records,
};

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

pub fn portfolio_value_at(
    conn: &mut DbConn,
    account_address: &str,
    curr_token_id: Decimal,
    curr_token_contract_address: Decimal,
    forest_project_states: &[ForestProjectState],
    token_contract_types: &[SecurityTokenContractType],
    at: NaiveDateTime,
) -> QueryResult<(Decimal, Decimal)> {
    use crate::schema::{
        cis2_token_holder_balance_updates, forest_project_prices, forest_project_token_contracts,
        forest_projects,
    };

    let balance_updates = cis2_token_holder_balance_updates::table
        .inner_join(
            forest_project_token_contracts::table
                .on(forest_project_token_contracts::contract_address
                    .eq(cis2_token_holder_balance_updates::cis2_address)),
        )
        .inner_join(
            forest_projects::table
                .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
        )
        .select((TokenHolderBalanceUpdate::as_select(), forest_projects::id))
        .filter(
            cis2_token_holder_balance_updates::holder_address
                .eq(account_address)
                .and(cis2_token_holder_balance_updates::create_time.le(at))
                .and(forest_projects::state.eq_any(forest_project_states))
                .and(forest_project_token_contracts::contract_type.eq_any(token_contract_types)),
        )
        .distinct_on((
            cis2_token_holder_balance_updates::holder_address,
            cis2_token_holder_balance_updates::cis2_address,
            cis2_token_holder_balance_updates::token_id,
        ))
        .order_by((
            cis2_token_holder_balance_updates::holder_address,
            cis2_token_holder_balance_updates::cis2_address,
            cis2_token_holder_balance_updates::token_id,
            cis2_token_holder_balance_updates::create_time.desc(),
        ))
        .load::<(TokenHolderBalanceUpdate, uuid::Uuid)>(conn)?;

    let prices = forest_project_prices::table
        .filter(
            forest_project_prices::currency_token_id
                .eq(curr_token_id)
                .and(
                    forest_project_prices::currency_token_contract_address
                        .eq(curr_token_contract_address),
                ),
        )
        .filter(forest_project_prices::price_at.le(at))
        .distinct_on(forest_project_prices::project_id)
        .order_by((
            forest_project_prices::project_id,
            forest_project_prices::price_at.desc(),
        ))
        .select((
            forest_project_prices::project_id,
            forest_project_prices::price,
        ))
        .load::<(uuid::Uuid, Decimal)>(conn)?
        .into_iter()
        .collect::<BTreeMap<uuid::Uuid, Decimal>>();

    let mut total_value_un_frozen = Decimal::ZERO;
    let mut total_value_frozen = Decimal::ZERO;
    for (balance_update, project_id) in balance_updates {
        if let Some(price) = prices.get(&project_id) {
            total_value_un_frozen += price * balance_update.un_frozen_balance;
            total_value_frozen += price * balance_update.frozen_balance;
        }
    }

    Ok((total_value_un_frozen, total_value_frozen))
}

pub fn total_invested_value_till(
    conn: &mut DbConn,
    account_address: &str,
    curr_token_id: Decimal,
    curr_token_contract_address: Decimal,
    forest_project_states: &[ForestProjectState],
    token_contract_types: &[SecurityTokenContractType],
    at: NaiveDateTime,
) -> QueryResult<(Decimal, Decimal)> {
    let (funds_invested, funds_locked) = total_invested_value_via_funds(
        conn,
        account_address,
        curr_token_id,
        curr_token_contract_address,
        forest_project_states,
        token_contract_types,
        at,
    )?;
    let market_invested = total_invested_value_via_markets(
        conn,
        account_address,
        curr_token_id,
        curr_token_contract_address,
        forest_project_states,
        token_contract_types,
        at,
    )?;
    let invested = funds_invested + market_invested;
    Ok((invested, funds_locked))
}

fn total_invested_value_via_markets(
    conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    account_address: &str,
    curr_token_id: Decimal,
    curr_token_contract_address: Decimal,
    forest_project_states: &[ForestProjectState],
    token_contract_types: &[SecurityTokenContractType],
    at: NaiveDateTime,
) -> QueryResult<Decimal> {
    let buy_value = security_p2p_exchange_records::table
        .inner_join(
            forest_project_token_contracts::table
                .on(forest_project_token_contracts::contract_address
                    .eq(security_p2p_exchange_records::token_contract_address)),
        )
        .inner_join(
            forest_projects::table
                .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
        )
        .filter(
            security_p2p_exchange_records::buyer
                .eq(account_address)
                .and(security_p2p_exchange_records::currency_token_id.eq(curr_token_id))
                .and(
                    security_p2p_exchange_records::currency_token_contract_address
                        .eq(curr_token_contract_address),
                )
                .and(security_p2p_exchange_records::create_time.le(at)),
        )
        .filter(
            forest_projects::state
                .eq_any(forest_project_states)
                .and(forest_project_token_contracts::contract_type.eq_any(token_contract_types)),
        )
        .select(sum(security_p2p_exchange_records::currency_amount))
        .first::<Option<Decimal>>(conn)?
        .unwrap_or(Decimal::ZERO);
    let sell_value = security_p2p_exchange_records::table
        .inner_join(
            forest_project_token_contracts::table
                .on(forest_project_token_contracts::contract_address
                    .eq(security_p2p_exchange_records::token_contract_address)),
        )
        .inner_join(
            forest_projects::table
                .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
        )
        .filter(
            security_p2p_exchange_records::seller
                .eq(account_address)
                .and(security_p2p_exchange_records::currency_token_id.eq(curr_token_id))
                .and(
                    security_p2p_exchange_records::currency_token_contract_address
                        .eq(curr_token_contract_address),
                )
                .and(security_p2p_exchange_records::create_time.le(at)),
        )
        .filter(
            forest_projects::state
                .eq_any(forest_project_states)
                .and(forest_project_token_contracts::contract_type.eq_any(token_contract_types)),
        )
        .select(sum(security_p2p_exchange_records::currency_amount))
        .first::<Option<Decimal>>(conn)?
        .unwrap_or(Decimal::ZERO);

    Ok(buy_value - sell_value)
}

fn total_invested_value_via_funds(
    conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    account_address: &str,
    curr_token_id: Decimal,
    curr_token_contract_address: Decimal,
    forest_project_states: &[ForestProjectState],
    token_contract_types: &[SecurityTokenContractType],
    at: NaiveDateTime,
) -> QueryResult<(Decimal, Decimal)> {
    let invested = security_mint_fund_investment_records::table
        .inner_join(
            forest_project_token_contracts::table
                .on(forest_project_token_contracts::contract_address
                    .eq(security_mint_fund_investment_records::investment_token_contract_address)),
        )
        .inner_join(
            forest_projects::table
                .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
        )
        .filter(
            security_mint_fund_investment_records::investment_record_type
                .eq(InvestmentRecordType::Invested),
        )
        .filter(
            security_mint_fund_investment_records::investor
                .eq(account_address)
                .and(security_mint_fund_investment_records::currency_token_id.eq(curr_token_id))
                .and(
                    security_mint_fund_investment_records::currency_token_contract_address
                        .eq(curr_token_contract_address),
                )
                .and(security_mint_fund_investment_records::create_time.le(at)),
        )
        .filter(
            forest_projects::state
                .eq_any(forest_project_states)
                .and(forest_project_token_contracts::contract_type.eq_any(token_contract_types)),
        )
        .select(sum(security_mint_fund_investment_records::currency_amount))
        .first::<Option<Decimal>>(conn)?
        .unwrap_or(Decimal::ZERO);
    let claimed = security_mint_fund_investment_records::table
        .inner_join(
            forest_project_token_contracts::table
                .on(forest_project_token_contracts::contract_address
                    .eq(security_mint_fund_investment_records::investment_token_contract_address)),
        )
        .inner_join(
            forest_projects::table
                .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
        )
        .filter(
            security_mint_fund_investment_records::investment_record_type
                .eq(InvestmentRecordType::Claimed),
        )
        .filter(
            security_mint_fund_investment_records::investor
                .eq(account_address)
                .and(security_mint_fund_investment_records::currency_token_id.eq(curr_token_id))
                .and(
                    security_mint_fund_investment_records::currency_token_contract_address
                        .eq(curr_token_contract_address),
                )
                .and(security_mint_fund_investment_records::create_time.le(at)),
        )
        .filter(
            forest_projects::state
                .eq_any(forest_project_states)
                .and(forest_project_token_contracts::contract_type.eq_any(token_contract_types)),
        )
        .select(sum(security_mint_fund_investment_records::currency_amount))
        .first::<Option<Decimal>>(conn)?
        .unwrap_or(Decimal::ZERO);
    let cancelled = security_mint_fund_investment_records::table
        .inner_join(
            forest_project_token_contracts::table
                .on(forest_project_token_contracts::contract_address
                    .eq(security_mint_fund_investment_records::investment_token_contract_address)),
        )
        .inner_join(
            forest_projects::table
                .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
        )
        .filter(
            security_mint_fund_investment_records::investment_record_type
                .eq(InvestmentRecordType::Cancelled),
        )
        .filter(
            security_mint_fund_investment_records::investor
                .eq(account_address)
                .and(security_mint_fund_investment_records::currency_token_id.eq(curr_token_id))
                .and(
                    security_mint_fund_investment_records::currency_token_contract_address
                        .eq(curr_token_contract_address),
                )
                .and(security_mint_fund_investment_records::create_time.le(at)),
        )
        .filter(
            forest_projects::state
                .eq_any(forest_project_states)
                .and(forest_project_token_contracts::contract_type.eq_any(token_contract_types)),
        )
        .select(sum(security_mint_fund_investment_records::currency_amount))
        .first::<Option<Decimal>>(conn)?
        .unwrap_or(Decimal::ZERO);

    let invested = invested - cancelled;
    let locked = invested - claimed;
    Ok((invested, locked))
}
