create table
    news_articles (
        id uuid primary key,
        title varchar not null,
        label varchar not null,
        content text not null,
        image_url text not null,
        article_url text not null,
        created_at timestamp not null,
        order_index integer not null
    );

create table
    platform_updates (
        id uuid primary key,
        title varchar not null,
        label varchar not null,
        created_at timestamp not null,
        order_index integer not null
    );

create table
    maintenance_messages (
        id uuid primary key,
        message text not null,
        created_at timestamp not null,
        order_index integer not null
    );

create table
    guides (
        id uuid primary key,
        title varchar not null,
        label varchar not null,
        created_at timestamp not null,
        order_index integer not null
    );

create table
    support_questions (
        id uuid primary key,
        cognito_user_id varchar not null references users (cognito_user_id) on delete cascade,
        user_email varchar not null,
        message text not null,
        created_at timestamp not null
    );
