use auth_service_core::{
    requests::SignupEndpointRequest, responses::signup::SignupEndpointResponse,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{app_state::AppState, domain::user::User, services::user_store::UserStoreAddUserError};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupEndpointRequest>,
) -> impl IntoResponse {
    let user = User::new(
        request.email,
        request.password,
        request.two_factor_authentication,
    );
    let mut user_store = state.user_store.write().await;
    match user_store.add_user(user) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(SignupEndpointResponse {
                message: "User created successfully!".into(),
            }),
        ),
        Err(e) => match e {
            UserStoreAddUserError::UserEmailAlreadyInUse(_) => (
                StatusCode::CONFLICT,
                Json(SignupEndpointResponse {
                    message: e.to_string(),
                }),
            ),
        },
    }
}
