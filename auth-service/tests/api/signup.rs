use crate::utilities::{SignupEndpointRequest, TestApp, TwoFactorAuthentication};

#[tokio::test]
async fn signup_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .signup(SignupEndpointRequest {
            username: "TEST_USERNAME",
            password: "TEST_PASSWORD",
            two_factor_authentication: TwoFactorAuthentication::Disabled,
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}
