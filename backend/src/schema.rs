// @generated automatically by Diesel CLI.

diesel::table! {
    identity_registry_agents (contract_index, contract_sub_index, agent_address) {
        contract_index -> Numeric,
        contract_sub_index -> Numeric,
        agent_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    identity_registry_identities (contract_index, contract_sub_index, identity_address) {
        contract_index -> Numeric,
        contract_sub_index -> Numeric,
        identity_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    identity_registry_issuers (contract_index, contract_sub_index, issuer_address) {
        contract_index -> Numeric,
        contract_sub_index -> Numeric,
        issuer_address -> Varchar,
        create_time -> Timestamp,
    }
}

diesel::table! {
    listener_config (id) {
        id -> Int4,
        last_block_height -> Numeric,
        last_block_hash -> Bytea,
    }
}

diesel::table! {
    listener_contracts (index) {
        module_ref -> Bytea,
        contract_name -> Varchar,
        index -> Numeric,
        sub_index -> Numeric,
    }
}

diesel::table! {
    token_market (market_contract_index, market_contract_sub_index, token_contract_index, token_contract_sub_index, token_id, token_owner) {
        market_contract_index -> Numeric,
        market_contract_sub_index -> Numeric,
        token_contract_index -> Numeric,
        token_contract_sub_index -> Numeric,
        token_id -> Numeric,
        token_owner -> Varchar,
        token_listed_amount -> Numeric,
        token_unlisted_amount -> Numeric,
    }
}

diesel::table! {
    verifier_challenges (id) {
        id -> Int4,
        create_time -> Timestamp,
        update_time -> Timestamp,
        challenge -> Bytea,
        account_address -> Bytea,
        verifier_account_address -> Bytea,
        identity_registry_index -> Numeric,
        identity_registry_sub_index -> Numeric,
        txn_hash -> Nullable<Bytea>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    identity_registry_agents,
    identity_registry_identities,
    identity_registry_issuers,
    listener_config,
    listener_contracts,
    token_market,
    verifier_challenges,
);
