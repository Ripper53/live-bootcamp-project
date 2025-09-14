use crate::domain::Token;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct VerifyTokenEndpointRequest {
    pub token: Token,
}
