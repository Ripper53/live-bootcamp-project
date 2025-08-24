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
