CREATE TABLE companies (
    id uuid PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    registration_address TEXT,
    vat_no VARCHAR(255),
    country VARCHAR(255),
    profile_picture_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE company_invitations (
    id uuid PRIMARY KEY,
    company_id uuid NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_by VARCHAR(255) NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users (cognito_user_id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE users
ADD COLUMN company_id uuid;

ALTER TABLE users
ADD FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE SET NULL;
