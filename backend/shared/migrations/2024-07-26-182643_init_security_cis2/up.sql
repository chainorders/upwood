create table
    cis2_compliances (
        cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        compliance_address varchar not null,
        primary key (cis2_address, compliance_address)
    );

create table
    cis2_identity_registries (
        cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        identity_registry_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        primary key (cis2_address, identity_registry_address)
    );

create table
    cis2_tokens (
        cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        is_paused boolean not null default false,
        metadata_url varchar not null,
        metadata_hash varchar,
        supply numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (cis2_address, token_id)
    );

create index cis2_token_metadata_url on cis2_tokens (metadata_url);

create table
    cis2_token_holders (
        cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        holder_address varchar not null,
        frozen_balance numeric(78) not null,
        un_frozen_balance numeric(78) not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (cis2_address, token_id, holder_address),
        foreign key (cis2_address, token_id) references cis2_tokens (cis2_address, token_id)
    );

create index cis2_token_holder on cis2_token_holders (cis2_address, holder_address);

create type cis2_token_holder_balance_update_type as enum ('mint', 'burn', 'transfer_out', 'transfer_in', 'freeze', 'un_freeze');

create table cis2_token_holder_balance_updates (
    id uuid primary key,
    cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
    token_id numeric(20) not null,
    holder_address varchar not null,
    amount numeric(78) not null,
    frozen_balance numeric(78) not null,
    un_frozen_balance numeric(78) not null,
    update_type cis2_token_holder_balance_update_type not null,
    create_time timestamp not null,
    foreign key (cis2_address, token_id, holder_address) references cis2_token_holders (cis2_address, token_id, holder_address) on delete cascade
);

create table
    cis2_agents (
        id bigserial primary key,
        cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        agent_address varchar not null,
        roles text[] not null,
        created_at timestamp default now()
    );

create table
    cis2_operators (
        cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        holder_address varchar not null,
        operator_address varchar not null,
        primary key (cis2_address, holder_address, operator_address)
    );

create table
    cis2_recovery_records (
        cis2_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        holder_address varchar not null,
        recovered_address varchar not null,
        primary key (cis2_address, holder_address)
    );
