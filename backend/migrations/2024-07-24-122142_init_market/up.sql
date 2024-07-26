create table
    token_market (
        market_contract_index numeric(20) not null,
        market_contract_sub_index numeric(20) not null,
        token_contract_index numeric(20) not null,
        token_contract_sub_index numeric(20) not null,
        token_id numeric(78) not null,
        token_owner varchar not null,
        token_listed_amount numeric(78) not null,
        token_unlisted_amount numeric(78) not null,
        primary key (
            market_contract_index,
            market_contract_sub_index,
            token_contract_index,
            token_contract_sub_index,
            token_id,
            token_owner
        )
    );
create index token_market_market_contract on token_market(market_contract_index, market_contract_sub_index);
