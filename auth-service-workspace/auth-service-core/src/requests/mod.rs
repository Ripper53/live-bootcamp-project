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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UncheckedEmail(String);
impl UncheckedEmail {
    pub fn validate(self) -> Result<ValidEmail, InvalidEmailError> {
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
pub struct Password(String);
impl Password {
    pub fn new(password: String) -> Self {
        Password(password)
    }
}
impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password(\"****\")")
    }
}

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
