use email_address::EmailAddress;

mod login;
mod logout;
mod signup;
mod verify_2fa;
mod verify_token;

pub use login::*;
pub use logout::*;
pub use signup::*;
pub use verify_2fa::*;
pub use verify_token::*;

pub trait RequestValidation {
    type Valid;
    type Error: std::error::Error;
    fn validate(self) -> Result<Self::Valid, Self::Error>;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UncheckedEmail(String);
impl RequestValidation for UncheckedEmail {
    type Valid = ValidEmail;
    type Error = InvalidEmailError;
    fn validate(self) -> Result<Self::Valid, Self::Error> {
        ValidEmail::try_new(self.0)
    }
}
impl From<ValidEmail> for UncheckedEmail {
    fn from(email: ValidEmail) -> Self {
        UncheckedEmail(email.0)
    }
}
#[derive(serde::Serialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct ValidEmail(String);
impl ValidEmail {
    pub fn try_new(email: String) -> Result<Self, InvalidEmailError> {
        if EmailAddress::is_valid(&email) {
            Ok(ValidEmail(email))
        } else {
            Err(InvalidEmailError(email))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("{0} is not a valid email")]
pub struct InvalidEmailError(String);
impl std::fmt::Display for ValidEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_email(&self.0, f)
    }
}
impl std::fmt::Display for UncheckedEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_email(&self.0, f)
    }
}
fn fmt_email(email: &str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some((a, b)) = email.split_once('@')
        && let Some(a) = a.chars().next()
        && let Some((b, c)) = b.split_once('.')
        && let Some(b) = b.chars().next()
        && let Some(c) = c.chars().next()
    {
        write!(f, "{a}***@{b}***.{c}**")
    } else {
        write!(f, "****@****.***")
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UncheckedPassword(String);
impl UncheckedPassword {
    pub fn new(password: String) -> Self {
        UncheckedPassword(password)
    }
}
impl UncheckedPassword {
    pub fn validate(self) -> Result<ValidPassword, InvalidPasswordError> {
        ValidPassword::try_new(self.0)
    }
}
impl std::fmt::Debug for UncheckedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UncheckedPassword(\"****\")")
    }
}
#[derive(serde::Serialize)]
pub struct ValidPassword(String);
impl ValidPassword {
    pub fn try_new(password: String) -> Result<Self, InvalidPasswordError> {
        if password.len() < 8 {
            Err(InvalidPasswordError)
        } else {
            Ok(ValidPassword(password))
        }
    }
}
impl From<ValidPassword> for UncheckedPassword {
    fn from(password: ValidPassword) -> Self {
        UncheckedPassword(password.0)
    }
}
impl std::fmt::Debug for ValidPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidPassword(\"****\")")
    }
}
#[derive(thiserror::Error, Debug)]
#[error("password is less than 8 characters")]
pub struct InvalidPasswordError;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Token(String);
impl Token {
    pub fn new(token: String) -> Self {
        Token(token)
    }
}
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token(\"****\")")
    }
}
