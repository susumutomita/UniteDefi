use fusion_core::chains::near::{NearConnector, NEAR_TESTNET_RPC};
use fusion_core::htlc;
use std::time::Duration;
use tokio;

#[ignore] // This requires a real NEAR testnet account
#[tokio::test]
async fn test_full_htlc_flow_on_testnet() {
    // This test requires:
    // 1. A deployed HTLC contract on testnet
    // 2. Two testnet accounts with private keys
    // 3. Some NEAR tokens for gas
    
    let resolver_key = std::env::var("NEAR_RESOLVER_KEY").expect("NEAR_RESOLVER_KEY not set");
    let beneficiary_key = std::env::var("NEAR_BENEFICIARY_KEY").expect("NEAR_BENEFICIARY_KEY not set");
    let contract_id = std::env::var("NEAR_HTLC_CONTRACT").expect("NEAR_HTLC_CONTRACT not set");

    // Create connectors for resolver and beneficiary
    let resolver = NearConnector::new(NEAR_TESTNET_RPC)
        .with_account_id("resolver.testnet".to_string())
        .with_private_key(&resolver_key)
        .with_contract(&contract_id);

    let beneficiary = NearConnector::new(NEAR_TESTNET_RPC)
        .with_account_id("beneficiary.testnet".to_string())
        .with_private_key(&beneficiary_key)
        .with_contract(&contract_id);

    // Generate a secret and its hash
    let secret = [42u8; 32];
    let secret_hash = htlc::hash_secret(&secret);

    // 1. Create HTLC
    let amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR
    let timeout_seconds = 3600; // 1 hour
    let recipient = "beneficiary.testnet".to_string();

    let escrow_id = resolver
        .create_htlc(amount, secret_hash, timeout_seconds, recipient)
        .await
        .expect("Failed to create HTLC");

    println!("Created HTLC with ID: {}", escrow_id);

    // 2. Verify the escrow was created
    let escrow = resolver
        .get_escrow(&escrow_id)
        .await
        .expect("Failed to get escrow");

    assert_eq!(escrow.state, "Active");
    assert_eq!(escrow.amount, amount.to_string());
    assert_eq!(escrow.beneficiary, "beneficiary.testnet");

    // 3. Claim with the secret
    let claim_result = beneficiary
        .claim(&escrow_id, secret)
        .await
        .expect("Failed to claim");

    assert_eq!(claim_result, "success");

    // 4. Verify the escrow was claimed
    tokio::time::sleep(Duration::from_secs(2)).await; // Wait for transaction finality

    let escrow_after_claim = resolver
        .get_escrow(&escrow_id)
        .await
        .expect("Failed to get escrow after claim");

    assert_eq!(escrow_after_claim.state, "Claimed");
}

#[ignore] // This requires a real NEAR testnet account
#[tokio::test]
async fn test_htlc_refund_flow() {
    let resolver_key = std::env::var("NEAR_RESOLVER_KEY").expect("NEAR_RESOLVER_KEY not set");
    let contract_id = std::env::var("NEAR_HTLC_CONTRACT").expect("NEAR_HTLC_CONTRACT not set");

    let resolver = NearConnector::new(NEAR_TESTNET_RPC)
        .with_account_id("resolver.testnet".to_string())
        .with_private_key(&resolver_key)
        .with_contract(&contract_id);

    // Create an HTLC with very short timeout
    let secret_hash = htlc::hash_secret(&[0u8; 32]);
    let amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR
    let timeout_seconds = 10; // Very short timeout for testing

    let escrow_id = resolver
        .create_htlc(amount, secret_hash, timeout_seconds, "beneficiary.testnet".to_string())
        .await
        .expect("Failed to create HTLC");

    // Wait for timeout
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Refund after timeout
    let refund_result = resolver
        .refund(&escrow_id)
        .await
        .expect("Failed to refund");

    assert_eq!(refund_result, "success");

    // Verify the escrow was refunded
    let escrow_after_refund = resolver
        .get_escrow(&escrow_id)
        .await
        .expect("Failed to get escrow after refund");

    assert_eq!(escrow_after_refund.state, "Cancelled");
}