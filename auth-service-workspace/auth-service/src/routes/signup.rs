use std::error::Error;

use auth_service_core::{
    requests::{InvalidEmailError, SignupEndpointRequest},
    responses::signup::SignupEndpointResponse,
};
use axum::{
    extract::{
        rejection::{JsonDataError, JsonRejection},
        FromRequest, MatchedPath, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};

use crate::{app_state::AppState, domain::user::User, services::user_store::UserStoreAddUserError};

pub async fn signup(
    State(state): State<AppState>,
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
    let user = User::new(email, password, two_factor_authentication);
    let mut user_store = state.user_store.write().await;
    match user_store.add_user(user) {
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

pub struct SignupJsonExtractor(SignupEndpointRequest);

#[axum::async_trait]
impl<S: Send + Sync> FromRequest<S> for SignupJsonExtractor {
    type Rejection = Response;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = axum::body::Bytes::from_request(req, state).await.unwrap();

        match serde_json::from_slice::<SignupEndpointRequest>(&bytes) {
            Ok(v) => Ok(SignupJsonExtractor(v)),
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()).into_response()),
        }
        /*let value = match serde_path_to_error::deserialize(deserializer) {
            Ok(value) => value,
            Err(err) => {
                let rejection = match err.inner().classify() {
                    serde_json::error::Category::Data => JsonDataError::from_err(err).into(),
                    serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                        JsonSyntaxError::from_err(err).into()
                    }
                    serde_json::error::Category::Io => {
                        if cfg!(debug_assertions) {
                            // we don't use `serde_json::from_reader` and instead always buffer
                            // bodies first, so we shouldn't encounter any IO errors
                            unreachable!()
                        } else {
                            JsonSyntaxError::from_err(err).into()
                        }
                    }
                };
                return Err(rejection);
            }
        };

        Ok(Json(value))*/
        /*match Json::<SignupEndpointRequest>::from_request(req, state).await {
            Ok(Json(value)) => Ok(SignupJsonExtractor(value)),
            Err(e) => match e {
                JsonRejection::JsonDataError(e) => {
                    let message = e.source().unwrap().to_string();
                    if message.contains("not a valid email") {
                        Err((
                            StatusCode::BAD_REQUEST,
                            Json(SignupEndpointResponse {
                                message: message[7..].to_string(),
                            }),
                        )
                            .into_response())
                    } else {
                        Err(e.into_response())
                    }
                }
                e => Err(e.into_response()),
            },
        }*/
    }
}
