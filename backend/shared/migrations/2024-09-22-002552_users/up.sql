/* plpgsql-language-server:disable validation */
   CREATE TABLE users (
          cognito_user_id VARCHAR(255) PRIMARY KEY NOT NULL,
          email VARCHAR(255) NOT NULL,
          account_address VARCHAR(255),
          desired_investment_amount INTEGER,
          affiliate_commission NUMERIC(6, 5) NOT NULL,
          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
          );

   CREATE INDEX users_account_address_index ON users (account_address);

   CREATE INDEX users_created_at_index ON users (created_at);

   CREATE TABLE user_challenges (
          id uuid PRIMARY KEY,
          cognito_user_id VARCHAR(255) NOT NULL REFERENCES users (cognito_user_id) ON DELETE cascade,
          challenge bytea NOT NULL,
          account_address VARCHAR(255) NOT NULL,
          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
          );

   CREATE INDEX user_challenges_user_id_index ON user_challenges (cognito_user_id);

   CREATE INDEX user_challenges_created_at_index ON user_challenges (created_at);

   CREATE TABLE user_affiliates (
          id serial PRIMARY KEY,
          cognito_user_id VARCHAR(255) NOT NULL,
          -- cannot be a foreign key because the user may not exist yet
          affiliate_account_address VARCHAR(255) NOT NULL,
          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
          );
