pub mod filebase {
    use aws_config::{BehaviorVersion, Region, SdkConfig};
    use aws_credential_types::Credentials;
    use aws_sdk_s3::config::SharedCredentialsProvider;
    use aws_sdk_s3::error::SdkError;
    use aws_sdk_s3::operation::delete_object::DeleteObjectError;
    use aws_sdk_s3::operation::put_object::PutObjectError;
    use aws_sdk_s3::presigning::{PresigningConfig, PresigningConfigError};
    use aws_sdk_s3::Client;

    use crate::utils::S3Client;

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("presign config: {0}")]
        PresigningConfig(#[from] PresigningConfigError),
        #[error("put object: {0}")]
        PutObject(#[from] SdkError<PutObjectError>),
        #[error("delete object: {0}")]
        DeleteObject(#[from] SdkError<DeleteObjectError>),
        #[error("head: {0}")]
        Head(#[from] SdkError<aws_sdk_s3::operation::head_object::HeadObjectError>),
    }

    #[derive(Debug, Clone)]
    pub struct FilesBucket {
        pub files_bucket_name: String,
        pub client:            Client,
        pub expires_in:        std::time::Duration,
    }

    impl FilesBucket {
        pub fn new(
            filebase_s3_endpoint_url: &str,
            filebase_access_key_id: &str,
            filebase_secret_access_key: &str,
            files_bucket_name: &str,
            expires_in: std::time::Duration,
        ) -> Self {
            Self {
                files_bucket_name: files_bucket_name.to_string(),
                client: S3Client::new(
                    &SdkConfig::builder()
                        .behavior_version(BehaviorVersion::latest())
                        .endpoint_url(filebase_s3_endpoint_url)
                        .region(Region::new("us-east-1"))
                        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
                            filebase_access_key_id,
                            filebase_secret_access_key,
                            None,
                            None,
                            "filebase",
                        )))
                        .build(),
                ),
                expires_in,
            }
        }

        /// Create a presigned URL to upload a file to IPFS
        pub async fn create_presigned_url(&self, file_name: &str) -> Result<String> {
            let expires_in: PresigningConfig = PresigningConfig::expires_in(self.expires_in)?;
            let req = self
                .client
                .put_object()
                .bucket(&self.files_bucket_name)
                .key(file_name)
                .presigned(expires_in)
                .await?;
            Ok(req.uri().into())
        }

        pub async fn exists(&self, file_name: &str) -> Result<bool> {
            let resp = self
                .client
                .head_object()
                .bucket(&self.files_bucket_name)
                .key(file_name)
                .send()
                .await?;
            Ok(resp.content_length().is_some())
        }

        pub async fn delete(&self, file_name: &str) -> Result<()> {
            self.client
                .delete_object()
                .bucket(&self.files_bucket_name)
                .key(file_name)
                .send()
                .await?;
            Ok(())
        }
    }
}
