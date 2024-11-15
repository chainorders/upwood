// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "forest_project_state"))]
    pub struct ForestProjectState;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "security_p2p_trading_record_type"))]
    pub struct SecurityP2pTradingRecordType;
}

diesel::table! {
    cis2_agents (id) {
        id -> Int8,
        cis2_address -> Numeric,
        agent_address -> Varchar,
        roles -> Array<Nullable<Text>>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    cis2_compliances (cis2_address, compliance_address) {
        cis2_address -> Numeric,
        compliance_address -> Varchar,
    }
}

diesel::table! {
    cis2_identity_registries (cis2_address, identity_registry_address) {
        cis2_address -> Numeric,
        identity_registry_address -> Numeric,
    }
}

diesel::table! {
    cis2_operators (cis2_address, holder_address, operator_address) {
        cis2_address -> Numeric,
        holder_address -> Varchar,
        operator_address -> Varchar,
    }
}

diesel::table! {
    cis2_recovery_records (cis2_address, holder_address) {
        cis2_address -> Numeric,
        holder_address -> Varchar,
        recovered_address -> Varchar,
    }
}

diesel::table! {
    cis2_token_holder_balance_updates (id) {
        id -> Uuid,
        cis2_address -> Numeric,
        token_id -> Numeric,
        holder_address -> Varchar,
        amount -> Numeric,
        frozen_balance -> Numeric,
        un_frozen_balance -> Numeric,
        update_type -> Int4,
        create_time -> Timestamp,
    }
}

diesel::table! {
    cis2_token_holders (cis2_address, token_id, holder_address) {
        cis2_address -> Numeric,
        token_id -> Numeric,
        holder_address -> Varchar,
        frozen_balance -> Numeric,
        un_frozen_balance -> Numeric,
        create_time -> Timestamp,
    }
}

diesel::table! {
    cis2_tokens (cis2_address, token_id) {
        cis2_address -> Numeric,
        token_id -> Numeric,
        is_paused -> Bool,
        metadata_url -> Varchar,
        metadata_hash -> Nullable<Varchar>,
        supply -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    forest_project_legal_contract_user_signatures (project_id, cognito_user_id) {
        project_id -> Uuid,
        cognito_user_id -> Varchar,
        user_account -> Varchar,
        user_signature -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_legal_contracts (project_id) {
        project_id -> Uuid,
        text_url -> Varchar,
        edoc_url -> Varchar,
        pdf_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_notifications (id) {
        id -> Uuid,
        project_id -> Uuid,
        cognito_user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_prices (project_id, price_at) {
        project_id -> Uuid,
        price -> Numeric,
        price_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_property_media (id) {
        id -> Uuid,
        project_id -> Uuid,
        image_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ForestProjectState;

    forest_projects (id) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> ForestProjectState,
        image_small_url -> Varchar,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        contract_address -> Numeric,
        p2p_trade_contract_address -> Nullable<Numeric>,
        mint_fund_contract_address -> Nullable<Numeric>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Text,
        property_media_footer -> Text,
        latest_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    guides (id) {
        id -> Uuid,
        title -> Varchar,
        label -> Varchar,
        created_at -> Timestamp,
        order_index -> Int4,
    }
}

diesel::table! {
    identity_registry_agents (identity_registry_address, agent_address) {
        identity_registry_address -> Numeric,
        agent_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    identity_registry_identities (identity_registry_address, identity_address) {
        identity_registry_address -> Numeric,
        identity_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    identity_registry_issuers (identity_registry_address, issuer_address) {
        identity_registry_address -> Numeric,
        issuer_address -> Numeric,
        create_time -> Timestamp,
    }
}

diesel::table! {
    listener_blocks (block_height) {
        block_height -> Numeric,
        block_hash -> Bytea,
        block_slot_time -> Timestamp,
    }
}

diesel::table! {
    listener_contract_calls (id) {
        id -> Int8,
        transaction_hash -> Bytea,
        contract_address -> Numeric,
        entrypoint_name -> Varchar,
        ccd_amount -> Numeric,
        instigator -> Varchar,
        sender -> Varchar,
        events_count -> Int4,
        call_type -> Int4,
        is_processed -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    listener_contracts (contract_address) {
        contract_address -> Numeric,
        module_ref -> Varchar,
        contract_name -> Varchar,
        owner -> Varchar,
        processor_type -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    listener_transactions (transaction_hash) {
        transaction_hash -> Varchar,
        block_hash -> Bytea,
        block_height -> Numeric,
        block_slot_time -> Timestamp,
        transaction_index -> Numeric,
    }
}

diesel::table! {
    maintenance_messages (id) {
        id -> Uuid,
        message -> Text,
        created_at -> Timestamp,
        order_index -> Int4,
    }
}

diesel::table! {
    news_articles (id) {
        id -> Uuid,
        title -> Varchar,
        label -> Varchar,
        content -> Text,
        image_url -> Text,
        article_url -> Text,
        created_at -> Timestamp,
        order_index -> Int4,
    }
}

diesel::table! {
    nft_multi_address_nonces (contract_address, address) {
        contract_address -> Numeric,
        address -> Varchar,
        nonce -> Int8,
    }
}

diesel::table! {
    nft_multi_rewarded_contracts (contract_address) {
        contract_address -> Numeric,
        reward_token_id -> Numeric,
        reward_token_address -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    platform_updates (id) {
        id -> Uuid,
        title -> Varchar,
        label -> Varchar,
        created_at -> Timestamp,
        order_index -> Int4,
    }
}

diesel::table! {
    security_mint_fund_contracts (contract_address) {
        contract_address -> Numeric,
        token_contract_address -> Numeric,
        token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        investment_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        currency_token_id -> Numeric,
        rate -> Numeric,
        fund_state -> Int4,
        receiver_address -> Nullable<Varchar>,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_mint_fund_investment_records (id) {
        id -> Uuid,
        contract_address -> Numeric,
        investor -> Varchar,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        currency_amount_balance -> Numeric,
        token_amount_balance -> Numeric,
        investment_record_type -> Int4,
        create_time -> Timestamp,
    }
}

diesel::table! {
    security_mint_fund_investors (contract_address, investor) {
        contract_address -> Numeric,
        investor -> Varchar,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_p2p_trading_contracts (contract_address) {
        contract_address -> Numeric,
        token_contract_address -> Numeric,
        token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        currency_token_id -> Numeric,
        token_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_p2p_trading_deposits (contract_address, trader_address) {
        contract_address -> Numeric,
        trader_address -> Varchar,
        token_amount -> Numeric,
        rate -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SecurityP2pTradingRecordType;

    security_p2p_trading_records (id) {
        id -> Uuid,
        contract_address -> Numeric,
        trader_address -> Varchar,
        record_type -> SecurityP2pTradingRecordType,
        token_amount -> Numeric,
        currency_amount -> Numeric,
        token_amount_balance -> Numeric,
        currency_amount_balance -> Numeric,
        create_time -> Timestamp,
    }
}

diesel::table! {
    security_p2p_trading_trades (id) {
        id -> Uuid,
        contract_address -> Numeric,
        seller_address -> Varchar,
        buyer_address -> Varchar,
        token_amount -> Numeric,
        currency_amount -> Numeric,
        rate -> Numeric,
        create_time -> Timestamp,
    }
}

diesel::table! {
    security_sft_rewards_claimed_reward (id) {
        id -> Uuid,
        contract_address -> Numeric,
        token_id -> Numeric,
        holder_address -> Varchar,
        token_amount -> Numeric,
        rewarded_token_contract -> Numeric,
        rewarded_token_id -> Numeric,
        reward_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_sft_rewards_contract_rewards (contract_address, rewarded_token_contract, rewarded_token_id) {
        contract_address -> Numeric,
        rewarded_token_contract -> Numeric,
        rewarded_token_id -> Numeric,
        reward_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_sft_rewards_reward_tokens (contract_address, token_id) {
        contract_address -> Numeric,
        token_id -> Numeric,
        rewarded_token_contract -> Numeric,
        rewarded_token_id -> Numeric,
        reward_amount -> Numeric,
        reward_rate -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    support_questions (id) {
        id -> Uuid,
        cognito_user_id -> Varchar,
        user_email -> Varchar,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tree_nft_metadatas (id) {
        #[max_length = 16]
        id -> Bpchar,
        metadata_url -> Text,
        #[max_length = 64]
        metadata_hash -> Nullable<Bpchar>,
        probablity_percentage -> Int2,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_affiliate_accounts (account_address) {
        #[max_length = 255]
        account_address -> Varchar,
        #[max_length = 255]
        cognito_user_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_affiliates (id) {
        id -> Int4,
        #[max_length = 255]
        cognito_user_id -> Varchar,
        #[max_length = 255]
        affiliate_account_address -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_challenges (id) {
        id -> Uuid,
        #[max_length = 255]
        cognito_user_id -> Varchar,
        challenge -> Bytea,
        #[max_length = 255]
        account_address -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (cognito_user_id) {
        #[max_length = 255]
        cognito_user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        account_address -> Nullable<Varchar>,
        desired_investment_amount -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(cis2_agents -> listener_contracts (cis2_address));
diesel::joinable!(cis2_compliances -> listener_contracts (cis2_address));
diesel::joinable!(cis2_operators -> listener_contracts (cis2_address));
diesel::joinable!(cis2_recovery_records -> listener_contracts (cis2_address));
diesel::joinable!(cis2_token_holder_balance_updates -> listener_contracts (cis2_address));
diesel::joinable!(cis2_token_holders -> listener_contracts (cis2_address));
diesel::joinable!(cis2_tokens -> listener_contracts (cis2_address));
diesel::joinable!(forest_project_legal_contract_user_signatures -> forest_projects (project_id));
diesel::joinable!(forest_project_legal_contract_user_signatures -> users (cognito_user_id));
diesel::joinable!(forest_project_legal_contracts -> forest_projects (project_id));
diesel::joinable!(forest_project_notifications -> forest_projects (project_id));
diesel::joinable!(forest_project_notifications -> users (cognito_user_id));
diesel::joinable!(forest_project_prices -> forest_projects (project_id));
diesel::joinable!(forest_project_property_media -> forest_projects (project_id));
diesel::joinable!(forest_projects -> listener_contracts (contract_address));
diesel::joinable!(forest_projects -> security_mint_fund_contracts (mint_fund_contract_address));
diesel::joinable!(forest_projects -> security_p2p_trading_contracts (p2p_trade_contract_address));
diesel::joinable!(identity_registry_agents -> listener_contracts (identity_registry_address));
diesel::joinable!(identity_registry_identities -> listener_contracts (identity_registry_address));
diesel::joinable!(identity_registry_issuers -> listener_contracts (identity_registry_address));
diesel::joinable!(listener_contract_calls -> listener_contracts (contract_address));
diesel::joinable!(nft_multi_address_nonces -> nft_multi_rewarded_contracts (contract_address));
diesel::joinable!(nft_multi_rewarded_contracts -> listener_contracts (contract_address));
diesel::joinable!(security_mint_fund_investment_records -> security_mint_fund_contracts (contract_address));
diesel::joinable!(security_mint_fund_investors -> security_mint_fund_contracts (contract_address));
diesel::joinable!(security_p2p_trading_contracts -> listener_contracts (contract_address));
diesel::joinable!(security_p2p_trading_deposits -> security_p2p_trading_contracts (contract_address));
diesel::joinable!(security_p2p_trading_records -> security_p2p_trading_contracts (contract_address));
diesel::joinable!(security_p2p_trading_trades -> security_p2p_trading_contracts (contract_address));
diesel::joinable!(security_sft_rewards_claimed_reward -> listener_contracts (contract_address));
diesel::joinable!(security_sft_rewards_contract_rewards -> listener_contracts (contract_address));
diesel::joinable!(security_sft_rewards_reward_tokens -> listener_contracts (contract_address));
diesel::joinable!(support_questions -> users (cognito_user_id));
diesel::joinable!(user_affiliate_accounts -> users (cognito_user_id));
diesel::joinable!(user_affiliates -> users (cognito_user_id));
diesel::joinable!(user_challenges -> users (cognito_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cis2_agents,
    cis2_compliances,
    cis2_identity_registries,
    cis2_operators,
    cis2_recovery_records,
    cis2_token_holder_balance_updates,
    cis2_token_holders,
    cis2_tokens,
    forest_project_legal_contract_user_signatures,
    forest_project_legal_contracts,
    forest_project_notifications,
    forest_project_prices,
    forest_project_property_media,
    forest_projects,
    guides,
    identity_registry_agents,
    identity_registry_identities,
    identity_registry_issuers,
    listener_blocks,
    listener_contract_calls,
    listener_contracts,
    listener_transactions,
    maintenance_messages,
    news_articles,
    nft_multi_address_nonces,
    nft_multi_rewarded_contracts,
    platform_updates,
    security_mint_fund_contracts,
    security_mint_fund_investment_records,
    security_mint_fund_investors,
    security_p2p_trading_contracts,
    security_p2p_trading_deposits,
    security_p2p_trading_records,
    security_p2p_trading_trades,
    security_sft_rewards_claimed_reward,
    security_sft_rewards_contract_rewards,
    security_sft_rewards_reward_tokens,
    support_questions,
    tree_nft_metadatas,
    user_affiliate_accounts,
    user_affiliates,
    user_challenges,
    users,
);
