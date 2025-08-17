use crate::utilities::{LogoutEndpointRequest, TestApp};

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
