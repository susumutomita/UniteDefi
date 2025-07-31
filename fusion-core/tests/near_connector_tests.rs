use fusion_core::chains::near::NearConnector;
use fusion_core::htlc::SecretHash;

#[tokio::test]
async fn test_near_connector_should_have_account_id_field() {
    let account_id = "test.near".to_string();
    let connector = NearConnector::new("https://rpc.testnet.near.org")
        .with_account_id(account_id.clone());
    
    assert_eq!(connector.account_id(), &account_id);
}

#[tokio::test]
async fn test_near_connector_should_have_private_key() {
    let connector = NearConnector::new("https://rpc.testnet.near.org")
        .with_private_key("ed25519:...");
    
    assert!(connector.has_private_key());
}

#[tokio::test]
async fn test_create_htlc_should_create_escrow_on_near() {
    let connector = NearConnector::new("https://rpc.testnet.near.org")
        .with_account_id("test.near".to_string())
        .with_private_key("ed25519:...")
        .with_contract("htlc.testnet");

    let secret_hash = SecretHash::from_bytes(&[0u8; 32]);
    let amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR
    let timeout_seconds = 3600;
    let recipient = "recipient.near".to_string();

    let result = connector
        .create_htlc(amount, secret_hash, timeout_seconds, recipient)
        .await;

    assert!(result.is_ok());
    let escrow_id = result.unwrap();
    assert!(escrow_id.starts_with("fusion_"));
}

#[tokio::test]
async fn test_claim_should_claim_escrow_with_secret() {
    let connector = NearConnector::new("https://rpc.testnet.near.org")
        .with_account_id("beneficiary.near".to_string())
        .with_private_key("ed25519:...")
        .with_contract("htlc.testnet");

    let escrow_id = "fusion_0";
    let secret = [1u8; 32];

    let result = connector.claim(escrow_id, secret).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_refund_should_cancel_expired_escrow() {
    let connector = NearConnector::new("https://rpc.testnet.near.org")
        .with_account_id("resolver.near".to_string())
        .with_private_key("ed25519:...")
        .with_contract("htlc.testnet");

    let escrow_id = "fusion_0";

    let result = connector.refund(escrow_id).await;

    assert!(result.is_ok());
}