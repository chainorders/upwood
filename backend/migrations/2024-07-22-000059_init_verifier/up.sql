create table
    verifier_challenges (
        id serial primary key,
        create_time timestamp not null,
        update_time timestamp not null,
        challenge bytea not null,
        account_address bytea not null,
        verifier_account_address bytea not null,
        identity_registry_index numeric(20) not null,
        identity_registry_sub_index numeric(20) not null,
        txn_hash bytea
    )
