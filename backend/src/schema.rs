// @generated automatically by Diesel CLI.

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
    token_market (market_address, token_contract_address, token_id, token_owner_address) {
        market_address -> Varchar,
        token_contract_address -> Varchar,
        token_id -> Varchar,
        token_owner_address -> Varchar,
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
        account_address -> Varchar,
        verifier_address -> Varchar,
        identity_registry_address -> Varchar,
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
