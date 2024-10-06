// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "security_p2p_trading_record_type"))]
    pub struct SecurityP2pTradingRecordType;
}

diesel::table! {
    cis2_agents (id) {
        id -> Int8,
        cis2_address -> Varchar,
        agent_address -> Varchar,
    }
}

diesel::table! {
    cis2_compliances (cis2_address) {
        cis2_address -> Varchar,
        compliance_address -> Varchar,
    }
}

diesel::table! {
    cis2_identity_registries (cis2_address) {
        cis2_address -> Varchar,
        identity_registry_address -> Varchar,
    }
}

diesel::table! {
    cis2_operators (cis2_address, holder_address, operator_address) {
        cis2_address -> Varchar,
        holder_address -> Varchar,
        operator_address -> Varchar,
    }
}

diesel::table! {
    cis2_recovery_records (cis2_address, holder_address) {
        cis2_address -> Varchar,
        holder_address -> Varchar,
        recovered_address -> Varchar,
    }
}

diesel::table! {
    cis2_token_holders (cis2_address, token_id, holder_address) {
        cis2_address -> Varchar,
        token_id -> Varchar,
        holder_address -> Varchar,
        balance -> Numeric,
        frozen_balance -> Numeric,
        create_time -> Timestamp,
    }
}

diesel::table! {
    cis2_tokens (cis2_address, token_id) {
        cis2_address -> Varchar,
        token_id -> Varchar,
        is_paused -> Bool,
        metadata_url -> Varchar,
        metadata_hash -> Nullable<Bytea>,
        supply -> Numeric,
        create_time -> Timestamp,
    }
}

diesel::table! {
    identity_registry_agents (identity_registry_address, agent_address) {
        identity_registry_address -> Varchar,
        agent_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    identity_registry_identities (identity_registry_address, identity_address) {
        identity_registry_address -> Varchar,
        identity_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    identity_registry_issuers (identity_registry_address, issuer_address) {
        identity_registry_address -> Varchar,
        issuer_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    listener_config (id) {
        id -> Int4,
        last_block_height -> Numeric,
        last_block_hash -> Bytea,
        last_block_slot_time -> Timestamp,
    }
}

diesel::table! {
    listener_contract_calls (id) {
        id -> Int8,
        transaction_hash -> Bytea,
        index -> Numeric,
        sub_index -> Numeric,
        entrypoint_name -> Varchar,
        ccd_amount -> Numeric,
        instigator -> Varchar,
        sender -> Varchar,
        events_count -> Int4,
        call_type -> Int4,
    }
}

diesel::table! {
    listener_contracts (index) {
        module_ref -> Bytea,
        contract_name -> Varchar,
        index -> Numeric,
        sub_index -> Numeric,
        owner -> Varchar,
    }
}

diesel::table! {
    listener_transactions (transaction_hash) {
        transaction_hash -> Bytea,
        block_hash -> Bytea,
        block_height -> Numeric,
        block_slot_time -> Timestamp,
        transaction_index -> Numeric,
    }
}

diesel::table! {
    nft_multi_address_nonces (contract_address, address) {
        contract_address -> Varchar,
        address -> Varchar,
        nonce -> Int8,
    }
}

diesel::table! {
    nft_multi_rewarded_contracts (contract_address) {
        contract_address -> Varchar,
        reward_token_id -> Varchar,
        reward_token_address -> Varchar,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_mint_fund_contracts (contract_address) {
        contract_address -> Varchar,
        token_contract_address -> Varchar,
        token_id -> Varchar,
        investment_token_contract_address -> Varchar,
        investment_token_id -> Varchar,
        currency_token_contract_address -> Varchar,
        currency_token_id -> Varchar,
        rate_numerator -> Int8,
        rate_denominator -> Int8,
        fund_state -> Int4,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_mint_fund_investment_records (id) {
        id -> Int8,
        contract_address -> Varchar,
        investor -> Varchar,
        currency_amount -> Nullable<Numeric>,
        token_amount -> Nullable<Numeric>,
        investment_record_type -> Int4,
        create_time -> Timestamp,
    }
}

diesel::table! {
    security_mint_fund_investors (contract_address, investor) {
        contract_address -> Varchar,
        investor -> Varchar,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_p2p_trading_contracts (contract_address) {
        contract_address -> Varchar,
        token_contract_address -> Varchar,
        token_id -> Varchar,
        currency_token_contract_address -> Varchar,
        currency_token_id -> Varchar,
        token_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_p2p_trading_deposits (contract_address, trader_address) {
        contract_address -> Varchar,
        trader_address -> Varchar,
        rate_numerator -> Int8,
        rate_denominator -> Int8,
        token_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SecurityP2pTradingRecordType;

    security_p2p_trading_records (id) {
        id -> Int8,
        contract_address -> Varchar,
        trader_address -> Varchar,
        record_type -> SecurityP2pTradingRecordType,
        token_amount -> Numeric,
        metadata -> Jsonb,
        create_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cis2_agents,
    cis2_compliances,
    cis2_identity_registries,
    cis2_operators,
    cis2_recovery_records,
    cis2_token_holders,
    cis2_tokens,
    identity_registry_agents,
    identity_registry_identities,
    identity_registry_issuers,
    listener_config,
    listener_contract_calls,
    listener_contracts,
    listener_transactions,
    nft_multi_address_nonces,
    nft_multi_rewarded_contracts,
    security_mint_fund_contracts,
    security_mint_fund_investment_records,
    security_mint_fund_investors,
    security_p2p_trading_contracts,
    security_p2p_trading_deposits,
    security_p2p_trading_records,
);
