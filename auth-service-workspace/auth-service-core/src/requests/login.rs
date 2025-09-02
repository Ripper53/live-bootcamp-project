use crate::domain::{UncheckedEmail, UncheckedPassword, ValidEmail, ValidPassword};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginEndpointRequest {
    email: UncheckedEmail,
    password: UncheckedPassword,
}

impl LoginEndpointRequest {
    pub fn new(email: ValidEmail, password: ValidPassword) -> Self {
        LoginEndpointRequest {
            email: email.into(),
            password: password.into(),
        }
    }
    pub fn take_content(self) -> (UncheckedEmail, UncheckedPassword) {
        (self.email, self.password)
    }
}
