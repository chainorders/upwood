use diesel::dsl::sql;
use diesel::prelude::*;
use sha2::Digest;
use shared::db::DbConn;

use crate::db::DbResult;
use crate::schema::tree_nft_metadatas;

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::tree_nft_metadatas)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TreeNftMetadataInsert {
    pub id:                    String,
    pub metadata_url:          String,
    pub metadata_hash:         Option<String>,
    pub probablity_percentage: i16,
}

impl TreeNftMetadataInsert {
    pub fn new(metadata_url: nft_multi_rewarded::MetadataUrl, probablity_percentage: i16) -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(metadata_url.url.as_bytes());
        let hash = hasher.finalize();
        let hash = hex::encode(hash)[..16].to_string();

        Self {
            id: hash,
            metadata_url: metadata_url.url,
            metadata_hash: metadata_url.hash.map(hex::encode),
            probablity_percentage,
        }
    }
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::tree_nft_metadatas)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TreeNftMetadata {
    pub id:                    String,
    pub metadata_url:          String,
    pub metadata_hash:         Option<String>,
    pub probablity_percentage: i16,
    pub created_at:            chrono::NaiveDateTime,
}

define_sql_function! {fn random() -> diesel::sql_types::Float8}
define_sql_function! {fn power(base: diesel::sql_types::Float8, exponent: diesel::sql_types::Float8) -> diesel::sql_types::Float8}
define_sql_function! {fn div(dividend: diesel::sql_types::Integer, divisor: diesel::sql_types::Integer) -> diesel::sql_types::Integer}

pub fn insert(
    conn: &mut DbConn,
    metadata: &TreeNftMetadataInsert,
) -> DbResult<Option<TreeNftMetadata>> {
    let inserted = diesel::insert_into(tree_nft_metadatas::table)
        .values(metadata)
        .on_conflict_do_nothing()
        .returning(TreeNftMetadata::as_returning())
        .get_result(conn)
        .optional()?;
    Ok(inserted)
}

pub fn find(conn: &mut DbConn, id: &str) -> DbResult<Option<TreeNftMetadata>> {
    tree_nft_metadatas::table
        .filter(tree_nft_metadatas::id.eq(id))
        .first(conn)
        .optional()
}

pub fn find_random(conn: &mut DbConn) -> DbResult<Option<TreeNftMetadata>> {
    tree_nft_metadatas::table
        // .order_by(power(
        //     random(),
        //     sql("1.0 / ").,
        // ))
        .order(sql::<diesel::sql_types::Float>("random() ^ (1.0 / probablity_percentage)"))
        .first(conn)
        .optional()
}

pub fn list(conn: &mut DbConn, page_size: i64, page: i64) -> DbResult<Vec<TreeNftMetadata>> {
    tree_nft_metadatas::table
        .order_by(tree_nft_metadatas::created_at.desc())
        .limit(page_size)
        .offset(page * page_size)
        .load(conn)
}

pub fn delete(conn: &mut DbConn, id: &str) -> DbResult<usize> {
    diesel::delete(tree_nft_metadatas::table.filter(tree_nft_metadatas::id.eq(id))).execute(conn)
}
