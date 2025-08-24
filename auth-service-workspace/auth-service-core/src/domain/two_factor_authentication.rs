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
