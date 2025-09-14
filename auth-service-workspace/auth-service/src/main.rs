use std::sync::Arc;

use auth_service::{
    services::user_store::{TemporaryBannedTokenStore, TemporaryUserStore},
    Application,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let app = Application::build(
        Arc::new(RwLock::new(TemporaryUserStore::default())),
        Arc::new(RwLock::new(TemporaryBannedTokenStore::default())),
        "0.0.0.0:3000",
    )
    .await
    .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
