/* plpgsql-language-server:disable validation */
CREATE TABLE cis2_compliances (
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       compliance_address VARCHAR NOT NULL,
       PRIMARY KEY (cis2_address, compliance_address)
);

CREATE TABLE cis2_identity_registries (
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       identity_registry_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       PRIMARY KEY (cis2_address, identity_registry_address)
);

CREATE TABLE cis2_tokens (
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       token_id NUMERIC(20) NOT NULL,
       is_paused BOOLEAN NOT NULL DEFAULT FALSE,
       metadata_url VARCHAR NOT NULL,
       metadata_hash VARCHAR,
       supply NUMERIC(78) NOT NULL DEFAULT 0,
       create_time TIMESTAMP NOT NULL,
       update_time TIMESTAMP NOT NULL,
       PRIMARY KEY (cis2_address, token_id)
);

CREATE INDEX cis2_token_metadata_url ON cis2_tokens (metadata_url);

CREATE TABLE cis2_token_holders (
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       token_id NUMERIC(20) NOT NULL,
       holder_address VARCHAR NOT NULL,
       frozen_balance NUMERIC(78) NOT NULL,
       un_frozen_balance NUMERIC(78) NOT NULL,
       create_time TIMESTAMP NOT NULL,
       update_time TIMESTAMP NOT NULL,
       PRIMARY KEY (cis2_address, token_id, holder_address),
       FOREIGN KEY (cis2_address, token_id) REFERENCES cis2_tokens (cis2_address, token_id)
);

CREATE INDEX cis2_token_holder ON cis2_token_holders (cis2_address, holder_address);

CREATE TYPE cis2_token_holder_balance_update_type AS ENUM(
       'mint',
       'burn',
       'transfer_out',
       'transfer_in',
       'freeze',
       'un_freeze'
);

CREATE TABLE cis2_token_holder_balance_updates (
       id UUID PRIMARY KEY,
       id_serial BIGSERIAL,
       block_height NUMERIC(20) NOT NULL,
       txn_index NUMERIC(20) NOT NULL,
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       token_id NUMERIC(20) NOT NULL,
       holder_address VARCHAR NOT NULL,
       amount NUMERIC(78) NOT NULL,
       frozen_balance NUMERIC(78) NOT NULL,
       un_frozen_balance NUMERIC(78) NOT NULL,
       update_type cis2_token_holder_balance_update_type NOT NULL,
       create_time TIMESTAMP NOT NULL,
       FOREIGN KEY (cis2_address, token_id, holder_address) REFERENCES cis2_token_holders (cis2_address, token_id, holder_address) ON DELETE CASCADE
);

CREATE TABLE cis2_agents (
       id bigserial PRIMARY KEY,
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       agent_address VARCHAR NOT NULL,
       roles TEXT[] NOT NULL,
       created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE cis2_operators (
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       holder_address VARCHAR NOT NULL,
       operator_address VARCHAR NOT NULL,
       PRIMARY KEY (cis2_address, holder_address, operator_address)
);

CREATE TABLE cis2_recovery_records (
       cis2_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
       holder_address VARCHAR NOT NULL,
       recovered_address VARCHAR NOT NULL,
       PRIMARY KEY (cis2_address, holder_address)
);
