use crate::domain::Token;

#[derive(serde::Serialize, Debug)]
pub struct VerifyTokenEndpointRequest {
    pub token: Token,
}
