use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use shared::db::DbPool;

use super::PAGE_SIZE;
use crate::api::*;
use crate::db;
use crate::db::tree_nft_metadata::TreeNftMetadata;

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/admin/tree_nft/metadata", method = "post", tag = "ApiTags::TreeNft")]
    /// Inserts a new TreeNftMetadata record in the database.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `req`: The request body containing the metadata to be inserted.
    ///
    /// # Returns
    /// The newly inserted `TreeNftMetadata` record.
    pub async fn metadata_insert(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(req): Json<AddMetadataRequest>,
    ) -> JsonResult<TreeNftMetadata> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::insert(&mut conn, req.try_into()?)?.ok_or(
            Error::BadRequest(PlainText("Failed to insert metadata".to_string())),
        )?;
        Ok(Json(metadata))
    }

    #[oai(
        path = "/admin/tree_nft/metadata/list/:page",
        method = "get",
        tag = "ApiTags::TreeNft"
    )]
    /// Lists all TreeNftMetadata records in the database, paginated by the given page number.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `page`: The page number to retrieve, starting from 0.
    ///
    /// # Returns
    /// A vector of `TreeNftMetadata` records for the given page.
    pub async fn metadata_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
    ) -> JsonResult<Vec<TreeNftMetadata>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::list(&mut conn, PAGE_SIZE, page)?;
        Ok(Json(metadata.into_iter().collect()))
    }

    #[oai(
        path = "/admin/tree_nft/metadata/:id",
        method = "get",
        tag = "ApiTags::TreeNft"
    )]
    /// Retrieves a TreeNftMetadata record from the database by its ID.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `id`: The ID of the TreeNftMetadata record to retrieve.
    ///
    /// # Returns
    /// The requested TreeNftMetadata record, or a NotFound error if the record is not found.
    pub async fn metadata_get(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<String>,
    ) -> JsonResult<TreeNftMetadata> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::find(&mut conn, &id)?
            .ok_or(Error::NotFound(PlainText("Metadata not found".to_string())))?;

        Ok(Json(metadata))
    }

    #[oai(
        path = "/admin/tree_nft/metadata/:id",
        method = "delete",
        tag = "ApiTags::TreeNft"
    )]
    /// Deletes a TreeNftMetadata record from the database by its ID.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `id`: The ID of the TreeNftMetadata record to delete.
    ///
    /// # Returns
    /// A NoResResult indicating success or failure of the deletion.
    pub async fn metadata_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<String>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let row_count = db::tree_nft_metadata::delete(&mut conn, &id)?;
        if row_count != 1 {
            return Err(Error::NotFound(PlainText("Metadata not found".to_string())));
        }
        Ok(())
    }
}

#[derive(Object, Debug)]
pub struct MetadataUrl {
    pub url:  String,
    pub hash: Option<String>,
}

#[derive(Object, Debug)]
pub struct AddMetadataRequest {
    pub metadata_url:          MetadataUrl,
    /// The probability of the metadata to be chosen for minting
    /// between 1 and 100
    pub probablity_percentage: i16,
}

impl AddMetadataRequest {
    pub fn probablity_percentage(&self) -> Result<i16> {
        if self.probablity_percentage < 1 || self.probablity_percentage > 100 {
            return Err(Error::BadRequest(PlainText(
                "Probablity percentage must be between 1 and 100".to_string(),
            )));
        }

        Ok(self.probablity_percentage)
    }

    pub fn metadata_url(&self) -> Result<::nft_multi_rewarded::MetadataUrl> {
        Ok(::nft_multi_rewarded::MetadataUrl {
            url:  self.metadata_url.url.clone(),
            hash: self
                .metadata_url
                .hash
                .as_ref()
                .map(hex::decode)
                .transpose()
                .map_err(|_| {
                    Error::BadRequest(PlainText(
                        "Metadata hash must be a valid hex string".to_string(),
                    ))
                })?
                .map(|hash| hash.try_into())
                .transpose()
                .map_err(|_| {
                    Error::BadRequest(PlainText(
                        "Metadata hash must be a valid hex string".to_string(),
                    ))
                })?,
        })
    }
}

impl TryInto<db::tree_nft_metadata::TreeNftMetadataInsert> for AddMetadataRequest {
    type Error = Error;

    fn try_into(self) -> Result<db::tree_nft_metadata::TreeNftMetadataInsert> {
        Ok(db::tree_nft_metadata::TreeNftMetadataInsert::new(
            self.metadata_url()?,
            self.probablity_percentage()?,
        ))
    }
}
