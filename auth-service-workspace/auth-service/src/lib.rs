use std::sync::Arc;

use axum::{routing::post, serve::Serve, Router};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::domain::data_stores::UserStore;

pub mod domain;
pub mod routes;
pub mod services;
pub mod utilities;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    address: String,
}

impl Application {
    pub async fn build<TUserStore: UserStore>(
        user_store: TUserStore,
        address: &str,
    ) -> Result<Self, ApplicationBuildError> {
        let ALLOWED_ORIGINS = [
            #[cfg(not(feature = "dev"))]
            "http//localhost".parse(),
            #[cfg(feature = "dev")]
            "http//localhost:8000".parse(),
        ];
        let cors_layer = CorsLayer::new()
            .allow_methods([reqwest::Method::GET, reqwest::Method::POST])
            .allow_credentials(true)
            .allow_origin(ALLOWED_ORIGINS);

        let router = Router::new()
            .route_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup::<TUserStore>))
            .route("/login", post(routes::login::<TUserStore>))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(Arc::new(RwLock::new(user_store)))
            .layer(cors_layer);

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
