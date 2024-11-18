#![allow(dead_code)]

use test_api::ApiTestClient;
use test_cognito::CognitoTestClient;
use upwood::api::user::UserRegisterReq;

pub mod conversions;
pub mod test_api;
pub mod test_chain;
pub mod test_cognito;
pub mod test_user;

pub async fn create_login_admin_user(
    cognito: &mut CognitoTestClient,
    api: &mut ApiTestClient,
    email: &str,
    password: &str,
) -> (String, String) {
    let user_id = api.user_send_invitation(email).await;
    cognito.admin_set_user_password(&user_id, password).await;
    let id_token = cognito
        .user_change_password(email, password, password)
        .await;
    api.user_register(id_token, &UserRegisterReq {
        desired_investment_amount: 100,
    })
    .await;
    cognito.admin_add_to_admin_group(&user_id).await;

    (user_id, cognito.user_login(email, password).await)
}
