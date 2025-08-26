CREATE TABLE security_sft_multi_yielder_treasuries (
    contract_address NUMERIC(20) PRIMARY KEY NOT NULL,
    treasury_address VARCHAR NOT NULL,
    create_time TIMESTAMP NOT NULL,
    update_time TIMESTAMP NOT NULL,
    CONSTRAINT chk_treasury_address CHECK (treasury_address <> '')
)
