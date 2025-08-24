use crate::requests::RequestValidation;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UncheckedPassword(String);
impl UncheckedPassword {
    pub fn new(password: String) -> Self {
        UncheckedPassword(password)
    }
}
impl RequestValidation for UncheckedPassword {
    type Valid = ValidPassword;
    type Error = InvalidPasswordError;
    fn validate(self) -> Result<ValidPassword, InvalidPasswordError> {
        ValidPassword::try_new(self.0)
    }
}
impl std::fmt::Debug for UncheckedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UncheckedPassword(\"****\")")
    }
}
#[derive(serde::Serialize)]
pub struct ValidPassword(String);
impl ValidPassword {
    pub fn try_new(password: String) -> Result<Self, InvalidPasswordError> {
        if password.len() < 8 {
            Err(InvalidPasswordError)
        } else {
            Ok(ValidPassword(password))
        }
    }
}
impl From<ValidPassword> for UncheckedPassword {
    fn from(password: ValidPassword) -> Self {
        UncheckedPassword(password.0)
    }
}
impl std::fmt::Debug for ValidPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidPassword(\"****\")")
    }
}
#[derive(thiserror::Error, Debug)]
#[error("password is less than 8 characters")]
pub struct InvalidPasswordError;
