use fusion_core::htlc::{generate_secret, hash_secret, Htlc, HtlcError, HtlcState};
use std::time::Duration;

#[test]
fn test_generate_secret_creates_32_bytes() {
    let secret = generate_secret();
    assert_eq!(secret.len(), 32, "Secret should be 32 bytes");
}

#[test]
fn test_generate_secret_is_random() {
    let secret1 = generate_secret();
    let secret2 = generate_secret();
    assert_ne!(
        secret1, secret2,
        "Two generated secrets should be different"
    );
}

#[test]
fn test_hash_secret_produces_consistent_output() {
    let secret = vec![1u8; 32]; // テスト用の固定シークレット
    let hash1 = hash_secret(&secret);
    let hash2 = hash_secret(&secret);
    assert_eq!(hash1, hash2, "Same secret should produce same hash");
}

#[test]
fn test_hash_secret_produces_32_byte_output() {
    let secret = generate_secret();
    let hash = hash_secret(&secret);
    assert_eq!(hash.len(), 32, "Hash should be 32 bytes (SHA256)");
}

#[test]
fn test_different_secrets_produce_different_hashes() {
    let secret1 = vec![1u8; 32];
    let secret2 = vec![2u8; 32];
    let hash1 = hash_secret(&secret1);
    let hash2 = hash_secret(&secret2);
    assert_ne!(
        hash1, hash2,
        "Different secrets should produce different hashes"
    );
}

#[test]
fn test_htlc_creation() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);
    let amount = 1000u64;
    let timeout = Duration::from_secs(3600); // 1時間

    let htlc = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        amount,
        secret_hash.clone(),
        timeout,
    )
    .expect("Failed to create HTLC");

    assert_eq!(htlc.state(), &HtlcState::Pending);
    assert_eq!(htlc.sender(), "Alice");
    assert_eq!(htlc.recipient(), "Bob");
    assert_eq!(htlc.amount(), amount);
    assert_eq!(htlc.secret_hash(), &secret_hash);
}

#[test]
fn test_htlc_claim_with_correct_secret() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let mut htlc = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        1000,
        secret_hash,
        Duration::from_secs(3600),
    )
    .expect("Failed to create HTLC");

    // 正しいシークレットでクレーム
    let result = htlc.claim(&secret);
    assert!(result.is_ok());
    assert_eq!(htlc.state(), &HtlcState::Claimed);
}

#[test]
fn test_htlc_claim_with_wrong_secret() {
    let secret = generate_secret();
    let wrong_secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let mut htlc = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        1000,
        secret_hash,
        Duration::from_secs(3600),
    )
    .expect("Failed to create HTLC");

    // 間違ったシークレットでクレーム
    let result = htlc.claim(&wrong_secret);
    assert!(result.is_err());
    assert_eq!(htlc.state(), &HtlcState::Pending);
}

#[test]
fn test_htlc_refund_after_timeout() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let mut htlc = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        1000,
        secret_hash,
        Duration::from_secs(1), // 1秒でタイムアウト
    )
    .expect("Failed to create HTLC");

    // タイムアウト前のリファンドは失敗
    let result = htlc.refund();
    assert!(result.is_err());

    // 1秒待つ
    std::thread::sleep(Duration::from_secs(2));

    // タイムアウト後のリファンドは成功
    let result = htlc.refund();
    assert!(result.is_ok());
    assert_eq!(htlc.state(), &HtlcState::Refunded);
}

#[test]
fn test_htlc_creation_with_empty_sender() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let result = Htlc::new(
        "".to_string(), // 空の送信者
        "Bob".to_string(),
        1000,
        secret_hash,
        Duration::from_secs(3600),
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        HtlcError::InvalidInput(msg) => assert!(msg.contains("Sender cannot be empty")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_htlc_creation_with_empty_recipient() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let result = Htlc::new(
        "Alice".to_string(),
        "".to_string(), // 空の受信者
        1000,
        secret_hash,
        Duration::from_secs(3600),
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        HtlcError::InvalidInput(msg) => assert!(msg.contains("Recipient cannot be empty")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_htlc_creation_with_zero_amount() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let result = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        0, // ゼロ金額
        secret_hash,
        Duration::from_secs(3600),
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        HtlcError::InvalidInput(msg) => assert!(msg.contains("Amount must be positive")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_htlc_creation_with_invalid_hash_length() {
    let result = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        1000,
        vec![0u8; 16], // 不正なハッシュ長（16バイト）
        Duration::from_secs(3600),
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        HtlcError::InvalidInput(msg) => assert!(msg.contains("Secret hash must be 32 bytes")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn test_htlc_double_claim() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let mut htlc = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        1000,
        secret_hash,
        Duration::from_secs(3600),
    )
    .expect("Failed to create HTLC");

    // 最初のクレームは成功
    assert!(htlc.claim(&secret).is_ok());
    assert_eq!(htlc.state(), &HtlcState::Claimed);

    // 二回目のクレーム試行は失敗
    let result = htlc.claim(&secret);
    assert!(result.is_err());
    match result.unwrap_err() {
        HtlcError::InvalidState => (),
        _ => panic!("Expected InvalidState error"),
    }
}

#[test]
fn test_htlc_refund_after_claim() {
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    let mut htlc = Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        1000,
        secret_hash,
        Duration::from_secs(1), // 短いタイムアウト
    )
    .expect("Failed to create HTLC");

    // クレーム実行
    assert!(htlc.claim(&secret).is_ok());

    // タイムアウトを待つ
    std::thread::sleep(Duration::from_secs(2));

    // クレーム後のリファンドは失敗
    let result = htlc.refund();
    assert!(result.is_err());
    match result.unwrap_err() {
        HtlcError::InvalidState => (),
        _ => panic!("Expected InvalidState error"),
    }
}
