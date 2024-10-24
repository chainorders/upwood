use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use serde::Deserialize;
use uuid::Uuid;

use super::aws::s3::FilesBucket;
use super::{ensure_is_admin, BearerAuthorization, Error, JsonResult, NoResResult};

pub struct Api;

#[OpenApi]
impl Api {
    /// Upload a file
    #[oai(path = "/admin/files/s3/upload_url", method = "post")]
    pub async fn admin_s3_file_upload_url(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(files_bucket): Data<&FilesBucket>,
    ) -> JsonResult<S3UploadUrlResponse> {
        ensure_is_admin(&claims)?;
        let file_name = Uuid::new_v4();
        let presigned_url = files_bucket
            .create_presigned_url(&file_name.to_string())
            .await
            .map_err(|e| Error::InternalServer(PlainText(format!("Server Error: {}", e))))?;
        Ok(Json(S3UploadUrlResponse {
            file_name,
            presigned_url,
        }))
    }


    #[oai(path = "/admin/files/:file_name", method = "delete")]
    pub async fn admin_s3_file_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(files_bucket): Data<&FilesBucket>,
        Path(file_name): Path<Uuid>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        files_bucket
            .delete(&file_name.to_string())
            .await
            .map_err(|e| Error::InternalServer(PlainText(format!("Server Error: {}", e))))?;

        Ok(())
    }
}

#[derive(Object, Deserialize)]
pub struct S3UploadUrlResponse {
    pub presigned_url: String,
    pub file_name:     Uuid,
}
