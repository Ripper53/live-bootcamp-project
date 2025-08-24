use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::domain::data_stores::UserStore;

pub type UserStoreType<Store: UserStore> = Arc<RwLock<Store>>;

pub struct ConcreteAppState<Store: UserStore> {
    pub user_store: UserStoreType<Store>,
}

impl<Store: UserStore> Clone for ConcreteAppState<Store> {
    fn clone(&self) -> Self {
        ConcreteAppState {
            user_store: Arc::clone(&self.user_store),
        }
    }
}

impl<Store: UserStore + 'static> AppState for ConcreteAppState<Store> {
    type UserStore = Store;
    async fn user_store(&self) -> impl Deref<Target = Self::UserStore> {
        UserStoreRef {
            user_store: self.user_store.read().await,
        }
    }
    async fn user_store_mut(&mut self) -> impl DerefMut<Target = Self::UserStore> {
        UserStoreMut {
            user_store: self.user_store.write().await,
        }
    }
}

struct UserStoreRef<'a, Store: UserStore> {
    user_store: RwLockReadGuard<'a, Store>,
}
impl<'a, Store: UserStore> Deref for UserStoreRef<'a, Store> {
    type Target = Store;
    fn deref(&self) -> &Self::Target {
        self.user_store.deref()
    }
}
struct UserStoreMut<'a, Store: UserStore> {
    user_store: RwLockWriteGuard<'a, Store>,
}
impl<'a, Store: UserStore> Deref for UserStoreMut<'a, Store> {
    type Target = Store;
    fn deref(&self) -> &Self::Target {
        self.user_store.deref()
    }
}
impl<'a, Store: UserStore> DerefMut for UserStoreMut<'a, Store> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.user_store.deref_mut()
    }
}

pub trait AppState: Clone + Send + Sync + Sized + 'static {
    type UserStore: UserStore;
    #[allow(async_fn_in_trait)]
    async fn user_store(&self) -> impl Deref<Target = Self::UserStore>;
    #[allow(async_fn_in_trait)]
    async fn user_store_mut(&mut self) -> impl DerefMut<Target = Self::UserStore>;
}

impl<Store: UserStore> ConcreteAppState<Store> {
    pub fn new(user_store: UserStoreType<Store>) -> Self {
        ConcreteAppState { user_store }
    }
}
