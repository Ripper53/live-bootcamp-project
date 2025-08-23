use crate::requests::{Password, UncheckedEmail, ValidEmail};

#[derive(serde::Serialize, Debug)]
pub struct LoginEndpointRequest {
    email: UncheckedEmail,
    password: Password,
}

impl LoginEndpointRequest {
    pub fn new(email: ValidEmail, password: Password) -> Self {
        LoginEndpointRequest {
            email: email.into(),
            password,
        }
    }
}
