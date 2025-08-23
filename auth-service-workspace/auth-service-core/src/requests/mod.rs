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

#[derive(serde::Serialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct Email(String);
impl Email {
    pub fn try_new(email: String) -> Result<Self, InvalidEmailError> {
        if EmailAddress::is_valid(&email) {
            Ok(Email(email))
        } else {
            Err(InvalidEmailError(email))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("{0} is not a valid email")]
pub struct InvalidEmailError(String);
impl<'de> serde::Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmailVisitor;
        impl<'de> serde::de::Visitor<'de> for EmailVisitor {
            type Value = Email;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 100")
            }

            fn visit_string<E>(self, sus_email: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match Email::try_new(sus_email) {
                    Ok(email) => Ok(email),
                    Err(e) => Err(E::custom(e)),
                }
            }
        }
        let sus_email = String::deserialize(deserializer)?;
        serde::de::Visitor::visit_string(EmailVisitor, sus_email)
    }
}
impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((a, b)) = self.0.split_once('@')
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
