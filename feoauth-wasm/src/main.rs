use dotenvy::dotenv;

use feoauth_core::server;

#[tokio::main]
async fn main() {
    dotenv().ok();
    server::run().await;
}
