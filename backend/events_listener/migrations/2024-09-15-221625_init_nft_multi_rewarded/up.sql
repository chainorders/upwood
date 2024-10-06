create table
    nft_multi_rewarded_contracts (
        contract_address varchar primary key not null,
        reward_token_id varchar not null,
        reward_token_address varchar not null,
        update_time timestamp not null
    );

create table
    nft_multi_address_nonces (
        contract_address varchar not null,
        address varchar not null,
        nonce bigint not null,
        primary key (contract_address, address)
    );
