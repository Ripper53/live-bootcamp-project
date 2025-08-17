use crate::requests::{Email, Password};

#[derive(serde::Serialize, Debug)]
pub struct LoginEndpointRequest {
    pub email: Email,
    pub password: Password,
}
