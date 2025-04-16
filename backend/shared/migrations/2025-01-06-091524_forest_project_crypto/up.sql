CREATE TABLE forest_project_prices (
     project_id uuid NOT NULL REFERENCES forest_projects (id),
     price NUMERIC(78) NOT NULL,
     price_at TIMESTAMP NOT NULL DEFAULT NOW(),
     currency_token_id NUMERIC(20) NOT NULL,
     currency_token_contract_address NUMERIC(20) NOT NULL,
     created_at TIMESTAMP NOT NULL DEFAULT NOW(),
     updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
     PRIMARY KEY (project_id, price_at)
);

CREATE TYPE forest_project_security_token_contract_type AS ENUM(
     'property',
     'bond',
     'property_pre_sale',
     'bond_pre_sale'
);

CREATE TABLE forest_project_token_contracts (
     contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE CASCADE,
     forest_project_id uuid NOT NULL REFERENCES forest_projects (id) ON DELETE CASCADE,
     contract_type forest_project_security_token_contract_type NOT NULL,
     fund_token_id NUMERIC(20),
     market_token_id NUMERIC(20),
     symbol VARCHAR(10) NOT NULL DEFAULT '',
     decimals INTEGER NOT NULL DEFAULT 0,
     metadata_url VARCHAR NOT NULL,
     metadata_hash VARCHAR,
     created_at TIMESTAMP NOT NULL DEFAULT NOW(),
     updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
     PRIMARY KEY (forest_project_id, contract_type),
     UNIQUE (contract_address)
);

-- Token Metadatas for all the tokens which are not Forest Project Security Tokens
CREATE TABLE token_metadatas (
     contract_address NUMERIC(20) NOT NULL,
     token_id NUMERIC(20) NOT NULL,
     symbol VARCHAR(10),
     decimals INTEGER,
     PRIMARY KEY (contract_address, token_id)
);

-- Forest Project Aggregate Supply across all tokens
CREATE VIEW forest_project_supply AS
SELECT
     forest_projects.id AS forest_project_id,
     forest_projects.state AS forest_project_state,
     SUM(token.supply) AS supply,
     COALESCE(token_contract.symbol, '') AS symbol,
     COALESCE(token_contract.decimals, 0) AS decimals
FROM
     forest_projects
     JOIN forest_project_token_contracts token_contract ON forest_projects.id = token_contract.forest_project_id
     JOIN cis2_tokens token ON token_contract.contract_address = token.cis2_address
GROUP BY
     forest_projects.id,
     token_contract.symbol,
     token_contract.decimals;

CREATE VIEW forest_project_token_contract_user_balance_agg AS
SELECT
     forest_projects.id AS forest_project_id,
     forest_projects.state AS forest_project_state,
     forest_projects.name AS forest_project_name,
     usr.cognito_user_id,
     token_contract.contract_address,
     token_contract.contract_type,
     COALESCE(token_contract.symbol, '') AS token_symbol,
     COALESCE(token_contract.decimals, 0) AS token_decimals,
     SUM(holder.un_frozen_balance + holder.frozen_balance) AS total_balance,
     SUM(holder.un_frozen_balance) AS un_frozen_balance
FROM
     forest_projects
     JOIN forest_project_token_contracts token_contract ON forest_projects.id = token_contract.forest_project_id
     JOIN cis2_token_holders holder ON token_contract.contract_address = holder.cis2_address
     JOIN users usr ON holder.holder_address = usr.account_address
GROUP BY
     forest_projects.id,
     forest_projects.state,
     forest_projects.name,
     usr.cognito_user_id,
     token_contract.forest_project_id,
     token_contract.contract_address,
     token_contract.contract_type,
     token_contract.symbol,
     token_contract.decimals;

CREATE VIEW forest_project_user_balance_agg AS
SELECT
     usr.cognito_user_id,
     forest_projects.id AS forest_project_id,
     SUM(holder.un_frozen_balance + holder.frozen_balance) AS total_balance
FROM
     forest_projects
     JOIN forest_project_token_contracts token_contract ON forest_projects.id = token_contract.forest_project_id
     JOIN cis2_token_holders holder ON token_contract.contract_address = holder.cis2_address
     JOIN users usr ON holder.holder_address = usr.account_address
GROUP BY
     forest_projects.id,
     usr.cognito_user_id;
