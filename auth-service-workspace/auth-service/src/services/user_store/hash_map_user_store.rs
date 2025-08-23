use std::collections::HashMap;

use auth_service_core::requests::ValidEmail;

use crate::{
    domain::user::User,
    services::user_store::{UserStore, UserStoreAddUserError},
};

#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<ValidEmail, User>,
}

impl UserStore for HashMapUserStore {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreAddUserError> {
        if self.users.contains_key(user.email()) {
            Err(UserStoreAddUserError::UserEmailAlreadyInUse(user))
        } else {
            self.users.insert(user.email().clone(), user);
            Ok(())
        }
    }
}
