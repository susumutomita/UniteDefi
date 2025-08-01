use fusion_core::chains::near::NearHtlcConnector;
use fusion_core::htlc::SecretHash;
use near_sdk::json_types::U128;

#[tokio::test]
async fn should_create_htlc_on_near_testnet() {
    // Given: A NEAR connection and HTLC parameters
    let near_rpc = "https://rpc.testnet.near.org";
    let htlc_contract = "fusion-htlc.testnet";
    let account_id = "test-account.testnet";
    let private_key = "ed25519:test_key"; // Would come from env in real usage
    
    let connector = NearHtlcConnector::new(near_rpc)
        .with_contract(htlc_contract)
        .with_account(account_id, private_key)
        .expect("Should create connector");
    
    let secret_hash: SecretHash = [1u8; 32];
    let amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR
    let timeout_seconds = 3600;
    let recipient = "recipient.testnet";
    
    // When: We create an HTLC
    let result = connector.create_htlc(
        amount,
        secret_hash,
        timeout_seconds,
        recipient
    ).await;
    
    // Then: We should get a valid HTLC ID
    assert!(result.is_ok());
    let htlc_id = result.unwrap();
    assert!(!htlc_id.is_empty());
}

#[tokio::test]
async fn should_claim_htlc_with_valid_secret() {
    // Given: An existing HTLC and the correct secret
    let near_rpc = "https://rpc.testnet.near.org";
    let htlc_contract = "fusion-htlc.testnet";
    let account_id = "claimer.testnet";
    let private_key = "ed25519:test_key";
    
    let connector = NearHtlcConnector::new(near_rpc)
        .with_contract(htlc_contract)
        .with_account(account_id, private_key)
        .expect("Should create connector");
    
    let htlc_id = "htlc_12345678";
    let secret = [2u8; 32];
    
    // When: We claim the HTLC
    let result = connector.claim_htlc(htlc_id, secret).await;
    
    // Then: The claim should succeed
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert!(!tx_hash.is_empty());
}

#[tokio::test]
async fn should_refund_expired_htlc() {
    // Given: An expired HTLC
    let near_rpc = "https://rpc.testnet.near.org";
    let htlc_contract = "fusion-htlc.testnet";
    let account_id = "creator.testnet";
    let private_key = "ed25519:test_key";
    
    let connector = NearHtlcConnector::new(near_rpc)
        .with_contract(htlc_contract)
        .with_account(account_id, private_key)
        .expect("Should create connector");
    
    let htlc_id = "htlc_expired_123";
    
    // When: We try to refund
    let result = connector.refund_htlc(htlc_id).await;
    
    // Then: The refund should succeed
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert!(!tx_hash.is_empty());
}

#[tokio::test]
async fn should_get_htlc_status() {
    // Given: An HTLC ID
    let near_rpc = "https://rpc.testnet.near.org";
    let htlc_contract = "fusion-htlc.testnet";
    
    let connector = NearHtlcConnector::new(near_rpc)
        .with_contract(htlc_contract);
    
    let htlc_id = "htlc_12345678";
    
    // When: We query the status
    let result = connector.get_htlc_status(htlc_id).await;
    
    // Then: We should get status information
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(["active", "claimed", "refunded"].contains(&status.as_str()));
}