use auth_service::domain::data_stores::BannedTokenStore;
use auth_service_core::{domain::Token, requests::VerifyTokenEndpointRequest};

use crate::utilities::{TestApp, get_valid_token};

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
async fn verify_token_returns_422_if_banned_token() {
    let app = TestApp::new().await;
    let token = get_valid_token();
    let response = app
        .verify_token(VerifyTokenEndpointRequest {
            token: token.clone(),
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
    app.banned_user_store
        .write()
        .await
        .ban_token(token.clone())
        .await;
    let response = app.verify_token(VerifyTokenEndpointRequest { token }).await;
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
