use crate::requests::{UncheckedEmail, ValidEmail};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTwoFactorAuthenticationEndpointRequest {
    email: UncheckedEmail,
    login_attempt_id: LoginAttemptID,
    #[serde(rename = "2FACode")]
    two_factor_authentication_code: TwoFactorAuthenticationCode,
}
impl VerifyTwoFactorAuthenticationEndpointRequest {
    pub fn new(
        email: ValidEmail,
        login_attempt_id: LoginAttemptID,
        two_factor_authentication_code: TwoFactorAuthenticationCode,
    ) -> Self {
        VerifyTwoFactorAuthenticationEndpointRequest {
            email: email.into(),
            login_attempt_id,
            two_factor_authentication_code,
        }
    }
}

#[derive(serde::Serialize)]
pub struct LoginAttemptID(String);
impl LoginAttemptID {
    pub fn new(login_attempt_id: String) -> Self {
        LoginAttemptID(login_attempt_id)
    }
}
impl std::fmt::Debug for LoginAttemptID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LoginAttemptID(\"****\")")
    }
}

#[derive(serde::Serialize)]
pub struct TwoFactorAuthenticationCode(String);
impl TwoFactorAuthenticationCode {
    pub fn new(two_factor_authentication_code: String) -> Self {
        TwoFactorAuthenticationCode(two_factor_authentication_code)
    }
}
impl std::fmt::Debug for TwoFactorAuthenticationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TwoFactorAuthenticationCode(\"****\")")
    }
}
