use std::sync::Arc;

use auth_service_core::{
    requests::{RequestValidation, SignupEndpointRequest},
    responses::signup::SignupEndpointResponse,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{UserStore, UserStoreAddUserError},
    user::User,
};

pub async fn signup<S: UserStore>(
    State(user_store): State<Arc<RwLock<S>>>,
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
    match user_store.write().await.add_user(user).await {
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
