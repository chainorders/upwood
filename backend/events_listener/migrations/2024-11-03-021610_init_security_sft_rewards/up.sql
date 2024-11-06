create table
    security_sft_rewards_rewards (
        contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        rewarded_token_id numeric(20) not null,
        rewarded_contract_address numeric(20) not null,
        reward_amount numeric(78) not null,
        reward_rate_numerator bigint not null,
        reward_rate_denominator bigint not null,
        create_time timestamp not null,
        update_time timestamp not null,
        primary key (contract_address, token_id),
        foreign key (contract_address, token_id) references cis2_tokens (cis2_address, token_id) on delete cascade
    );

create table
    security_sft_rewards_claims (
        id uuid primary key not null,
        contract_address numeric(20) not null references listener_contracts (contract_address) on delete cascade,
        token_id numeric(20) not null,
        amount numeric(78) not null,
        holder_address varchar not null,
        rewarded_token_contract numeric(20) not null,
        rewarded_token_id numeric(20) not null,
        reward_amount numeric(78) not null,
        create_time timestamp not null,
        update_time timestamp not null
    );

create
or replace view reward_holder_aggregates as
select
    contracts.contract_address,
    holders.holder_address,
    holders.token_id,
    rewards.rewarded_contract_address,
    rewards.rewarded_token_id,
    sum(
        holders.frozen_balance * rewards.reward_rate_numerator / rewards.reward_rate_denominator
    ) as rewards_frozen,
    sum(
        holders.un_frozen_balance * rewards.reward_rate_numerator / rewards.reward_rate_denominator
    ) as rewards_un_frozen
from
    listener_contracts as contracts
    join cis2_token_holders as holders on holders.cis2_address = contracts.contract_address
    and contracts.processor_type = 3 -- filter out non reward contracts
    and holders.token_id > 0 -- filter out tracked tokens
    join security_sft_rewards_rewards as rewards on rewards.contract_address = contracts.contract_address
    and rewards.token_id >= holders.token_id
group by
    contracts.contract_address,
    holders.holder_address,
    holders.token_id,
    rewards.rewarded_contract_address,
    rewards.rewarded_token_id;

create
or replace view reward_holder as
select
    holders.cis2_address,
    holders.token_id,
    holders.holder_address,
    holders.frozen_balance,
    holders.un_frozen_balance,
    rewards.rewarded_contract_address,
    rewards.rewarded_token_id,
    holders.frozen_balance * rewards.reward_rate_numerator / rewards.reward_rate_denominator as rewards_frozen,
    holders.un_frozen_balance * rewards.reward_rate_numerator / rewards.reward_rate_denominator as rewards_unfrozen,
    rewards.reward_rate_numerator,
    rewards.reward_rate_denominator
from
    listener_contracts as contracts
    join cis2_token_holders as holders on holders.cis2_address = contracts.contract_address
    and contracts.processor_type = 3 -- filter out non reward contracts
    and holders.token_id > 0 -- filter out tracked tokens
    join security_sft_rewards_rewards as rewards on (
        rewards.rewarded_contract_address,
        rewards.rewarded_token_id
    ) = (holders.cis2_address, holders.token_id);
