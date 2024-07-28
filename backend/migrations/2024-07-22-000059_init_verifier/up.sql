create table
    verifier_challenges (
        id serial primary key,
        create_time timestamp not null,
        update_time timestamp not null,
        challenge bytea not null,
        account_address varchar not null,
        verifier_address varchar not null,
        identity_registry_address varchar not null,
        txn_hash bytea
    )
