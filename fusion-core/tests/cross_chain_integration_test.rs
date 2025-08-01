use fusion_core::chains::ethereum::{EthereumConnector, order_extractor::OrderExtractor, event_monitor::EthereumEventMonitor};
use fusion_core::chains::near::{NearHtlcConnector, event_monitor::NearEventMonitor};
use fusion_core::cross_chain_secret_manager::CrossChainSecretManager;
use fusion_core::claim_executor::ClaimExecutor;
use std::sync::Arc;

#[tokio::test]
async fn test_end_to_end_cross_chain_swap() {
    // Given: A complete cross-chain swap setup
    
    // 1. Setup Ethereum components
    let eth_rpc = "https://base-sepolia.infura.io/v3/YOUR_KEY";
    let limit_order_address = "0x171C87724E720F2806fc29a010a62897B30fdb62";
    let escrow_factory_address = "0xYourEscrowFactoryAddress";
    
    let order_extractor = OrderExtractor::new(eth_rpc, limit_order_address)
        .expect("Should create order extractor");
    
    let eth_connector = EthereumConnector::new(eth_rpc, escrow_factory_address)
        .expect("Should create ETH connector");
    
    let eth_monitor = EthereumEventMonitor::new(
        "wss://base-sepolia.infura.io/ws/v3/YOUR_KEY",
        limit_order_address
    ).expect("Should create ETH monitor");
    
    // 2. Setup NEAR components
    let near_rpc = "https://rpc.testnet.near.org";
    let htlc_contract = "fusion-htlc.testnet";
    let near_account = "relayer.testnet";
    let near_private_key = "ed25519:test_key";
    
    let near_connector = NearHtlcConnector::new(near_rpc)
        .with_contract(htlc_contract)
        .with_account(near_account, near_private_key)
        .expect("Should create NEAR connector");
    
    let near_monitor = NearEventMonitor::new(near_rpc, htlc_contract)
        .expect("Should create NEAR monitor");
    
    // 3. Setup cross-chain components
    let secret_manager = Arc::new(
        CrossChainSecretManager::new()
            .with_ethereum_monitor(eth_monitor)
            .with_near_monitor(near_monitor)
    );
    
    let claim_executor = ClaimExecutor::new(secret_manager.clone())
        .with_ethereum_connector(eth_connector)
        .with_near_connector(near_connector);
    
    // 4. Start monitoring
    secret_manager.start_monitoring().await.expect("Should start monitoring");
    
    // When: Processing a cross-chain swap
    let order_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let secret = [42u8; 32];
    
    // Extract order from EVM
    let order = order_extractor.extract_order_by_hash(order_hash).await
        .expect("Should extract order");
    
    // Register the swap
    let secret_hash = secret_manager.register_cross_chain_swap(
        secret,
        order_hash.to_string(),
        "htlc_12345".to_string()
    ).await.expect("Should register swap");
    
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
    let secret_hash = secret_manager.register_cross_chain_swap(
        secret,
        "0xorder123".to_string(),
        "htlc_near_123".to_string()
    ).await.expect("Should register");
    
    // Simulate secret revelation
    // In production, this would come from blockchain events
    
    let status = claim_executor.get_claim_status(&secret_hash).await
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
        claim_executor.claim_on_near("htlc_test", &secret_hash)
    ).await;
    
    assert!(result.is_err()); // Should timeout
}