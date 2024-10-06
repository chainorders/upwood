create table
    user_challenges (
        id serial primary key,
        user_id varchar(255) not null,
        challenge bytea not null,
        account_address varchar(255) not null,
        created_at timestamptz not null default now ()
    );

create index user_challenges_user_id_index on user_challenges (user_id);

create index user_challenges_created_at_index on user_challenges (created_at);
