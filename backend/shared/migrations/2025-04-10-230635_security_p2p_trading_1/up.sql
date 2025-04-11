DROP VIEW IF EXISTS forest_project_current_token_fund_markets CASCADE;
ALTER TABLE forest_project_token_contracts DROP COLUMN market_token_id;

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
ADD COLUMN market_type security_p2p_trading_market_type NOT NULL;

-- Re-add the primary key constraint
DELETE FROM security_p2p_trading_markets;

ALTER TABLE security_p2p_trading_markets
ADD CONSTRAINT security_p2p_trading_markets_pkey PRIMARY KEY (contract_address, token_contract_address);
