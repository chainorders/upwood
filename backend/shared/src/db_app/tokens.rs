use chrono::NaiveDateTime;
use diesel::dsl::Nullable;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult};
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::forest_project::ForestProjectState;
use super::forest_project_crypto::prelude::SecurityTokenContractType;
use crate::db::cis2_security::TokenHolderBalanceUpdateType;
use crate::db::security_mint_fund::InvestmentRecordType;
use crate::db::security_p2p_trading::ExchangeRecordType;
use crate::db::txn_listener::ProcessorType;
use crate::db_shared::DbConn;
use crate::schema::{
    cis2_compliances, cis2_identity_registries,
    cis2_token_holder_balance_updates as balance_updates, cis2_token_holders as holders,
    forest_project_token_contracts, forest_projects, listener_contracts,
    security_mint_fund_investment_records as investment_records,
    security_mint_fund_investors as investors, security_p2p_exchange_records as exchange_records,
    security_p2p_trading_traders as traders,
    security_sft_multi_yielder_yeild_distributions as yield_distributions, users,
};

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct TokenContract {
    pub contract_address:    Decimal,
    pub module_ref:          String,
    pub contract_name:       String,
    pub owner:               String,
    pub created_at:          NaiveDateTime,
    pub identity_registry:   Option<Decimal>,
    pub compliance_contract: Option<String>,
}

impl<DB> Selectable<DB> for TokenContract
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        listener_contracts::contract_address,
        listener_contracts::module_ref,
        listener_contracts::contract_name,
        listener_contracts::owner,
        listener_contracts::created_at,
        Nullable<cis2_identity_registries::identity_registry_address>,
        Nullable<cis2_compliances::compliance_address>,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            listener_contracts::contract_address,
            listener_contracts::module_ref,
            listener_contracts::contract_name,
            listener_contracts::owner,
            listener_contracts::created_at,
            cis2_identity_registries::identity_registry_address.nullable(),
            cis2_compliances::compliance_address.nullable(),
        )
    }
}

impl TokenContract {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Option<Decimal>,
        module_ref: Option<&str>,
        contract_name: Option<&str>,
        owner: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = listener_contracts::table
            .left_join(cis2_identity_registries::table.on(
                listener_contracts::contract_address.eq(cis2_identity_registries::cis2_address),
            ))
            .left_join(
                cis2_compliances::table
                    .on(listener_contracts::contract_address.eq(cis2_compliances::cis2_address)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        query = query
            .filter(listener_contracts::processor_type.eq_any(token_contract_processor_types()));
        count_query = count_query
            .filter(listener_contracts::processor_type.eq_any(token_contract_processor_types()));
        if let Some(contract_address) = contract_address {
            query = query.filter(listener_contracts::contract_address.eq(contract_address));
            count_query =
                count_query.filter(listener_contracts::contract_address.eq(contract_address));
        }
        if let Some(module_ref) = module_ref {
            query = query.filter(listener_contracts::module_ref.eq(module_ref));
            count_query = count_query.filter(listener_contracts::module_ref.eq(module_ref));
        }
        if let Some(contract_name) = contract_name {
            query = query.filter(listener_contracts::contract_name.eq(contract_name));
            count_query = count_query.filter(listener_contracts::contract_name.eq(contract_name));
        }
        if let Some(owner) = owner {
            query = query.filter(listener_contracts::owner.eq(owner));
            count_query = count_query.filter(listener_contracts::owner.eq(owner));
        }
        let total_count = count_query
            .select(diesel::dsl::count(listener_contracts::contract_address))
            .first::<i64>(conn)?;
        let records = query
            .select(TokenContract::as_select())
            .order(listener_contracts::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;

        Ok((records, total_count))
    }

    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> QueryResult<Option<Self>> {
        let query = listener_contracts::table
            .left_join(
                cis2_identity_registries::table.on(listener_contracts::contract_address
                    .eq(cis2_identity_registries::identity_registry_address)),
            )
            .left_join(
                cis2_compliances::table
                    .on(listener_contracts::contract_address.eq(cis2_compliances::cis2_address)),
            )
            .filter(listener_contracts::contract_address.eq(contract_address))
            .filter(listener_contracts::processor_type.eq_any(token_contract_processor_types()));
        let record = query
            .select(TokenContract::as_select())
            .first(conn)
            .optional()?;
        Ok(record)
    }
}

fn token_contract_processor_types() -> Vec<ProcessorType> {
    vec![
        ProcessorType::SecuritySftSingle,
        ProcessorType::SecuritySftMulti,
        ProcessorType::NftMultiRewarded,
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct TokenHolderUser {
    pub cis2_address: Decimal,
    pub token_id: Decimal,
    pub holder_address: String,
    pub frozen_balance: Decimal,
    pub un_frozen_balance: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub cognito_user_id: Option<String>,
    pub email: Option<String>,
    pub forest_project_id: Option<Uuid>,
    pub forest_project_name: Option<String>,
    pub forest_project_contract_type: Option<SecurityTokenContractType>,
}

impl<DB> Selectable<DB> for TokenHolderUser
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        holders::cis2_address,
        holders::token_id,
        holders::holder_address,
        holders::frozen_balance,
        holders::un_frozen_balance,
        holders::create_time,
        holders::update_time,
        Nullable<users::cognito_user_id>,
        Nullable<users::email>,
        Nullable<forest_projects::id>,
        Nullable<forest_projects::name>,
        Nullable<forest_project_token_contracts::contract_type>,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            holders::cis2_address,
            holders::token_id,
            holders::holder_address,
            holders::frozen_balance,
            holders::un_frozen_balance,
            holders::create_time,
            holders::update_time,
            users::cognito_user_id.nullable(),
            users::email.nullable(),
            forest_projects::id.nullable(),
            forest_projects::name.nullable(),
            forest_project_token_contracts::contract_type.nullable(),
        )
    }
}

impl TokenHolderUser {
    pub fn list(
        conn: &mut DbConn,
        forest_project_id: Option<Uuid>,
        cis2_address: Option<Decimal>,
        token_id: Option<Decimal>,
        holder_address: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = holders::table
            .left_join(users::table.on(holders::holder_address.eq(users::account_address)))
            .left_join(
                forest_project_token_contracts::table
                    .on(holders::cis2_address.eq(forest_project_token_contracts::contract_address)),
            )
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();

        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }
        if let Some(cis2_address) = cis2_address {
            query = query.filter(holders::cis2_address.eq(cis2_address));
            count_query = count_query.filter(holders::cis2_address.eq(cis2_address));
        }
        if let Some(token_id) = token_id {
            query = query.filter(holders::token_id.eq(token_id));
            count_query = count_query.filter(holders::token_id.eq(token_id));
        }
        if let Some(holder_address) = holder_address {
            query = query.filter(holders::holder_address.eq(holder_address));
            count_query = count_query.filter(holders::holder_address.eq(holder_address));
        }

        let total_count = count_query
            .select(diesel::dsl::count(holders::holder_address))
            .first::<i64>(conn)?;
        let records = query
            .select(TokenHolderUser::as_select())
            .order(holders::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }

    pub fn find(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &str,
    ) -> QueryResult<Option<Self>> {
        let query = holders::table
            .left_join(users::table.on(holders::holder_address.eq(users::account_address)))
            .left_join(
                forest_project_token_contracts::table
                    .on(holders::cis2_address.eq(forest_project_token_contracts::contract_address)),
            )
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            )
            .filter(holders::cis2_address.eq(cis2_address))
            .filter(holders::token_id.eq(token_id))
            .filter(holders::holder_address.eq(holder_address));
        let record = query
            .select(TokenHolderUser::as_select())
            .first(conn)
            .optional()?;
        Ok(record)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct TokenHolderUserBalanceUpdate {
    pub id: uuid::Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub cis2_address: Decimal,
    pub token_id: Decimal,
    pub holder_address: String,
    pub amount: Decimal,
    pub frozen_balance: Decimal,
    pub un_frozen_balance: Decimal,
    pub txn_sender: String,
    pub txn_instigator: String,
    pub update_type: TokenHolderBalanceUpdateType,
    pub create_time: NaiveDateTime,
    pub cognito_user_id: Option<String>,
    pub email: Option<String>,
    pub forest_project_id: Option<Uuid>,
    pub forest_project_name: Option<String>,
    pub forest_project_contract_type: Option<SecurityTokenContractType>,
}

impl<DB> Selectable<DB> for TokenHolderUserBalanceUpdate
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        balance_updates::id,
        balance_updates::block_height,
        balance_updates::txn_index,
        balance_updates::cis2_address,
        balance_updates::token_id,
        balance_updates::holder_address,
        balance_updates::amount,
        balance_updates::frozen_balance,
        balance_updates::un_frozen_balance,
        balance_updates::txn_sender,
        balance_updates::txn_instigator,
        balance_updates::update_type,
        balance_updates::create_time,
        Nullable<users::cognito_user_id>,
        Nullable<users::email>,
        Nullable<forest_projects::id>,
        Nullable<forest_projects::name>,
        Nullable<forest_project_token_contracts::contract_type>,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            balance_updates::id,
            balance_updates::block_height,
            balance_updates::txn_index,
            balance_updates::cis2_address,
            balance_updates::token_id,
            balance_updates::holder_address,
            balance_updates::amount,
            balance_updates::frozen_balance,
            balance_updates::un_frozen_balance,
            balance_updates::txn_sender,
            balance_updates::txn_instigator,
            balance_updates::update_type,
            balance_updates::create_time,
            users::cognito_user_id.nullable(),
            users::email.nullable(),
            forest_projects::id.nullable(),
            forest_projects::name.nullable(),
            forest_project_token_contracts::contract_type.nullable(),
        )
    }
}

impl TokenHolderUserBalanceUpdate {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        forest_project_id: Option<Uuid>,
        cis2_address: Option<Decimal>,
        token_id: Option<Decimal>,
        holder_address: Option<&str>,
        update_type: Option<TokenHolderBalanceUpdateType>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = balance_updates::table
            .left_join(users::table.on(balance_updates::holder_address.eq(users::account_address)))
            .left_join(forest_project_token_contracts::table.on(
                balance_updates::cis2_address.eq(forest_project_token_contracts::contract_address),
            ))
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();

        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }
        if let Some(cis2_address) = cis2_address {
            query = query.filter(balance_updates::cis2_address.eq(cis2_address));
            count_query = count_query.filter(balance_updates::cis2_address.eq(cis2_address));
        }
        if let Some(token_id) = token_id {
            query = query.filter(balance_updates::token_id.eq(token_id));
            count_query = count_query.filter(balance_updates::token_id.eq(token_id));
        }
        if let Some(holder_address) = holder_address {
            query = query.filter(balance_updates::holder_address.eq(holder_address));
            count_query = count_query.filter(balance_updates::holder_address.eq(holder_address));
        }
        if let Some(update_type) = update_type {
            query = query.filter(balance_updates::update_type.eq(update_type));
            count_query = count_query.filter(balance_updates::update_type.eq(update_type));
        }

        let total_count = count_query
            .select(diesel::dsl::count(balance_updates::id))
            .first::<i64>(conn)?;
        let records = query
            .select(TokenHolderUserBalanceUpdate::as_select())
            .order(balance_updates::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct InvestorUser {
    pub contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub investor: String,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub currency_amount: Decimal,
    pub currency_amount_total: Decimal,
    pub token_amount: Decimal,
    pub token_amount_total: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub cognito_user_id: Option<String>,
    pub email: Option<String>,
    pub forest_project_id: Option<Uuid>,
    pub forest_project_name: Option<String>,
    pub forest_project_contract_type: Option<SecurityTokenContractType>,
}

impl<DB> Selectable<DB> for InvestorUser
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        investors::contract_address,
        investors::investment_token_id,
        investors::investment_token_contract_address,
        investors::investor,
        investors::currency_token_id,
        investors::currency_token_contract_address,
        investors::currency_amount,
        investors::currency_amount_total,
        investors::token_amount,
        investors::token_amount_total,
        investors::create_time,
        investors::update_time,
        Nullable<users::cognito_user_id>,
        Nullable<users::email>,
        Nullable<forest_projects::id>,
        Nullable<forest_projects::name>,
        Nullable<forest_project_token_contracts::contract_type>,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            investors::contract_address,
            investors::investment_token_id,
            investors::investment_token_contract_address,
            investors::investor,
            investors::currency_token_id,
            investors::currency_token_contract_address,
            investors::currency_amount,
            investors::currency_amount_total,
            investors::token_amount,
            investors::token_amount_total,
            investors::create_time,
            investors::update_time,
            users::cognito_user_id.nullable(),
            users::email.nullable(),
            forest_projects::id.nullable(),
            forest_projects::name.nullable(),
            forest_project_token_contracts::contract_type.nullable(),
        )
    }
}

impl InvestorUser {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        forest_project_id: Option<Uuid>,
        investment_contract_address: Option<Decimal>,
        investment_token_id: Option<Decimal>,
        investor: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = investors::table
            .left_join(users::table.on(investors::investor.eq(users::account_address)))
            .left_join(
                forest_project_token_contracts::table
                    .on(investors::investment_token_contract_address
                        .eq(forest_project_token_contracts::contract_address)),
            )
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        query = query.filter(investors::contract_address.eq(contract_address));
        count_query = count_query.filter(investors::contract_address.eq(contract_address));

        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }
        if let Some(investment_contract_address) = investment_contract_address {
            query = query.filter(
                investors::investment_token_contract_address.eq(investment_contract_address),
            );
            count_query = count_query.filter(
                investors::investment_token_contract_address.eq(investment_contract_address),
            );
        }
        if let Some(investment_token_id) = investment_token_id {
            query = query.filter(investors::investment_token_id.eq(investment_token_id));
            count_query =
                count_query.filter(investors::investment_token_id.eq(investment_token_id));
        }
        if let Some(investor) = investor {
            query = query.filter(investors::investor.eq(investor));
            count_query = count_query.filter(investors::investor.eq(investor));
        }

        let total_count = count_query
            .select(diesel::dsl::count(investors::investor))
            .first::<i64>(conn)?;
        let records = query
            .select(InvestorUser::as_select())
            .order(investors::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct UserInvestmentRecord {
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub contract_address: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub investor: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub currency_amount_balance: Decimal,
    pub token_amount_balance: Decimal,
    pub investment_record_type: InvestmentRecordType,
    pub create_time: NaiveDateTime,
    pub cognito_user_id: Option<String>,
    pub email: Option<String>,
    pub forest_project_id: Option<Uuid>,
    pub forest_project_name: Option<String>,
    pub forest_project_contract_type: Option<SecurityTokenContractType>,
}

impl<DB> Selectable<DB> for UserInvestmentRecord
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        investment_records::id,
        investment_records::block_height,
        investment_records::txn_index,
        investment_records::contract_address,
        investment_records::investment_token_id,
        investment_records::investment_token_contract_address,
        investment_records::currency_token_id,
        investment_records::currency_token_contract_address,
        investment_records::investor,
        investment_records::currency_amount,
        investment_records::token_amount,
        investment_records::currency_amount_balance,
        investment_records::token_amount_balance,
        investment_records::investment_record_type,
        investment_records::create_time,
        Nullable<users::cognito_user_id>,
        Nullable<users::email>,
        Nullable<forest_projects::id>,
        Nullable<forest_projects::name>,
        Nullable<forest_project_token_contracts::contract_type>,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            investment_records::id,
            investment_records::block_height,
            investment_records::txn_index,
            investment_records::contract_address,
            investment_records::investment_token_id,
            investment_records::investment_token_contract_address,
            investment_records::currency_token_id,
            investment_records::currency_token_contract_address,
            investment_records::investor,
            investment_records::currency_amount,
            investment_records::token_amount,
            investment_records::currency_amount_balance,
            investment_records::token_amount_balance,
            investment_records::investment_record_type,
            investment_records::create_time,
            users::cognito_user_id.nullable(),
            users::email.nullable(),
            forest_projects::id.nullable(),
            forest_projects::name.nullable(),
            forest_project_token_contracts::contract_type.nullable(),
        )
    }
}

impl UserInvestmentRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        forest_project_id: Option<Uuid>,
        investment_contract_address: Option<Decimal>,
        investment_token_id: Option<Decimal>,
        investor: Option<&str>,
        investment_record_type: Option<InvestmentRecordType>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = investment_records::table
            .left_join(users::table.on(investment_records::investor.eq(users::account_address)))
            .left_join(
                forest_project_token_contracts::table
                    .on(investment_records::investment_token_contract_address
                        .eq(forest_project_token_contracts::contract_address)),
            )
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        query = query.filter(investment_records::contract_address.eq(contract_address));
        count_query = count_query.filter(investment_records::contract_address.eq(contract_address));

        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }
        if let Some(investment_contract_address) = investment_contract_address {
            query = query.filter(
                investment_records::investment_token_contract_address
                    .eq(investment_contract_address),
            );
            count_query = count_query.filter(
                investment_records::investment_token_contract_address
                    .eq(investment_contract_address),
            );
        }
        if let Some(investment_token_id) = investment_token_id {
            query = query.filter(investment_records::investment_token_id.eq(investment_token_id));
            count_query =
                count_query.filter(investment_records::investment_token_id.eq(investment_token_id));
        }
        if let Some(investor) = investor {
            query = query.filter(investment_records::investor.eq(investor));
            count_query = count_query.filter(investment_records::investor.eq(investor));
        }
        if let Some(investment_record_type) = investment_record_type {
            query =
                query.filter(investment_records::investment_record_type.eq(investment_record_type));
            count_query = count_query
                .filter(investment_records::investment_record_type.eq(investment_record_type));
        }

        let total_count = count_query
            .select(diesel::dsl::count(investment_records::id))
            .first::<i64>(conn)?;
        let records = query
            .select(UserInvestmentRecord::as_select())
            .order(investment_records::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct TraderUser {
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub trader: String,
    pub token_in_amount: Decimal,
    pub token_out_amount: Decimal,
    pub currency_in_amount: Decimal,
    pub currency_out_amount: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub cognito_user_id: Option<String>,
    pub email: Option<String>,
    pub forest_project_id: Option<Uuid>,
    pub forest_project_name: Option<String>,
    pub forest_project_contract_type: Option<SecurityTokenContractType>,
}

impl<DB> Selectable<DB> for TraderUser
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        traders::contract_address,
        traders::token_id,
        traders::token_contract_address,
        traders::currency_token_id,
        traders::currency_token_contract_address,
        traders::trader,
        traders::token_in_amount,
        traders::token_out_amount,
        traders::currency_in_amount,
        traders::currency_out_amount,
        traders::create_time,
        traders::update_time,
        Nullable<users::cognito_user_id>,
        Nullable<users::email>,
        Nullable<forest_projects::id>,
        Nullable<forest_projects::name>,
        Nullable<forest_project_token_contracts::contract_type>,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            traders::contract_address,
            traders::token_id,
            traders::token_contract_address,
            traders::currency_token_id,
            traders::currency_token_contract_address,
            traders::trader,
            traders::token_in_amount,
            traders::token_out_amount,
            traders::currency_in_amount,
            traders::currency_out_amount,
            traders::create_time,
            traders::update_time,
            users::cognito_user_id.nullable(),
            users::email.nullable(),
            forest_projects::id.nullable(),
            forest_projects::name.nullable(),
            forest_project_token_contracts::contract_type.nullable(),
        )
    }
}

impl TraderUser {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        forest_project_id: Option<Uuid>,
        token_contract_address: Option<Decimal>,
        token_id: Option<Decimal>,
        trader: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = traders::table
            .left_join(users::table.on(traders::trader.eq(users::account_address)))
            .left_join(
                forest_project_token_contracts::table.on(traders::token_contract_address
                    .eq(forest_project_token_contracts::contract_address)),
            )
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        query = query.filter(traders::contract_address.eq(contract_address));
        count_query = count_query.filter(traders::contract_address.eq(contract_address));
        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }
        if let Some(token_contract_address) = token_contract_address {
            query = query.filter(traders::token_contract_address.eq(token_contract_address));
            count_query =
                count_query.filter(traders::token_contract_address.eq(token_contract_address));
        }
        if let Some(token_id) = token_id {
            query = query.filter(traders::token_id.eq(token_id));
            count_query = count_query.filter(traders::token_id.eq(token_id));
        }
        if let Some(trader) = trader {
            query = query.filter(traders::trader.eq(trader));
            count_query = count_query.filter(traders::trader.eq(trader));
        }
        let total_count = count_query
            .select(diesel::dsl::count(traders::trader))
            .first::<i64>(conn)?;
        let records = query
            .select(TraderUser::as_select())
            .order(traders::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct UserExchangeRecord {
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub token_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub seller: String,
    pub buyer: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
    pub rate: Decimal,
    pub create_time: NaiveDateTime,
    pub exchange_record_type: ExchangeRecordType,
    pub buyer_cognito_user_id: Option<String>,
    pub buyer_email: Option<String>,
    pub seller_cognito_user_id: Option<String>,
    pub seller_email: Option<String>,
    pub forest_project_id: Option<Uuid>,
    pub forest_project_name: Option<String>,
    pub forest_project_contract_type: Option<SecurityTokenContractType>,
}

impl UserExchangeRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        forest_project_id: Option<Uuid>,
        token_contract_address: Option<Decimal>,
        token_id: Option<Decimal>,
        trader: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let (buyers, sellers) = diesel::alias!(
            crate::schema::users as buyers,
            crate::schema::users as sellers
        );
        let query = exchange_records::table
            .left_join(buyers.on(exchange_records::buyer.eq(buyers.field(users::account_address))))
            .left_join(
                sellers.on(exchange_records::seller.eq(sellers.field(users::account_address))),
            )
            .left_join(
                forest_project_token_contracts::table.on(exchange_records::token_contract_address
                    .eq(forest_project_token_contracts::contract_address)),
            )
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        query = query.filter(exchange_records::contract_address.eq(contract_address));
        count_query = count_query.filter(exchange_records::contract_address.eq(contract_address));
        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }
        if let Some(token_contract_address) = token_contract_address {
            query =
                query.filter(exchange_records::token_contract_address.eq(token_contract_address));
            count_query = count_query
                .filter(exchange_records::token_contract_address.eq(token_contract_address));
        }
        if let Some(token_id) = token_id {
            query = query.filter(exchange_records::token_id.eq(token_id));
            count_query = count_query.filter(exchange_records::token_id.eq(token_id));
        }
        if let Some(trader) = trader {
            query = query.filter(
                exchange_records::buyer
                    .eq(trader)
                    .or(exchange_records::seller.eq(trader)),
            );
            count_query = count_query.filter(
                exchange_records::buyer
                    .eq(trader)
                    .or(exchange_records::seller.eq(trader)),
            );
        }
        let total_count = count_query
            .select(diesel::dsl::count(exchange_records::id))
            .first::<i64>(conn)?;
        let records = query
            .select((
                exchange_records::id,
                exchange_records::block_height,
                exchange_records::txn_index,
                exchange_records::contract_address,
                exchange_records::token_id,
                exchange_records::token_contract_address,
                exchange_records::currency_token_id,
                exchange_records::currency_token_contract_address,
                exchange_records::seller,
                exchange_records::buyer,
                exchange_records::currency_amount,
                exchange_records::token_amount,
                exchange_records::rate,
                exchange_records::create_time,
                exchange_records::exchange_record_type,
                buyers.field(users::cognito_user_id).nullable(),
                buyers.field(users::email).nullable(),
                sellers.field(users::cognito_user_id).nullable(),
                sellers.field(users::email).nullable(),
                forest_projects::id.nullable(),
                forest_projects::name.nullable(),
                forest_project_token_contracts::contract_type.nullable(),
            ))
            .order(exchange_records::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct UserYieldDistribution {
    #[diesel(select_expression = yield_distributions::id)]
    pub id: uuid::Uuid,
    pub contract_address: Decimal,
    pub token_contract_address: Decimal,
    pub from_token_version: Decimal,
    pub to_token_version: Decimal,
    pub token_amount: Decimal,
    pub yield_contract_address: Decimal,
    pub yield_token_id: Decimal,
    pub yield_amount: Decimal,
    pub to_address: String,
    pub create_time: NaiveDateTime,
    pub cognito_user_id: Option<String>,
    pub email: Option<String>,
    pub forest_project_id: Option<Uuid>,
    pub forest_project_name: Option<String>,
    pub forest_project_contract_type: Option<SecurityTokenContractType>,
}

impl<DB> Selectable<DB> for UserYieldDistribution
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        yield_distributions::id,
        yield_distributions::contract_address,
        yield_distributions::token_contract_address,
        yield_distributions::from_token_version,
        yield_distributions::to_token_version,
        yield_distributions::token_amount,
        yield_distributions::yield_contract_address,
        yield_distributions::yield_token_id,
        yield_distributions::yield_amount,
        yield_distributions::to_address,
        yield_distributions::create_time,
        Nullable<users::cognito_user_id>,
        Nullable<users::email>,
        Nullable<forest_projects::id>,
        Nullable<forest_projects::name>,
        Nullable<forest_project_token_contracts::contract_type>,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            yield_distributions::id,
            yield_distributions::contract_address,
            yield_distributions::token_contract_address,
            yield_distributions::from_token_version,
            yield_distributions::to_token_version,
            yield_distributions::token_amount,
            yield_distributions::yield_contract_address,
            yield_distributions::yield_token_id,
            yield_distributions::yield_amount,
            yield_distributions::to_address,
            yield_distributions::create_time,
            users::cognito_user_id.nullable(),
            users::email.nullable(),
            forest_projects::id.nullable(),
            forest_projects::name.nullable(),
            forest_project_token_contracts::contract_type.nullable(),
        )
    }
}

impl UserYieldDistribution {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        contract_address: Decimal,
        forest_project_id: Option<Uuid>,
        token_contract_address: Option<Decimal>,
        to_address: Option<&str>,
        yielded_token_contract_address: Option<Decimal>,
        yielded_token_id: Option<Decimal>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = yield_distributions::table
            .left_join(
                forest_project_token_contracts::table
                    .on(yield_distributions::token_contract_address
                        .eq(forest_project_token_contracts::contract_address)),
            )
            .left_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            )
            .left_join(users::table.on(yield_distributions::to_address.eq(users::account_address)));
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        query = query.filter(yield_distributions::contract_address.eq(contract_address));
        count_query =
            count_query.filter(yield_distributions::contract_address.eq(contract_address));
        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }
        if let Some(token_contract_address) = token_contract_address {
            query = query
                .filter(yield_distributions::token_contract_address.eq(token_contract_address));
            count_query = count_query
                .filter(yield_distributions::token_contract_address.eq(token_contract_address));
        }
        if let Some(to_address) = to_address {
            query = query.filter(yield_distributions::to_address.eq(to_address));
            count_query = count_query.filter(yield_distributions::to_address.eq(to_address));
        }
        if let Some(yielded_token_contract_address) = yielded_token_contract_address {
            query = query.filter(
                yield_distributions::yield_contract_address.eq(yielded_token_contract_address),
            );
            count_query = count_query.filter(
                yield_distributions::yield_contract_address.eq(yielded_token_contract_address),
            );
        }
        if let Some(yielded_token_id) = yielded_token_id {
            query = query.filter(yield_distributions::yield_token_id.eq(yielded_token_id));
            count_query =
                count_query.filter(yield_distributions::yield_token_id.eq(yielded_token_id));
        }
        let total_count = count_query
            .select(diesel::dsl::count(yield_distributions::id))
            .first::<i64>(conn)?;
        let records = query
            .select(UserYieldDistribution::as_select())
            .order(yield_distributions::create_time.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load::<UserYieldDistribution>(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize, Queryable)]
pub struct ForestProjectContract {
    pub contract_address:    Decimal,
    pub module_ref:          String,
    pub contract_name:       String,
    pub owner:               String,
    pub identity_registry:   Option<Decimal>,
    pub compliance_contract: Option<String>,
    pub forest_project_id:   Uuid,
    pub forest_project_name: String,
    pub contract_type:       SecurityTokenContractType,
    pub fund_token_id:       Option<Decimal>,
    pub symbol:              String,
    pub decimals:            i32,
    pub metadata_url:        String,
    pub metadata_hash:       Option<String>,
    pub created_at:          chrono::NaiveDateTime,
    pub updated_at:          chrono::NaiveDateTime,
}

impl<DB> Selectable<DB> for ForestProjectContract
where DB: diesel::backend::Backend
{
    type SelectExpression = (
        listener_contracts::contract_address,
        listener_contracts::module_ref,
        listener_contracts::contract_name,
        listener_contracts::owner,
        Nullable<cis2_identity_registries::identity_registry_address>,
        Nullable<cis2_compliances::compliance_address>,
        forest_projects::id,
        forest_projects::name,
        forest_project_token_contracts::contract_type,
        Nullable<forest_project_token_contracts::fund_token_id>,
        forest_project_token_contracts::symbol,
        forest_project_token_contracts::decimals,
        forest_project_token_contracts::metadata_url,
        Nullable<forest_project_token_contracts::metadata_hash>,
        forest_project_token_contracts::created_at,
        forest_project_token_contracts::updated_at,
    );

    fn construct_selection() -> Self::SelectExpression {
        (
            listener_contracts::contract_address,
            listener_contracts::module_ref,
            listener_contracts::contract_name,
            listener_contracts::owner,
            cis2_identity_registries::identity_registry_address.nullable(),
            cis2_compliances::compliance_address.nullable(),
            forest_projects::id,
            forest_projects::name,
            forest_project_token_contracts::contract_type,
            forest_project_token_contracts::fund_token_id.nullable(),
            forest_project_token_contracts::symbol,
            forest_project_token_contracts::decimals,
            forest_project_token_contracts::metadata_url,
            forest_project_token_contracts::metadata_hash.nullable(),
            forest_project_token_contracts::created_at,
            forest_project_token_contracts::updated_at,
        )
    }
}

impl ForestProjectContract {
    pub fn list(
        conn: &mut DbConn,
        project_id: Option<Uuid>,
        contract_type: Option<SecurityTokenContractType>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = listener_contracts::table
            .inner_join(
                forest_project_token_contracts::table.on(listener_contracts::contract_address
                    .eq(forest_project_token_contracts::contract_address)),
            )
            .inner_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            )
            .left_join(cis2_identity_registries::table.on(
                listener_contracts::contract_address.eq(cis2_identity_registries::cis2_address),
            ))
            .left_join(
                cis2_compliances::table
                    .on(listener_contracts::contract_address.eq(cis2_compliances::cis2_address)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();

        query = query.filter(forest_projects::state.ne(ForestProjectState::Archived));
        count_query = count_query.filter(forest_projects::state.ne(ForestProjectState::Archived));

        if let Some(project_id) = project_id {
            query = query.filter(forest_projects::id.eq(project_id));
            count_query = count_query.filter(forest_projects::id.eq(project_id));
        }
        if let Some(contract_type) = contract_type {
            query = query.filter(forest_project_token_contracts::contract_type.eq(contract_type));
            count_query =
                count_query.filter(forest_project_token_contracts::contract_type.eq(contract_type));
        }
        let total_count = count_query
            .select(diesel::dsl::count(
                forest_project_token_contracts::contract_address,
            ))
            .first::<i64>(conn)?;
        let records = query
            .select(ForestProjectContract::as_select())
            .order(forest_project_token_contracts::forest_project_id.desc())
            .then_order_by(forest_project_token_contracts::contract_address.asc())
            .then_order_by(forest_project_token_contracts::created_at.asc())
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?;
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }

    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> QueryResult<Option<Self>> {
        let query = listener_contracts::table
            .inner_join(
                forest_project_token_contracts::table.on(listener_contracts::contract_address
                    .eq(forest_project_token_contracts::contract_address)),
            )
            .inner_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            )
            .left_join(cis2_identity_registries::table.on(
                listener_contracts::contract_address.eq(cis2_identity_registries::cis2_address),
            ))
            .left_join(
                cis2_compliances::table
                    .on(listener_contracts::contract_address.eq(cis2_compliances::cis2_address)),
            );
        let record = query
            .filter(listener_contracts::contract_address.eq(contract_address))
            .select(ForestProjectContract::as_select())
            .first(conn)
            .optional()?;
        Ok(record)
    }
}
