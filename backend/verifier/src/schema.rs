// @generated automatically by Diesel CLI.

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
