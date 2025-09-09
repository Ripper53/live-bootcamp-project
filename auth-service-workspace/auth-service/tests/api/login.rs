use auth_service::utilities::constants::JWT_COOKIE_NAME;
use auth_service_core::{
    domain::{TwoFactorAuthentication, ValidPassword},
    requests::{LoginEndpointRequest, SignupEndpointRequest},
};

use crate::utilities::{get_random_email, get_random_password, TestApp};

#[tokio::test]
async fn login_returns_200_if_correct_credentials() {
    let app = TestApp::new().await;
    let email = get_random_email();
    let password = get_random_password();
    let response = app
        .signup(SignupEndpointRequest::new(
            email.clone(),
            password.clone(),
            TwoFactorAuthentication::Disabled,
        ))
        .await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app.login(LoginEndpointRequest::new(email, password)).await;
    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("no jwt auth token found");
    assert!(!auth_cookie.value().is_empty());
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}

#[tokio::test]
async fn login_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let response = app
        .post_request(
            "login",
            serde_json::json!({
                "email": "TEST@EMAIL.COM",
            }),
        )
        .await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn login_return_400_if_malformed_credentials() {
    let app = TestApp::new().await;
    let response = app
        .post_request(
            "login",
            serde_json::json!({
                "email": "INVALID_EMAIL.COM",
                "password": "TEST_PASSWORD",
            }),
        )
        .await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn login_returns_401_if_user_password_incorrect() {
    let app = TestApp::new().await;
    let email = get_random_email();
    let password = get_random_password();
    let response = app
        .signup(SignupEndpointRequest::new(
            email.clone(),
            password.clone(),
            TwoFactorAuthentication::Disabled,
        ))
        .await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app
        .login(LoginEndpointRequest::new(
            email,
            ValidPassword::try_new("INCORRECT_TEST_PASSWORD".into()).unwrap(),
        ))
        .await;
    assert_eq!(response.status().as_u16(), 401);
}
