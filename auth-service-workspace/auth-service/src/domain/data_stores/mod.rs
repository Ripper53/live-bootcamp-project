use std::future::Future;

use crate::domain::user::User;

pub trait UserStore: Send + Sync + 'static {
    fn add_user(
        &mut self,
        user: User,
    ) -> impl Future<Output = Result<(), UserStoreAddUserError>> + Send;
}

#[derive(thiserror::Error, Debug)]
pub enum UserStoreAddUserError {
    #[error("email already in use")]
    UserEmailAlreadyInUse(User),
}
