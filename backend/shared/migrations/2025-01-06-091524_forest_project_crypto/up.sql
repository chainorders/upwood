/* plpgsql-language-server:disable validation */
CREATE TYPE forest_project_security_token_contract_type AS ENUM(
     'property',
     'bond',
     'property_pre_sale',
     'bond_pre_sale'
);

CREATE TABLE forest_project_token_contracts (
     contract_address NUMERIC(20) NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
     token_id NUMERIC(20),
     forest_project_id uuid NOT NULL REFERENCES forest_projects (id) ON DELETE cascade,
     contract_type forest_project_security_token_contract_type NOT NULL,
     PRIMARY KEY (forest_project_id, contract_type)
);

-- Forest project crypto information
CREATE VIEW forest_project_supply AS
SELECT
     forest_projects.id AS forest_project_id,
     SUM(token.supply) AS supply
FROM
     forest_projects
     JOIN forest_project_token_contracts token_contract ON forest_projects.id = token_contract.forest_project_id
     JOIN cis2_tokens token ON token_contract.contract_address = token.cis2_address
GROUP BY
     forest_projects.id;

-- All the security tokens for all the contracts
CREATE VIEW forest_project_security_tokens AS
SELECT
     token.*,
     forest_projects.id AS forest_project_id,
     token_contract.contract_type,
     token.token_id = token_contract.token_id AS is_default
FROM
     forest_projects
     JOIN forest_project_token_contracts token_contract ON forest_projects.id = token_contract.forest_project_id
     JOIN cis2_tokens token ON token_contract.contract_address = token.cis2_address;

-- All currently active funds for forest projects
CREATE VIEW forest_project_funds AS
SELECT
     fund.*,
     token.forest_project_id,
     token.contract_type AS fund_type,
     token.is_default
FROM
     security_mint_funds fund
     JOIN forest_project_security_tokens token ON fund.investment_token_contract_address = token.cis2_address
     AND fund.investment_token_id = token.token_id;

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
     usr.cognito_user_id AS investor_cognito_user_id,
     investment_record.investment_token_id = token_contract.token_id AS is_default
FROM
     security_mint_fund_investment_records investment_record
     JOIN forest_project_token_contracts token_contract ON investment_record.investment_token_contract_address = token_contract.contract_address
     JOIN users usr ON investment_record.investor = usr.account_address;

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
     claims.id AS claim_id,
     claims.contract_address AS claims_contract_address,
     COALESCE(claims.reward_amount, 0) AS reward_amount,
     investment_record.currency_amount * affiliate.affiliate_commission - COALESCE(claims.reward_amount, 0) AS remaining_reward_amount,
     affiliate.cognito_user_id AS affiliate_cognito_user_id,
     affiliate.affiliate_commission
FROM
     forest_project_funds_investment_records investment_record
     JOIN user_affiliates AS user_affiliate ON investor_cognito_user_id = user_affiliate.cognito_user_id
     JOIN users AS affiliate ON user_affiliate.affiliate_account_address = affiliate.account_address
     LEFT JOIN offchain_reward_claims AS claims ON claims.reward_id = DECODE(
          REPLACE(investment_record.id::TEXT, '-', ''),
          'hex'
     )
WHERE
     investment_record.investment_record_type = 'claimed';

-- Active forest projects
CREATE VIEW active_forest_projects AS
SELECT
     forest_projects.*,
     token_contracts.contract_address,
     token_contracts.token_id,
     COALESCE(supply.supply, 0) AS total_supply,
     fund.contract_address AS fund_contract_address,
     fund.token_contract_address AS pre_sale_token_contract_address,
     fund.token_id AS pre_sale_token_id,
     fund.rate_numerator AS fund_rate_numerator,
     fund.rate_denominator AS fund_rate_denominator
FROM
     forest_projects
     LEFT JOIN forest_project_supply supply ON supply.forest_project_id = forest_projects.id
     LEFT JOIN forest_project_token_contracts token_contracts ON forest_projects.id = token_contracts.forest_project_id
     AND token_contracts.contract_type = 'property'
     AND token_contracts.token_id IS NOT NULL
     LEFT JOIN security_mint_funds fund ON token_contracts.contract_address = fund.investment_token_contract_address
     AND token_contracts.token_id = fund.investment_token_id
WHERE
     forest_projects.state = 'active'
ORDER BY
     forest_projects.created_at DESC;

-- Active forest project user
CREATE VIEW active_forest_project_users AS
SELECT
     project.*,
     notification.id AS notification_id,
     notification.cognito_user_id AS cognito_user_id,
     signature IS NOT NULL AS has_signed_contract
FROM
     active_forest_projects project
     LEFT JOIN forest_project_notifications notification ON project.id = notification.project_id
     LEFT JOIN forest_project_legal_contract_user_signatures signature ON project.id = signature.project_id
WHERE
     signature.cognito_user_id = notification.cognito_user_id
     OR notification.cognito_user_id IS NULL
ORDER BY
     project.created_at DESC;

-- Funded forest projects
CREATE VIEW funded_forest_projects AS
SELECT
     forest_projects.*,
     token_contract.contract_address AS token_contract_address,
     token_contract.token_id AS token_id,
     COALESCE(supply.supply, 0) AS total_supply,
     market.contract_address AS market_contract_address,
     market.liquidity_provider AS market_liquidity_provider,
     market.sell_rate_numerator AS market_sell_rate_numerator,
     market.sell_rate_denominator AS market_sell_rate_denominator
FROM
     forest_projects
     LEFT JOIN forest_project_supply supply ON supply.forest_project_id = forest_projects.id
     LEFT JOIN forest_project_token_contracts token_contract ON forest_projects.id = token_contract.forest_project_id
     AND token_contract.contract_type = 'property'
     LEFT JOIN security_p2p_trading_markets market ON token_contract.contract_address = market.token_contract_address
WHERE
     forest_projects.state = 'funded'
ORDER BY
     forest_projects.created_at DESC;

-- Funded forest project user
CREATE VIEW funded_forest_project_users AS
SELECT
     project.*,
     notification.id AS notification_id,
     notification.cognito_user_id AS cognito_user_id,
     signature IS NOT NULL AS has_signed_contract
FROM
     funded_forest_projects project
     LEFT JOIN forest_project_notifications notification ON project.id = notification.project_id
     LEFT JOIN forest_project_legal_contract_user_signatures signature ON project.id = signature.project_id
WHERE
     signature.cognito_user_id = notification.cognito_user_id
     OR notification.cognito_user_id IS NULL
ORDER BY
     project.created_at DESC;

-- Forest projects owned by user
CREATE VIEW forest_projects_owned_by_user AS
SELECT
     project.*,
     usr.cognito_user_id,
     usr.account_address,
     SUM(holder.un_frozen_balance + holder.frozen_balance) AS total_balance,
     property_token_contract.contract_address AS property_contract_address,
     property_token_contract.token_id AS property_token_id,
     property_market.contract_address AS market_contract_address,
     property_market.liquidity_provider AS market_liquidity_provider,
     property_market.buy_rate_numerator AS market_buy_rate_numerator,
     property_market.buy_rate_denominator AS market_buy_rate_denominator,
     bond_token_contract.contract_address AS bond_contract_address,
     bond_token_contract.token_id AS bond_token_id,
     bond_fund.contract_address AS bond_fund_contract_address,
     bond_fund.rate_numerator AS bond_fund_rate_numerator,
     bond_fund.rate_denominator AS bond_fund_rate_denominator
FROM
     forest_projects project
     JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
     -- user information
     JOIN cis2_token_holders holder ON token_contract.contract_address = holder.cis2_address
     JOIN users usr ON holder.holder_address = usr.account_address
     -- sell information
     LEFT JOIN forest_project_token_contracts property_token_contract ON project.id = property_token_contract.forest_project_id
     AND property_token_contract.contract_type = 'property'
     AND property_token_contract.contract_address = token_contract.contract_address
     LEFT JOIN security_p2p_trading_markets property_market ON property_token_contract.contract_address = property_market.token_contract_address
     AND property_market.token_id = property_token_contract.token_id
     -- bond fund information
     LEFT JOIN forest_project_token_contracts bond_token_contract ON project.id = bond_token_contract.forest_project_id
     AND bond_token_contract.contract_type = 'bond'
     AND bond_token_contract.contract_address = token_contract.contract_address
     LEFT JOIN security_mint_funds bond_fund ON bond_token_contract.contract_address = bond_fund.investment_token_contract_address
     AND bond_fund.investment_token_id = bond_token_contract.token_id
     AND bond_fund.fund_state = 'open'
WHERE
     holder.un_frozen_balance + holder.frozen_balance > 0
GROUP BY
     project.id,
     usr.cognito_user_id,
     usr.account_address,
     property_token_contract.contract_address,
     property_token_contract.token_id,
     property_market.contract_address,
     property_market.liquidity_provider,
     property_market.buy_rate_numerator,
     property_market.buy_rate_denominator,
     bond_token_contract.contract_address,
     bond_token_contract.token_id,
     bond_fund.contract_address,
     bond_fund.rate_numerator,
     bond_fund.rate_denominator
ORDER BY
     project.created_at DESC;

-- Yeilds for Forest Project, Tokens, Token Owner (User)
CREATE VIEW forest_project_user_yields_for_each_owned_token AS
SELECT
     project.id AS forest_project_id,
     holder.token_id,
     holder.cis2_address AS token_contract_address,
     holder.holder_address,
     holder.un_frozen_balance AS token_balance,
     usr.cognito_user_id,
     yield.contract_address AS yielder_contract_address,
     yield.yield_token_id,
     yield.yield_contract_address,
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
     AND yield.token_id >= holder.token_id
     JOIN users usr ON holder.holder_address = usr.account_address
ORDER BY
     token_id DESC,
     token_contract_address DESC;

-- Aggregate yeilds for Forest Project, Token Owner (User)
CREATE VIEW forest_project_user_yields_aggregate AS
SELECT
     yield.cognito_user_id,
     yield.yielder_contract_address,
     yield.yield_token_id,
     yield.yield_contract_address,
     SUM(yield.yield_amount) AS yield_amount
FROM
     forest_project_user_yields_for_each_owned_token yield
GROUP BY
     yield.cognito_user_id,
     yield.yielder_contract_address,
     yield.yield_token_id,
     yield.yield_contract_address
ORDER BY
     yield.yielder_contract_address DESC,
     yield.yield_token_id DESC;
