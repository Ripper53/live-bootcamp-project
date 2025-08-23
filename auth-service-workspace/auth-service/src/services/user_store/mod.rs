use crate::domain::user::User;

mod hash_map_user_store;
pub use hash_map_user_store::*;

pub trait UserStore {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreAddUserError>;
}

#[derive(thiserror::Error, Debug)]
pub enum UserStoreAddUserError {
    #[error("user {0} already exists")]
    UserAlreadyExists(User),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_user() {}
}
