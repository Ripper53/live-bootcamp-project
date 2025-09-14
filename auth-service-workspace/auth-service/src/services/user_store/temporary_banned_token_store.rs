use std::collections::HashSet;

use auth_service_core::domain::Token;

use crate::domain::data_stores::BannedTokenStore;

#[derive(Default)]
pub struct TemporaryBannedTokenStore {
    banned_tokens: HashSet<Token>,
}

impl BannedTokenStore for TemporaryBannedTokenStore {
    async fn ban_token(&mut self, token: Token) {
        let _ = self.banned_tokens.insert(token);
    }
    async fn is_banned_token(&self, token: &Token) -> bool {
        self.banned_tokens.contains(token)
    }
}
