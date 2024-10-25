mod test_utils;

use passwords::PasswordGenerator;
use test_utils::create_login_admin_user;
use test_utils::test_api::TestApi;
use test_utils::test_cognito::TestCognito;
use upwood::api;
use uuid::Uuid;

#[tokio::test]
async fn test_s3_public_files() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("secure.env"))
        .ok();

    let config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let pass_generator = PasswordGenerator::new()
        .length(10)
        .numbers(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .symbols(true)
        .spaces(false)
        .strict(true);

    let mut api = TestApi::new(config.clone()).await;
    let mut cognito = TestCognito::new(
        &sdk_config,
        &config.aws_user_pool_id,
        &config.aws_user_pool_client_id,
    );

    let email = format!("s3_files_{}@yopmail.com", Uuid::new_v4());
    let password = pass_generator.generate_one().unwrap();
    let (cognito_user_id, id_token) =
        create_login_admin_user(&mut cognito, &mut api, &email, &password).await;
    println!("test user id: {}", cognito_user_id);
    // create upload url
    let create_url_res = api.admin_file_upload_url_s3(&id_token).await;
    println!("upload url: {}", create_url_res.presigned_url);
    // upload file
    let client = reqwest::Client::new();
    let res = client
        .put(&create_url_res.presigned_url)
        .header("Content-Type", "image/jpeg")
        .body(include_bytes!("test_forest_image.jpeg").to_vec())
        .send()
        .await
        .expect("Failed to upload file");
    assert_eq!(res.status(), 200);
    println!("file uploaded");
    // clear test data
    api.admin_delete_file_s3(&id_token, &create_url_res.file_name)
        .await;
    println!("file deleted");
    api.admin_user_delete(&id_token, &cognito_user_id).await;
    println!("user deleted");
}

#[tokio::test]
async fn test_ipfs_public_files() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("secure.env"))
        .ok();

    let config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let pass_generator = PasswordGenerator::new()
        .length(10)
        .numbers(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .symbols(true)
        .spaces(false)
        .strict(true);

    let mut api = TestApi::new(config.clone()).await;
    let mut cognito = TestCognito::new(
        &sdk_config,
        &config.aws_user_pool_id,
        &config.aws_user_pool_client_id,
    );

    let email = format!("s3_files_{}@yopmail.com", Uuid::new_v4());
    let password = pass_generator.generate_one().unwrap();
    let (cognito_user_id, id_token) =
        create_login_admin_user(&mut cognito, &mut api, &email, &password).await;
    println!("test user id: {}", cognito_user_id);
    // create upload url
    let create_url_res = api.admin_file_upload_url_ipfs(&id_token).await;
    println!("upload url: {}", create_url_res.presigned_url);
    // upload file
    let client = reqwest::Client::new();
    let res = client
        .put(&create_url_res.presigned_url)
        .header("Content-Type", "image/jpeg")
        .body(include_bytes!("test_forest_image.jpeg").to_vec())
        .send()
        .await
        .expect("Failed to upload file");
    assert_eq!(res.status(), 200);
    println!("file uploaded");
    // clear test data
    api.admin_delete_file_ipfs(&id_token, &create_url_res.file_name)
        .await;
    println!("file deleted");
    api.admin_user_delete(&id_token, &cognito_user_id).await;
    println!("user deleted");
}
