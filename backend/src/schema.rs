// @generated automatically by Diesel CLI.

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
    listener_config,
    listener_contracts,
    verifier_challenges,
);
