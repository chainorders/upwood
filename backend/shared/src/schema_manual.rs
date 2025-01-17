diesel::table! {
    forest_project_property_funds (contract_address, investment_token_id, investment_token_contract_address) {
        contract_address -> Numeric,
        investment_token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        token_id -> Numeric,
        token_contract_address -> Numeric,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        receiver_address -> Nullable<Varchar>,
        rate_numerator -> Numeric,
        rate_denominator -> Numeric,
        fund_state -> crate::schema::sql_types::SecurityMintFundState,
        create_time -> Timestamp,
        update_time -> Timestamp,
        forest_project_id -> Numeric,
        mint_fund_type -> Varchar,
    }
}

diesel::table! {
    forest_project_bond_funds (contract_address, investment_token_id, investment_token_contract_address) {
        contract_address -> Numeric,
        investment_token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        token_id -> Numeric,
        token_contract_address -> Numeric,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        receiver_address -> Nullable<Varchar>,
        rate_numerator -> Numeric,
        rate_denominator -> Numeric,
        fund_state -> crate::schema::sql_types::SecurityMintFundState,
        create_time -> Timestamp,
        update_time -> Timestamp,
        forest_project_id -> Numeric,
        mint_fund_type -> Varchar,
    }
}

diesel::table! {
    forest_project_funds (contract_address, investment_token_id, investment_token_contract_address) {
        contract_address -> Numeric,
        investment_token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        token_id -> Numeric,
        token_contract_address -> Numeric,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        receiver_address -> Nullable<Varchar>,
        rate_numerator -> Numeric,
        rate_denominator -> Numeric,
        fund_state -> crate::schema::sql_types::SecurityMintFundState,
        create_time -> Timestamp,
        update_time -> Timestamp,
        forest_project_id -> Numeric,
        mint_fund_type -> Varchar,
        fund_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        is_default -> Bool,
    }
}

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
        claim_id -> Nullable<Uuid>,
        claims_contract_address -> Nullable<Numeric>,
        reward_amount -> Numeric,
        remaining_reward_amount -> Numeric,
        affiliate_cognito_user_id -> Varchar,
        affiliate_commission -> Numeric,
    }
}

diesel::table! {
    active_forest_projects (id) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> crate::schema::sql_types::ForestProjectState,
        image_small_url -> Varchar,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Text,
        property_media_footer -> Text,
        latest_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        contract_address -> Nullable<Numeric>,
        token_id -> Nullable<Numeric>,
        total_supply -> Numeric,
        fund_contract_address -> Nullable<Numeric>,
        pre_sale_token_contract_address -> Nullable<Numeric>,
        pre_sale_token_id -> Nullable<Numeric>,
        fund_rate_numerator -> Nullable<Numeric>,
        fund_rate_denominator -> Nullable<Numeric>,
    }
}

diesel::table! {
    funded_forest_projects (id) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> crate::schema::sql_types::ForestProjectState,
        image_small_url -> Varchar,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Text,
        property_media_footer -> Text,
        latest_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        token_contract_address -> Nullable<Numeric>,
        token_id -> Nullable<Numeric>,
        total_supply -> Numeric,
        market_contract_address -> Nullable<Numeric>,
        market_liquidity_provider -> Nullable<Varchar>,
        market_sell_rate_numerator -> Nullable<Numeric>,
        market_sell_rate_denominator -> Nullable<Numeric>,
    }
}

diesel::table! {
    active_forest_project_users (id, cognito_user_id) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> crate::schema::sql_types::ForestProjectState,
        image_small_url -> Varchar,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Text,
        property_media_footer -> Text,
        latest_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        contract_address -> Nullable<Numeric>,
        token_id -> Nullable<Numeric>,
        total_supply -> Numeric,
        fund_contract_address -> Nullable<Numeric>,
        pre_sale_token_contract_address -> Nullable<Numeric>,
        pre_sale_token_id -> Nullable<Numeric>,
        fund_rate_numerator -> Nullable<Numeric>,
        fund_rate_denominator -> Nullable<Numeric>,
        notification_id -> Nullable<Uuid>,
        cognito_user_id -> Nullable<Varchar>,
        has_signed_contract -> Bool,
    }
}

diesel::table! {
    funded_forest_project_users (id, cognito_user_id) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> crate::schema::sql_types::ForestProjectState,
        image_small_url -> Varchar,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Text,
        property_media_footer -> Text,
        latest_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        token_contract_address -> Nullable<Numeric>,
        token_id -> Nullable<Numeric>,
        total_supply -> Numeric,
        market_contract_address -> Nullable<Numeric>,
        market_liquidity_provider -> Nullable<Varchar>,
        market_sell_rate_numerator -> Nullable<Numeric>,
        market_sell_rate_denominator -> Nullable<Numeric>,
        notification_id -> Nullable<Uuid>,
        cognito_user_id -> Nullable<Varchar>,
        has_signed_contract -> Bool,
    }
}

diesel::table! {
    forest_projects_owned_by_user (id, cognito_user_id) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> crate::schema::sql_types::ForestProjectState,
        image_small_url -> Varchar,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Text,
        property_media_footer -> Text,
        latest_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        cognito_user_id -> Varchar,
        account_address -> Varchar,
        total_balance -> Numeric,
        property_contract_address -> Nullable<Numeric>,
        property_token_id -> Nullable<Numeric>,
        market_contract_address -> Nullable<Numeric>,
        market_liquidity_provider -> Nullable<Varchar>,
        market_buy_rate_numerator -> Nullable<Numeric>,
        market_buy_rate_denominator -> Nullable<Numeric>,
        bond_contract_address -> Nullable<Numeric>,
        bond_token_id -> Nullable<Numeric>,
        bond_fund_contract_address -> Nullable<Numeric>,
        bond_fund_rate_numerator -> Nullable<Numeric>,
        bond_fund_rate_denominator -> Nullable<Numeric>,
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
    }
}

diesel::table! {
    forest_project_user_yields_aggregate (cognito_user_id, yielder_contract_address, yield_token_id, yield_contract_address) {
        cognito_user_id -> Varchar,
        yielder_contract_address -> Numeric,
        yield_token_id -> Numeric,
        yield_contract_address -> Numeric,
        yield_amount -> Numeric,
    }
}

diesel::table! {
    forest_project_supply (forest_project_id) {
        forest_project_id -> Uuid,
        supply -> Nullable<Numeric>,
    }
}

diesel::table! {
    forest_project_security_tokens (cis2_address, token_id) {
        cis2_address -> Numeric,
        token_id -> Numeric,
        is_paused -> Bool,
        metadata_url -> Varchar,
        metadata_hash -> Nullable<Varchar>,
        supply -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
        forest_project_id -> Uuid,
        contract_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        is_default -> Bool,
    }
}

diesel::table! {
    forest_project_fund_investor (fund_contract_address, investor_cognito_user_id) {
        forest_project_id -> Uuid,
        fund_contract_address -> Numeric,
        fund_token_id -> Numeric,
        fund_token_contract_address -> Numeric,
        investment_token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        fund_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
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
    }
}

diesel::table! {
    forest_project_investor (cognito_user_id) {
        cognito_user_id -> Varchar,
        total_currency_amount_locked -> Numeric,
        total_currency_amount_invested -> Numeric,
    }
}

diesel::table! {
    forest_project_trader (cognito_user_id) {
        cognito_user_id -> Varchar,
        total_currency_in_amount -> Numeric,
        total_currency_out_amount -> Numeric,
    }
}

diesel::table! {
    forest_project_user_investment_amounts(cognito_user_id) {
        cognito_user_id -> Varchar,
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
        time_at: diesel::sql_types::Timestamp
    ) -> Nullable<Numeric>
);

diesel::define_sql_function!(
    fn user_exchange_profits(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp
    ) -> Numeric
);

diesel::define_sql_function!(
    fn user_fund_profits(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp
    ) -> Numeric
);

diesel::define_sql_function!(
    fn user_exchange_input_amount(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp
    ) -> Numeric
);

diesel::define_sql_function!(
    fn user_exchange_output_amount(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp
    ) -> Numeric
);

diesel::define_sql_function!(
    fn user_fund_investment_amount(
        user_id: diesel::sql_types::Text,
        from_time: diesel::sql_types::Timestamp,
        to_time: diesel::sql_types::Timestamp
    ) -> Numeric
);
