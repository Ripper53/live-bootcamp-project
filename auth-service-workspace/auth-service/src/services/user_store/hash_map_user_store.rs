use std::collections::HashMap;

use auth_service_core::requests::ValidEmail;

use crate::domain::{
    data_stores::{UserStore, UserStoreAddUserError},
    user::User,
};

#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<ValidEmail, User>,
}

impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreAddUserError> {
        if self.users.contains_key(user.email()) {
            Err(UserStoreAddUserError::UserEmailAlreadyInUse(user))
        } else {
            self.users.insert(user.email().clone(), user);
            Ok(())
        }
    }
}
