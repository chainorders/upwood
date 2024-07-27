create table
    identity_registry_identities (
        contract_index numeric(20) not null,
        contract_sub_index numeric(20) not null,
        identity_address varchar not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        primary key (
            contract_index,
            contract_sub_index,
            identity_address
        )
    );

create table
    identity_registry_issuers (
        contract_index numeric(20) not null,
        contract_sub_index numeric(20) not null,
        issuer_address varchar not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        primary key (
            contract_index,
            contract_sub_index,
            issuer_address
        )
    );

create table
    identity_registry_agents (
        contract_index numeric(20) not null,
        contract_sub_index numeric(20) not null,
        agent_address varchar not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        primary key (contract_index, contract_sub_index, agent_address)
    );
