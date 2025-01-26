/* plpgsql-language-server:disable validation */
CREATE TYPE forest_project_state AS ENUM('draft', 'active', 'funded', 'archived');

CREATE TABLE forest_projects (
       id uuid PRIMARY KEY NOT NULL,
       name VARCHAR NOT NULL,
       label VARCHAR NOT NULL,
       desc_short TEXT NOT NULL,
       desc_long TEXT NOT NULL,
       area VARCHAR NOT NULL,
       carbon_credits INTEGER NOT NULL,
       roi_percent REAL NOT NULL,
       state forest_project_state NOT NULL,
       image_small_url VARCHAR NOT NULL,
       image_large_url VARCHAR NOT NULL,
       geo_spatial_url VARCHAR,
       shares_available INTEGER NOT NULL,
       offering_doc_link VARCHAR,
       property_media_header TEXT NOT NULL,
       property_media_footer TEXT NOT NULL,
       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE forest_project_property_media (
       id uuid PRIMARY KEY NOT NULL,
       project_id uuid NOT NULL REFERENCES forest_projects (id),
       image_url VARCHAR NOT NULL,
       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE forest_project_notifications (
       id uuid PRIMARY KEY NOT NULL,
       project_id uuid NOT NULL REFERENCES forest_projects (id),
       cognito_user_id VARCHAR NOT NULL REFERENCES users (cognito_user_id),
       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE forest_project_legal_contracts (
       project_id uuid PRIMARY KEY NOT NULL REFERENCES forest_projects (id),
       text_url VARCHAR NOT NULL,
       edoc_url VARCHAR NOT NULL,
       pdf_url VARCHAR NOT NULL,
       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE forest_project_legal_contract_user_signatures (
       project_id uuid NOT NULL REFERENCES forest_projects (id),
       cognito_user_id VARCHAR NOT NULL REFERENCES users (cognito_user_id),
       user_account VARCHAR NOT NULL,
       user_signature TEXT NOT NULL,
       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
       PRIMARY KEY (project_id, cognito_user_id)
);
