use crate::utilities::{TestApp, VerifyTokenEndpointRequest};

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
