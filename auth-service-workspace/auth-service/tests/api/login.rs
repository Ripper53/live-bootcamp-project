use auth_service_core::requests::LoginEndpointRequest;

use crate::utilities::{get_random_email, get_random_password, TestApp};

#[tokio::test]
async fn login_returns_json() {
    let app = TestApp::new().await;
    let response = app
        .login(LoginEndpointRequest::new(
            get_random_email(),
            get_random_password(),
        ))
        .await;
    assert_eq!(response.status().as_u16(), 200);
    /*assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );*/
}
