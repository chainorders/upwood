use aws_sdk_cognitoidentityprovider::types::{AuthFlowType, ChallengeNameType};
use concordium_smart_contract_testing::AccountAddress;
use passwords::PasswordGenerator;
use poem::http::StatusCode;
use poem::test::TestClient;
use poem::Route;
use shared::api::PagedResponse;
use tracing_test::traced_test;
use upwood::api;
use upwood::api::user::{
    AdminUser, User, UserRegisterReq, UserRegistrationInvitationSendReq,
    UserUpdateAccountAddressRequest,
};
use uuid::Uuid;

#[traced_test]
#[tokio::test]
async fn cognito_auth_test() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    let config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let cognito_client = aws_sdk_cognitoidentityprovider::Client::new(&sdk_config);
    let pass_generator = PasswordGenerator::new()
        .length(10)
        .numbers(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .symbols(true)
        .spaces(false)
        .strict(true);
    // User Attributes
    let email = format!("cognito_auth_test_{}@yopmail.com", Uuid::new_v4());
    let password_temp = pass_generator.generate_one().unwrap();

    let api = api::create_web_app(&config).await;
    let cli = TestClient::new(api);

    let user_id = api_user_send_invitation(&cli, &email).await;
    // This is needed just to ensure that temp passwords match
    // API call sets random passwords for Cognito users (It it set by Cognito)
    cognito_admin_set_user_password(
        &cognito_client,
        &config.aws_user_pool_id,
        &user_id,
        &password_temp,
    )
    .await;

    let password = pass_generator.generate_one().unwrap();
    // When the user uses the temp password first his authentication response will have Force Password Change Challenge
    let id_token = cognito_user_change_password(
        &cognito_client,
        &config.aws_user_pool_client_id,
        &email,
        &password_temp,
        &password,
    )
    .await;
    let get_user_req = api_user_self_req(&cli, &id_token).await.0;
    // User is still not registered with the API hence it is not found
    assert_eq!(get_user_req.status(), StatusCode::NOT_FOUND);
    // Upon setting the password user gets back a new id token
    let user_update = api_user_register(&cli, &id_token, &UserRegisterReq {
        desired_investment_amount: 100,
    })
    .await;
    let user = api_user_self(&cli, &id_token).await;
    assert_eq!(user_update, user);
    assert_eq!(user, User {
        email:                     email.to_owned(),
        cognito_user_id:           user_id.to_owned(),
        account_address:           None,
        desired_investment_amount: Some(100),
        is_admin:                  false,
        kyc_verified:              false,
    });

    cognito_admin_add_to_admin_group(&cognito_client, &config.aws_user_pool_id, &user_id).await;
    let id_token = cognito_user_login(
        &cognito_client,
        &config.aws_user_pool_client_id,
        &email,
        &password,
    )
    .await;
    let user = api_user_self(&cli, &id_token).await;
    assert_eq!(user, User {
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
    api_admin_user_update_account_address(&cli, &id_token, &user.cognito_user_id, &account_address)
        .await;
    println!("updated account address: {}", user.cognito_user_id);
    let id_token = cognito_user_login(
        &cognito_client,
        &config.aws_user_pool_client_id,
        &email,
        &password,
    )
    .await;
    let user = api_user_self(&cli, &id_token).await;
    assert_eq!(user, User {
        email:                     email.to_owned(),
        cognito_user_id:           user_id.to_owned(),
        account_address:           Some(account_address.clone()),
        desired_investment_amount: Some(100),
        is_admin:                  true,
        kyc_verified:              false,
    });

    println!("listing users");
    let users = api_admin_list_users(&cli, &id_token, 0).await;
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
    api_admin_user_delete(&cli, &id_token, &user.cognito_user_id).await;
    println!("deleted user: {}", user.cognito_user_id);
    let get_user_req = api_user_self_req(&cli, &id_token).await.0;
    assert_eq!(get_user_req.status(), StatusCode::NOT_FOUND);
}

async fn api_user_send_invitation(api: &TestClient<Route>, email: &str) -> String {
    let mut invitation_res = api
        .post("/users/invitation")
        .body_json(&UserRegistrationInvitationSendReq {
            email:                     email.to_owned(),
            affiliate_account_address: None,
        })
        .send()
        .await
        .0;
    match invitation_res.status() {
        StatusCode::OK => {
            let id: String = invitation_res
                .into_body()
                .into_json()
                .await
                .expect("Failed to parse invitation response");
            id.to_owned()
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            let res = invitation_res
                .take_body()
                .into_string()
                .await
                .expect("Failed to parse invitation response");
            panic!("Failed to send invitation: {}", res);
        }
        res => panic!("Unexpected response: {}", res),
    }
}

async fn api_user_register(api: &TestClient<Route>, id_token: &str, req: &UserRegisterReq) -> User {
    let res = api
        .post("/users")
        .header("Authorization", format!("Bearer {}", id_token))
        .body_json(req)
        .send()
        .await
        .0;
    assert_eq!(res.status(), StatusCode::OK);
    res.into_body()
        .into_json()
        .await
        .expect("Failed to parse update user response")
}

async fn api_user_self(api: &TestClient<Route>, id_token: &str) -> User {
    let mut res = api_user_self_req(api, id_token).await.0;
    match res.status() {
        StatusCode::OK => {
            let res: User = res
                .into_body()
                .into_json()
                .await
                .expect("Failed to parse get user response");
            res
        }
        status_code => {
            let res = res
                .take_body()
                .into_string()
                .await
                .expect("Failed to parse get user response");
            panic!("Failed to get user: {} {}", status_code, res);
        }
    }
}

async fn api_user_self_req(api: &TestClient<Route>, id_token: &str) -> poem::test::TestResponse {
    api.get("/users")
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
}

async fn api_admin_user_delete(api: &TestClient<Route>, id_token: &str, cognito_user_id: &str) {
    let mut res = api
        .delete(format!("/admin/users/{}", cognito_user_id))
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
        .0;
    match res.status() {
        StatusCode::OK => {}
        status_code => {
            let res = res
                .take_body()
                .into_string()
                .await
                .expect("Failed to parse delete user response");
            panic!("Failed to delete user: {} {}", status_code, res);
        }
    }
}

async fn api_admin_user_update_account_address(
    api: &TestClient<Route>,
    id_token: &str,
    cognito_user_id: &str,
    address: &str,
) {
    let res = api
        .put(format!("/admin/users/{}/account_address", cognito_user_id))
        .header("Authorization", format!("Bearer {}", id_token))
        .body_json(&UserUpdateAccountAddressRequest {
            account_address: address.to_owned(),
        })
        .send()
        .await
        .0;
    assert_eq!(res.status(), StatusCode::OK);
}

async fn api_admin_list_users(
    api: &TestClient<Route>,
    id_token: &str,
    page: i64,
) -> PagedResponse<AdminUser> {
    let res = api
        .get(format!("/admin/users/list/{}", page))
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
        .0;
    assert_eq!(res.status(), StatusCode::OK);
    res.into_body()
        .into_json()
        .await
        .expect("Failed to parse list users response")
}

async fn cognito_admin_add_to_admin_group(
    cognito_client: &aws_sdk_cognitoidentityprovider::Client,
    aws_user_pool_id: &str,
    username: &str,
) {
    cognito_client
        .admin_add_user_to_group()
        .user_pool_id(aws_user_pool_id.to_owned())
        .username(username.to_owned())
        .group_name("admin".to_owned())
        .send()
        .await
        .expect("Failed to add user to admin group");
}

async fn cognito_admin_set_user_password(
    cognito_client: &aws_sdk_cognitoidentityprovider::Client,
    aws_user_pool_id: &str,
    username: &str,
    password: &str,
) {
    cognito_client
        .admin_set_user_password()
        .user_pool_id(aws_user_pool_id.to_owned())
        .username(username.to_owned())
        .password(password.to_owned())
        .permanent(false)
        .send()
        .await
        .expect("Failed to set user password");
}

async fn cognito_user_change_password(
    cognito_client: &aws_sdk_cognitoidentityprovider::Client,
    aws_user_pool_client_id: &str,
    email: &str,
    password_temp: &str,
    password: &str,
) -> String {
    let auth_response = cognito_user_initiate_auth_req(
        cognito_client,
        aws_user_pool_client_id,
        email,
        password_temp,
    )
    .await;
    assert!(auth_response
        .challenge_name
        .is_some_and(|c| c.eq(&ChallengeNameType::NewPasswordRequired)));

    let res = cognito_client
        .respond_to_auth_challenge()
        .client_id(aws_user_pool_client_id)
        .challenge_name(ChallengeNameType::NewPasswordRequired)
        .challenge_responses("USERNAME", email)
        .challenge_responses("NEW_PASSWORD", password)
        .set_session(auth_response.session)
        .send()
        .await
        .expect("Failed to respond to new password required challenge");
    res.authentication_result
        .expect("Failed to get auth response")
        .id_token
        .expect("Failed to get id token")
}

async fn cognito_user_login(
    cognito_client: &aws_sdk_cognitoidentityprovider::Client,
    aws_user_pool_client_id: &str,
    email: &str,
    password: &str,
) -> String {
    let auth_response =
        cognito_user_initiate_auth_req(cognito_client, aws_user_pool_client_id, email, password)
            .await;
    assert!(auth_response.challenge_name.is_none());

    let auth_response = auth_response
        .authentication_result
        .expect("Failed to get auth response");
    auth_response.id_token.expect("Failed to get id token")
}

async fn cognito_user_initiate_auth_req(
    cognito_client: &aws_sdk_cognitoidentityprovider::Client,
    aws_user_pool_client_id: &str,
    email: &str,
    password: &str,
) -> aws_sdk_cognitoidentityprovider::operation::initiate_auth::InitiateAuthOutput {
    cognito_client
        .initiate_auth()
        .client_id(aws_user_pool_client_id)
        .auth_flow(AuthFlowType::UserPasswordAuth)
        .auth_parameters("USERNAME", email.to_owned())
        .auth_parameters("PASSWORD", password.to_owned())
        .send()
        .await
        .expect("Failed to initiate auth")
}
