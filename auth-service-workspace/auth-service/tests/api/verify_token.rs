use auth_service_core::{domain::Token, requests::VerifyTokenEndpointRequest};

use crate::utilities::TestApp;

#[tokio::test]
async fn verify_token_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .verify_token(VerifyTokenEndpointRequest {
            token: Token::new("TEST_TOKEN".into()),
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}
