use std::sync::Arc;

use jage::Warehouse;
use parking_lot::RwLock;
use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello Jage!");
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(Targets::new().with_target("jage", Level::DEBUG))
        .init();
    let warehouse = Arc::new(RwLock::new(Warehouse::new()));
    jage::spawn_grpc_server(Arc::clone(&warehouse));
    jage::run_web_server(warehouse).await;

    Ok(())
}
