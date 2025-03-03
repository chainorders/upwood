use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use serde::Deserialize;
use uuid::Uuid;

use super::aws::s3;
use super::*;

pub struct Api;

#[OpenApi]
impl Api {
    /// Create a presigned URL to upload a file to S3
    ///
    /// Requires admin privileges
    ///
    /// # Arguments
    /// * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
    /// * `Data(files_bucket): Data<&s3::FilesBucket>` - The S3 bucket for files
    ///
    /// # Returns
    /// * `Json<UploadUrlResponse>` - The presigned URL to upload the file & file name
    #[oai(
        path = "/admin/files/s3/upload_url",
        method = "post",
        tag = "ApiTags::Files"
    )]
    pub async fn admin_s3_file_upload_url(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(files_bucket): Data<&s3::FilesBucket>,
    ) -> JsonResult<UploadUrlResponse> {
        ensure_is_admin(&claims)?;
        let file_name = Uuid::new_v4();
        let presigned_url = files_bucket
            .create_presigned_url(&file_name.to_string())
            .await
            .map_err(|e| Error::InternalServer(PlainText(format!("Server Error: {}", e))))?;
        Ok(Json(UploadUrlResponse {
            file_name: file_name.to_string(),
            presigned_url,
        }))
    }

    /// Delete a file from S3
    ///
    /// Requires admin privileges
    ///
    /// # Arguments
    /// * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
    /// * `Data(files_bucket): Data<&s3::FilesBucket>` - The S3 bucket for files
    /// * `Path(file_name): Path<Uuid>` - The file name to delete
    ///
    /// # Returns
    /// * `NoResResult` - The result of the operation
    #[oai(
        path = "/admin/files/s3/:file_name",
        method = "delete",
        tag = "ApiTags::Files"
    )]
    pub async fn admin_s3_file_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(files_bucket): Data<&s3::FilesBucket>,
        Path(file_name): Path<String>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        files_bucket
            .delete(&file_name)
            .await
            .map_err(|e| Error::InternalServer(PlainText(format!("Server Error: {}", e))))?;

        Ok(())
    }

    /// Create a presigned URL to upload a file to IPFS
    ///
    /// Requires admin privileges
    ///
    /// # Arguments
    /// * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
    /// * `Data(files_bucket): Data<&ipfs::filebase::FilesBucket>` - The IPFS bucket for files
    ///
    /// # Returns
    /// * `Json<UploadUrlResponse>` - The presigned URL to upload the file & file name
    #[oai(
        path = "/admin/files/ipfs/upload_url",
        method = "post",
        tag = "ApiTags::Files"
    )]
    pub async fn admin_ipfs_file_upload_url(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(files_bucket): Data<&ipfs::filebase::FilesBucket>,
    ) -> JsonResult<UploadUrlResponse> {
        ensure_is_admin(&claims)?;
        let file_name = Uuid::new_v4();
        let presigned_url = files_bucket
            .create_presigned_url(&file_name.to_string())
            .await
            .map_err(|e| Error::InternalServer(PlainText(format!("Server Error: {}", e))))?;
        Ok(Json(UploadUrlResponse {
            file_name: file_name.to_string(),
            presigned_url,
        }))
    }

    /// Delete a file from IPFS
    ///
    /// Requires admin privileges
    ///
    /// # Arguments
    /// * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
    /// * `Data(files_bucket): Data<&ipfs::filebase::FilesBucket>` - The IPFS bucket for files
    /// * `Path(file_name): Path<Uuid>` - The file name to delete
    ///
    /// # Returns
    /// * `NoResResult` - The result of the operation
    #[oai(
        path = "/admin/files/ipfs/:file_name",
        method = "delete",
        tag = "ApiTags::Files"
    )]
    pub async fn admin_ipfs_file_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(files_bucket): Data<&ipfs::filebase::FilesBucket>,
        Path(file_name): Path<String>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        files_bucket
            .delete(&file_name)
            .await
            .map_err(|e| Error::InternalServer(PlainText(format!("Server Error: {}", e))))?;

        Ok(())
    }

    #[oai(
        path = "/files/s3/profile_picture_upload_url",
        method = "post",
        tag = "ApiTags::Files"
    )]
    pub async fn s3_profile_picture_upload_url(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(files_bucket): Data<&s3::FilesBucket>,
    ) -> JsonResult<UploadUrlResponse> {
        let file_name = format!(
            "user/{}/profile_picture_{}",
            claims.sub,
            chrono::Utc::now().timestamp()
        );
        let presigned_url = files_bucket
            .create_presigned_url(&file_name.to_string())
            .await
            .map_err(|e| Error::InternalServer(PlainText(format!("Server Error: {}", e))))?;
        Ok(Json(UploadUrlResponse {
            file_name,
            presigned_url,
        }))
    }
}

/// Represents the response from an IPFS file upload request, containing the
/// presigned URL for uploading the file and the generated file name.
#[derive(Object, Deserialize)]
pub struct UploadUrlResponse {
    pub presigned_url: String,
    pub file_name:     String,
}
