/* plpgsql-language-server:disable validation */
-- View: forest_project_investor
CREATE VIEW forest_project_investor AS
SELECT
    usr.cognito_user_id,
    investor.currency_token_id,
    investor.currency_token_contract_address,
    SUM(investor.currency_amount) AS total_currency_amount_locked,
    SUM(investor.currency_amount_total) AS total_currency_amount_invested
FROM
    forest_projects project
    JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
    JOIN security_mint_fund_investors investor ON token_contracts.contract_address = investor.investment_token_contract_address
    JOIN users usr ON investor.investor = usr.account_address
GROUP BY
    usr.cognito_user_id,
    investor.currency_token_id,
    investor.currency_token_contract_address;

-- View: forest_project_trader
CREATE VIEW forest_project_trader AS
SELECT
    usr.cognito_user_id,
    trader.currency_token_id,
    trader.currency_token_contract_address,
    SUM(trader.currency_in_amount) AS total_currency_in_amount,
    SUM(trader.currency_out_amount) AS total_currency_out_amount
FROM
    forest_projects project
    JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
    JOIN security_p2p_trading_traders trader ON token_contracts.contract_address = trader.token_contract_address
    JOIN users usr ON trader.trader = usr.account_address
GROUP BY
    usr.cognito_user_id,
    trader.currency_token_id,
    trader.currency_token_contract_address;

-- View: forest_project_user_investment_amounts
CREATE VIEW forest_project_user_investment_amounts AS
SELECT
    t.cognito_user_id,
    t.currency_token_id,
    t.currency_token_contract_address,
    COALESCE(SUM(t.total_currency_amount_locked), 0) AS total_currency_amount_locked,
    COALESCE(SUM(t.total_currency_amount_invested), 0) AS total_currency_amount_invested
FROM
    (
        SELECT
            usr.cognito_user_id,
            investor.currency_token_id AS currency_token_id,
            investor.currency_token_contract_address AS currency_token_contract_address,
            investor.total_currency_amount_locked,
            investor.total_currency_amount_invested
        FROM
            users usr
            JOIN forest_project_investor investor ON usr.cognito_user_id = investor.cognito_user_id
        UNION
        SELECT
            usr.cognito_user_id,
            trader.currency_token_id AS currency_token_id,
            trader.currency_token_contract_address AS currency_token_contract_address,
            0 AS total_currency_amount_locked,
            trader.total_currency_out_amount - trader.total_currency_in_amount AS total_currency_amount_invested
        FROM
            users usr
            JOIN forest_project_trader trader ON usr.cognito_user_id = trader.cognito_user_id
    ) t
GROUP BY
    t.cognito_user_id,
    t.currency_token_id,
    t.currency_token_contract_address;

-- Function: user_currency_value_for_forest_project_owned_tokens_at(cognito_user_id TEXT, time_at TIMESTAMP)
CREATE FUNCTION user_currency_value_for_forest_project_owned_tokens_at (
    user_id TEXT,
    time_at TIMESTAMP,
    curr_token_id NUMERIC,
    curr_token_contract_address NUMERIC
) RETURNS NUMERIC LANGUAGE plpgsql AS $$
    DECLARE
        portfolio_value NUMERIC;
    BEGIN
        SELECT
            SUM(COALESCE(t2.un_frozen_balance, 0) * COALESCE(t1.price, 0)) AS portfolio_value
        FROM
            (
            SELECT DISTINCT
                    ON (
                        project.id,
                        token_contracts.contract_address,
                        balance_updates.token_id,
                        usr.cognito_user_id
                    ) project.id AS forest_project_id,
                    token_contracts.contract_address,
                    balance_updates.token_id,
                    usr.cognito_user_id,
                    FIRST_VALUE(balance_updates.un_frozen_balance) OVER w AS un_frozen_balance
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
                    JOIN cis2_token_holder_balance_updates balance_updates ON token_contracts.contract_address = balance_updates.cis2_address
                    AND balance_updates.create_time <= time_at
                    JOIN users usr ON balance_updates.holder_address = usr.account_address
                WINDOW
                    w AS (
                        PARTITION BY
                            project.id,
                            token_contracts.contract_address,
                            balance_updates.token_id,
                            usr.cognito_user_id
                        ORDER BY
                            balance_updates.id_serial DESC
                    )
            ) t2
            JOIN
            (
                SELECT DISTINCT
                    ON (project.id) project.id AS forest_project_id,
                    FIRST_VALUE(price.price) OVER w AS price
                FROM
                    forest_projects project
                    JOIN forest_project_prices price ON project.id = price.project_id
                    AND price.price_at <= time_at
                    AND price.currency_token_id = curr_token_id
                    AND price.currency_token_contract_address = curr_token_contract_address
                WINDOW
                    w AS (
                        PARTITION BY
                            project.id
                        ORDER BY
                            price.price_at DESC
                    )
            ) t1
            ON t1.forest_project_id = t2.forest_project_id
            GROUP BY
                t2.cognito_user_id
            HAVING
                t2.cognito_user_id = user_id
            INTO portfolio_value;

        RETURN COALESCE(portfolio_value, 0);
    END;
$$;

CREATE FUNCTION user_exchange_input_amount (
    user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP,
    curr_token_id NUMERIC,
    curr_token_contract_address NUMERIC
) RETURNS NUMERIC LANGUAGE plpgsql AS $$
    DECLARE
        bought_currency_amount NUMERIC;
    BEGIN
        SELECT
            SUM(currency_amount)
        FROM
            forest_projects project
        JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
        JOIN security_p2p_exchange_records buy_exchange_records ON token_contracts.contract_address = buy_exchange_records.token_contract_address
            AND buy_exchange_records.create_time > from_time
            AND buy_exchange_records.create_time <= to_time
            AND buy_exchange_records.currency_token_id = curr_token_id
            AND buy_exchange_records.currency_token_contract_address = curr_token_contract_address
        JOIN users usr ON buy_exchange_records.buyer = usr.account_address
        WHERE usr.cognito_user_id = user_id
        GROUP BY usr.cognito_user_id
        INTO bought_currency_amount;

        RETURN COALESCE(bought_currency_amount, 0);
    END;
$$;

CREATE FUNCTION user_exchange_output_amount (
    user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP,
    curr_token_id NUMERIC,
    curr_token_contract_address NUMERIC
) RETURNS NUMERIC LANGUAGE plpgsql AS $$
    DECLARE
        sold_currency_amount NUMERIC;
    BEGIN
        SELECT
            SUM(currency_amount)
        FROM
            forest_projects project
        JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
        JOIN security_p2p_exchange_records sell_exchange_records ON token_contracts.contract_address = sell_exchange_records.token_contract_address
            AND sell_exchange_records.create_time > from_time
            AND sell_exchange_records.create_time <= to_time
            AND sell_exchange_records.currency_token_id = curr_token_id
            AND sell_exchange_records.currency_token_contract_address = curr_token_contract_address
        JOIN users usr ON sell_exchange_records.seller = usr.account_address
        WHERE usr.cognito_user_id = user_id
        GROUP BY usr.cognito_user_id
        INTO sold_currency_amount;

        RETURN COALESCE(sold_currency_amount, 0);
    END;
$$;

-- Function: total funds got from selling - total funds spent on buying
CREATE FUNCTION user_exchange_profits (
    user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP,
    curr_token_id NUMERIC,
    curr_token_contract_address NUMERIC
) RETURNS NUMERIC LANGUAGE plpgsql AS $$
    DECLARE
        profit NUMERIC;
    BEGIN
        profit := user_exchange_output_amount(user_id, from_time, to_time, curr_token_id, curr_token_contract_address)
                - user_exchange_input_amount(user_id, from_time, to_time, curr_token_id, curr_token_contract_address);
        RETURN profit;
    END;
$$;

CREATE FUNCTION user_fund_investment_amount (
    user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP,
    curr_token_id NUMERIC,
    curr_token_contract_address NUMERIC
) RETURNS NUMERIC LANGUAGE plpgsql AS $$
    DECLARE
        invested_currency_amount NUMERIC;
        cancelled_currency_amount numeric;
    BEGIN
        SELECT
            SUM(currency_amount)
        FROM
            forest_projects project
        JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
        JOIN security_mint_fund_investment_records investment_records ON token_contracts.contract_address = investment_records.investment_token_contract_address
            AND investment_records.create_time >= from_time
            AND investment_records.create_time < to_time
            AND investment_records.investment_record_type = 'invested'::security_mint_fund_investment_record_type
            AND investment_records.currency_token_id = curr_token_id
            AND investment_records.currency_token_contract_address = curr_token_contract_address
        JOIN users usr ON investment_records.investor = usr.account_address
        GROUP BY usr.cognito_user_id
        HAVING usr.cognito_user_id = user_id
        INTO invested_currency_amount;

        SELECT
            SUM(currency_amount)
        FROM
            forest_projects project
        JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
        JOIN security_mint_fund_investment_records investment_records ON token_contracts.contract_address = investment_records.investment_token_contract_address
            AND investment_records.create_time >= from_time
            AND investment_records.create_time < to_time
            AND investment_records.investment_record_type = 'cancelled'::security_mint_fund_investment_record_type
            AND investment_records.currency_token_id = curr_token_id
            AND investment_records.currency_token_contract_address = currency_token_contract_address
        JOIN users usr ON investment_records.investor = usr.account_address
        GROUP BY usr.cognito_user_id
        HAVING usr.cognito_user_id = user_id
        INTO cancelled_currency_amount;

        RETURN COALESCE(invested_currency_amount, 0) - COALESCE(cancelled_currency_amount, 0);
    END;
$$;

-- Function: this function gets the -1 * locked amount
CREATE FUNCTION user_fund_profits (
    user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP,
    curr_token_id NUMERIC,
    curr_token_contract_address NUMERIC
) returns NUMERIC language plpgsql AS $$
    DECLARE
        claimed_currency_amount numeric;
    BEGIN
        SELECT
            SUM(currency_amount)
        FROM
            forest_projects project
            JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
            JOIN security_mint_fund_investment_records investment_records ON token_contracts.contract_address = investment_records.investment_token_contract_address
                AND investment_records.create_time >= from_time
                AND investment_records.create_time < to_time
                AND investment_records.investment_record_type = 'claimed'::security_mint_fund_investment_record_type
                AND investment_records.currency_token_id = curr_token_id
                AND investment_records.currency_token_contract_address = curr_token_contract_address
            JOIN users usr ON investment_records.investor = usr.account_address
        GROUP BY
            usr.cognito_user_id
        HAVING
            usr.cognito_user_id = user_id
        INTO claimed_currency_amount;

        return COALESCE(claimed_currency_amount, 0) - user_fund_investment_amount(user_id, from_time, to_time, curr_token_id, curr_token_contract_address);
    END;
$$;

-- Function: user token manual transfers out currency amounts
CREATE FUNCTION user_token_manual_transfer_profits (
    user_id TEXT,
    from_time TIMESTAMP,
    to_time TIMESTAMP,
    curr_token_id NUMERIC,
    curr_token_contract_address NUMERIC
) RETURNS NUMERIC LANGUAGE plpgsql AS $$
    DECLARE
        transferred_out_currency_amount NUMERIC;
        transferred_in_currency_amount NUMERIC;
        profit NUMERIC;
    BEGIN
        SELECT
            SUM(currency_amount) AS currency_amount
        FROM
            (
                SELECT DISTINCT
                    ON (
                        project.id,
                        token_contracts.contract_address,
                        balance_updates.token_id,
                        balance_updates.id_serial,
                        usr.cognito_user_id
                    ) project.id AS forest_project_id,
                    token_contracts.contract_address,
                    balance_updates.token_id,
                    usr.cognito_user_id,
                    balance_updates.id_serial,
                    balance_updates.amount AS token_amount,
                    balance_updates.create_time AS update_time,
                    balance_updates.update_type,
                    FIRST_VALUE(price.price) OVER w AS price,
                    FIRST_VALUE(price.price_at) OVER w AS price_time,
                    balance_updates.amount * FIRST_VALUE(price.price) OVER w AS currency_amount
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
                    JOIN cis2_token_holder_balance_updates balance_updates ON token_contracts.contract_address = balance_updates.cis2_address
                        AND balance_updates.create_time >= from_time
                        AND balance_updates.create_time < to_time
                        AND balance_updates.txn_sender = balance_updates.holder_address
                    JOIN forest_project_prices price ON project.id = price.project_id
                        AND price.price_at <= balance_updates.create_time
                        AND price.currency_token_id = curr_token_id
                        AND price.currency_token_contract_address = curr_token_contract_address
                    JOIN users usr ON balance_updates.holder_address = usr.account_address
                WINDOW
                    w AS (
                        PARTITION BY
                            project.id,
                            token_contracts.contract_address,
                            balance_updates.token_id,
                            usr.cognito_user_id,
                            balance_updates.id_serial
                        ORDER BY
                            price.price_at DESC
                    )
            ) t1
        GROUP BY
            cognito_user_id,
            update_type
        HAVING
            cognito_user_id = user_id
            AND update_type = 'transfer_out'::cis2_token_holder_balance_update_type
        INTO transferred_out_currency_amount;

        SELECT
            SUM(currency_amount) AS currency_amount
        FROM
            (
                SELECT DISTINCT
                    ON (
                        project.id,
                        token_contracts.contract_address,
                        balance_updates.token_id,
                        balance_updates.id_serial,
                        usr.cognito_user_id
                    ) project.id AS forest_project_id,
                    token_contracts.contract_address,
                    balance_updates.token_id,
                    usr.cognito_user_id,
                    balance_updates.id_serial,
                    balance_updates.amount AS token_amount,
                    balance_updates.create_time AS update_time,
                    balance_updates.update_type,
                    FIRST_VALUE(price.price) OVER w AS price,
                    FIRST_VALUE(price.price_at) OVER w AS price_time,
                    balance_updates.amount * FIRST_VALUE(price.price) OVER w AS currency_amount
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contracts ON project.id = token_contracts.forest_project_id
                    JOIN cis2_token_holder_balance_updates balance_updates ON token_contracts.contract_address = balance_updates.cis2_address
                        AND balance_updates.create_time >= from_time
                        AND balance_updates.create_time < to_time
                        AND balance_updates.txn_sender = balance_updates.holder_address
                    JOIN forest_project_prices price ON project.id = price.project_id
                        AND price.price_at <= balance_updates.create_time
                        AND price.currency_token_id = curr_token_id
                        AND price.currency_token_contract_address = curr_token_contract_address
                    JOIN users usr ON balance_updates.holder_address = usr.account_address
                WINDOW
                    w AS (
                        PARTITION BY
                            project.id,
                            token_contracts.contract_address,
                            balance_updates.token_id,
                            usr.cognito_user_id,
                            balance_updates.id_serial
                        ORDER BY
                            price.price_at DESC
                    )
            ) t1
        GROUP BY
            cognito_user_id,
            update_type
        HAVING
            cognito_user_id = user_id
            AND update_type = 'transfer_in'::cis2_token_holder_balance_update_type
        INTO transferred_in_currency_amount;

        profit := COALESCE(transferred_out_currency_amount, 0) - COALESCE(transferred_in_currency_amount, 0);
        RETURN  profit;
    END;
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
                    investment_record.currency_token_id,
                    investment_record.currency_token_contract_address,
                    investment_record.currency_amount,
                    COALESCE(token_metadata.symbol, '') AS currency_token_symbol,
                    COALESCE(token_metadata.decimals, 0) AS currency_token_decimals,
                    usr.cognito_user_id,
                    investment_record.investment_record_type::TEXT AS transaction_type
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
                    JOIN security_mint_fund_investment_records investment_record ON token_contract.contract_address = investment_record.investment_token_contract_address
                    JOIN users usr ON investment_record.investor = usr.account_address
                    JOIN listener_transactions txn ON investment_record.txn_index = txn.transaction_index
                    AND txn.block_height = investment_record.block_height
                    LEFT JOIN token_metadatas token_metadata ON investment_record.currency_token_id = token_metadata.token_id
                    AND investment_record.currency_token_contract_address = token_metadata.contract_address
            )
            UNION
            (
                SELECT
                    txn.transaction_hash,
                    txn.block_height,
                    project.id AS forest_project_id,
                    exchange_record.currency_token_id,
                    exchange_record.currency_token_contract_address,
                    exchange_record.currency_amount,
                    COALESCE(token_metadata.symbol, '') AS currency_token_symbol,
                    COALESCE(token_metadata.decimals, 0) AS currency_token_decimals,
                    usr.cognito_user_id,
                    'buy' AS transaction_type
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
                    JOIN security_p2p_exchange_records exchange_record ON token_contract.contract_address = exchange_record.token_contract_address
                    JOIN users usr ON exchange_record.buyer = usr.account_address
                    JOIN listener_transactions txn ON exchange_record.txn_index = txn.transaction_index
                    AND txn.block_height = exchange_record.block_height
                    LEFT JOIN token_metadatas token_metadata ON exchange_record.currency_token_id = token_metadata.token_id
                    AND exchange_record.currency_token_contract_address = token_metadata.contract_address
            )
            UNION
            (
                SELECT
                    txn.transaction_hash,
                    txn.block_height,
                    project.id AS forest_project_id,
                    exchange_record.currency_token_id,
                    exchange_record.currency_token_contract_address,
                    exchange_record.currency_amount,
                    COALESCE(token_metadata.symbol, '') AS currency_token_symbol,
                    COALESCE(token_metadata.decimals, 0) AS currency_token_decimals,
                    usr.cognito_user_id,
                    'sell' AS transaction_type
                FROM
                    forest_projects project
                    JOIN forest_project_token_contracts token_contract ON project.id = token_contract.forest_project_id
                    JOIN security_p2p_exchange_records exchange_record ON token_contract.contract_address = exchange_record.token_contract_address
                    JOIN users usr ON exchange_record.seller = usr.account_address
                    JOIN listener_transactions txn ON exchange_record.txn_index = txn.transaction_index
                    AND txn.block_height = exchange_record.block_height
                    LEFT JOIN token_metadatas token_metadata ON exchange_record.currency_token_id = token_metadata.token_id
                    AND exchange_record.currency_token_contract_address = token_metadata.contract_address
            )
            ORDER BY
                block_height DESC
        ) t2
)
