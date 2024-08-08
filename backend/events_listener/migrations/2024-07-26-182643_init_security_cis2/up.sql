create table
    cis2_compliances (
        cis2_address varchar not null,
        compliance_address varchar not null,
        primary key (cis2_address)
    );

create table
    cis2_identity_registries (
        cis2_address varchar not null,
        identity_registry_address varchar not null,
        primary key (cis2_address)
    );

create table
    cis2_tokens (
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
    cis2_token_holders (
        cis2_address varchar not null,
        token_id varchar not null,
        holder_address varchar not null,
        balance numeric(78) not null,
        frozen_balance numeric(78) not null,
        create_time timestamp not null,
        primary key (cis2_address, token_id, holder_address)
    );

create index cis2_token_holder on cis2_token_holders (cis2_address, holder_address);

create table
    cis2_agents (
        id bigserial primary key,
        cis2_address varchar not null,
        agent_address varchar not null
    );

create table
    cis2_operators (
        cis2_address varchar not null,
        holder_address varchar not null,
        operator_address varchar not null,
        primary key (cis2_address, holder_address, operator_address)
    );

create table
    cis2_recovery_records (
        cis2_address varchar not null,
        holder_address varchar not null,
        recovered_address varchar not null,
        primary key (cis2_address, holder_address)
    );

create table
    cis2_deposits (
        cis2_address varchar not null,
        deposited_cis2_address varchar not null,
        deposited_token_id varchar not null,
        deposited_holder_address varchar not null,
        deposited_amount numeric(78) not null,
        primary key (
            cis2_address,
            deposited_cis2_address,
            deposited_token_id,
            deposited_holder_address
        )
    )
