use auth_service_core::requests::{TwoFactorAuthentication, ValidEmail, ValidPassword};

#[derive(Debug)]
pub struct User {
    email: ValidEmail,
    password: ValidPassword,
    two_factor_authentication: TwoFactorAuthentication,
}

impl User {
    pub fn new(
        email: ValidEmail,
        password: ValidPassword,
        two_factor_authentication: TwoFactorAuthentication,
    ) -> Self {
        User {
            email,
            password,
            two_factor_authentication,
        }
    }
    pub fn email(&self) -> &ValidEmail {
        &self.email
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.email.fmt(f)
    }
}
