create table
    security_p2p_trading_contracts (
        contract_address numeric(20) primary key not null references listener_contracts (contract_address) on delete cascade,
        currency_token_contract_address numeric(20) not null,
        currency_token_id numeric(20) not null,
        total_sell_currency_amount numeric(78) not null default 0,
        create_time timestamp not null
    );

create table
    security_p2p_trading_markets (
        contract_address numeric(20) not null references security_p2p_trading_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        token_contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        buyer varchar not null,
        rate numeric(40, 20) not null,
        total_sell_token_amount numeric(78) not null default 0,
        total_sell_currency_amount numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (
            contract_address,
            token_id,
            token_contract_address
        )
    );

create table
    security_p2p_trading_sell_records (
        id uuid primary key not null,
        block_height numeric(20) not null,
        txn_index numeric(20) not null,
        contract_address numeric(20) not null references security_p2p_trading_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        token_contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        seller varchar not null,
        currency_amount numeric(78) not null,
        token_amount numeric(78) not null,
        rate numeric(40, 20) not null,
        create_time timestamp not null
    );
