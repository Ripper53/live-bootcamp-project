use auth_service::{services::user_store::HashMapUserStore, Application};

#[tokio::main]
async fn main() {
    let app = Application::build(HashMapUserStore::default(), "0.0.0.0:3000")
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
