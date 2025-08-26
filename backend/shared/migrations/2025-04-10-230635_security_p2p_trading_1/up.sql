DELETE FROM security_p2p_trading_contracts;

DELETE FROM security_p2p_trading_markets;

DELETE FROM security_p2p_exchange_records;

DELETE FROM security_p2p_trading_traders;

DROP VIEW IF EXISTS forest_project_current_token_fund_markets CASCADE;

ALTER TABLE forest_project_token_contracts
DROP COLUMN market_token_id;

CREATE TYPE security_p2p_trading_market_type AS ENUM('transfer', 'mint');

-- Remove the primary key constraint
ALTER TABLE security_p2p_trading_markets
DROP CONSTRAINT security_p2p_trading_markets_pkey;

-- Drop NOT NULL constraints
ALTER TABLE security_p2p_trading_markets
ALTER COLUMN token_id
DROP NOT NULL,
ALTER COLUMN buy_rate_numerator
DROP NOT NULL,
ALTER COLUMN buy_rate_denominator
DROP NOT NULL,
ALTER COLUMN sell_rate_numerator
DROP NOT NULL,
ALTER COLUMN sell_rate_denominator
DROP NOT NULL;

-- Add new columns
ALTER TABLE security_p2p_trading_markets
ADD COLUMN token_id_calculation_start NUMERIC(78),
ADD COLUMN token_id_calculation_diff_millis NUMERIC(78),
ADD COLUMN token_id_calculation_base_token_id NUMERIC(20),
ADD COLUMN market_type security_p2p_trading_market_type NOT NULL,
ADD COLUMN max_token_amount NUMERIC(78) NOT NULL,
ADD COLUMN max_currency_amount NUMERIC(78),
ADD COLUMN token_in_amount NUMERIC(78) NOT NULL DEFAULT 0,
ADD COLUMN currency_out_amount NUMERIC(78) NOT NULL DEFAULT 0,
ADD COLUMN token_out_amount NUMERIC(78) NOT NULL DEFAULT 0,
ADD COLUMN currency_in_amount NUMERIC(78) NOT NULL DEFAULT 0;

-- Drop old columns
ALTER TABLE security_p2p_trading_markets
DROP COLUMN IF EXISTS total_sell_token_amount,
DROP COLUMN IF EXISTS total_sell_currency_amount;

-- Re-add the primary key constraint
ALTER TABLE security_p2p_trading_markets
ADD CONSTRAINT security_p2p_trading_markets_pkey PRIMARY KEY (contract_address, token_contract_address);

CREATE TYPE security_p2p_trading_exchange_record_type AS ENUM('buy', 'sell', 'mint');

ALTER TABLE security_p2p_exchange_records
ADD COLUMN exchange_record_type security_p2p_trading_exchange_record_type NOT NULL;
