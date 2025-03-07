mod test_utils;

use concordium_smart_contract_testing::AccountAddress;
use shared::db_app::users::UserKYCModel;
use test_utils::test_api::ApiTestClient;
use test_utils::{create_login_admin_user, create_login_user, PASS_GENERATOR};
use tracing_test::traced_test;
use upwood::api;
use upwood::utils::aws::cognito::UserPool;
use uuid::Uuid;

#[traced_test]
#[tokio::test]
async fn cognito_auth_test() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".secure.env"))
        .ok();
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
    let api_config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");
    let api_config = api::Config {
        postgres_db: db_config.postgres_db,
        postgres_host: db_config.postgres_host,
        postgres_password: db_config.postgres_password.into(),
        postgres_port: db_config.postgres_port,
        postgres_user: db_config.postgres_user,
        ..api_config
    };
    let api = ApiTestClient::new(api_config.clone()).await;
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let mut user_pool = UserPool::new(
        &sdk_config,
        &api_config.aws_user_pool_id,
        &api_config.aws_user_pool_client_id,
        &api_config.aws_user_pool_region,
    )
    .await
    .expect("Failed to create user pool");

    let email_admin = format!("cognito_auth_test_{}_admin@yopmail.com", Uuid::new_v4());
    let password_admin = PASS_GENERATOR.generate_one().unwrap();
    let (id_token_admin, _) = create_login_admin_user(
        &mut user_pool,
        email_admin.as_str(),
        password_admin.as_str(),
        AccountAddress([0; 32]).to_string().as_str(),
    )
    .await;

    // User Attributes
    let email = format!("cognito_auth_test_{}@yopmail.com", Uuid::new_v4());
    let password = PASS_GENERATOR.generate_one().unwrap();

    println!("logging in user");
    let (_, user) = create_login_user(
        &mut user_pool,
        &api,
        &id_token_admin,
        &email,
        &password,
        AccountAddress([1; 32]).to_string().as_str(),
        None,
        None,
    )
    .await;
    assert_eq!(user, UserKYCModel {
        kyc_verified:              false,
        account_address:           AccountAddress([1; 32]).to_string(),
        cognito_user_id:           user.cognito_user_id.clone(),
        email:                     email.clone(),
        first_name:                "Test First Name".to_owned(),
        last_name:                 "Test Last Name".to_owned(),
        nationality:               "IN".to_owned(),
        affiliate_commission:      api_config.affiliate_commission,
        desired_investment_amount: Some(100),
        affiliate_account_address: None,
    });

    println!("listing users");
    let users = api
        .admin_user_list(id_token_admin.clone(), None, None)
        .await;
    assert_eq!(users.page_count, 1);
    assert_eq!(users.data.len(), 1);
    let user = users.data.first().expect("No user found");
    assert_eq!(user, &UserKYCModel {
        account_address:           AccountAddress([1; 32]).to_string(),
        cognito_user_id:           user.cognito_user_id.clone(),
        email:                     email.clone(),
        first_name:                "Test First Name".to_owned(),
        last_name:                 "Test Last Name".to_owned(),
        nationality:               "IN".to_owned(),
        affiliate_commission:      api_config.affiliate_commission,
        desired_investment_amount: Some(100),
        affiliate_account_address: None,
        kyc_verified:              false,
    });

    println!("deleteting user: {}", user.cognito_user_id);
    user_pool
        .disable_user(&user.cognito_user_id)
        .await
        .expect("Failed to disable user");
    user_pool
        .user_login(&email, &password)
        .await
        .expect_err("User should not be able to login after being disabled");
}
