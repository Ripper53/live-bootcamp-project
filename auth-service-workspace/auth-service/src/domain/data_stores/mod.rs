use std::future::Future;

use auth_service_core::domain::{ValidEmail, ValidPassword};

use crate::domain::user::User;

pub trait UserStore: Send + Sync + 'static {
    fn add_user(
        &mut self,
        user: User,
    ) -> impl Future<Output = Result<(), UserStoreAddUserError>> + Send;
    fn get_user(
        &self,
        email: ValidEmail,
        password: ValidPassword,
    ) -> impl Future<Output = Option<&User>> + Send;
}

#[derive(thiserror::Error, Debug)]
pub enum UserStoreAddUserError {
    #[error("email already in use")]
    UserEmailAlreadyInUse(User),
}
