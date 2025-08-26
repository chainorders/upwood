/* plpgsql-language-server:disable validation */
CREATE TABLE listener_blocks (
    block_height NUMERIC(20) PRIMARY KEY NOT NULL,
    block_hash bytea NOT NULL,
    block_slot_time TIMESTAMP NOT NULL
);

CREATE UNIQUE index listener_blocks_block_height ON listener_blocks (block_height DESC);

CREATE TABLE listener_contracts (
    contract_address NUMERIC(20) PRIMARY KEY NOT NULL,
    module_ref VARCHAR NOT NULL,
    contract_name VARCHAR NOT NULL,
    owner VARCHAR NOT NULL,
    processor_type INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE listener_transactions (
    transaction_hash VARCHAR PRIMARY KEY NOT NULL,
    block_hash bytea NOT NULL,
    -- do not use references listener_blocks (block_hash) because the block may not be in the db yet
    block_height NUMERIC(20) NOT NULL,
    -- do not use references listener_blocks (block_height) because the block may not be in the db yet
    block_slot_time TIMESTAMP NOT NULL,
    transaction_index NUMERIC(20) NOT NULL
);

CREATE TABLE listener_contract_calls (
    id bigserial PRIMARY KEY,
    transaction_hash bytea NOT NULL,
    -- do not use references listener_transactions (transaction_hash) because the transaction may not be in the db yet
    contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address),
    entrypoint_name VARCHAR NOT NULL,
    ccd_amount NUMERIC(20) NOT NULL,
    instigator VARCHAR NOT NULL,
    sender VARCHAR NOT NULL,
    events_count INT NOT NULL,
    call_type INT NOT NULL,
    is_processed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL
);
