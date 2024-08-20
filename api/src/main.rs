//! Main component of the backend which ties everything together.
//!
//! Starts the backend server and listens for incoming requests.
use std::env;

use api::server::start_server;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();

    let ip = env::var("SERVER_IP_ADDRESS").unwrap_or_else(|_| "localhost:3000".to_string());
    start_server(&ip).await;
}
