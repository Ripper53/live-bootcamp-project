use std::sync::Arc;

use auth_service::{app_state::AppState, services::user_store::HashMapUserStore, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let app = Application::build(
        AppState::new(Arc::new(RwLock::new(Box::new(HashMapUserStore::default())))),
        "0.0.0.0:3000",
    )
    .await
    .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
