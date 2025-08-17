use auth_service_core::requests::SignupEndpointRequest;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn signup(Json(request): Json<SignupEndpointRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
