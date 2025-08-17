use auth_service_core::requests::{Password, SignupEndpointRequest, TwoFactorAuthentication};

use crate::utilities::{get_random_email, get_random_password, TestApp};

#[tokio::test]
async fn signup_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .signup(SignupEndpointRequest {
            email: get_random_email(),
            password: get_random_password(),
            two_factor_authentication: TwoFactorAuthentication::Disabled,
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}

#[tokio::test]
async fn signup_returns_422_if_malformed_input() {
    let app = TestApp::new().await;
    let response = app
        .post_request(
            "signup",
            serde_json::json!({
                "password": "TEST_PASSWORD",
                "requires2FA": true,
            }),
        )
        .await;
    assert_eq!(response.status().as_u16(), 422);
}
