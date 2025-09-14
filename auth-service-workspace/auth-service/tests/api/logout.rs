use auth_service::utilities::constants::JWT_COOKIE_NAME;
use auth_service_core::{auth::generate_jwt_token, domain::Token};
use reqwest::Url;

use crate::utilities::{get_random_email, TestApp};

#[tokio::test]
async fn logout_returns_200_if_valid_token() {
    let app = TestApp::new().await;
    let token = generate_jwt_token(&get_random_email()).unwrap();
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            token.as_str(),
        ),
        &Url::parse("http://127.0.0.1").expect("failed to parse URL"),
    );
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_should_return_400_if_missing_jwt_cookie() {
    let app = TestApp::new().await;
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn logout_should_return_400_if_called_twice() {
    let app = TestApp::new().await;
    let token = generate_jwt_token(&get_random_email()).unwrap();
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            token.as_str(),
        ),
        &Url::parse("http://127.0.0.1").expect("failed to parse URL"),
    );
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn logout_should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("failed to parse URL"),
    );
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 401);
}
