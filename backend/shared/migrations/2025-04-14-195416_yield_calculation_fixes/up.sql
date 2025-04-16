DROP VIEW IF EXISTS user_yields_aggregate CASCADE;

DROP VIEW IF EXISTS forest_project_token_user_yields CASCADE;

DROP VIEW IF EXISTS forest_project_token_contract_user_yields CASCADE;

DROP VIEW IF EXISTS forest_project_funds CASCADE;

DROP VIEW IF EXISTS forest_project_funds_affiliate_reward_records CASCADE;

DROP VIEW IF EXISTS forest_project_funds_investment_records CASCADE;

CREATE OR REPLACE VIEW holder_yields AS
WITH
    yields AS (
        SELECT DISTINCT
            ON (
                yields.contract_address,
                yields.token_contract_address,
                yields.token_id,
                yields.yield_contract_address,
                yields.yield_token_id
            ) yields.contract_address,
            yields.token_contract_address,
            yields.token_id,
            y2.token_id AS previous_yield_token_id,
            yields.yield_contract_address,
            yields.yield_token_id,
            yields.yield_rate_denominator,
            yields.yield_rate_numerator,
            yields.yield_type
        FROM
            security_sft_multi_yielder_yields AS yields
            JOIN forest_project_token_contracts AS contracts ON yields.token_contract_address = contracts.contract_address
            JOIN forest_projects AS projects ON contracts.forest_project_id = projects.id
            LEFT JOIN security_sft_multi_yielder_yields AS y2 ON y2.token_contract_address = yields.token_contract_address
            AND y2.token_id < yields.token_id
            AND y2.token_contract_address = yields.token_contract_address
        ORDER BY
            yields.contract_address,
            yields.token_contract_address,
            yields.token_id,
            yields.yield_contract_address,
            yields.yield_token_id,
            yields.yield_rate_denominator,
            yields.yield_rate_numerator,
            yields.yield_type,
            y2.token_id DESC
    )
SELECT
    yields.contract_address AS yielder_contract_address,
    holder.holder_address,
    holder.un_frozen_balance,
    projects.id AS forest_project_id,
    -- Yields
    holder.cis2_address,
    holder.token_id,
    yields.token_id AS token_ver_to,
    yields.previous_yield_token_id,
    yields.yield_contract_address,
    yields.yield_token_id,
    yields.yield_rate_numerator,
    yields.yield_rate_denominator,
    yields.yield_type,
    -- Yield Calculations
    yields.token_id - GREATEST(
        COALESCE(yields.previous_yield_token_id, 0),
        holder.token_id
    ) AS yield_period,
    CASE
        WHEN yields.yield_type = 'quantity' THEN holder.un_frozen_balance * (
            yields.yield_rate_numerator::NUMERIC / yields.yield_rate_denominator::NUMERIC
        )
        WHEN yields.yield_type = 'simple_intrest' THEN holder.un_frozen_balance * (
            yields.yield_rate_numerator::NUMERIC / yields.yield_rate_denominator::NUMERIC
        ) * (
            yields.token_id - GREATEST(
                COALESCE(yields.previous_yield_token_id, 0),
                holder.token_id
            )
        )
        ELSE NULL
    END AS yield_value
FROM
    cis2_token_holders AS holder
    JOIN forest_project_token_contracts AS contracts ON holder.cis2_address = contracts.contract_address
    JOIN forest_projects AS projects ON contracts.forest_project_id = projects.id
    JOIN yields ON yields.token_contract_address = holder.cis2_address
    AND yields.token_id > holder.token_id
WHERE
    holder.un_frozen_balance > 0
ORDER BY
    holder.cis2_address,
    holder.token_id,
    holder.holder_address,
    yields.contract_address,
    yields.token_id;

CREATE OR REPLACE VIEW affiliate_claims AS
WITH
    records AS (
        SELECT
            records.contract_address,
            id,
            block_height,
            txn_index,
            investment_token_contract_address AS token_contract_address,
            investment_token_id AS token_id,
            currency_token_id,
            currency_token_contract_address,
            investor AS account_address,
            currency_amount,
            token_amount,
            create_time
        FROM
            security_mint_fund_investment_records records
        WHERE
            investment_record_type = 'claimed'
        UNION
        SELECT
            records.contract_address,
            id,
            block_height,
            txn_index,
            token_contract_address,
            token_id,
            currency_token_id,
            currency_token_contract_address,
            buyer AS account_address,
            currency_amount,
            token_amount,
            create_time
        FROM
            security_p2p_exchange_records records
        WHERE
            records.exchange_record_type = 'mint'
    )
SELECT DISTINCT
    ON (records.account_address, users.cognito_user_id) project.id AS forest_project_id,
    records.*,
    users.cognito_user_id AS user_cognito_user_id,
    users.email AS user_email,
    affiliates.account_address AS affiliate_account_address,
    affiliates.cognito_user_id AS affiliate_cognito_user_id,
    affiliates.affiliate_commission,
    affiliates.affiliate_commission * records.currency_amount AS affiliate_reward,
    claims.nonce AS claim_nonce,
    claims.reward_amount AS claim_amount,
    affiliates.affiliate_commission * records.currency_amount - COALESCE(claims.reward_amount, 0) AS affiliate_remaining_reward
FROM
    records
    JOIN forest_project_token_contracts AS contract ON records.token_contract_address = contract.contract_address
    JOIN forest_projects AS project ON contract.forest_project_id = project.id
    JOIN users ON users.account_address = records.account_address
    JOIN users AS affiliates ON users.affiliate_account_address = affiliates.account_address
    LEFT JOIN offchain_reward_claims AS claims ON claims.reward_id = DECODE(REPLACE(records.id::TEXT, '-', ''), 'hex')
ORDER BY
    records.account_address,
    users.cognito_user_id,
    records.create_time ASC;
