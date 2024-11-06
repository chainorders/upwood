create type forest_project_state as enum ('draft', 'funding', 'funded', 'archived');

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
        contract_address numeric(20) not null,
        p2p_trade_contract_address numeric(20),
        mint_fund_contract_address numeric(20),
        shares_available integer not null,
        offering_doc_link varchar,
        property_media_header text not null,
        property_media_footer text not null,
        created_at timestamp not null default now (),
        updated_at timestamp not null default now ()
    );

create table
    forest_project_prices (
        project_id uuid not null references forest_projects (id),
        price numeric(78) not null,
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
