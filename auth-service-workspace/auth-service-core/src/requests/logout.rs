use axum_core::response::IntoResponse;
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum LogoutEndpointError {
    #[error("token is missing")]
    MissingToken,
    #[error("invalid token")]
    InvalidToken,
}

impl IntoResponse for LogoutEndpointError {
    fn into_response(self) -> axum_core::response::Response {
        match self {
            LogoutEndpointError::MissingToken => StatusCode::BAD_REQUEST.into_response(),
            LogoutEndpointError::InvalidToken => StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}
