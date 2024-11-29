mod test_utils;

use concordium_smart_contract_testing::AccountAddress;
use passwords::PasswordGenerator;
use poem::http::StatusCode;
use rust_decimal::Decimal;
use test_utils::test_api::ApiTestClient;
use test_utils::test_cognito::CognitoTestClient;
use tracing_test::traced_test;
use upwood::api;
use upwood::api::user::{
    AdminUser, ApiUser, UserRegisterReq, UserRegistrationInvitationSendReq,
    UserUpdateAccountAddressRequest,
};
use uuid::Uuid;

#[traced_test]
#[tokio::test]
async fn cognito_auth_test() {
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

    let mut api = ApiTestClient::new(config.clone()).await;
    let mut cognito = CognitoTestClient::new(
        &sdk_config,
        config.aws_user_pool_id,
        config.aws_user_pool_client_id,
    );

    // User Attributes
    let email = format!("cognito_auth_test_{}@yopmail.com", Uuid::new_v4());
    let password_temp = pass_generator.generate_one().unwrap();

    let user_id = api
        .user_send_invitation(&UserRegistrationInvitationSendReq {
            email:                     email.clone(),
            affiliate_account_address: None,
        })
        .await;
    // This is needed just to ensure that temp passwords match
    // API call sets random passwords for Cognito users (It it set by Cognito)
    cognito
        .admin_set_user_password(&user_id, &password_temp)
        .await;

    let password = pass_generator.generate_one().unwrap();
    // When the user uses the temp password first his authentication response will have Force Password Change Challenge
    let id_token = cognito
        .user_change_password(&email, &password_temp, &password)
        .await;
    let get_user_req = api.user_self_req(id_token.clone()).await.0;
    // User is still not registered with the API hence it is not found
    assert_eq!(get_user_req.status(), StatusCode::NOT_FOUND);
    // Upon setting the password user gets back a new id token
    let user_update = api
        .user_register(id_token.clone(), &UserRegisterReq {
            desired_investment_amount: 100,
        })
        .await;
    let user = api.user_self(id_token).await;
    assert_eq!(user_update, user);
    assert_eq!(user, ApiUser {
        email:                     email.to_owned(),
        cognito_user_id:           user_id.to_owned(),
        account_address:           None,
        desired_investment_amount: Some(100),
        is_admin:                  false,
        kyc_verified:              false,
    });

    cognito.admin_add_to_admin_group(&user_id).await;
    let id_token = cognito.user_login(&email, &password).await;
    let user = api.user_self(id_token.clone()).await;
    assert_eq!(user, ApiUser {
        email:                     email.to_owned(),
        cognito_user_id:           user_id.to_owned(),
        account_address:           None,
        desired_investment_amount: Some(100),
        is_admin:                  true,
        kyc_verified:              false,
    });

    // Enhancement: mock the broswer wallet in order to create identity proofs
    println!("updating account address: {}", user.cognito_user_id);
    let account_address = AccountAddress([1; 32]).to_string();
    api.admin_update_account_address(
        id_token,
        user.cognito_user_id.clone(),
        &UserUpdateAccountAddressRequest {
            account_address:      account_address.clone(),
            affiliate_commission: Decimal::ZERO,
        },
    )
    .await;
    println!("updated account address: {}", user.cognito_user_id);
    let id_token = cognito.user_login(&email, &password).await;
    let user = api.user_self(id_token.clone()).await;
    assert_eq!(user, ApiUser {
        email:                     email.to_owned(),
        cognito_user_id:           user_id.to_owned(),
        account_address:           Some(account_address.clone()),
        desired_investment_amount: Some(100),
        is_admin:                  true,
        kyc_verified:              false,
    });

    println!("listing users");
    let users = api.admin_list_users(id_token.clone(), 0).await;
    assert_eq!(users.page, 0);
    assert!(users.data.contains(&AdminUser {
        account_address:           Some(account_address),
        cognito_user_id:           user_id.to_owned(),
        email:                     email.to_owned(),
        desired_investment_amount: Some(100),
        kyc_verified:              false,
    }));
    println!("listed users");
    println!("deleteting user: {}", user.cognito_user_id);
    api.admin_user_delete(id_token.clone(), user.cognito_user_id.clone())
        .await;
    println!("deleted user: {}", user.cognito_user_id);
    let get_user_req = api.user_self_req(id_token).await.0;
    assert_eq!(get_user_req.status(), StatusCode::NOT_FOUND);
}
