/* plpgsql-language-server:disable validation */
   CREATE TABLE tree_nft_metadatas (
          -- hex of first 64 bit of sha256 hash of metadata_url
          id CHAR(16) PRIMARY KEY,
          metadata_url TEXT NOT NULL,
          -- hex of sha256 hash of metadata_url
          metadata_hash CHAR(64),
          probablity_percentage SMALLINT NOT NULL,
          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
          );
