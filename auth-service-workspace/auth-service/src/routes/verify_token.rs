use std::sync::Arc;

use auth_service_core::{auth::validate_token, requests::VerifyTokenEndpointRequest};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tokio::sync::RwLock;

use crate::domain::data_stores::BannedTokenStore;

pub async fn verify_token<S: BannedTokenStore>(
    State(banned_token_store): State<Arc<RwLock<S>>>,
    Json(verify_token): Json<VerifyTokenEndpointRequest>,
) -> impl IntoResponse {
    if let Ok(_claims) = validate_token(verify_token.token.as_str())
        && !banned_token_store
            .read()
            .await
            .is_banned_token(&verify_token.token)
            .await
    {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}
