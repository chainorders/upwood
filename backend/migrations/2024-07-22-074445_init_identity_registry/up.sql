create table
    identity_registry_identities (
        identity_address varchar primary key not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        contract_index numeric(20) not null,
        contract_sub_index numeric(20) not null
    );

create table
    identity_registry_issuers (
        issuer_address varchar primary key not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        contract_index numeric(20) not null,
        contract_sub_index numeric(20) not null
    );

create table
    identity_registry_agents (
        agent_address varchar primary key not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        contract_index numeric(20) not null,
        contract_sub_index numeric(20) not null
    );
