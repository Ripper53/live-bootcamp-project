use std::sync::Arc;

use tokio::sync::RwLock;

use crate::services::user_store::UserStore;

pub type UserStoreType = Arc<RwLock<Box<dyn UserStore + Send + Sync>>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        AppState { user_store }
    }
}
