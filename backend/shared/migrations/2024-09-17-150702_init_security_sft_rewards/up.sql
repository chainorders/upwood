create table
    security_sft_rewards_contract_rewards (
        contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        rewarded_token_contract numeric(20) not null,
        rewarded_token_id numeric(20) not null,
        reward_amount numeric(78) not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (
            contract_address,
            rewarded_token_contract,
            rewarded_token_id
        )
    );

create table
    security_sft_rewards_reward_tokens (
        contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        rewarded_token_contract numeric(20) not null,
        rewarded_token_id numeric(20) not null,
        reward_amount numeric(78) not null,
        reward_rate numeric(20, 20) not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, token_id),
        foreign key (contract_address, token_id) references cis2_tokens (cis2_address, token_id) on delete cascade
    );

create table
    security_sft_rewards_claimed_reward (
        id uuid primary key not null,
        contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        holder_address varchar not null,
        token_amount numeric(78) not null,
        rewarded_token_contract numeric(20) not null,
        rewarded_token_id numeric(20) not null,
        reward_amount numeric(78) not null,
        create_time timestamp not null,
        update_time timestamp not null
    );

-- Rewards summed by reward token & holder
-- This view is used to calculate the total rewards for a holder for a particular reward token
-- These are summed rewards and not actual rewards that can be claimed
create
or replace view security_sft_rewards_holder_rewards_agg_by_reward_token_view as
select
    tokens1.contract_address,
    tokens1.token_id,
    tokens1.rewarded_token_contract,
    tokens1.rewarded_token_id,
    holders.holder_address,
    sum(holders.un_frozen_balance) as total_un_frozen_balance,
    sum(holders.frozen_balance) as total_frozen_balance,
    sum(holders.un_frozen_balance) * tokens1.reward_rate as total_un_frozen_reward,
    sum(holders.frozen_balance) * tokens1.reward_rate as total_frozen_reward
from
    security_sft_rewards_reward_tokens as tokens1
    join security_sft_rewards_reward_tokens as tokens2 on tokens1.contract_address = tokens2.contract_address
    and tokens1.token_id >= tokens2.token_id
    join cis2_token_holders as holders on (holders.cis2_address, holders.token_id) = (tokens2.contract_address, tokens2.token_id)
group by
    tokens1.contract_address,
    tokens1.token_id,
    tokens1.rewarded_token_contract,
    tokens1.rewarded_token_id,
    holders.holder_address;

-- Rewards by reward token and holder
-- This view is used to calculate the rewards for a holder for a particular reward token
-- These are actual rewards that can be claimed
create
or replace view security_sft_rewards_holder_rewards_by_reward_token_view as
select
    reward_tokens.contract_address,
    reward_tokens.token_id,
    holders.holder_address,
    holders.frozen_balance,
    holders.un_frozen_balance,
    reward_tokens.rewarded_token_contract,
    reward_tokens.rewarded_token_id,
    holders.frozen_balance * reward_tokens.reward_rate as frozen_reward,
    holders.un_frozen_balance * reward_tokens.reward_rate as un_frozen_reward
from
    security_sft_rewards_reward_tokens as reward_tokens
    join cis2_token_holders as holders on (
        reward_tokens.contract_address,
        reward_tokens.token_id
    ) = (holders.cis2_address, holders.token_id);
