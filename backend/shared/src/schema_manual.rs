use crate::schema::{
    forest_project_legal_contract_user_signatures, forest_project_legal_contracts,
};

diesel::table! {
    forest_project_funds_affiliate_reward_records (investment_record_id) {
        investment_record_id -> Uuid,
        fund_contract_address -> Numeric,
        investment_token_contract_address -> Numeric,
        investment_token_id -> Numeric,
        fund_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        forest_project_id -> Uuid,
        is_default -> Nullable<Bool>,
        investor_cognito_user_id -> Varchar,
        investor_account_address -> Varchar,
        currency_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        investment_token_symbol -> Varchar,
        investment_token_decimals -> Integer,
        currency_token_symbol -> Varchar,
        currency_token_decimals -> Integer,
        claim_id -> Nullable<Uuid>,
        claims_contract_address -> Nullable<Numeric>,
        reward_amount -> Numeric,
        remaining_reward_amount -> Numeric,
        affiliate_cognito_user_id -> Varchar,
        affiliate_commission -> Numeric,
    }
}

diesel::table! {
    forest_project_token_user_yields (forest_project_id, token_id, token_contract_address, holder_address, yielder_contract_address, yield_token_id, yield_contract_address) {
        forest_project_id -> Uuid,
        token_id -> Numeric,
        token_contract_address -> Numeric,
        holder_address -> Varchar,
        token_balance -> Numeric,
        cognito_user_id -> Varchar,
        yielder_contract_address -> Numeric,
        yield_token_id -> Numeric,
        yield_contract_address -> Numeric,
        yield_amount -> Numeric,
        max_token_id -> Numeric,
        token_symbol -> Varchar,
        token_decimals -> Integer,
        yield_token_symbol -> Varchar,
        yield_token_decimals -> Integer,
    }
}

diesel::table! {
    user_yields_aggregate (cognito_user_id, yielder_contract_address, yield_token_id, yield_contract_address) {
        cognito_user_id -> Varchar,
        yielder_contract_address -> Numeric,
        yield_token_id -> Numeric,
        yield_contract_address -> Numeric,
        yield_amount -> Numeric,
    }
}

diesel::table! {
    forest_project_user_investment_amounts(cognito_user_id) {
        cognito_user_id -> Varchar,
        currency_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        total_currency_amount_locked -> Numeric,
        total_currency_amount_invested -> Numeric,
    }
}

diesel::table! {
    user_transactions (transaction_hash) {
        transaction_hash -> Varchar,
        block_height -> Numeric,
        forest_project_id -> Uuid,
        currency_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        currency_amount -> Numeric,
        cognito_user_id -> Varchar,
        transaction_type -> Varchar,
        account_address -> Varchar,
    }
}

diesel::define_sql_function!(
    fn user_currency_value_for_forest_project_owned_tokens_at(
        user_id: diesel::sql_types::Text,
        time_at: diesel::sql_types::Timestamp,
        currency_token_id: diesel::sql_types::Numeric,
        currency_token_contract_address: diesel::sql_types::Numeric
    ) -> diesel::sql_types::Numeric
);

diesel::define_sql_function!(
    fn user_exchange_input_amount(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp,
        currency_token_id: diesel::sql_types::Numeric,
        currency_token_contract_address: diesel::sql_types::Numeric
    ) -> diesel::sql_types::Numeric
);

diesel::define_sql_function!(
    fn user_exchange_output_amount(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp,
        currency_token_id: diesel::sql_types::Numeric,
        currency_token_contract_address: diesel::sql_types::Numeric
    ) -> diesel::sql_types::Numeric
);

diesel::define_sql_function!(
    fn user_exchange_profits(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp,
        currency_token_id: diesel::sql_types::Numeric,
        currency_token_contract_address: diesel::sql_types::Numeric
    ) -> diesel::sql_types::Numeric
);

diesel::define_sql_function!(
    fn user_fund_investment_amount(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp,
        currency_token_id: diesel::sql_types::Numeric,
        currency_token_contract_address: diesel::sql_types::Numeric
    ) -> diesel::sql_types::Numeric
);

diesel::define_sql_function!(
    fn user_fund_profits(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp,
        currency_token_id: diesel::sql_types::Numeric,
        currency_token_contract_address: diesel::sql_types::Numeric
    ) -> diesel::sql_types::Numeric
);

diesel::define_sql_function!(
    fn user_token_manual_transfer_profits(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp,
        currency_token_id: diesel::sql_types::Numeric,
        currency_token_contract_address: diesel::sql_types::Numeric
    ) -> diesel::sql_types::Numeric
);

diesel::table! {
    forest_project_supply (forest_project_id) {
        forest_project_id -> Uuid,
        forest_project_state -> crate::schema::sql_types::ForestProjectState,
        supply -> Nullable<Numeric>,
        symbol -> Varchar,
        decimals -> Integer,
    }
}

diesel::table! {
    forest_project_user_balance_agg (cognito_user_id, forest_project_id) {
        cognito_user_id -> Varchar,
        forest_project_id -> Uuid,
        total_balance -> Numeric,
    }
}

diesel::table! {
    forest_project_token_contract_user_balance_agg (forest_project_id, cognito_user_id, contract_address) {
        forest_project_id -> Uuid,
        forest_project_state -> crate::schema::sql_types::ForestProjectState,
        forest_project_name -> Varchar,
        cognito_user_id -> Varchar,
        contract_address -> Numeric,
        contract_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        token_symbol -> Varchar,
        token_decimals -> Integer,
        total_balance -> Numeric,
        un_frozen_balance -> Numeric,
    }
}

diesel::table! {
    forest_project_token_contract_user_yields (forest_project_id, token_contract_address, cognito_user_id, yielder_contract_address, yield_token_id, yield_contract_address) {
        forest_project_id -> Uuid,
        token_contract_address -> Numeric,
        token_symbol -> Varchar,
        token_decimals -> Integer,
        cognito_user_id -> Varchar,
        yielder_contract_address -> Numeric,
        yield_token_id -> Numeric,
        yield_contract_address -> Numeric,
        yield_token_symbol -> Varchar,
        yield_token_decimals -> Integer,
        yield_amount -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    forest_project_legal_contracts,
    forest_project_user_balance_agg
);

diesel::allow_tables_to_appear_in_same_query!(
    forest_project_legal_contract_user_signatures,
    forest_project_user_balance_agg,
);
