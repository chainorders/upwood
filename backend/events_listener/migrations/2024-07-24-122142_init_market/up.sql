create table
    token_market (
        market_address varchar not null,
        token_contract_address varchar not null,
        token_id varchar not null,
        token_owner_address varchar not null,
        token_listed_amount numeric(78) not null,
        token_unlisted_amount numeric(78) not null,
        primary key (
            market_address,
            token_contract_address,
            token_id,
            token_owner_address
        )
    );

create index token_market_market_contract on token_market (market_address);
