use crate::requests::Token;

#[derive(serde::Serialize, Debug)]
pub struct LogoutEndpointRequest {
    pub jwt: Token,
}
