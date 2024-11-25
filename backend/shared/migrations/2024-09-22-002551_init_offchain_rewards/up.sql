create table
    offchain_rewards_contracts (
        contract_address numeric(20) primary key not null,
        treasury_address varchar not null,
        create_time timestamp not null,
        update_time timestamp not null
    );

create table
    offchain_reward_contract_agents (
        contract_address numeric(20) not null references offchain_rewards_contracts (contract_address) on delete cascade,
        agent_address varchar not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, agent_address)
    );

create table
    offchain_rewardees (
        contract_address numeric(20) not null references offchain_rewards_contracts (contract_address) on delete cascade,
        account_address varchar not null,
        nonce numeric(20) not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, account_address)
    );

create table
    offchain_reward_claims (
        id uuid primary key not null,
        block_height numeric(20) not null,
        txn_index numeric(20) not null,
        contract_address numeric(20) not null,
        reward_id bytea not null,
        account_address varchar not null,
        nonce numeric(20) not null,
        reward_amount numeric(78) not null,
        reward_token_id numeric(20) not null,
        reward_token_contract_address numeric(20) not null,
        create_time timestamp not null,
        foreign key (contract_address, account_address) references offchain_rewardees (contract_address, account_address) on delete cascade
    );
