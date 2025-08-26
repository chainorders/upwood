/* plpgsql-language-server:disable validation */
   CREATE TABLE offchain_rewards_contracts (
          contract_address NUMERIC(20) PRIMARY KEY NOT NULL,
          treasury_address VARCHAR NOT NULL,
          create_time TIMESTAMP NOT NULL,
          update_time TIMESTAMP NOT NULL
          );

   CREATE TABLE offchain_reward_contract_agents (
          contract_address NUMERIC(20) NOT NULL REFERENCES offchain_rewards_contracts (contract_address) ON DELETE cascade,
          agent_address VARCHAR NOT NULL,
          create_time TIMESTAMP NOT NULL,
          update_time TIMESTAMP NOT NULL,
          PRIMARY KEY (contract_address, agent_address)
          );

   CREATE TABLE offchain_rewardees (
          contract_address NUMERIC(20) NOT NULL REFERENCES offchain_rewards_contracts (contract_address) ON DELETE cascade,
          account_address VARCHAR NOT NULL,
          nonce NUMERIC(20) NOT NULL,
          create_time TIMESTAMP NOT NULL,
          update_time TIMESTAMP NOT NULL,
          PRIMARY KEY (contract_address, account_address)
          );

   CREATE TABLE offchain_reward_claims (
          id uuid PRIMARY KEY NOT NULL,
          block_height NUMERIC(20) NOT NULL,
          txn_index NUMERIC(20) NOT NULL,
          contract_address NUMERIC(20) NOT NULL,
          reward_id bytea NOT NULL,
          account_address VARCHAR NOT NULL,
          nonce NUMERIC(20) NOT NULL,
          reward_amount NUMERIC(78) NOT NULL,
          reward_token_id NUMERIC(20) NOT NULL,
          reward_token_contract_address NUMERIC(20) NOT NULL,
          create_time TIMESTAMP NOT NULL,
          FOREIGN key (contract_address, account_address) REFERENCES offchain_rewardees (contract_address, account_address) ON DELETE cascade
          );
