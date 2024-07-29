create table
    security_cis2_contract_compliances (
        cis2_address varchar not null,
        compliance_address varchar not null,
        primary key (cis2_address)
    );

create table
    security_cis2_contract_identity_registries (
        cis2_address varchar not null,
        identity_registry_address varchar not null,
        primary key (cis2_address)
    );

create table
    security_cis2_contract_tokens (
        cis2_address varchar not null,
        token_id varchar not null,
        is_paused boolean not null default false,
        metadata_url varchar not null,
        metadata_hash bytea,
        supply numeric(78) not null default 0,
        create_time timestamp not null,
        primary key (cis2_address, token_id)
    );

create table
    security_cis2_contract_token_holders (
        cis2_address varchar not null,
        token_id varchar not null,
        holder_address varchar not null,
        balance numeric(78) not null,
        frozen_balance numeric(78) not null,
        create_time timestamp not null,
        primary key (cis2_address, token_id, holder_address)
    );

create index security_cis2_contract_token_holder on security_cis2_contract_token_holders (cis2_address, holder_address);

create table
    security_cis2_contract_agents (
        id bigserial primary key,
        cis2_address varchar not null,
        agent_address varchar not null
    );

create table
    security_cis2_contract_operators (
        cis2_address varchar not null,
        holder_address varchar not null,
        operator_address varchar not null,
        primary key (cis2_address, holder_address, operator_address)
    );

create table
    security_cis2_contract_recovery_records (
        cis2_address varchar not null,
        holder_address varchar not null,
        recovered_address varchar not null,
        primary key (cis2_address, holder_address)
    );
