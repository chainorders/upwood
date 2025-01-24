#![allow(dead_code)]

use concordium_rust_sdk::id::types::ACCOUNT_ADDRESS_SIZE;
use concordium_smart_contract_testing::AccountAddress;
use passwords::PasswordGenerator;
use rust_decimal::Decimal;
use upwood::api::user::{LoginReq, UserCreatePostReqAdmin, UserRegistrationRequestApi};
use upwood::utils::aws::cognito::{
    account_address_attribute, email_attribute, email_verified_attribute, first_name_attribute,
    last_name_attribute, nationality_attribute, UserPool,
};

pub mod conversions;
pub mod test_api;
pub mod test_chain;
pub mod test_cognito;
pub mod test_user;

pub async fn create_login_admin_user(
    user_pool: &mut UserPool,
    api: &test_api::ApiTestClient,
    email: &str,
    password: &str,
    account_address: &str,
) -> upwood::api::user::LoginRes {
    user_pool
        .admin_create_user(email, password, vec![
            email_attribute(email),
            email_verified_attribute(true),
            account_address_attribute(account_address),
            first_name_attribute("Admin"),
            last_name_attribute("Admin"),
            nationality_attribute(""),
        ])
        .await
        .expect("Failed to create user");
    user_pool.admin_add_to_admin_group(email).await;
    api.user_login(LoginReq {
        email:    email.to_string(),
        password: password.to_string(),
    })
    .await
}

pub async fn create_login_user(
    api: &test_api::ApiTestClient,
    id_token_admin: &str,
    email: &str,
    password: &str,
    account_address: &str,
    affiliate_account_address: Option<String>,
    affiliate_commission: Option<Decimal>,
) -> upwood::api::user::LoginRes {
    api.user_registration_request(UserRegistrationRequestApi {
        email: email.to_string(),
        affiliate_account_address,
    })
    .await;
    let req_list = api
        .admin_registration_request_list(id_token_admin.to_string(), 0)
        .await;
    let reg_req = req_list.data.first().expect("No user reg requests found");
    api.admin_registration_request_accept(id_token_admin.to_string(), reg_req.id, true)
        .await;
    // Enhancement: mock the broswer wallet in order to create identity proofs
    api.admin_user_register(
        id_token_admin.to_string(),
        reg_req.id,
        UserCreatePostReqAdmin {
            account_address: account_address.to_string(),
            first_name: "Test First Name".to_owned(),
            last_name: "Test Last Name".to_owned(),
            nationality: "IN".to_owned(),
            password: password.to_string(),
            desired_investment_amount: Some(100),
            affiliate_commission,
        },
    )
    .await;
    api.user_login(LoginReq {
        email:    email.to_string(),
        password: password.to_string(),
    })
    .await
}

pub const PASS_GENERATOR: PasswordGenerator = PasswordGenerator::new()
    .length(10)
    .numbers(true)
    .lowercase_letters(true)
    .uppercase_letters(true)
    .symbols(true)
    .spaces(false)
    .strict(true);
