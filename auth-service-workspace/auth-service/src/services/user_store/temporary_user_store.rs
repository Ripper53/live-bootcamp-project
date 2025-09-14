use std::collections::HashMap;

use auth_service_core::domain::{ValidEmail, ValidPassword};

use crate::domain::{
    data_stores::{UserStore, UserStoreAddUserError},
    user::User,
};

#[derive(Default)]
pub struct TemporaryUserStore {
    users: HashMap<ValidEmail, User>,
}

impl UserStore for TemporaryUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreAddUserError> {
        if self.users.contains_key(user.email()) {
            Err(UserStoreAddUserError::UserEmailAlreadyInUse(user))
        } else {
            self.users.insert(user.email().clone(), user);
            Ok(())
        }
    }
    async fn get_user(&self, email: &ValidEmail, password: &ValidPassword) -> Option<&User> {
        if let Some(user) = self.users.get(email) {
            if *password == *user.password() {
                Some(user)
            } else {
                None
            }
        } else {
            None
        }
    }
}
