use crate::requests::{Password, UncheckedEmail, ValidEmail};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SignupEndpointRequest {
    email: UncheckedEmail,
    password: Password,
    #[serde(rename = "2fa")]
    two_factor_authentication: TwoFactorAuthentication,
}

impl SignupEndpointRequest {
    pub fn new(
        email: ValidEmail,
        password: Password,
        two_factor_authentication: TwoFactorAuthentication,
    ) -> Self {
        SignupEndpointRequest {
            email: email.into(),
            password,
            two_factor_authentication,
        }
    }
}

impl SignupEndpointRequest {
    pub fn take_all(self) -> (UncheckedEmail, Password, TwoFactorAuthentication) {
        (self.email, self.password, self.two_factor_authentication)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TwoFactorAuthentication {
    Disabled,
    Required,
}

impl serde::Serialize for TwoFactorAuthentication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TwoFactorAuthentication::Disabled => serializer.serialize_bool(false),
            TwoFactorAuthentication::Required => serializer.serialize_bool(true),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TwoFactorAuthentication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if bool::deserialize(deserializer)? {
            Ok(TwoFactorAuthentication::Required)
        } else {
            Ok(TwoFactorAuthentication::Disabled)
        }
    }
}
