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
    security_cis2_contract_agents (id) {
        id -> Int8,
        cis2_address -> Varchar,
        agent_address -> Varchar,
    }
}

diesel::table! {
    security_cis2_contract_compliances (cis2_address) {
        cis2_address -> Varchar,
        compliance_address -> Varchar,
    }
}

diesel::table! {
    security_cis2_contract_identity_registries (cis2_address) {
        cis2_address -> Varchar,
        identity_registry_address -> Varchar,
    }
}

diesel::table! {
    security_cis2_contract_operators (cis2_address, holder_address, operator_address) {
        cis2_address -> Varchar,
        holder_address -> Varchar,
        operator_address -> Varchar,
    }
}

diesel::table! {
    security_cis2_contract_recovery_records (cis2_address, holder_address) {
        cis2_address -> Varchar,
        holder_address -> Varchar,
        recovered_address -> Varchar,
    }
}

diesel::table! {
    security_cis2_contract_token_holders (cis2_address, token_id, holder_address) {
        cis2_address -> Varchar,
        token_id -> Varchar,
        holder_address -> Varchar,
        balance -> Numeric,
        frozen_balance -> Numeric,
        create_time -> Timestamp,
    }
}

diesel::table! {
    security_cis2_contract_tokens (cis2_address, token_id) {
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
    security_cis2_contract_agents,
    security_cis2_contract_compliances,
    security_cis2_contract_identity_registries,
    security_cis2_contract_operators,
    security_cis2_contract_recovery_records,
    security_cis2_contract_token_holders,
    security_cis2_contract_tokens,
    token_market,
    verifier_challenges,
);
