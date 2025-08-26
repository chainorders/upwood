/* plpgsql-language-server:disable validation */
CREATE TABLE identity_registry_identities (
       identity_registry_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       identity_address VARCHAR NOT NULL,
       create_time TIMESTAMP NOT NULL DEFAULT (NOW() at TIME ZONE 'utc'),
       PRIMARY KEY (identity_registry_address, identity_address)
);

CREATE TABLE identity_registry_issuers (
       identity_registry_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       issuer_address NUMERIC(20) NOT NULL,
       create_time TIMESTAMP NOT NULL DEFAULT (NOW() at TIME ZONE 'utc'),
       PRIMARY KEY (identity_registry_address, issuer_address)
);

CREATE TABLE identity_registry_agents (
       identity_registry_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       agent_address VARCHAR NOT NULL,
       create_time TIMESTAMP NOT NULL DEFAULT (NOW() at TIME ZONE 'utc'),
       PRIMARY KEY (identity_registry_address, agent_address)
);
