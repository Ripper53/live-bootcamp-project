use std::sync::Arc;

use axum::{Router, routing::post, serve::Serve};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::domain::data_stores::{BannedTokenStore, UserStore};

pub mod domain;
pub mod routes;
pub mod services;
pub mod utilities;

pub struct Application {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    address: String,
}

impl Application {
    pub async fn build<TUserStore: UserStore, TBannedTokenStore: BannedTokenStore>(
        user_store: Arc<RwLock<TUserStore>>,
        banned_token_store: Arc<RwLock<TBannedTokenStore>>,
        address: &str,
    ) -> Result<Self, ApplicationBuildError> {
        let allowed_origins = [
            #[cfg(not(feature = "dev"))]
            "http://localhost".parse().unwrap(),
            #[cfg(feature = "dev")]
            "http://localhost".parse().unwrap(),
        ];
        let cors_layer = CorsLayer::new()
            .allow_methods([reqwest::Method::GET, reqwest::Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .route_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup::<TUserStore>))
            .route("/login", post(routes::login::<TUserStore>))
            .route("/verify-2fa", post(routes::verify_2fa))
            .with_state(user_store)
            .route(
                "/verify-token",
                post(routes::verify_token::<TBannedTokenStore>),
            )
            .route("/logout", post(routes::logout::<TBannedTokenStore>))
            .with_state(banned_token_store)
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
