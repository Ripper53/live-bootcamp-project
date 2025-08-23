use auth_service_core::{
    requests::{SignupEndpointRequest, TwoFactorAuthentication},
    responses::signup::SignupEndpointResponse,
};

use crate::utilities::{get_random_email, get_random_password, TestApp};

#[tokio::test]
async fn signup_returns_201_and_json_if_valid_input() {
    let app = TestApp::new().await;
    let response = app
        .signup(SignupEndpointRequest {
            email: get_random_email(),
            password: get_random_password(),
            two_factor_authentication: TwoFactorAuthentication::Disabled,
        })
        .await;
    assert_eq!(response.status().as_u16(), 201);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );
    let response = response
        .json::<SignupEndpointResponse>()
        .await
        .expect("Failed to deserialize response body to SignupEndpointResponse");
    assert_eq!("user created successfully", response.message);
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
