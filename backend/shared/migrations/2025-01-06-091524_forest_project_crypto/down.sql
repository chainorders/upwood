/* plpgsql-language-server:disable validation */
-- Drop views in reverse dependency order
DROP VIEW IF EXISTS forest_project_user_balance_agg CASCADE;

DROP VIEW IF EXISTS forest_project_token_contract_user_balance_agg CASCADE;

DROP VIEW IF EXISTS forest_project_supply CASCADE;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS forest_project_token_contracts CASCADE;

DROP TABLE IF EXISTS token_metadatas CASCADE;

DROP TABLE IF EXISTS forest_project_prices CASCADE;

-- Drop types last
DROP TYPE IF EXISTS forest_project_security_token_contract_type;
