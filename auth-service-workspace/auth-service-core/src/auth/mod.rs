use crate::domain::ValidEmail;

pub mod constants;

const TOKEN_TTL_SECONDS: i64 = 10;
const TOKEN_SECRET: &str = "SECRET";

pub fn generate_jwt_token(email: &ValidEmail) -> Result<String, GenerateTokenError> {
    let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS).unwrap();

    // Create JWT expiration time
    let exp = chrono::Utc::now()
        .checked_add_signed(delta)
        .ok_or(GenerateTokenError::ExpiryDateOutOfRange)?
        .timestamp();

    let sub = email.as_ref().to_owned();

    let claims = Claims { sub, exp };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(TOKEN_SECRET.as_bytes()),
    )
    .map_err(GenerateTokenError::Encode)
}
#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    exp: i64,
}

#[derive(thiserror::Error, Debug)]
pub enum GenerateTokenError {
    #[error("expiry date is out of range")]
    ExpiryDateOutOfRange,
    #[error(transparent)]
    Encode(jsonwebtoken::errors::Error),
}
