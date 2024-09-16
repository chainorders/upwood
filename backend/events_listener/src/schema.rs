// @generated automatically by Diesel CLI.

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
    cis2_deposits (cis2_address, deposited_cis2_address, deposited_token_id, deposited_holder_address) {
        cis2_address -> Varchar,
        deposited_cis2_address -> Varchar,
        deposited_token_id -> Varchar,
        deposited_holder_address -> Varchar,
        deposited_amount -> Numeric,
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
    nft_multi_rewarded_contracts (contract_address) {
        contract_address -> Varchar,
        reward_token_id -> Varchar,
        reward_token_address -> Varchar,
        update_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cis2_agents,
    cis2_compliances,
    cis2_deposits,
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
    nft_multi_rewarded_contracts,
);
