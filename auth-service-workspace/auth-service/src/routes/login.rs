use std::sync::Arc;

use auth_service_core::{
    auth::{generate_jwt_token, GenerateTokenError},
    requests::{LoginEndpointRequest, RequestValidation},
};
use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use tokio::sync::RwLock;

use crate::{domain::data_stores::UserStore, utilities::constants::JWT_COOKIE_NAME};

pub async fn login<S: UserStore>(
    State(user_store): State<Arc<RwLock<S>>>,
    cookie_jar: CookieJar,
    Json(request): Json<LoginEndpointRequest>,
) -> impl IntoResponse {
    let (unchecked_email, unchecked_password) = request.take_content();
    let email = match unchecked_email.validate() {
        Ok(email) => email,
        Err(_) => {
            return StatusCode::BAD_REQUEST.into_response();
        }
    };
    let password = match unchecked_password.validate() {
        Ok(password) => password,
        Err(_) => {
            return StatusCode::BAD_REQUEST.into_response();
        }
    };
    if let Some(user) = user_store.read().await.get_user(&email, &password).await {
        match generate_jwt_token(&email) {
            Ok(token) => {
                let cookie_jar = cookie_jar.add(
                    Cookie::build((JWT_COOKIE_NAME, token.take_string()))
                        .path("/")
                        .http_only(true)
                        .same_site(axum_extra::extract::cookie::SameSite::Lax)
                        .build(),
                );
                (StatusCode::OK, cookie_jar).into_response()
            }
            Err(e) => {
                eprintln!("generate_jwt_token error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
