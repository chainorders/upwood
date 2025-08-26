/* plpgsql-language-server:disable validation */
   CREATE TABLE nft_multi_rewarded_contracts (
          contract_address NUMERIC(20) PRIMARY KEY NOT NULL REFERENCES listener_contracts (contract_address) ON DELETE cascade,
          reward_token_id NUMERIC(20) NOT NULL,
          reward_token_address NUMERIC(20) NOT NULL,
          create_time TIMESTAMP NOT NULL,
          update_time TIMESTAMP NOT NULL
          );

   CREATE TABLE nft_multi_address_nonces (
          contract_address NUMERIC(20) NOT NULL REFERENCES nft_multi_rewarded_contracts (contract_address) ON DELETE cascade,
          address VARCHAR NOT NULL,
          nonce BIGINT NOT NULL,
          PRIMARY KEY (contract_address, address)
          );
