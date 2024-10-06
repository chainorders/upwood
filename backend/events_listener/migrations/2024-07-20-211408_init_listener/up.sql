create table
    listener_config (
        id serial primary key,
        last_block_height numeric(20) not null,
        last_block_hash bytea not null,
        last_block_slot_time timestamp not null
    );

create unique index listener_config_block_height on listener_config (last_block_height desc);

create table
    listener_contracts (
        module_ref bytea not null,
        contract_name varchar not null,
        index numeric(20) primary key,
        sub_index numeric(20) not null,
        owner varchar not null
    );

create unique index listener_contracts_contract on listener_contracts (index, sub_index);

create table
    listener_transactions (
        transaction_hash bytea primary key not null,
        block_hash bytea not null,
        block_height numeric(20) not null,
        block_slot_time timestamp not null,
        transaction_index numeric(20) not null
    );

create table
    listener_contract_calls (
        id bigserial primary key,
        transaction_hash bytea not null,
        index numeric(20) not null,
        sub_index numeric(20) not null,
        entrypoint_name varchar not null,
        ccd_amount numeric(20) not null,
        instigator varchar not null,
        sender varchar not null,
        events_count int not null,
        call_type int not null
    );
