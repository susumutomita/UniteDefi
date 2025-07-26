use fusion_core::htlc::{HTLCContract, HTLCError, HTLCState};
use fusion_core::crypto::{generate_hash_lock, verify_preimage};
use test_case::test_case;

#[test]
fn should_create_new_htlc_contract() {
    let sender = "sender_address".to_string();
    let receiver = "receiver_address".to_string();
    let amount = 1000u64;
    let hash_lock = generate_hash_lock(b"secret");
    let timeout = 3600u64; // 1 hour

    let htlc = HTLCContract::new(
        sender.clone(),
        receiver.clone(),
        amount,
        hash_lock.clone(),
        timeout,
    );

    assert_eq!(htlc.sender(), &sender);
    assert_eq!(htlc.receiver(), &receiver);
    assert_eq!(htlc.amount(), amount);
    assert_eq!(htlc.hash_lock(), &hash_lock);
    assert_eq!(htlc.timeout(), timeout);
    assert_eq!(htlc.state(), HTLCState::Pending);
}

#[test]
fn should_generate_valid_hash_lock() {
    let preimage = b"my_secret_preimage";
    let hash_lock = generate_hash_lock(preimage);
    
    assert_eq!(hash_lock.len(), 64); // SHA256 produces 32 bytes = 64 hex chars
    assert!(verify_preimage(preimage, &hash_lock));
}

#[test]
fn should_fail_with_invalid_preimage() {
    let preimage = b"my_secret_preimage";
    let wrong_preimage = b"wrong_preimage";
    let hash_lock = generate_hash_lock(preimage);
    
    assert!(!verify_preimage(wrong_preimage, &hash_lock));
}

#[test_case(HTLCState::Pending => true; "pending state allows claim")]
#[test_case(HTLCState::Claimed => false; "claimed state denies claim")]
#[test_case(HTLCState::Refunded => false; "refunded state denies claim")]
fn should_validate_claim_based_on_state(state: HTLCState) -> bool {
    state.can_claim()
}

#[tokio::test]
async fn should_claim_htlc_with_valid_preimage() {
    let sender = "sender_address".to_string();
    let receiver = "receiver_address".to_string();
    let amount = 1000u64;
    let preimage = b"secret";
    let hash_lock = generate_hash_lock(preimage);
    let timeout = 3600u64;

    let mut htlc = HTLCContract::new(sender, receiver, amount, hash_lock, timeout);
    
    let result = htlc.claim(preimage).await;
    assert!(result.is_ok());
    assert_eq!(htlc.state(), HTLCState::Claimed);
}

#[tokio::test]
async fn should_fail_claim_with_invalid_preimage() {
    let sender = "sender_address".to_string();
    let receiver = "receiver_address".to_string();
    let amount = 1000u64;
    let preimage = b"secret";
    let wrong_preimage = b"wrong";
    let hash_lock = generate_hash_lock(preimage);
    let timeout = 3600u64;

    let mut htlc = HTLCContract::new(sender, receiver, amount, hash_lock, timeout);
    
    let result = htlc.claim(wrong_preimage).await;
    assert!(matches!(result, Err(HTLCError::InvalidPreimage)));
    assert_eq!(htlc.state(), HTLCState::Pending);
}

#[tokio::test]
async fn should_refund_after_timeout() {
    let sender = "sender_address".to_string();
    let receiver = "receiver_address".to_string();
    let amount = 1000u64;
    let hash_lock = generate_hash_lock(b"secret");
    let timeout = 0u64; // Already expired

    let mut htlc = HTLCContract::new(sender, receiver, amount, hash_lock, timeout);
    
    let result = htlc.refund().await;
    assert!(result.is_ok());
    assert_eq!(htlc.state(), HTLCState::Refunded);
}

#[tokio::test]
async fn should_fail_refund_before_timeout() {
    let sender = "sender_address".to_string();
    let receiver = "receiver_address".to_string();
    let amount = 1000u64;
    let hash_lock = generate_hash_lock(b"secret");
    let timeout = 3600u64; // 1 hour from now

    let mut htlc = HTLCContract::new(sender, receiver, amount, hash_lock, timeout);
    
    let result = htlc.refund().await;
    assert!(matches!(result, Err(HTLCError::TimeoutNotReached)));
    assert_eq!(htlc.state(), HTLCState::Pending);
}