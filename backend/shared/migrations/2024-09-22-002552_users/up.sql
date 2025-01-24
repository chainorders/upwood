/* plpgsql-language-server:disable validation */
CREATE TABLE users (
       cognito_user_id VARCHAR(255) PRIMARY KEY NOT NULL,
       email VARCHAR(255) NOT NULL,
       first_name VARCHAR(255) NOT NULL,
       last_name VARCHAR(255) NOT NULL,
       nationality VARCHAR(255) NOT NULL,
       account_address VARCHAR(255) NOT NULL,
       desired_investment_amount INTEGER DEFAULT NULL,
       affiliate_commission NUMERIC(6, 5) NOT NULL DEFAULT 0,
       affiliate_account_address VARCHAR(255),
       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       UNIQUE (email),
       UNIQUE (account_address)
);

CREATE INDEX users_account_address_index ON users (account_address);

CREATE INDEX users_created_at_index ON users (created_at);

CREATE TABLE user_registration_requests (
       id uuid PRIMARY KEY,
       email VARCHAR(255) NOT NULL,
       affiliate_account_address VARCHAR(255) DEFAULT NULL,
       is_accepted BOOLEAN NOT NULL DEFAULT FALSE,
       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
