/* plpgsql-language-server:disable validation */
CREATE TYPE security_sft_multi_yielder_yield_type AS ENUM('quantity', 'simple_intrest');

CREATE TABLE security_sft_multi_yielder_yields (
       contract_address NUMERIC(20) NOT NULL,
       -- security token and contract
       token_contract_address NUMERIC(20) NOT NULL,
       token_id NUMERIC(20) NOT NULL,
       -- token and contract which will be yielded
       yield_contract_address NUMERIC(20) NOT NULL,
       yield_token_id NUMERIC(20) NOT NULL,
       yield_rate_numerator NUMERIC(20) NOT NULL,
       yield_rate_denominator NUMERIC(20) NOT NULL,
       yield_type security_sft_multi_yielder_yield_type NOT NULL,
       create_time TIMESTAMP NOT NULL,
       PRIMARY KEY (
              contract_address,
              token_contract_address,
              token_id,
              yield_contract_address,
              yield_token_id
       )
);

CREATE TABLE security_sft_multi_yielder_yeild_distributions (
       id uuid PRIMARY KEY,
       contract_address NUMERIC(20) NOT NULL,
       token_contract_address NUMERIC(20) NOT NULL,
       from_token_version NUMERIC(20) NOT NULL,
       to_token_version NUMERIC(20) NOT NULL,
       token_amount NUMERIC(78) NOT NULL,
       yield_contract_address NUMERIC(20) NOT NULL,
       yield_token_id NUMERIC(20) NOT NULL,
       yield_amount NUMERIC(78) NOT NULL,
       to_address VARCHAR NOT NULL,
       create_time TIMESTAMP NOT NULL
);
