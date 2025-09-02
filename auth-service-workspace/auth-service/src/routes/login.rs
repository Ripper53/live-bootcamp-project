use std::sync::Arc;

use auth_service_core::requests::{LoginEndpointRequest, RequestValidation};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tokio::sync::RwLock;

use crate::domain::data_stores::UserStore;

pub async fn login<S: UserStore>(
    State(user_store): State<Arc<RwLock<S>>>,
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
    if let Some(user) = user_store.read().await.get_user(email, password).await {
        StatusCode::OK.into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
