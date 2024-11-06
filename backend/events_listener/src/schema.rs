// @generated automatically by Diesel CLI.

pub mod sql_types {
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
    cis2_compliances (cis2_address) {
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
    security_mint_fund_contracts (contract_address) {
        contract_address -> Numeric,
        token_contract_address -> Numeric,
        token_id -> Numeric,
        investment_token_contract_address -> Numeric,
        investment_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        currency_token_id -> Numeric,
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
        contract_address -> Numeric,
        investor -> Varchar,
        currency_amount -> Nullable<Numeric>,
        token_amount -> Nullable<Numeric>,
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
        rate_numerator -> Int8,
        rate_denominator -> Int8,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_p2p_trading_deposits (contract_address, trader_address) {
        contract_address -> Numeric,
        trader_address -> Varchar,
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
        contract_address -> Numeric,
        trader_address -> Varchar,
        record_type -> SecurityP2pTradingRecordType,
        token_amount -> Numeric,
        metadata -> Jsonb,
        create_time -> Timestamp,
    }
}

diesel::table! {
    security_sft_rewards_claims (id) {
        id -> Uuid,
        contract_address -> Numeric,
        token_id -> Numeric,
        amount -> Numeric,
        holder_address -> Varchar,
        rewarded_token_contract -> Numeric,
        rewarded_token_id -> Numeric,
        reward_amount -> Numeric,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    security_sft_rewards_rewards (contract_address, token_id) {
        contract_address -> Numeric,
        token_id -> Numeric,
        rewarded_token_id -> Numeric,
        rewarded_contract_address -> Numeric,
        reward_amount -> Numeric,
        reward_rate_numerator -> Int8,
        reward_rate_denominator -> Int8,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::joinable!(cis2_agents -> listener_contracts (cis2_address));
diesel::joinable!(cis2_compliances -> listener_contracts (cis2_address));
diesel::joinable!(cis2_operators -> listener_contracts (cis2_address));
diesel::joinable!(cis2_recovery_records -> listener_contracts (cis2_address));
diesel::joinable!(cis2_token_holders -> listener_contracts (cis2_address));
diesel::joinable!(cis2_tokens -> listener_contracts (cis2_address));
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
diesel::joinable!(security_sft_rewards_claims -> listener_contracts (contract_address));
diesel::joinable!(security_sft_rewards_rewards -> listener_contracts (contract_address));

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
    security_sft_rewards_claims,
    security_sft_rewards_rewards,
);
