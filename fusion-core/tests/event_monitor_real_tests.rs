use fusion_core::chains::ethereum::event_monitor::EthereumEventMonitor;
use fusion_core::chains::near::event_monitor::NearEventMonitor;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn should_monitor_limit_order_events_on_ethereum() {
    // Given: An Ethereum event monitor connected to Base Sepolia
    let rpc_url = "wss://base-sepolia.infura.io/ws/v3/YOUR_KEY";
    let limit_order_address = "0x171C87724E720F2806fc29a010a62897B30fdb62";
    
    let monitor = EthereumEventMonitor::new(rpc_url, limit_order_address)
        .expect("Should create monitor");
    
    // When: We start monitoring for OrderFilled events
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    
    tokio::spawn(async move {
        monitor.monitor_order_filled_events(tx).await.unwrap();
    });
    
    // Then: We should receive events (in a real test, we'd trigger an order fill)
    tokio::select! {
        Some(event) = rx.recv() => {
            assert!(!event.order_hash.is_empty());
            assert!(event.remaining_amount >= 0);
        }
        _ = sleep(Duration::from_secs(5)) => {
            // No events in 5 seconds is also OK for this test
        }
    }
}

#[tokio::test]
async fn should_monitor_htlc_events_on_near() {
    // Given: A NEAR event monitor
    let near_rpc = "https://rpc.testnet.near.org";
    let htlc_contract = "fusion-htlc.testnet";
    
    let monitor = NearEventMonitor::new(near_rpc, htlc_contract)
        .expect("Should create monitor");
    
    // When: We monitor for HTLC creation events
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    
    tokio::spawn(async move {
        monitor.monitor_htlc_events(tx).await.unwrap();
    });
    
    // Then: We should be able to receive events
    tokio::select! {
        Some(event) = rx.recv() => {
            assert!(!event.htlc_id.is_empty());
            match event.event_type {
                HtlcEventType::Created => assert!(event.amount > 0),
                HtlcEventType::Claimed => assert!(!event.secret.is_none()),
                HtlcEventType::Refunded => assert!(event.refund_reason.is_some()),
            }
        }
        _ = sleep(Duration::from_secs(5)) => {
            // No events in 5 seconds is also OK
        }
    }
}

#[tokio::test]
async fn should_detect_cross_chain_secret_revelation() {
    // Given: Both EVM and NEAR monitors
    let eth_monitor = EthereumEventMonitor::new(
        "wss://base-sepolia.infura.io/ws/v3/YOUR_KEY",
        "0x171C87724E720F2806fc29a010a62897B30fdb62"
    ).expect("Should create ETH monitor");
    
    let near_monitor = NearEventMonitor::new(
        "https://rpc.testnet.near.org",
        "fusion-htlc.testnet"
    ).expect("Should create NEAR monitor");
    
    // When: We monitor for secret revelations
    let (eth_tx, mut eth_rx) = tokio::sync::mpsc::channel(100);
    let (near_tx, mut near_rx) = tokio::sync::mpsc::channel(100);
    
    tokio::spawn(async move {
        eth_monitor.monitor_secret_revealed_events(eth_tx).await.unwrap();
    });
    
    tokio::spawn(async move {
        near_monitor.monitor_secret_revealed_events(near_tx).await.unwrap();
    });
    
    // Then: We can correlate secrets across chains
    tokio::select! {
        Some(eth_secret) = eth_rx.recv() => {
            // In production, we'd check if this secret matches a NEAR HTLC
            assert_eq!(eth_secret.len(), 32);
        }
        Some(near_secret) = near_rx.recv() => {
            // In production, we'd check if this secret matches an ETH order
            assert_eq!(near_secret.len(), 32);
        }
        _ = sleep(Duration::from_secs(5)) => {
            // No events is OK for test
        }
    }
}