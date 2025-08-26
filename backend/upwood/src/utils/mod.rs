pub mod aws;
pub mod concordium;
pub mod ipfs;

pub type CognitoClient = aws_sdk_cognitoidentityprovider::Client;
pub type S3Client = aws_sdk_s3::Client;
