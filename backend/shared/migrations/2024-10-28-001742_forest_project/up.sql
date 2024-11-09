create type forest_project_state as enum ('draft', 'listed', 'archived');

create table
    forest_projects (
        id uuid primary key not null,
        name varchar not null,
        label varchar not null,
        desc_short text not null,
        desc_long text not null,
        area varchar not null,
        carbon_credits integer not null,
        roi_percent real not null,
        state forest_project_state not null,
        image_small_url varchar not null,
        image_large_url varchar not null,
        geo_spatial_url varchar,
        contract_address numeric(20) not null references listener_contracts (contract_address),
        p2p_trade_contract_address numeric(20) references security_p2p_trading_contracts (contract_address) on delete set null,
        mint_fund_contract_address numeric(20) references security_mint_fund_contracts (contract_address) on delete set null,
        shares_available integer not null,
        offering_doc_link varchar,
        property_media_header text not null,
        property_media_footer text not null,
        latest_price numeric(10,10) not null,
        created_at timestamp not null default now (),
        updated_at timestamp not null default now ()
    );

create table
    forest_project_prices (
        project_id uuid not null references forest_projects (id),
        price numeric(10,10) not null,
        price_at timestamp not null default now (),
        created_at timestamp not null default now (),
        updated_at timestamp not null default now (),
        primary key (project_id, price_at)
    );

create table
    forest_project_property_media (
        id uuid primary key not null,
        project_id uuid not null references forest_projects (id),
        image_url varchar not null,
        created_at timestamp not null default now (),
        updated_at timestamp not null default now ()
    );

create table
    forest_project_notifications (
        id uuid primary key not null,
        project_id uuid not null references forest_projects (id),
        cognito_user_id varchar not null references users (cognito_user_id),
        created_at timestamp not null default now (),
        updated_at timestamp not null default now ()
    );

create table
    forest_project_legal_contracts (
        project_id uuid primary key not null references forest_projects (id),
        text_url varchar not null,
        edoc_url varchar not null,
        pdf_url varchar not null,
        created_at timestamp not null default now (),
        updated_at timestamp not null default now ()
    );

create table
    forest_project_legal_contract_user_signatures (
        project_id uuid not null references forest_projects (id),
        cognito_user_id varchar not null references users (cognito_user_id),
        user_account varchar not null,
        user_signature text not null,
        created_at timestamp not null default now (),
        updated_at timestamp not null default now (),
        primary key (project_id, cognito_user_id)
    );

-- Rewards summed by forest project & holder
-- This view is used to calculate the total rewards for a holder in a forest project
create
or replace view forest_project_holder_rewards_agg_view as
select
    forest_projects.id,
    forest_projects.contract_address,
    hra.holder_address,
    hra.rewarded_token_contract,
    hra.rewarded_token_id,
    sum(hra.total_un_frozen_reward) as total_un_frozen_reward,
    sum(hra.total_frozen_reward) as total_frozen_reward
from
    forest_projects
    join security_sft_rewards_holder_rewards_agg_by_reward_token_view as hra on forest_projects.contract_address = hra.contract_address
group by
    forest_projects.id,
    hra.holder_address,
    hra.rewarded_token_contract,
    hra.rewarded_token_id;

-- Rewards summed by holder
-- This view is used to calculate the total rewards for a holder across all forest projects
create
or replace view forest_project_holder_rewards_total_view as
select
    hra.holder_address,
    hra.rewarded_token_contract,
    hra.rewarded_token_id,
    sum(hra.total_un_frozen_reward) as total_un_frozen_reward,
    sum(hra.total_frozen_reward) as total_frozen_reward
from
    forest_projects
    join security_sft_rewards_holder_rewards_agg_by_reward_token_view as hra on forest_projects.contract_address = hra.contract_address
group by
    hra.holder_address,
    hra.rewarded_token_contract,
    hra.rewarded_token_id;

-- Rewards by forest project reward token and holder
-- This view is used to calculate the rewards for a holder for a particular reward token in a forest project
-- These are actual rewards that can be claimed
create
or replace view forest_project_holder_rewards_view as
select
    forest_projects.id,
    hr.contract_address,
    hr.token_id,
    hr.holder_address,
    hr.frozen_balance,
    hr.un_frozen_balance,
    hr.rewarded_token_contract,
    hr.rewarded_token_id,
    hr.frozen_reward,
    hr.un_frozen_reward
from
    forest_projects
    join security_sft_rewards_holder_rewards_by_reward_token_view as hr on forest_projects.contract_address = hr.contract_address;

create
or replace view forest_project_user_view as
select
    -- forest_projects.*,
    forest_projects.*,
    -- forest_project_notifications.*,
    forest_project_notifications.id as notification_id,
    forest_project_notifications.cognito_user_id as notification_cognito_user_id,
    -- project_legal_contract_user_signatures.*,
    forest_project_legal_contract_user_signatures.project_id as legal_contract_signed,
    forest_project_legal_contract_user_signatures.cognito_user_id as legal_contract_signer,
    -- project_tokens.*,
    project_tokens.is_paused as project_token_is_paused,
    project_tokens.metadata_url as project_token_metadata_url,
    -- project_token_holders.*,
    project_token_holders.holder_address as project_token_holder_address,
    project_token_holders.frozen_balance as project_token_frozen_balance,
    project_token_holders.un_frozen_balance as project_token_un_frozen_balance,
    -- security_mint_fund_contracts.*,
    security_mint_fund_contracts.rate as mint_fund_rate,
    security_mint_fund_contracts.fund_state as mint_fund_state,
    security_mint_fund_contracts.token_contract_address as mint_fund_token_contract_address,
    security_mint_fund_contracts.token_id as mint_fund_token_id,
    -- mint_fund_tokens.*,
    mint_fund_tokens.is_paused as mint_fund_token_is_paused,
    mint_fund_tokens.metadata_url as mint_fund_token_metadata_url,
    -- mint_fund_token_holders.*,
    mint_fund_token_holders.holder_address as mint_fund_token_holder_address,
    mint_fund_token_holders.frozen_balance as mint_fund_token_frozen_balance,
    mint_fund_token_holders.un_frozen_balance as mint_fund_token_un_frozen_balance,
    -- security_p2p_trading_contracts.*,
    security_p2p_trading_contracts.contract_address as p2p_trading_contract_address,
    -- security_p2p_trading_deposits.*,
    security_p2p_trading_deposits.rate as p2p_trading_rate,
    security_p2p_trading_deposits.token_amount as p2p_trading_token_amount,
    -- forest_project_holder_rewards_agg_view.*,
    json_agg (
        json_build_object (
            'rewarded_token_contract',
            forest_project_holder_rewards_agg_view.rewarded_token_contract,
            'rewarded_token_id',
            forest_project_holder_rewards_agg_view.rewarded_token_id,
            'total_un_frozen_reward',
            forest_project_holder_rewards_agg_view.total_un_frozen_reward,
            'total_frozen_reward',
            forest_project_holder_rewards_agg_view.total_frozen_reward
        )
    ) as holder_rewards
from
    forest_projects
    left join forest_project_notifications on forest_projects.id = forest_project_notifications.project_id
    left join forest_project_legal_contract_user_signatures on forest_projects.id = forest_project_legal_contract_user_signatures.project_id
    join cis2_tokens as project_tokens on forest_projects.contract_address = project_tokens.cis2_address
    and project_tokens.token_id = 0
    left join cis2_token_holders as project_token_holders on (
        project_tokens.cis2_address,
        project_tokens.token_id
    ) = (
        project_token_holders.cis2_address,
        project_token_holders.token_id
    )
    join security_mint_fund_contracts on forest_projects.contract_address = security_mint_fund_contracts.contract_address
    join cis2_tokens as mint_fund_tokens on (
        security_mint_fund_contracts.contract_address,
        security_mint_fund_contracts.token_id
    ) = (
        mint_fund_tokens.cis2_address,
        mint_fund_tokens.token_id
    )
    left join cis2_token_holders as mint_fund_token_holders on (
        mint_fund_tokens.cis2_address,
        mint_fund_tokens.token_id
    ) = (
        mint_fund_token_holders.cis2_address,
        mint_fund_token_holders.token_id
    )
    and mint_fund_token_holders.holder_address = project_token_holders.holder_address
    join security_p2p_trading_contracts on forest_projects.contract_address = security_p2p_trading_contracts.contract_address
    left join security_p2p_trading_deposits on security_p2p_trading_contracts.contract_address = security_p2p_trading_deposits.contract_address
    and security_p2p_trading_deposits.trader_address = project_token_holders.holder_address
    left join forest_project_holder_rewards_agg_view on forest_project_holder_rewards_agg_view.contract_address = forest_projects.contract_address
    and forest_project_holder_rewards_agg_view.holder_address = project_token_holders.holder_address
group by
    forest_projects.id,
    forest_project_notifications.cognito_user_id,
    forest_project_notifications.id,
    forest_project_legal_contract_user_signatures.project_id,
    forest_project_legal_contract_user_signatures.cognito_user_id,
    project_tokens.cis2_address,
    project_tokens.token_id,
    project_token_holders.cis2_address,
    project_token_holders.token_id,
    project_token_holders.holder_address,
    security_mint_fund_contracts.contract_address,
    mint_fund_tokens.cis2_address,
    mint_fund_tokens.token_id,
    mint_fund_token_holders.cis2_address,
    mint_fund_token_holders.token_id,
    mint_fund_token_holders.holder_address,
    security_p2p_trading_contracts.contract_address,
    security_p2p_trading_deposits.contract_address,
    security_p2p_trading_deposits.trader_address,
    forest_project_holder_rewards_agg_view.id,
    forest_project_holder_rewards_agg_view.contract_address,
    forest_project_holder_rewards_agg_view.holder_address
