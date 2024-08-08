create table
    identity_registry_identities (
        identity_registry_address varchar not null,
        identity_address varchar not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        primary key (identity_registry_address, identity_address)
    );

create table
    identity_registry_issuers (
        identity_registry_address varchar not null,
        issuer_address varchar not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        primary key (identity_registry_address, issuer_address)
    );

create table
    identity_registry_agents (
        identity_registry_address varchar not null,
        agent_address varchar not null,
        create_time timestamp not null default (now () at time zone 'utc'),
        primary key (identity_registry_address, agent_address)
    );
