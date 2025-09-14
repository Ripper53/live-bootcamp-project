use std::future::Future;

use auth_service_core::domain::Token;

pub trait BannedTokenStore: Send + Sync + 'static {
    fn ban_token(&mut self, token: Token) -> impl Future<Output = ()> + Send;
    fn is_banned_token(&self, token: &Token) -> impl Future<Output = bool> + Send;
}
