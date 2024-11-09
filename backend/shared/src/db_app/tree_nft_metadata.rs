use chrono::{DateTime, Utc};
use diesel::dsl::sql;
use diesel::prelude::*;
use sha2::Digest;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::tree_nft_metadatas;

#[derive(
    poem_openapi::Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable,
)]
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

impl TreeNftMetadata {
    pub fn new(
        metadata_url: String,
        metadata_hash: Option<String>,
        probablity_percentage: i16,
        now: DateTime<Utc>,
    ) -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(&metadata_url);
        let hash = hasher.finalize();
        let hash = hex::encode(hash)[..16].to_string();

        Self {
            id: hash,
            metadata_url,
            metadata_hash,
            probablity_percentage,
            created_at: now.naive_utc(),
        }
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<Option<TreeNftMetadata>> {
        let inserted = diesel::insert_into(tree_nft_metadatas::table)
            .values(self)
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
            .order(sql::<diesel::sql_types::Float>(
                "random() ^ (1.0 / probablity_percentage)",
            ))
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
        diesel::delete(tree_nft_metadatas::table.filter(tree_nft_metadatas::id.eq(id)))
            .execute(conn)
    }
}
