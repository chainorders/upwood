create table
    listener_config (
        id serial primary key,
        last_block_height numeric(20) not null,
        last_block_hash bytea not null
    );

create unique index listener_config_block_height on listener_config (last_block_height desc);

create table
    listener_contracts (
        module_ref bytea not null,
        contract_name varchar not null,
        index numeric(20) primary key,
        sub_index numeric(20) not null
    );

create unique index listener_contracts_contract on listener_contracts (index, sub_index);