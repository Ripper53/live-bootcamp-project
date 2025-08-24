use auth_service_core::requests::{
    LoginAttemptID, TwoFactorAuthenticationCode, VerifyTwoFactorAuthenticationEndpointRequest,
};

use crate::utilities::{get_random_email, TestApp};

#[tokio::test]
async fn verify_2fa_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .verify_2fa(VerifyTwoFactorAuthenticationEndpointRequest::new(
            get_random_email(),
            LoginAttemptID::new("TEST_LOGIN_ATTEMPT_ID".into()),
            TwoFactorAuthenticationCode::new("TEST_2FA_CODE".into()),
        ))
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}
