/* plpgsql-language-server:disable validation */
CREATE TYPE security_mint_fund_state AS ENUM('open', 'success', 'fail');

CREATE TABLE security_mint_fund_contracts (
       contract_address NUMERIC(20) PRIMARY KEY NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       currency_token_contract_address NUMERIC(20) NOT NULL,
       currency_token_id NUMERIC(20) NOT NULL,
       create_time TIMESTAMP NOT NULL
);

CREATE TABLE security_mint_funds (
       contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       investment_token_id NUMERIC(20) NOT NULL,
       investment_token_contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       token_id NUMERIC(20) NOT NULL,
       token_contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       currency_token_id NUMERIC(20) NOT NULL,
       currency_token_contract_address NUMERIC(20) NOT NULL,
       currency_amount NUMERIC(78) NOT NULL DEFAULT 0,
       token_amount NUMERIC(78) NOT NULL DEFAULT 0,
       receiver_address VARCHAR,
       rate_numerator NUMERIC(78) NOT NULL,
       rate_denominator NUMERIC(78) NOT NULL,
       fund_state security_mint_fund_state NOT NULL,
       create_time TIMESTAMP NOT NULL,
       update_time TIMESTAMP NOT NULL,
       PRIMARY KEY (
              contract_address,
              investment_token_id,
              investment_token_contract_address
       )
);

CREATE TABLE security_mint_fund_investors (
       contract_address NUMERIC(20) NOT NULL REFERENCES security_mint_fund_contracts (contract_address) ON DELETE cascade,
       investment_token_id NUMERIC(20) NOT NULL,
       investment_token_contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       investor VARCHAR NOT NULL,
       currency_token_id NUMERIC(20) NOT NULL,
       currency_token_contract_address NUMERIC(20) NOT NULL,
       currency_amount NUMERIC(78) NOT NULL DEFAULT 0,
       currency_amount_total NUMERIC(78) NOT NULL DEFAULT 0,
       token_amount NUMERIC(78) NOT NULL DEFAULT 0,
       token_amount_total NUMERIC(78) NOT NULL DEFAULT 0,
       create_time TIMESTAMP NOT NULL,
       update_time TIMESTAMP NOT NULL,
       PRIMARY KEY (
              contract_address,
              investment_token_id,
              investment_token_contract_address,
              investor
       )
);

CREATE TYPE security_mint_fund_investment_record_type AS ENUM('invested', 'cancelled', 'claimed');

CREATE TABLE security_mint_fund_investment_records (
       id uuid PRIMARY KEY NOT NULL,
       block_height NUMERIC(20) NOT NULL,
       txn_index NUMERIC(20) NOT NULL,
       contract_address NUMERIC(20) NOT NULL REFERENCES security_mint_fund_contracts (contract_address) ON DELETE cascade,
       investment_token_id NUMERIC(20) NOT NULL,
       investment_token_contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       currency_token_id NUMERIC(20) NOT NULL,
       currency_token_contract_address NUMERIC(20) NOT NULL,
       investor VARCHAR NOT NULL,
       currency_amount NUMERIC(78) NOT NULL,
       token_amount NUMERIC(78) NOT NULL,
       currency_amount_balance NUMERIC(78) NOT NULL,
       token_amount_balance NUMERIC(78) NOT NULL,
       investment_record_type security_mint_fund_investment_record_type NOT NULL,
       create_time TIMESTAMP NOT NULL,
       FOREIGN Key (
              contract_address,
              investment_token_id,
              investment_token_contract_address,
              investor
       ) REFERENCES security_mint_fund_investors (
              contract_address,
              investment_token_id,
              investment_token_contract_address,
              investor
       )
);
