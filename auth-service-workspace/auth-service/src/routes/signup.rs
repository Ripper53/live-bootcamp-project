use auth_service_core::{
    requests::{RequestValidation, SignupEndpointRequest},
    responses::signup::SignupEndpointResponse,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state::AppState,
    domain::{
        data_stores::{UserStore, UserStoreAddUserError},
        user::User,
    },
};

//#[axum::debug_handler]
pub async fn signup<S: AppState>(
    State(mut state): State<S>,
    Json(request): Json<SignupEndpointRequest>,
) -> impl IntoResponse {
    let (email, password, two_factor_authentication) = request.take_all();
    let email = match email.validate() {
        Ok(email) => email,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SignupEndpointResponse {
                    message: e.to_string(),
                }),
            );
        }
    };
    let password = match password.validate() {
        Ok(password) => password,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SignupEndpointResponse {
                    message: e.to_string(),
                }),
            );
        }
    };
    let user = User::new(email, password, two_factor_authentication);
    let mut user_store = state.user_store_mut().await;
    match user_store.add_user(user).await {
        Ok(()) => (
            StatusCode::CREATED,
            Json(SignupEndpointResponse {
                message: "user created successfully".into(),
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
