-- Your SQL goes here

-- Drop views
DROP VIEW IF EXISTS forest_project_user_investment_amounts;
DROP VIEW IF EXISTS forest_project_trader;
DROP VIEW IF EXISTS forest_project_investor;

-- Drop functions
DROP FUNCTION IF EXISTS user_currency_value_for_forest_project_owned_tokens_at(TEXT, TIMESTAMP, NUMERIC, NUMERIC);
DROP FUNCTION IF EXISTS user_exchange_input_amount(TEXT, TIMESTAMP, TIMESTAMP, NUMERIC, NUMERIC);
DROP FUNCTION IF EXISTS user_exchange_output_amount(TEXT, TIMESTAMP, TIMESTAMP, NUMERIC, NUMERIC);
DROP FUNCTION IF EXISTS user_exchange_profits(TEXT, TIMESTAMP, TIMESTAMP, NUMERIC, NUMERIC);
DROP FUNCTION IF EXISTS user_fund_investment_amount(TEXT, TIMESTAMP, TIMESTAMP, NUMERIC, NUMERIC);
DROP FUNCTION IF EXISTS user_fund_profits(TEXT, TIMESTAMP, TIMESTAMP, NUMERIC, NUMERIC);
DROP FUNCTION IF EXISTS user_token_manual_transfer_profits(TEXT, TIMESTAMP, TIMESTAMP, NUMERIC, NUMERIC);
