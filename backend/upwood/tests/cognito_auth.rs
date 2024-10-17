use aws_sdk_cognitoidentityprovider::types::{AuthFlowType, ChallengeNameType};
use passwords::PasswordGenerator;
use poem::http::StatusCode;
use poem::test::TestClient;
use poem::Route;
use tracing_test::traced_test;
use upwood::api;
use upwood::api::user::{User, UserDeleteReq, UserRegisterReq, UserRegistrationInvitationSendReq};

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
    let email = "admin.cognito_auth_test_5@yopmail.com";
    let password_temp = pass_generator.generate_one().unwrap();

    let api = api::create_web_app(&config).await;
    let cli = TestClient::new(api);

    let user_id = api_user_send_invitation(&cli, email).await;
    cognito_admin_set_user_password(
        &cognito_client,
        &config.aws_user_pool_id,
        &user_id,
        &password_temp,
    )
    .await;

    let password = pass_generator.generate_one().unwrap();
    let id_token = cognito_user_change_password(
        &cognito_client,
        &config.aws_user_pool_client_id,
        email,
        &password_temp,
        &password,
    )
    .await;
    println!("id_token: {}", id_token);
    let get_user_req = api_get_user_req(&cli, &id_token).await.0;
    assert_eq!(get_user_req.status(), StatusCode::NOT_FOUND);
    let user_update = api_user_register(&cli, &id_token, &UserRegisterReq {
        desired_investment_amount: 100,
    })
    .await;
    let user = api_get_user(&cli, &id_token).await;
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
        email,
        &password,
    )
    .await;
    println!("admin id_token: {}", id_token);
    let user = api_get_user(&cli, &id_token).await;
    assert_eq!(user, User {
        email:                     email.to_owned(),
        cognito_user_id:           user_id.to_owned(),
        account_address:           None,
        desired_investment_amount: Some(100),
        is_admin:                  true,
        kyc_verified:              false,
    });

    // Enhancement: mock the broswer wallet in order to create identity proofs
    // let wallet_account = WalletAccount::from_json_str(WALLET_JSON_STR).unwrap();
    // let _ = api_get_challenge(&cli, &id_token, &CreateChallengeRequest {
    //     account_address: wallet_account.address.to_string(),
    // })
    // .await;

    // api_get_user_req(&cli, "Invalid Id Token")
    //     .await
    //     .assert_status(StatusCode::UNAUTHORIZED);

    api_delete_user(&cli, &id_token, &user.email).await;
    let get_user_req = api_get_user_req(&cli, &id_token).await.0;
    assert_eq!(get_user_req.status(), StatusCode::NOT_FOUND);
}

async fn api_user_send_invitation(api: &TestClient<Route>, email: &str) -> String {
    let invitation_res = api
        .post("/users/register/invitation")
        .body_json(&UserRegistrationInvitationSendReq {
            email: email.to_owned(),
        })
        .send()
        .await
        .0;
    assert_eq!(invitation_res.status(), StatusCode::OK);
    let id: String = invitation_res
        .into_body()
        .into_json()
        .await
        .expect("Failed to parse invitation response");
    id.to_owned()
}

async fn api_user_register(api: &TestClient<Route>, id_token: &str, req: &UserRegisterReq) -> User {
    let res = api
        .post("/users/register")
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

async fn api_get_user(api: &TestClient<Route>, id_token: &str) -> User {
    let res = api_get_user_req(api, id_token).await.0;
    assert_eq!(res.status(), StatusCode::OK);
    let res: User = res
        .into_body()
        .into_json()
        .await
        .expect("Failed to parse get user response");
    res
}

async fn api_get_user_req(api: &TestClient<Route>, id_token: &str) -> poem::test::TestResponse {
    api.get("/users/self")
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
}

async fn api_delete_user(api: &TestClient<Route>, id_token: &str, email: &str) {
    let res = api
        .delete("/users")
        .header("Authorization", format!("Bearer {}", id_token))
        .body_json(&UserDeleteReq {
            email: email.to_owned(),
        })
        .send()
        .await
        .0;
    assert_eq!(res.status(), StatusCode::OK);
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
