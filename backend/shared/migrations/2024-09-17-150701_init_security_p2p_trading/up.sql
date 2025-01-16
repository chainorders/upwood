/* plpgsql-language-server:disable validation */
CREATE TABLE security_p2p_trading_contracts (
       contract_address NUMERIC(20) PRIMARY KEY NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       currency_token_contract_address NUMERIC(20) NOT NULL,
       currency_token_id NUMERIC(20) NOT NULL,
       total_sell_currency_amount NUMERIC(78) NOT NULL DEFAULT 0,
       create_time TIMESTAMP NOT NULL
);

CREATE TABLE security_p2p_trading_markets (
       contract_address NUMERIC(20) NOT NULL REFERENCES security_p2p_trading_contracts (contract_address) ON DELETE cascade,
       token_id NUMERIC(20) NOT NULL,
       token_contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       liquidity_provider VARCHAR NOT NULL,
       buy_rate_numerator NUMERIC(78) NOT NULL,
       buy_rate_denominator NUMERIC(78) NOT NULL,
       sell_rate_numerator NUMERIC(78) NOT NULL,
       sell_rate_denominator NUMERIC(78) NOT NULL,
       total_sell_token_amount NUMERIC(78) NOT NULL DEFAULT 0,
       total_sell_currency_amount NUMERIC(78) NOT NULL DEFAULT 0,
       create_time TIMESTAMP NOT NULL,
       update_time TIMESTAMP NOT NULL,
       PRIMARY KEY (
              contract_address,
              token_id,
              token_contract_address
       )
);

CREATE TABLE security_p2p_trading_traders (
       contract_address NUMERIC(20) NOT NULL REFERENCES security_p2p_trading_contracts (contract_address) ON DELETE cascade,
       token_id NUMERIC(20) NOT NULL,
       token_contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       trader VARCHAR NOT NULL,
       token_in_amount NUMERIC(78) NOT NULL DEFAULT 0,
       currency_out_amount NUMERIC(78) NOT NULL DEFAULT 0,
       token_out_amount NUMERIC(78) NOT NULL DEFAULT 0,
       currency_in_amount NUMERIC(78) NOT NULL DEFAULT 0,
       create_time TIMESTAMP NOT NULL,
       update_time TIMESTAMP NOT NULL,
       PRIMARY KEY (
              contract_address,
              token_id,
              token_contract_address,
              trader
       )
);

CREATE TABLE security_p2p_exchange_records (
       id uuid PRIMARY KEY NOT NULL,
       block_height NUMERIC(20) NOT NULL,
       txn_index NUMERIC(20) NOT NULL,
       contract_address NUMERIC(20) NOT NULL REFERENCES security_p2p_trading_contracts (contract_address) ON DELETE cascade,
       token_id NUMERIC(20) NOT NULL,
       token_contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
       buyer VARCHAR NOT NULL,
       seller VARCHAR NOT NULL,
       currency_amount NUMERIC(78) NOT NULL,
       token_amount NUMERIC(78) NOT NULL,
       rate NUMERIC(40, 20) NOT NULL,
       create_time TIMESTAMP NOT NULL,
       FOREIGN KEY (
              contract_address,
              token_id,
              token_contract_address,
              buyer
       ) REFERENCES security_p2p_trading_traders (
              contract_address,
              token_id,
              token_contract_address,
              trader
       ),
       FOREIGN KEY (
              contract_address,
              token_id,
              token_contract_address,
              seller
       ) REFERENCES security_p2p_trading_traders (
              contract_address,
              token_id,
              token_contract_address,
              trader
       )
);
