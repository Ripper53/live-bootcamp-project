use auth_service_core::{domain::Token, requests::VerifyTokenEndpointRequest};

use crate::utilities::{get_valid_token, TestApp};

#[tokio::test]
async fn verify_token_returns_200_if_valid_token() {
    let app = TestApp::new().await;
    let response = app
        .verify_token(VerifyTokenEndpointRequest {
            token: get_valid_token(),
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_returns_422_if_invalid_token() {
    let app = TestApp::new().await;
    let response = app
        .verify_token(VerifyTokenEndpointRequest {
            token: Token::new("INVALID_TOKEN".into()),
        })
        .await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn verify_token_returns_422_if_malformed_input() {
    let app = TestApp::new().await;
    let response = app
        .post_request(
            "verify-token",
            serde_json::json!({
                "missing_token_field": "TEST_TOKEN",
            }),
        )
        .await;
    assert_eq!(response.status().as_u16(), 422);
}
