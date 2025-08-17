use std::collections::{HashMap, HashSet};

use auth_service::Application;

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

    pub async fn signup(&self, request_data: SignupEndpointRequest<'_>) -> reqwest::Response {
        self.post_request("signup", request_data).await
    }

    pub async fn login(&self, request_data: LoginEndpointRequest<'_>) -> reqwest::Response {
        self.post_request("login", request_data).await
    }

    pub async fn logout(&self, request_data: LogoutEndpointRequest<'_>) -> reqwest::Response {
        self.post_request("logout", request_data).await
    }

    pub async fn verify_2fa(
        &self,
        request_data: VerifyTwoFactorAuthenticationEndpointRequest<'_>,
    ) -> reqwest::Response {
        self.post_request("verify-2fa", request_data).await
    }

    pub async fn verify_token(
        &self,
        request_data: VerifyTokenEndpointRequest<'_>,
    ) -> reqwest::Response {
        self.post_request("verify-token", request_data).await
    }

    async fn post_request<T: serde::Serialize>(
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

#[derive(serde::Serialize, Debug)]
pub struct SignupEndpointRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub two_factor_authentication: TwoFactorAuthentication,
}

#[derive(Clone, Copy, Debug)]
pub enum TwoFactorAuthentication {
    Disabled,
    Required,
}

impl serde::Serialize for TwoFactorAuthentication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TwoFactorAuthentication::Disabled => serializer.serialize_bool(false),
            TwoFactorAuthentication::Required => serializer.serialize_bool(true),
        }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct LoginEndpointRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(serde::Serialize, Debug)]
pub struct LogoutEndpointRequest<'a> {
    pub jwt: &'a str,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTwoFactorAuthenticationEndpointRequest<'a> {
    pub email: &'a str,
    pub login_attempt_id: &'a str,
    #[serde(rename = "2FACode")]
    pub two_factor_authentication_code: &'a str,
}

#[derive(serde::Serialize, Debug)]
pub struct VerifyTokenEndpointRequest<'a> {
    pub token: &'a str,
}
