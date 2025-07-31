#[cfg(test)]
mod tests {
    use ethers::{
        core::types::Address,
        providers::{Provider, Ws},
    };
    use fusion_core::chains::ethereum::events::LimitOrderEventMonitor;
    use std::sync::Arc;

    const LIMIT_ORDER_PROTOCOL_ADDRESS: &str = "0x171C87724E720F2806fc29a010a62897B30fdb62";

    #[tokio::test]
    #[ignore = "Requires valid Infura API key"]
    async fn test_order_filled_event_monitoring() {
        // Skip if no valid Infura key is provided
        let infura_key =
            std::env::var("INFURA_API_KEY").unwrap_or_else(|_| "YOUR_INFURA_KEY".to_string());
        if infura_key == "YOUR_INFURA_KEY" {
            eprintln!("Skipping test: Set INFURA_API_KEY environment variable to run this test");
            return;
        }

        // Given
        let ws_url = format!("wss://mainnet.infura.io/ws/v3/{}", infura_key);
        let provider = Provider::<Ws>::connect(ws_url).await.unwrap();
        let contract_address = LIMIT_ORDER_PROTOCOL_ADDRESS.parse::<Address>().unwrap();
        let monitor = LimitOrderEventMonitor::new(Arc::new(provider), contract_address);

        // When
        let result = monitor.monitor_order_filled_events().await;

        // Then
        assert!(
            result.is_ok(),
            "OrderFilled event monitoring should succeed"
        );
    }

    #[tokio::test]
    #[ignore = "Requires valid Infura API key"]
    async fn test_order_cancelled_event_monitoring() {
        // Skip if no valid Infura key is provided
        let infura_key =
            std::env::var("INFURA_API_KEY").unwrap_or_else(|_| "YOUR_INFURA_KEY".to_string());
        if infura_key == "YOUR_INFURA_KEY" {
            eprintln!("Skipping test: Set INFURA_API_KEY environment variable to run this test");
            return;
        }

        // Given
        let ws_url = format!("wss://mainnet.infura.io/ws/v3/{}", infura_key);
        let provider = Provider::<Ws>::connect(ws_url).await.unwrap();
        let contract_address = LIMIT_ORDER_PROTOCOL_ADDRESS.parse::<Address>().unwrap();
        let monitor = LimitOrderEventMonitor::new(Arc::new(provider), contract_address);

        // When
        let result = monitor.monitor_order_cancelled_events().await;

        // Then
        assert!(
            result.is_ok(),
            "OrderCancelled event monitoring should succeed"
        );
    }

    #[tokio::test]
    #[ignore = "Not implemented yet"]
    async fn test_event_data_storage() {
        // Test that events are properly stored in the database
        // TODO: Implement database storage test
    }

    #[tokio::test]
    #[ignore = "Not implemented yet"]
    async fn test_reconnection_on_websocket_failure() {
        // Test that the monitor can recover from websocket disconnections
        // TODO: Implement reconnection test
    }
}
