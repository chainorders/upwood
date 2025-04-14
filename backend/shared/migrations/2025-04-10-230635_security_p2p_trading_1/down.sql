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

CREATE VIEW forest_project_current_token_fund_markets AS
SELECT
    forest_projects.id AS forest_project_id,
    forest_projects.state AS forest_project_state,
    token_contract.contract_address AS token_contract_address,
    token_contract.fund_token_id AS token_id,
    token_contract.contract_type AS token_contract_type,
    token_contract.market_token_id AS market_token_id,
    COALESCE(token_contract.symbol, '') AS token_symbol,
    COALESCE(token_contract.decimals, 0) AS token_decimals,
    fund.contract_address AS fund_contract_address,
    fund.rate_numerator AS fund_rate_numerator,
    fund.rate_denominator AS fund_rate_denominator,
    fund.fund_state,
    fund.token_contract_address AS fund_token_contract_address,
    fund.token_id AS fund_token_id,
    market.contract_address AS market_contract_address,
    market.sell_rate_numerator AS market_sell_rate_numerator,
    market.sell_rate_denominator AS market_sell_rate_denominator,
    market.buy_rate_numerator AS market_buy_rate_numerator,
    market.buy_rate_denominator AS market_buy_rate_denominator,
    market.liquidity_provider AS market_liquidity_provider
FROM
    forest_projects
    JOIN forest_project_token_contracts token_contract ON forest_projects.id = token_contract.forest_project_id
    LEFT JOIN security_mint_funds fund ON fund.investment_token_contract_address = token_contract.contract_address
    AND fund.investment_token_id = token_contract.fund_token_id
    LEFT JOIN security_p2p_trading_markets market ON market.token_contract_address = token_contract.contract_address
    AND market.token_id = token_contract.market_token_id
ORDER BY
    token_contract.created_at DESC;
