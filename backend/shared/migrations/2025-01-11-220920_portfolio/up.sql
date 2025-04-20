/* plpgsql-language-server:disable validation */
CREATE OR REPLACE VIEW user_transactions AS (
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
                    investment_record.investment_record_type::TEXT AS transaction_type,
                    investment_record.investor AS account_address
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
                    'buy' AS transaction_type,
                    exchange_record.buyer AS account_address
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
                    'sell' AS transaction_type,
                    exchange_record.seller AS account_address
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
