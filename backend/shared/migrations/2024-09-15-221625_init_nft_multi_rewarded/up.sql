create table
    nft_multi_rewarded_contracts (
        contract_address numeric(20) primary key not null references listener_contracts (contract_address) on delete cascade,
        reward_token_id numeric(20) not null,
        reward_token_address numeric(20) not null,
        create_time timestamp not null,
        update_time timestamp not null
    );

create table
    nft_multi_address_nonces (
        contract_address numeric(20) not null references nft_multi_rewarded_contracts (contract_address) on delete cascade,
        address varchar not null,
        nonce bigint not null,
        primary key (contract_address, address)
    );
