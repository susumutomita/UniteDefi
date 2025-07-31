# 1inch Limit Order Protocol Event Monitoring

## Overview

This module provides event monitoring functionality for the 1inch Limit Order Protocol, tracking `OrderFilled` and `OrderCancelled` events on-chain.

## Features

- WebSocket-based real-time event monitoring
- Automatic reconnection with configurable retry logic
- Pluggable storage interface
- Support for both OrderFilled and OrderCancelled events
- In-memory storage implementation for testing

## Usage

### Basic Example

```rust
use ethers::providers::{Provider, Ws};
use fusion_core::chains::ethereum::event_storage::InMemoryEventStorage;
use fusion_core::chains::ethereum::events::{LimitOrderEventMonitor, MonitorConfig};
use std::sync::Arc;

// Connect to Ethereum node
let provider = Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/YOUR_KEY").await?;
let contract_address = "0x171C87724E720F2806fc29a010a62897B30fdb62".parse()?;

// Create monitor
let monitor = LimitOrderEventMonitor::new(Arc::new(provider), contract_address);

// Create storage
let storage = Arc::new(InMemoryEventStorage::new());

// Start monitoring
let config = MonitorConfig::default();
monitor.monitor_with_storage(storage, config).await?;
```

### Event Structure

#### OrderFilled Event
```rust
pub struct OrderFilledEvent {
    pub order_hash: H256,
    pub remaining_amount: U256,
}
```

#### OrderCancelled Event
```rust
pub struct OrderCancelledEvent {
    pub order_hash: H256,
}
```

## Configuration

```rust
pub struct MonitorConfig {
    pub retry_delay: Duration,  // Default: 5 seconds
    pub max_retries: u32,       // Default: 3
}
```

## Storage Interface

Implement the `EventStorage` trait to create custom storage backends:

```rust
#[async_trait]
pub trait EventStorage: Send + Sync {
    async fn save_event(&self, event: StoredEvent) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_events_by_order_hash(&self, order_hash: &str) -> Result<Vec<StoredEvent>, Box<dyn std::error::Error>>;
    async fn get_all_events(&self) -> Result<Vec<StoredEvent>, Box<dyn std::error::Error>>;
}
```

## Running the Demo

```bash
# Set your Infura/Alchemy WebSocket URL
export WS_URL="wss://mainnet.infura.io/ws/v3/YOUR_INFURA_KEY"

# Run the demo
cargo run --example event_monitor_demo
```

## Testing

```bash
# Run tests
cargo test event_monitoring_tests

# Run with verbose output
cargo test event_monitoring_tests -- --nocapture
```

## Future Improvements

- [ ] Add PostgreSQL storage implementation
- [ ] Implement block reorganization handling
- [ ] Add metrics and monitoring
- [ ] Support for historical event queries
- [ ] Add event filtering by maker/taker address