create table
    security_p2p_trading_contracts (
        contract_address numeric(20) primary key not null references listener_contracts (contract_address) on delete cascade,
        token_contract_address numeric(20) not null,
        token_id numeric(20) not null,
        currency_token_contract_address numeric(20) not null,
        currency_token_id numeric(20) not null,
        token_amount numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null
    );

-- these are trade sell positions
create table
    security_p2p_trading_deposits (
        contract_address numeric(20) not null references security_p2p_trading_contracts (contract_address) on delete cascade,
        trader_address varchar not null,
        token_amount numeric(78) not null,
        rate numeric(20, 20) not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, trader_address)
    );

create type security_p2p_trading_record_type as enum ('sell', 'sell_cancel', 'exchange');

-- these are trader activity records / deposit updates
create table
    security_p2p_trading_records (
        id uuid primary key,
        contract_address numeric(20) not null references security_p2p_trading_contracts (contract_address) on delete cascade,
        trader_address varchar not null,
        record_type security_p2p_trading_record_type not null,
        token_amount numeric(78) not null,
        currency_amount numeric(78) not null,
        token_amount_balance numeric(78) not null,
        currency_amount_balance numeric(78) not null,
        create_time timestamp not null,
        foreign key (contract_address, trader_address) references security_p2p_trading_deposits (contract_address, trader_address) on delete cascade
    );

create table
    security_p2p_trading_trades (
        id uuid primary key,
        contract_address numeric(20) not null references security_p2p_trading_contracts (contract_address) on delete cascade,
        seller_address varchar not null,
        buyer_address varchar not null,
        token_amount numeric(78) not null,
        currency_amount numeric(78) not null,
        rate numeric(20, 20) not null,
        create_time timestamp not null,
        foreign key (contract_address, seller_address) references security_p2p_trading_deposits (contract_address, trader_address) on delete cascade
    );
