use crate::utilities::{TestApp, VerifyTwoFactorAuthenticationEndpointRequest};

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
