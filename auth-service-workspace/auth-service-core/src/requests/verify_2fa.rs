use crate::requests::Email;

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTwoFactorAuthenticationEndpointRequest {
    pub email: Email,
    pub login_attempt_id: LoginAttemptID,
    #[serde(rename = "2FACode")]
    pub two_factor_authentication_code: TwoFactorAuthenticationCode,
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
