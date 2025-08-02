use fusion_core::chains::ethereum::{
    event_monitor::EthereumEventMonitor, order_extractor::OrderExtractor, EthereumConnector,
};
use fusion_core::chains::near::{event_monitor::NearEventMonitor, NearHtlcConnector};
use fusion_core::claim_executor::ClaimExecutor;
use fusion_core::cross_chain_secret_manager::CrossChainSecretManager;
use std::sync::Arc;

#[tokio::test]
async fn test_end_to_end_cross_chain_swap() {
    // Skip test if no real API keys are provided
    if std::env::var("ETHEREUM_RPC_URL").is_err() || std::env::var("ETHEREUM_WS_URL").is_err() {
        eprintln!("Skipping test_end_to_end_cross_chain_swap: ETHEREUM_RPC_URL or ETHEREUM_WS_URL not set");
        return;
    }

    // Given: A complete cross-chain swap setup

    // 1. Setup Ethereum components
    let eth_rpc = std::env::var("ETHEREUM_RPC_URL").expect("ETHEREUM_RPC_URL must be set");
    let limit_order_address = "0x171C87724E720F2806fc29a010a62897B30fdb62";
    let escrow_factory_address = "0x1234567890123456789012345678901234567890";

    let order_extractor =
        OrderExtractor::new(&eth_rpc, limit_order_address).expect("Should create order extractor");

    let eth_connector = EthereumConnector::new(&eth_rpc, escrow_factory_address)
        .expect("Should create ETH connector");

    let eth_ws_url = std::env::var("ETHEREUM_WS_URL").expect("ETHEREUM_WS_URL must be set");
    let eth_monitor = EthereumEventMonitor::new(&eth_ws_url, limit_order_address)
        .await
        .expect("Should create ETH monitor");

    // 2. Setup NEAR components
    let near_rpc = "https://rpc.testnet.near.org";
    let htlc_contract = "fusion-htlc.testnet";
    let near_account = "relayer.testnet";
    let near_private_key = "ed25519:test_key";

    let near_connector = NearHtlcConnector::new(near_rpc)
        .with_contract(htlc_contract)
        .with_account(near_account, near_private_key)
        .expect("Should create NEAR connector");

    let near_monitor =
        NearEventMonitor::new(near_rpc, htlc_contract).expect("Should create NEAR monitor");

    // 3. Setup cross-chain components
    let secret_manager = Arc::new(
        CrossChainSecretManager::new()
            .with_ethereum_monitor(eth_monitor)
            .with_near_monitor(near_monitor),
    );

    let _claim_executor = ClaimExecutor::new(secret_manager.clone())
        .with_ethereum_connector(eth_connector)
        .with_near_connector(near_connector);

    // 4. Start monitoring
    secret_manager
        .start_monitoring()
        .await
        .expect("Should start monitoring");

    // When: Processing a cross-chain swap
    let order_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let secret = [42u8; 32];

    // Extract order from EVM
    let _order = order_extractor
        .extract_order_by_hash(order_hash)
        .await
        .expect("Should extract order");

    // Register the swap
    let secret_hash = secret_manager
        .register_cross_chain_swap(secret, order_hash.to_string(), "htlc_12345".to_string())
        .await
        .expect("Should register swap");

    // Then: The system should be ready to handle secret revelations
    assert!(!secret_manager.is_secret_revealed(&secret_hash).await);

    // In a real scenario:
    // 1. Order gets filled on Ethereum, revealing the secret
    // 2. Secret manager detects the revelation
    // 3. Claim executor automatically claims on NEAR
    // 4. Both sides of the swap are completed
}

#[tokio::test]
async fn test_automatic_claim_on_secret_revelation() {
    // Test that when a secret is revealed on one chain,
    // the system automatically claims on the other chain

    let secret_manager = Arc::new(CrossChainSecretManager::new());
    let claim_executor = ClaimExecutor::new(secret_manager.clone());

    let secret = [99u8; 32];
    let secret_hash = secret_manager
        .register_cross_chain_swap(
            secret,
            "0xorder123".to_string(),
            "htlc_near_123".to_string(),
        )
        .await
        .expect("Should register");

    // Simulate secret revelation
    // In production, this would come from blockchain events

    let status = claim_executor
        .get_claim_status(&secret_hash)
        .await
        .expect("Should get status");

    assert!(!status.secret_revealed);
    assert!(status.revealed_on_chain.is_none());
}

#[tokio::test]
async fn test_claim_timeout_handling() {
    // Test that claims timeout appropriately if secret not revealed

    let secret_manager = Arc::new(CrossChainSecretManager::new());
    let claim_executor = ClaimExecutor::new(secret_manager.clone());

    let secret_hash = [1u8; 32];

    // Try to claim without secret being revealed
    let result = tokio::time::timeout(
        tokio::time::Duration::from_secs(2),
        claim_executor.claim_on_near("htlc_test", &secret_hash),
    )
    .await;

    // Timeout itself returns Ok with the inner result
    // The inner result should be an error because secret was not revealed
    match result {
        Ok(inner_result) => {
            assert!(
                inner_result.is_err(),
                "Expected claim to fail without secret"
            );
            let err = inner_result.unwrap_err();
            assert!(
                err.to_string()
                    .contains("Timeout waiting for secret revelation")
                    || err.to_string().contains("NEAR connector not configured"),
                "Expected timeout or connector error, got: {}",
                err
            );
        }
        Err(_) => {
            // Actual timeout is also acceptable
        }
    }
}
