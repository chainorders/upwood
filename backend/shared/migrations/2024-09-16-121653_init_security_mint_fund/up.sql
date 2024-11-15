create table
    security_mint_fund_contracts (
        contract_address numeric(20) primary key not null references listener_contracts (contract_address) on delete cascade,
        token_contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        investment_token_contract_address numeric(20) not null,
        investment_token_id numeric(20) not null,
        currency_token_contract_address numeric(20) not null,
        currency_token_id numeric(20) not null,
        rate numeric(20, 20) not null,
        fund_state int not null,
        receiver_address varchar,
        currency_amount numeric(78) not null default 0,
        token_amount numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null,
        foreign key (token_contract_address, token_id) references cis2_tokens (cis2_address, token_id) on delete cascade,
        foreign key (
            investment_token_contract_address,
            investment_token_id
        ) references cis2_tokens (cis2_address, token_id) on delete cascade
    );

create table
    security_mint_fund_investors (
        contract_address numeric(20) not null references security_mint_fund_contracts (contract_address) on delete cascade,
        investor varchar not null,
        currency_amount numeric(78) not null default 0,
        token_amount numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, investor)
    );

create table
    security_mint_fund_investment_records (
        id uuid primary key not null,
        contract_address numeric(20) not null references security_mint_fund_contracts (contract_address) on delete cascade,
        investor varchar not null,
        currency_amount numeric(78) not null,
        token_amount numeric(78) not null,
        currency_amount_balance numeric(78) not null,
        token_amount_balance numeric(78) not null,
        investment_record_type int not null,
        create_time timestamp not null,
        foreign key (contract_address, investor) references security_mint_fund_investors (contract_address, investor) on delete cascade
    );
