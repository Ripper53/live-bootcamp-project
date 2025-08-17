use crate::requests::Token;

#[derive(serde::Serialize, Debug)]
pub struct VerifyTokenEndpointRequest {
    pub token: Token,
}
