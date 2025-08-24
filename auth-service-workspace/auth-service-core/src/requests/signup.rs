use crate::domain::{
    TwoFactorAuthentication, UncheckedEmail, UncheckedPassword, ValidEmail, ValidPassword,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SignupEndpointRequest {
    email: UncheckedEmail,
    password: UncheckedPassword,
    #[serde(rename = "2fa")]
    two_factor_authentication: TwoFactorAuthentication,
}

impl SignupEndpointRequest {
    pub fn new(
        email: ValidEmail,
        password: ValidPassword,
        two_factor_authentication: TwoFactorAuthentication,
    ) -> Self {
        SignupEndpointRequest {
            email: email.into(),
            password: password.into(),
            two_factor_authentication,
        }
    }
}

impl SignupEndpointRequest {
    pub fn take_all(self) -> (UncheckedEmail, UncheckedPassword, TwoFactorAuthentication) {
        (self.email, self.password, self.two_factor_authentication)
    }
}
