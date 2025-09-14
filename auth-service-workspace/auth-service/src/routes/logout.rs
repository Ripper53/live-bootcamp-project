use std::sync::Arc;

use auth_service_core::{auth::validate_token, domain::Token, requests::LogoutEndpointError};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use tokio::sync::RwLock;

use crate::{domain::data_stores::BannedTokenStore, utilities::constants::JWT_COOKIE_NAME};

pub async fn logout<S: BannedTokenStore>(
    State(banned_token_store): State<Arc<RwLock<S>>>,
    cookie_jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, LogoutEndpointError>) {
    if let Some(auth_cookie) = cookie_jar.get(JWT_COOKIE_NAME) {
        let auth_cookie = auth_cookie.clone();
        match validate_token(auth_cookie.value()) {
            Ok(_claims) => {
                banned_token_store
                    .write()
                    .await
                    .ban_token(Token::new(auth_cookie.value().into()))
                    .await;
                (cookie_jar.remove(auth_cookie), Ok(StatusCode::OK))
            }
            Err(_) => (
                cookie_jar.remove(auth_cookie),
                Err(LogoutEndpointError::InvalidToken),
            ),
        }
    } else {
        (cookie_jar, Err(LogoutEndpointError::MissingToken))
    }
}
