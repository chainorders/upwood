create type security_mint_fund_state as enum ('open', 'success', 'fail');

create table
    security_mint_fund_contracts (
        contract_address numeric(20) primary key not null references listener_contracts (contract_address) on delete cascade,
        currency_token_contract_address numeric(20) not null,
        currency_token_id numeric(20) not null,
        create_time timestamp not null
    );

create table
    security_mint_funds (
        id numeric(20) primary key not null,
        contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        token_contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        investment_token_id numeric(20) not null,
        investment_token_contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        currency_token_id numeric(20) not null,
        currency_token_contract_address numeric(20) not null,
        currency_amount numeric(78) not null default 0,
        token_amount numeric(78) not null default 0,
        receiver_address varchar,
        rate numeric(40, 20) not null,
        fund_state security_mint_fund_state not null,
        create_time timestamp not null,
        update_time timestamp not null
    );

create table
    security_mint_fund_investors (
        contract_address numeric(20) not null references security_mint_fund_contracts (contract_address) on delete cascade,
        fund_id numeric(20) not null references security_mint_funds (id) on delete cascade,
        investor varchar not null,
        currency_amount numeric(78) not null default 0,
        token_amount numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, fund_id, investor)
    );

create type security_mint_fund_investment_record_type as enum ('invested', 'cancelled', 'claimed');

create table
    security_mint_fund_investment_records (
        id uuid primary key not null,
        block_height numeric(20) not null,
        txn_index numeric(20) not null,
        contract_address numeric(20) not null references security_mint_fund_contracts (contract_address) on delete cascade,
        fund_id numeric(20) not null references security_mint_funds (id) on delete cascade,
        investor varchar not null,
        currency_amount numeric(78) not null,
        token_amount numeric(78) not null,
        currency_amount_balance numeric(78) not null,
        token_amount_balance numeric(78) not null,
        investment_record_type security_mint_fund_investment_record_type not null,
        create_time timestamp not null,
        foreign key (contract_address, fund_id, investor) references security_mint_fund_investors (contract_address, fund_id, investor) on delete cascade
    );
