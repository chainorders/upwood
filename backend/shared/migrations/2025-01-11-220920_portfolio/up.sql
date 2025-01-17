/* plpgsql-language-server:disable validation */
-- View: forest_project_investor
CREATE VIEW forest_project_investor AS
SELECT
    usr.cognito_user_id,
    SUM(investor.currency_amount) AS total_currency_amount_locked,
    SUM(investor.currency_amount_total) AS total_currency_amount_invested
FROM
    forest_projects project
    JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
    JOIN security_mint_fund_investors investor ON token_contracts.contract_address = investor.investment_token_contract_address
    JOIN users usr ON investor.investor = usr.account_address
GROUP BY
    usr.cognito_user_id;

-- View: forest_project_trader
CREATE VIEW forest_project_trader AS
SELECT
    usr.cognito_user_id,
    SUM(trader.currency_in_amount) AS total_currency_in_amount,
    SUM(trader.currency_out_amount) AS total_currency_out_amount
FROM
    forest_projects project
    JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
    JOIN security_p2p_trading_traders trader ON token_contracts.contract_address = trader.token_contract_address
    JOIN users usr ON trader.trader = usr.account_address
GROUP BY
    usr.cognito_user_id;

-- View: forest_project_user_investment_amounts
CREATE VIEW forest_project_user_investment_amounts AS
SELECT
    usr.cognito_user_id,
    COALESCE(investor.total_currency_amount_locked, 0) AS total_currency_amount_locked,
    COALESCE(investor.total_currency_amount_invested, 0) + COALESCE(trader.total_currency_in_amount, 0) - COALESCE(trader.total_currency_out_amount, 0) AS total_currency_amount_invested
FROM
    users usr
    LEFT JOIN forest_project_investor investor ON usr.cognito_user_id = investor.cognito_user_id
    LEFT JOIN forest_project_trader trader ON usr.cognito_user_id = trader.cognito_user_id;

-- Function: user_currency_value_for_forest_project_owned_tokens_at(cognito_user_id TEXT, time_at TIMESTAMP)
CREATE FUNCTION user_currency_value_for_forest_project_owned_tokens_at (cognito_user_id TEXT, time_at TIMESTAMP) RETURNS NUMERIC AS $$
        SELECT SUM(COALESCE(un_frozen_balance, 0) * COALESCE(price, 0)) FROM (
            SELECT
                project.id,
                token_contracts.contract_address,
                balance_updates.token_id,
                usr.cognito_user_id,
                FIRST_VALUE(balance_updates.un_frozen_balance) OVER (
                    PARTITION BY
                        project.id,
                        token_contracts.contract_address,
                        balance_updates.token_id,
                        usr.cognito_user_id
                    ORDER BY
                        balance_updates.create_time DESC
                ) AS un_frozen_balance,
                FIRST_VALUE(price.price) OVER (
                    PARTITION BY
                        project.id
                    ORDER BY
                        price.price_at DESC
                ) AS price
            FROM
                forest_projects project
                JOIN forest_project_prices price ON project.id = price.project_id AND price.price_at <= time_at
                JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
                JOIN cis2_token_holder_balance_updates balance_updates ON token_contracts.contract_address = balance_updates.cis2_address AND balance_updates.create_time <= time_at
                JOIN users usr ON balance_updates.holder_address = usr.account_address
            ) AS user_balances
        WHERE user_balances.cognito_user_id = cognito_user_id
        GROUP BY user_balances.cognito_user_id;
$$ language sql;

-- Function: total funds got from selling - total funds spent on buying
CREATE FUNCTION user_exchange_profits (
    cognito_user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP
) RETURNS NUMERIC LANGUAGE plpgsql AS $$
    DECLARE
        bought_currency_amount NUMERIC;
        sold_currency_amount NUMERIC;
        profit NUMERIC;
    BEGIN
        SELECT
            SUM(currency_amount)
        FROM
            forest_projects project
        JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
        JOIN security_p2p_exchange_records buy_exchange_records ON token_contracts.contract_address = buy_exchange_records.token_contract_address
            AND buy_exchange_records.create_time > from_time
            AND buy_exchange_records.create_time <= to_time
        JOIN users usr ON buy_exchange_records.buyer = usr.account_address
        WHERE usr.cognito_user_id = cognito_user_id
        GROUP BY usr.cognito_user_id
        INTO bought_currency_amount;

        SELECT
            SUM(currency_amount)
        FROM
            forest_projects project
        JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
        JOIN security_p2p_exchange_records sell_exchange_records ON token_contracts.contract_address = sell_exchange_records.token_contract_address
            AND sell_exchange_records.create_time > from_time
            AND sell_exchange_records.create_time <= to_time
        JOIN users usr ON sell_exchange_records.seller = usr.account_address
        WHERE usr.cognito_user_id = cognito_user_id
        GROUP BY usr.cognito_user_id
        INTO sold_currency_amount;

        profit := COALESCE(sold_currency_amount, 0) - COALESCE(bought_currency_amount, 0);
        RETURN profit;
    END;
$$;

-- Function: this function gets the -1 * locked amount
CREATE FUNCTION user_fund_profits (
    cognito_user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP
) returns NUMERIC language plpgsql AS $$
    declare
        invested_currency_amount numeric;
        cancelled_currency_amount numeric;
        claimed_currency_amount numeric;
        profit numeric;
    begin
        select
            sum(currency_amount)
        from
            forest_projects project
        join forest_project_token_contracts token_contracts on project.id = token_contracts.forest_project_id
        join security_mint_fund_investment_records investment_records on token_contracts.contract_address = investment_records.contract_address
            and investment_records.create_time > from_time
            and investment_records.create_time <= to_time
            and investment_records.type = 'invested'
        join users usr on investment_records.investor = usr.account_address
        where usr.cognito_user_id = cognito_user_id
        group by usr.cognito_user_id
        into invested_currency_amount;

        select
            sum(currency_amount)
        from
            forest_projects project
        join forest_project_token_contracts token_contracts on project.id = token_contracts.forest_project_id
        join security_mint_fund_investment_records investment_records on token_contracts.contract_address = investment_records.contract_address
            and investment_records.create_time > from_time
            and investment_records.create_time <= to_time
            and investment_records.type = 'cancelled'
        join users usr on investment_records.investor = usr.account_address
        where usr.cognito_user_id = cognito_user_id
        group by usr.cognito_user_id
        into cancelled_currency_amount;

        select
            sum(currency_amount)
        from

            forest_projects project
        join forest_project_token_contracts token_contracts on project.id = token_contracts.forest_project_id
        join security_mint_fund_investment_records investment_records on token_contracts.contract_address = investment_records.contract_address
            and investment_records.create_time > from_time
            and investment_records.create_time <= to_time
            and investment_records.type = 'claimed'
        join users usr on investment_records.investor = usr.account_address
        where usr.cognito_user_id = cognito_user_id
        group by usr.cognito_user_id
        into claimed_currency_amount;

        profit :=
            COALESCE(claimed_currency_amount, 0)
            - COALESCE(invested_currency_amount, 0)
            - COALESCE(cancelled_currency_amount, 0);
        return profit;
    end;
$$;

CREATE VIEW user_transactions AS (
    SELECT
        *
    FROM
        (
            (
                SELECT
                    txn.transaction_hash,
                    txn.block_height,
                    project.id AS forest_project_id,
                    investment_record.currency_amount,
                    usr.cognito_user_id,
                    investment_record.investment_record_type::TEXT AS transaction_type
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
                    JOIN security_mint_fund_investment_records investment_record ON token_contract.contract_address = investment_record.investment_token_contract_address
                    JOIN users usr ON investment_record.investor = usr.account_address
                    JOIN listener_transactions txn ON investment_record.txn_index = txn.transaction_index
                    AND txn.block_height = investment_record.block_height
            )
            UNION
            (
                SELECT
                    txn.transaction_hash,
                    txn.block_height,
                    project.id AS forest_project_id,
                    exchange_record.currency_amount,
                    usr.cognito_user_id,
                    'buy' AS transaction_type
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
                    JOIN security_p2p_exchange_records exchange_record ON token_contract.contract_address = exchange_record.token_contract_address
                    JOIN users usr ON exchange_record.buyer = usr.account_address
                    JOIN listener_transactions txn ON exchange_record.txn_index = txn.transaction_index
                    AND txn.block_height = exchange_record.block_height
            )
            UNION
            (
                SELECT
                    txn.transaction_hash,
                    txn.block_height,
                    project.id AS forest_project_id,
                    exchange_record.currency_amount,
                    usr.cognito_user_id,
                    'sell' AS transaction_type
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
                    JOIN security_p2p_exchange_records exchange_record ON token_contract.contract_address = exchange_record.token_contract_address
                    JOIN users usr ON exchange_record.seller = usr.account_address
                    JOIN listener_transactions txn ON exchange_record.txn_index = txn.transaction_index
                    AND txn.block_height = exchange_record.block_height
            )
            ORDER BY
                block_height DESC
        ) t2
)
