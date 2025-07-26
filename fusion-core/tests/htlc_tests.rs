use fusion_core::htlc::{generate_secret, hash_secret};

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
