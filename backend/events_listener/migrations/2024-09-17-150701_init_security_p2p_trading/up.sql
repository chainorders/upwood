create table
    security_p2p_trading_contracts (
        contract_address numeric(20) primary key not null references listener_contracts (contract_address) on delete cascade,
        token_contract_address numeric(20) not null,
        token_id numeric(20) not null,
        currency_token_contract_address numeric(20) not null,
        currency_token_id numeric(20) not null,
        token_amount numeric(78) not null default 0,
        rate_numerator bigint not null,
        rate_denominator bigint not null,
        create_time timestamp not null,
        update_time timestamp not null
    );

-- these are trade sell positions
create table
    security_p2p_trading_deposits (
        contract_address numeric(20) not null references security_p2p_trading_contracts (contract_address) on delete cascade,
        trader_address varchar not null,
        token_amount numeric(78) not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, trader_address)
    );

create type security_p2p_trading_record_type as enum (
    'sell',
    'sell_cancel',
    'exchange_sell',
    'exchange_buy'
);

-- these are activity records
-- each exchange record is recorded as 2 records in the table `exchange_pay` and `exchange_sell`
create table
    security_p2p_trading_records (
        id bigserial primary key,
        contract_address numeric(20) not null references security_p2p_trading_contracts (contract_address) on delete cascade,
        trader_address varchar not null,
        record_type security_p2p_trading_record_type not null,
        token_amount numeric(78) not null,
        metadata jsonb not null,
        create_time timestamp not null
    );
