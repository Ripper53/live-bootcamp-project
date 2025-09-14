use auth_service_core::{auth::validate_token, requests::VerifyTokenEndpointRequest};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn verify_token(
    Json(verify_token): Json<VerifyTokenEndpointRequest>,
) -> impl IntoResponse {
    if let Ok(_claims) = validate_token(verify_token.token.as_str()) {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}
