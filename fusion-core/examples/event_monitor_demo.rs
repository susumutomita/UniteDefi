use ethers::providers::{Provider, Ws};
use fusion_core::chains::ethereum::event_storage::InMemoryEventStorage;
use fusion_core::chains::ethereum::events::{LimitOrderEventMonitor, MonitorConfig};
use std::sync::Arc;

const LIMIT_ORDER_PROTOCOL_ADDRESS: &str = "0x171C87724E720F2806fc29a010a62897B30fdb62";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("1inch Limit Order Protocol Event Monitor Demo");
    println!("=============================================");

    // Configure WebSocket connection
    let ws_url = std::env::var("WS_URL").unwrap_or_else(|_| {
        println!("WS_URL not set, using default");
        "wss://mainnet.infura.io/ws/v3/YOUR_INFURA_KEY".to_string()
    });

    println!("Connecting to: {}", ws_url);

    // Create provider
    let provider = Provider::<Ws>::connect(&ws_url).await?;
    let contract_address = LIMIT_ORDER_PROTOCOL_ADDRESS.parse()?;

    // Create event monitor
    let monitor = LimitOrderEventMonitor::new(Arc::new(provider), contract_address);

    // Create storage
    let storage = Arc::new(InMemoryEventStorage::new());

    // Configure monitoring
    let config = MonitorConfig::default();

    println!(
        "Starting event monitoring for contract: {}",
        contract_address
    );
    println!("Monitoring OrderFilled and OrderCancelled events...");
    println!("Press Ctrl+C to stop");

    // Start monitoring with storage and retry logic
    monitor
        .monitor_with_storage(storage.clone(), config)
        .await?;

    Ok(())
}
