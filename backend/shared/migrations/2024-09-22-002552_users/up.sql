create table
    users (
        cognito_user_id varchar(255) primary key not null,
        email varchar(255) not null,
        account_address varchar(255),
        desired_investment_amount integer,
        affiliate_commission Numeric(6, 5),
        created_at timestamptz not null default now (),
        updated_at timestamptz not null default now ()
    );

create index users_account_address_index on users (account_address);

create index users_created_at_index on users (created_at);

create table
    user_challenges (
        id uuid primary key,
        cognito_user_id varchar(255) not null references users (cognito_user_id) on delete cascade,
        challenge bytea not null,
        account_address varchar(255) not null,
        created_at timestamptz not null default now ()
    );

create index user_challenges_user_id_index on user_challenges (cognito_user_id);

create index user_challenges_created_at_index on user_challenges (created_at);

create table
    user_affiliates (
        id serial primary key,
        cognito_user_id varchar(255) not null, -- cannot be a foreign key because the user may not exist yet
        affiliate_account_address varchar(255) not null,
        created_at timestamptz not null default now ()
    );
