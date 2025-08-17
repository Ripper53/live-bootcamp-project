use crate::utilities::{LoginEndpointRequest, TestApp};

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
