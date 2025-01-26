/* plpgsql-language-server:disable validation */
-- Drop views in reverse dependency order
DROP VIEW IF EXISTS forest_project_token_contract_user_yield_distribution_agg CASCADE;

DROP VIEW IF EXISTS forest_project_user_yield_distributions CASCADE;

DROP VIEW IF EXISTS user_yields_aggregate CASCADE;

DROP VIEW IF EXISTS forest_project_token_contract_user_yields CASCADE;

DROP VIEW IF EXISTS forest_project_token_user_yields CASCADE;

DROP VIEW IF EXISTS forest_project_user_balance_agg CASCADE;

DROP VIEW IF EXISTS forest_project_token_contract_user_balance_agg CASCADE;

DROP VIEW IF EXISTS forest_project_current_token_fund_markets CASCADE;

DROP VIEW IF EXISTS forest_project_supply CASCADE;

DROP VIEW IF EXISTS forest_project_funds_affiliate_reward_records CASCADE;

DROP VIEW IF EXISTS forest_project_funds_investment_records CASCADE;

DROP VIEW IF EXISTS forest_project_fund_investor CASCADE;

DROP VIEW IF EXISTS forest_project_funds CASCADE;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS forest_project_token_contracts CASCADE;

DROP TABLE IF EXISTS token_metadatas CASCADE;

DROP TABLE IF EXISTS forest_project_prices CASCADE;

-- Drop types last
DROP TYPE IF EXISTS forest_project_security_token_contract_type;
