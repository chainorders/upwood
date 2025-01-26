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
    forest_project_user_yields_for_each_owned_token (forest_project_id, token_id, token_contract_address, holder_address, yielder_contract_address, yield_token_id, yield_contract_address) {
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
    forest_project_user_yields_aggregate (cognito_user_id, yielder_contract_address, yield_token_id, yield_contract_address) {
        cognito_user_id -> Varchar,
        yielder_contract_address -> Numeric,
        yield_token_id -> Numeric,
        yield_contract_address -> Numeric,
        yield_amount -> Numeric,
        yield_token_symbol -> Varchar,
        yield_token_decimals -> Integer,
    }
}

diesel::table! {
    forest_project_fund_investor (forest_project_id, fund_contract_address, investor_cognito_user_id) {
        forest_project_id -> Uuid,
        fund_contract_address -> Numeric,
        fund_token_id -> Numeric,
        fund_token_contract_address -> Numeric,
        investment_token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        fund_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        currency_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        currency_token_symbol -> Varchar,
        currency_token_decimals -> Integer,
        investment_token_symbol -> Varchar,
        investment_token_decimals -> Integer,
        fund_token_symbol -> Varchar,
        fund_token_decimals -> Integer,
        investor_account_address -> Varchar,
        investment_token_amount -> Numeric,
        investment_currency_amount -> Numeric,
        investor_cognito_user_id -> Varchar,
        investor_email -> Varchar,
    }
}

diesel::table! {
    forest_project_funds_investment_records (id) {
        id -> Uuid,
        block_height -> Numeric,
        txn_index -> Numeric,
        contract_address -> Numeric,
        investment_token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        investor -> Varchar,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        currency_amount_balance -> Numeric,
        token_amount_balance -> Numeric,
        investment_record_type -> crate::schema::sql_types::SecurityMintFundInvestmentRecordType,
        create_time -> Timestamp,
        forest_project_id -> Uuid,
        fund_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        is_default -> Bool,
        investor_cognito_user_id -> Varchar,
        investment_token_symbol -> Varchar,
        investment_token_decimals -> Integer,
        currency_token_symbol -> Varchar,
        currency_token_decimals -> Integer,
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
        forest_project_id -> Uuid,
        currency_amount -> Numeric,
        cognito_user_id -> Varchar,
        transaction_type -> Varchar,
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
    forest_project_current_token_fund_markets (forest_project_id) {
        forest_project_id -> Uuid,
        forest_project_state -> crate::schema::sql_types::ForestProjectState,
        token_contract_address -> Numeric,
        token_id -> Nullable<Numeric>,
        token_contract_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        market_token_id -> Nullable<Numeric>,
        token_symbol -> Varchar,
        token_decimals -> Integer,
        fund_contract_address -> Nullable<Numeric>,
        fund_rate_numerator -> Nullable<Numeric>,
        fund_rate_denominator -> Nullable<Numeric>,
        fund_state -> Nullable<crate::schema::sql_types::SecurityMintFundState>,
        fund_token_contract_address -> Nullable<Numeric>,
        fund_token_id -> Nullable<Numeric>,
        market_contract_address -> Nullable<Numeric>,
        market_sell_rate_numerator -> Nullable<Numeric>,
        market_sell_rate_denominator -> Nullable<Numeric>,
        market_buy_rate_numerator -> Nullable<Numeric>,
        market_buy_rate_denominator -> Nullable<Numeric>,
        market_liquidity_provider -> Nullable<Varchar>,
    }
}

diesel::table! {
    forest_project_user_agg_balances (cognito_user_id, forest_project_id) {
        cognito_user_id -> Varchar,
        forest_project_id -> Uuid,
        total_balance -> Numeric,
        token_symbol -> Varchar,
        token_decimals -> Integer,
    }
}

diesel::table! {
    forest_project_user_yield_distributions (yield_distribution_id) {
        forest_project_id -> Uuid,
        forest_project_name -> Varchar,
        token_contract_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        cognito_user_id -> Varchar,
        yield_distribution_id -> Uuid,
        yielder_contract_address -> Numeric,
        token_contract_address -> Numeric,
        from_token_version -> Numeric,
        to_token_version -> Numeric,
        token_amount -> Numeric,
        yield_contract_address -> Numeric,
        yield_token_id -> Numeric,
        yield_amount -> Numeric,
        yield_token_symbol -> Varchar,
        yield_token_decimals -> Integer,
        token_symbol -> Varchar,
        token_decimals -> Integer,
        to_address -> Varchar,
        create_time -> Timestamp,
    }
}
