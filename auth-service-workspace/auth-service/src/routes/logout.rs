use auth_service_core::{auth::validate_token, requests::LogoutEndpointError};
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;

use crate::utilities::constants::JWT_COOKIE_NAME;

pub async fn logout(
    cookie_jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, LogoutEndpointError>) {
    if let Some(auth_cookie) = cookie_jar.get(JWT_COOKIE_NAME) {
        let auth_cookie = auth_cookie.clone();
        match validate_token(auth_cookie.value()) {
            Ok(claims) => (cookie_jar.remove(auth_cookie), Ok(StatusCode::OK)),
            Err(_) => (
                cookie_jar.remove(auth_cookie),
                Err(LogoutEndpointError::InvalidToken),
            ),
        }
    } else {
        (cookie_jar, Err(LogoutEndpointError::MissingToken))
    }
}
