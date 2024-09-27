create table
    users (
        id varchar(255) not null primary key,
        account_address varchar(50),
        created_at timestamptz not null default now (),
        updated_at timestamptz not null default now ()
    );
