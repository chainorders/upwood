DELETE FROM security_p2p_trading_contracts;

DELETE FROM security_p2p_trading_markets;

DELETE FROM security_p2p_exchange_records;

DELETE FROM security_p2p_trading_traders;

ALTER TABLE security_p2p_exchange_records
DROP COLUMN IF EXISTS exchange_record_type;

DROP TYPE IF EXISTS security_p2p_trading_exchange_record_type CASCADE;

ALTER TABLE security_p2p_trading_markets
DROP CONSTRAINT IF EXISTS security_p2p_trading_markets_pkey;

ALTER TABLE security_p2p_trading_markets
ADD COLUMN IF NOT EXISTS total_sell_token_amount NUMERIC(78) NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_sell_currency_amount NUMERIC(78) NOT NULL DEFAULT 0;

ALTER TABLE security_p2p_trading_markets
DROP COLUMN IF EXISTS token_id_calculation_diff_millis,
DROP COLUMN IF EXISTS token_id_calculation_start,
DROP COLUMN IF EXISTS market_type,
DROP COLUMN IF EXISTS max_token_amount,
DROP COLUMN IF EXISTS max_currency_amount,
DROP COLUMN IF EXISTS token_in_amount,
DROP COLUMN IF EXISTS currency_out_amount,
DROP COLUMN IF EXISTS token_out_amount,
DROP COLUMN IF EXISTS currency_in_amount,
DROP COLUMN IF EXISTS token_id_calculation_base_token_id;

ALTER TABLE security_p2p_trading_markets
ALTER COLUMN sell_rate_denominator
SET NOT NULL,
ALTER COLUMN sell_rate_numerator
SET NOT NULL,
ALTER COLUMN buy_rate_denominator
SET NOT NULL,
ALTER COLUMN buy_rate_numerator
SET NOT NULL,
ALTER COLUMN token_id
SET NOT NULL;

DROP TYPE IF EXISTS security_p2p_trading_market_type CASCADE;

ALTER TABLE security_p2p_trading_markets
ADD CONSTRAINT security_p2p_trading_markets_pkey PRIMARY KEY (
    contract_address,
    token_id,
    token_contract_address
);

ALTER TABLE forest_project_token_contracts
ADD COLUMN market_token_id NUMERIC(20);
