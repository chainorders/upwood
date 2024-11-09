create table
    tree_nft_metadatas (
        -- hex of first 64 bit of sha256 hash of metadata_url
        id char(16) primary key,
        metadata_url text not null,
        -- hex of sha256 hash of metadata_url
        metadata_hash char(64),
        probablity_percentage smallint not null,
        created_at timestamptz not null default now ()
    );
