use dotenv::dotenv;

mod config;
mod routes;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    server::run().await;
}
