use email_address::EmailAddress;

use crate::requests::RequestValidation;

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
impl AsRef<str> for UncheckedEmail {
    fn as_ref(&self) -> &str {
        &self.0
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
impl AsRef<str> for ValidEmail {
    fn as_ref(&self) -> &str {
        &self.0
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
