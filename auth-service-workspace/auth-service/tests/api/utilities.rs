use auth_service::Application;
use auth_service_core::requests::{
    Email, LoginEndpointRequest, LogoutEndpointRequest, Password, SignupEndpointRequest,
    VerifyTokenEndpointRequest, VerifyTwoFactorAuthenticationEndpointRequest,
};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        TestApp {
            address,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn signup(&self, request_data: SignupEndpointRequest) -> reqwest::Response {
        self.post_request("signup", request_data).await
    }

    pub async fn login(&self, request_data: LoginEndpointRequest) -> reqwest::Response {
        self.post_request("login", request_data).await
    }

    pub async fn logout(&self, request_data: LogoutEndpointRequest) -> reqwest::Response {
        self.post_request("logout", request_data).await
    }

    pub async fn verify_2fa(
        &self,
        request_data: VerifyTwoFactorAuthenticationEndpointRequest,
    ) -> reqwest::Response {
        self.post_request("verify-2fa", request_data).await
    }

    pub async fn verify_token(
        &self,
        request_data: VerifyTokenEndpointRequest,
    ) -> reqwest::Response {
        self.post_request("verify-token", request_data).await
    }

    pub async fn post_request<T: serde::Serialize>(
        &self,
        path: &str,
        request_data: T,
    ) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/{path}", self.address))
            .json(&request_data)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub fn get_random_email() -> Email {
    Email::new(format!("{}@email.com", uuid::Uuid::new_v4()))
}

pub fn get_random_password() -> Password {
    Password::new(uuid::Uuid::new_v4().to_string())
}
