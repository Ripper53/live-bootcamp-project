use auth_service_core::requests::{Email, Password, TwoFactorAuthentication};

#[derive(Debug)]
pub struct User {
    email: Email,
    password: Password,
    two_factor_authentication: TwoFactorAuthentication,
}

impl User {
    pub fn new(
        email: Email,
        password: Password,
        two_factor_authentication: TwoFactorAuthentication,
    ) -> Self {
        User {
            email,
            password,
            two_factor_authentication,
        }
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.email.fmt(f)
    }
}
