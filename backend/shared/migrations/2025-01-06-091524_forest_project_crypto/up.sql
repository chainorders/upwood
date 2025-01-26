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
     contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
     forest_project_id uuid NOT NULL REFERENCES forest_projects (id) ON DELETE cascade,
     contract_type forest_project_security_token_contract_type NOT NULL,
     fund_token_id NUMERIC(20),
     market_token_id NUMERIC(20),
     symbol VARCHAR(10),
     decimals INTEGER,
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

-- All currently active funds for forest projects
CREATE VIEW forest_project_funds AS
SELECT
     fund.*,
     token_contract.forest_project_id,
     token_contract.contract_type AS fund_type,
     COALESCE(token_contract.symbol, '') AS investment_token_symbol,
     COALESCE(token_contract.decimals, 0) AS investment_token_decimals,
     COALESCE(currency_token_metadata.symbol, '') AS currency_token_symbol,
     COALESCE(currency_token_metadata.decimals, 0) AS currency_token_decimals,
     COALESCE(fund_token_contract.symbol, '') AS token_symbol,
     COALESCE(fund_token_contract.decimals, 0) AS token_decimals
FROM
     security_mint_funds fund
     JOIN forest_project_token_contracts token_contract ON fund.investment_token_contract_address = token_contract.contract_address
     LEFT JOIN token_metadatas currency_token_metadata ON fund.currency_token_id = currency_token_metadata.token_id
     AND fund.currency_token_contract_address = currency_token_metadata.contract_address
     AND fund.investment_token_id = token_contract.fund_token_id
     LEFT JOIN forest_project_token_contracts fund_token_contract ON fund.token_contract_address = fund_token_contract.contract_address;

-- All investors for all the currently active funds
CREATE VIEW forest_project_fund_investor AS
SELECT
     fund.forest_project_id,
     fund.contract_address AS fund_contract_address,
     fund.token_id AS fund_token_id,
     fund.token_contract_address AS fund_token_contract_address,
     fund.investment_token_id AS investment_token_id,
     fund.investment_token_contract_address AS investment_token_contract_address,
     fund.fund_type,
     fund.currency_token_id AS currency_token_id,
     fund.currency_token_contract_address AS currency_token_contract_address,
     fund.currency_token_symbol AS currency_token_symbol,
     fund.currency_token_decimals AS currency_token_decimals,
     fund.investment_token_symbol AS investment_token_symbol,
     fund.investment_token_decimals AS investment_token_decimals,
     fund.token_symbol AS fund_token_symbol,
     fund.token_decimals AS fund_token_decimals,
     investor.investor AS investor_account_address,
     investor.token_amount AS investment_token_amount,
     investor.currency_amount AS investment_currency_amount,
     usr.cognito_user_id AS investor_cognito_user_id,
     usr.email AS investor_email
FROM
     forest_project_funds fund
     JOIN security_mint_fund_investors investor ON fund.investment_token_contract_address = investor.investment_token_contract_address
     AND fund.token_id = investor.investment_token_id
     AND investor.token_amount > 0
     JOIN users usr ON investor.investor = usr.account_address;

-- All investment records for all the funds
CREATE VIEW forest_project_funds_investment_records AS
SELECT
     investment_record.*,
     token_contract.contract_type AS fund_type,
     token_contract.forest_project_id AS forest_project_id,
     COALESCE(token_contract.symbol, '') AS investment_token_symbol,
     COALESCE(token_contract.decimals, 0) AS investment_token_decimals,
     usr.cognito_user_id AS investor_cognito_user_id,
     investment_record.investment_token_id = token_contract.fund_token_id AS is_default,
     COALESCE(currency_token_metadata.symbol, '') AS currency_token_symbol,
     COALESCE(currency_token_metadata.decimals, 0) AS currency_token_decimals
FROM
     security_mint_fund_investment_records investment_record
     JOIN forest_project_token_contracts token_contract ON investment_record.investment_token_contract_address = token_contract.contract_address
     JOIN users usr ON investment_record.investor = usr.account_address
     LEFT JOIN token_metadatas currency_token_metadata ON investment_record.currency_token_id = currency_token_metadata.token_id
     AND investment_record.currency_token_contract_address = currency_token_metadata.contract_address;

-- All affiliate reward records for forest projects including property and bond funds
CREATE VIEW forest_project_funds_affiliate_reward_records AS
SELECT
     investment_record.id AS investment_record_id,
     investment_record.contract_address AS fund_contract_address,
     investment_record.investment_token_contract_address,
     investment_record.investment_token_id,
     investment_record.fund_type,
     investment_record.forest_project_id,
     investment_record.is_default,
     investment_record.investor_cognito_user_id,
     investment_record.investor AS investor_account_address,
     investment_record.currency_token_id,
     investment_record.currency_token_contract_address,
     investment_record.currency_amount,
     investment_record.token_amount,
     investment_record.investment_token_symbol,
     investment_record.investment_token_decimals,
     investment_record.currency_token_symbol,
     investment_record.currency_token_decimals,
     claims.id AS claim_id,
     claims.contract_address AS claims_contract_address,
     COALESCE(claims.reward_amount, 0) AS reward_amount,
     investment_record.currency_amount * affiliate.affiliate_commission - COALESCE(claims.reward_amount, 0) AS remaining_reward_amount,
     affiliate.cognito_user_id AS affiliate_cognito_user_id,
     affiliate.affiliate_commission
FROM
     forest_project_funds_investment_records investment_record
     JOIN users AS usr ON investor_cognito_user_id = usr.cognito_user_id
     JOIN users AS affiliate ON usr.affiliate_account_address = affiliate.account_address
     LEFT JOIN offchain_reward_claims AS claims ON claims.reward_id = DECODE(
          REPLACE(investment_record.id::TEXT, '-', ''),
          'hex'
     )
WHERE
     investment_record.investment_record_type = 'claimed';

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

-- Yeilds for Forest Project, Tokens, Token Owner (User)
CREATE VIEW forest_project_token_user_yields AS
SELECT
     project.id AS forest_project_id,
     COALESCE(token_contract.symbol, '') AS token_symbol,
     COALESCE(token_contract.decimals, 0) AS token_decimals,
     holder.token_id,
     holder.cis2_address AS token_contract_address,
     holder.holder_address,
     holder.un_frozen_balance AS token_balance,
     usr.cognito_user_id,
     yield.contract_address AS yielder_contract_address,
     yield.yield_token_id,
     yield.yield_contract_address,
     COALESCE(yielded_token_metadata.symbol, '') AS yield_token_symbol,
     COALESCE(yielded_token_metadata.decimals, 0) AS yield_token_decimals,
     CASE
          WHEN yield.yield_type = 'quantity' THEN holder.un_frozen_balance * yield.yield_rate_numerator / yield.yield_rate_denominator
          WHEN yield.yield_type = 'simple_intrest' THEN holder.un_frozen_balance * yield.yield_rate_numerator * (yield.token_id - holder.token_id) / yield.yield_rate_denominator
          ELSE 0
     END AS yield_amount,
     (
          SELECT
               FIRST_VALUE(yield.token_id) OVER (
                    PARTITION BY
                         yield.contract_address,
                         yield.token_contract_address
                    ORDER BY
                         yield.token_id DESC
               )
     ) AS max_token_id
FROM
     forest_projects project
     JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
     JOIN cis2_token_holders holder ON token_contract.contract_address = holder.cis2_address
     AND holder.un_frozen_balance > 0
     JOIN security_sft_multi_yielder_yields yield ON token_contract.contract_address = yield.token_contract_address
     AND yield.token_id > holder.token_id
     LEFT JOIN token_metadatas yielded_token_metadata ON yield.yield_token_id = yielded_token_metadata.token_id
     AND yield.yield_contract_address = yielded_token_metadata.contract_address
     JOIN users usr ON holder.holder_address = usr.account_address
ORDER BY
     token_id DESC,
     token_contract_address DESC;

CREATE VIEW forest_project_token_contract_user_yields AS
SELECT
     forest_project_id,
     token_contract_address,
     token_symbol,
     token_decimals,
     cognito_user_id,
     yielder_contract_address,
     yield_token_id,
     yield_contract_address,
     yield_token_symbol,
     yield_token_decimals,
     SUM(yield_amount) AS yield_amount
FROM
     forest_project_token_user_yields
GROUP BY
     forest_project_id,
     token_contract_address,
     token_symbol,
     token_decimals,
     cognito_user_id,
     yielder_contract_address,
     yield_token_id,
     yield_contract_address,
     yield_token_symbol,
     yield_token_decimals
ORDER BY
     token_contract_address DESC,
     yield_token_id DESC;

-- Aggregate yeilds for Forest Project, Token Owner (User)
CREATE VIEW user_yields_aggregate AS
SELECT
     yield.cognito_user_id,
     yield.yielder_contract_address,
     yield.yield_token_id,
     yield.yield_contract_address,
     yield.yield_token_symbol,
     yield.yield_token_decimals,
     SUM(yield.yield_amount) AS yield_amount
FROM
     forest_project_token_user_yields yield
GROUP BY
     yield.cognito_user_id,
     yield.yielder_contract_address,
     yield.yield_token_id,
     yield.yield_contract_address,
     yield.yield_token_symbol,
     yield.yield_token_decimals
ORDER BY
     yield.yielder_contract_address DESC,
     yield.yield_token_id DESC;

-- Forest Project Yeild Distributions
CREATE VIEW forest_project_user_yield_distributions AS
SELECT
     project.id AS forest_project_id,
     project.name AS forest_project_name,
     token_contract.contract_type AS token_contract_type,
     usr.cognito_user_id,
     yd.id AS yield_distribution_id,
     yd.contract_address AS yielder_contract_address,
     yd.token_contract_address,
     yd.from_token_version,
     yd.to_token_version,
     yd.token_amount,
     COALESCE(token_contract.symbol, '') AS token_symbol,
     COALESCE(token_contract.decimals, 0) AS token_decimals,
     yd.yield_contract_address,
     yd.yield_token_id,
     yd.yield_amount,
     COALESCE(yielded_token_metadata.symbol, '') AS yield_token_symbol,
     COALESCE(yielded_token_metadata.decimals, 0) AS yield_token_decimals,
     yd.to_address,
     yd.create_time
FROM
     forest_projects project
     JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
     JOIN security_sft_multi_yielder_yeild_distributions yd ON token_contract.contract_address = yd.token_contract_address
     LEFT JOIN token_metadatas yielded_token_metadata ON yd.yield_token_id = yielded_token_metadata.token_id
     AND yd.yield_contract_address = yielded_token_metadata.contract_address
     JOIN users usr ON yd.to_address = usr.account_address
ORDER BY
     yd.create_time DESC,
     yd.contract_address DESC,
     yd.token_contract_address DESC,
     yd.yield_token_id DESC;

CREATE VIEW forest_project_token_contract_user_yield_distribution_agg AS
SELECT
     usr.cognito_user_id,
     token_contract.forest_project_id,
     token_contract.contract_address,
     token_contract.contract_type,
     COALESCE(token_contract.symbol, '') AS token_symbol,
     COALESCE(token_contract.decimals, 0) AS token_decimals,
     yd.contract_address as yielder_contract_address,
     yd.yield_contract_address,
     yd.yield_token_id,
     SUM(yd.yield_amount) AS total_yield_amount
FROM
     forest_project_token_contracts token_contract
     JOIN security_sft_multi_yielder_yeild_distributions yd ON token_contract.contract_address = yd.token_contract_address
     JOIN users usr ON yd.to_address = usr.account_address
GROUP BY
     usr.cognito_user_id,
     token_contract.forest_project_id,
     token_contract.contract_address,
     token_contract.contract_type,
     token_contract.symbol,
     token_contract.decimals,
     yd.contract_address,
     yd.yield_contract_address,
     yd.yield_token_id;
