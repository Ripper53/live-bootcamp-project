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
pub struct Email(String);
impl Email {
    pub fn new(email: String) -> Self {
        Email(email)
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
