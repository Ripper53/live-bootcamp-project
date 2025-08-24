use crate::domain::user::User;

pub trait UserStore: Send + Sync {
    #[allow(async_fn_in_trait)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreAddUserError>;
}

#[derive(thiserror::Error, Debug)]
pub enum UserStoreAddUserError {
    #[error("email already in use")]
    UserEmailAlreadyInUse(User),
}
