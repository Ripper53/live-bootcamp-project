use crate::helper::{
    LoginEndpointRequest, LogoutEndpointRequest, SignupEndpointRequest, TestApp,
    TwoFactorAuthentication, VerifyTokenEndpointRequest,
    VerifyTwoFactorAuthenticationEndpointRequest,
};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.
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

#[tokio::test]
async fn login_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .login(LoginEndpointRequest {
            username: "TEST_USERNAME",
            password: "TEST_PASSWORD",
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}

#[tokio::test]
async fn logout_returns_json() {
    let app = TestApp::new().await;
    let response = app.logout(LogoutEndpointRequest { jwt: "TEST_JWT" }).await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}

#[tokio::test]
async fn verify_2fa_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .verify_2fa(VerifyTwoFactorAuthenticationEndpointRequest {
            email: "TEST@EMAIL.COM",
            login_attempt_id: "TEST_LOGIN_ATTEMPT_ID",
            two_factor_authentication_code: "TEST_2FA_CODE",
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}

#[tokio::test]
async fn verify_token_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .verify_token(VerifyTokenEndpointRequest {
            token: "TEST_TOKEN",
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}
