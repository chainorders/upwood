create table
    security_mint_fund_contracts (
        contract_address varchar primary key not null,
        token_contract_address varchar not null,
        token_id varchar not null,
        investment_token_contract_address varchar not null,
        investment_token_id varchar not null,
        currency_token_contract_address varchar not null,
        currency_token_id varchar not null,
        rate_numerator bigint not null,
        rate_denominator bigint not null,
        fund_state int not null,
        currency_amount numeric(78) not null default 0,
        token_amount numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null
    );

create table
    security_mint_fund_investors (
        contract_address varchar not null,
        investor varchar not null,
        currency_amount numeric(78) not null default 0,
        token_amount numeric(78) not null default 0,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, investor)
    );

create table
    security_mint_fund_investment_records (
        id bigserial primary key not null,
        contract_address varchar not null,
        investor varchar not null,
        currency_amount numeric(78),
        token_amount numeric(78),
        investment_record_type int not null,
        create_time timestamp not null
    );
