mod test_utils;

use concordium_rust_sdk::id::types::ACCOUNT_ADDRESS_SIZE;
use concordium_smart_contract_testing::AccountAddress;
use test_utils::test_api::ApiTestClient;
use test_utils::{create_login_admin_user, PASS_GENERATOR};
use upwood::api;
use upwood::utils::aws::cognito::UserPool;
use uuid::Uuid;

#[tokio::test]
async fn test_s3_public_files() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".secure.env"))
        .ok();
    let api_config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");
    let (db_config, _container) = shared_tests::create_new_database_container().await;
    shared::db_setup::run_migrations(&db_config.db_url());
    // Uncomment the following lines to run the tests on the local database container
    // let db_config = shared_tests::PostgresTestConfig {
    //     postgres_db:       "concordium_rwa_dev".to_string(),
    //     postgres_host:     "localhost".to_string(),
    //     postgres_password: "concordium_rwa_dev_pswd".to_string(),
    //     postgres_port:     5432,
    //     postgres_user:     "concordium_rwa_dev_user".to_string(),
    // };
    let api_config = api::Config {
        postgres_db: db_config.postgres_db,
        postgres_host: db_config.postgres_host,
        postgres_password: db_config.postgres_password.into(),
        postgres_port: db_config.postgres_port,
        postgres_user: db_config.postgres_user,
        ..api_config
    };
    let mut api = ApiTestClient::new(api_config.clone()).await;
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let mut user_pool = UserPool::new(
        &sdk_config,
        api_config.aws_user_pool_id.to_string(),
        api_config.aws_user_pool_client_id.to_string(),
        api_config.aws_user_pool_region.to_string(),
    )
    .await
    .expect("Failed to create user pool");

    let email = format!("s3_files_{}@yopmail.com", Uuid::new_v4());
    let password = PASS_GENERATOR.generate_one().unwrap();
    let (id_token_admin, _) = create_login_admin_user(
        &mut user_pool,
        &email,
        &password,
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
            .to_string()
            .as_str(),
    )
    .await;
    // create upload url
    let create_url_res = api.admin_file_upload_url_s3(id_token_admin.clone()).await;
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
    api.admin_delete_file_s3(id_token_admin.clone(), create_url_res.file_name)
        .await;
    println!("file deleted");
    user_pool
        .disable_user(&email)
        .await
        .expect("Failed to disable user");
}

#[tokio::test]
async fn test_ipfs_public_files() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".secure.env"))
        .ok();
    let api_config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");
    let (db_config, _container) = shared_tests::create_new_database_container().await;
    shared::db_setup::run_migrations(&db_config.db_url());
    // Uncomment the following lines to run the tests on the local database container
    // let db_config = shared_tests::PostgresTestConfig {
    //     postgres_db:       "concordium_rwa_dev".to_string(),
    //     postgres_host:     "localhost".to_string(),
    //     postgres_password: "concordium_rwa_dev_pswd".to_string(),
    //     postgres_port:     5432,
    //     postgres_user:     "concordium_rwa_dev_user".to_string(),
    // };
    let api_config = api::Config {
        postgres_db: db_config.postgres_db,
        postgres_host: db_config.postgres_host,
        postgres_password: db_config.postgres_password.into(),
        postgres_port: db_config.postgres_port,
        postgres_user: db_config.postgres_user,
        ..api_config
    };
    let mut api = ApiTestClient::new(api_config.clone()).await;
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let mut user_pool = UserPool::new(
        &sdk_config,
        api_config.aws_user_pool_id.to_string(),
        api_config.aws_user_pool_client_id.to_string(),
        api_config.aws_user_pool_region.to_string(),
    )
    .await
    .expect("Failed to create user pool");

    let email = format!("s3_files_{}@yopmail.com", Uuid::new_v4());
    let password = PASS_GENERATOR.generate_one().unwrap();
    let (id_token_admin, _) = create_login_admin_user(
        &mut user_pool,
        &email,
        &password,
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
            .to_string()
            .as_str(),
    )
    .await;
    // create upload url
    let create_url_res = api.admin_file_upload_url_ipfs(id_token_admin.clone()).await;
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
    api.admin_delete_file_ipfs(id_token_admin.clone(), create_url_res.file_name)
        .await;
    println!("file deleted");
    user_pool
        .disable_user(&email)
        .await
        .expect("Failed to disable user");
}
