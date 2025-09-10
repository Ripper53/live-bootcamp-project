use std::sync::Arc;

use auth_service::{services::user_store::HashMapUserStore, Application};
use auth_service_core::{
    domain::{ValidEmail, ValidPassword},
    requests::{
        LoginEndpointRequest, SignupEndpointRequest, VerifyTokenEndpointRequest,
        VerifyTwoFactorAuthenticationEndpointRequest,
    },
};
use reqwest::cookie::Jar;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub cookie_jar: Arc<Jar>,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build(HashMapUserStore::default(), "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        TestApp {
            address,
            http_client: reqwest::Client::builder()
                .cookie_provider(Arc::clone(&cookie_jar))
                .build()
                .unwrap(),
            cookie_jar,
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

    pub async fn logout(&self) -> reqwest::Response {
        self.post_request("logout", ()).await
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

pub fn get_random_email() -> ValidEmail {
    ValidEmail::try_new(format!("{}@email.com", uuid::Uuid::new_v4())).unwrap()
}

pub fn get_random_password() -> ValidPassword {
    ValidPassword::try_new(uuid::Uuid::new_v4().to_string()).unwrap()
}
