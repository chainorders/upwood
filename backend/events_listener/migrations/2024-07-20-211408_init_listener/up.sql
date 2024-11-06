create table
    -- This SQL script is part of the migration for initializing the listener configuration.
    listener_config (
        id serial primary key,
        last_block_height numeric(20) not null,
        last_block_hash bytea not null,
        last_block_slot_time timestamp not null
    );

create unique index listener_config_block_height on listener_config (last_block_height desc);

create table
    listener_contracts (
        contract_address numeric(20) primary key not null,
        module_ref varchar not null,
        contract_name varchar not null,
        owner varchar not null,
        processor_type integer not null,
        created_at timestamp not null
    );

create table
    listener_transactions (
        transaction_hash varchar primary key not null,
        block_hash bytea not null, -- do not use references listener_blocks (block_hash) because the block may not be in the db yet
        block_height numeric(20) not null, -- do not use references listener_blocks (block_height) because the block may not be in the db yet
        block_slot_time timestamp not null,
        transaction_index numeric(20) not null
    );

create table
    listener_contract_calls (
        id bigserial primary key,
        transaction_hash bytea not null, -- do not use references listener_transactions (transaction_hash) because the transaction may not be in the db yet
        contract_address numeric(20) not null references listener_contracts (contract_address),
        entrypoint_name varchar not null,
        ccd_amount numeric(20) not null,
        instigator varchar not null,
        sender varchar not null,
        events_count int not null,
        call_type int not null,
        is_processed boolean not null default false,
        created_at timestamp not null
    );
