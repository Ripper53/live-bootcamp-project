use axum::{routing::post, serve::Serve, Router};
use tower_http::services::ServeDir;

use crate::{app_state::ConcreteAppState, services::user_store::HashMapUserStore};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    address: String,
}

impl Application {
    pub async fn build_in_memory(
        app_state: ConcreteAppState<HashMapUserStore>,
        address: &str,
    ) -> Result<Self, ApplicationBuildError> {
        let router = Router::new()
            .route_service("/", ServeDir::new("assets"))
            .route(
                "/signup",
                post(routes::signup::<ConcreteAppState<HashMapUserStore>>),
            )
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ApplicationBuildError(#[from] std::io::Error);
