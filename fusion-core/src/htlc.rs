use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime};
use subtle::ConstantTimeEq;
use thiserror::Error;

/// 32バイトのシークレット型
pub type Secret = [u8; 32];

/// 32バイトのシークレットハッシュ型
pub type SecretHash = [u8; 32];

/// 32バイトのランダムなシークレットを生成する
pub fn generate_secret() -> Secret {
    let mut rng = rand::thread_rng();
    let mut secret = [0u8; 32];
    rng.fill(&mut secret[..]);
    secret
}

/// シークレットのSHA256ハッシュを計算する
pub fn hash_secret(secret: &Secret) -> SecretHash {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.finalize().into()
}

/// HTLCのエラー型
#[derive(Error, Debug)]
pub enum HtlcError {
    #[error("Invalid secret provided")]
    InvalidSecret,
    #[error("HTLC has not timed out yet")]
    NotTimedOut,
    #[error("HTLC is not in pending state")]
    InvalidState,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// HTLCの状態
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HtlcState {
    /// 作成されたが、まだクレームもリファンドもされていない
    Pending,
    /// 正しいシークレットでクレームされた
    Claimed,
    /// タイムアウト後にリファンドされた
    Refunded,
}

/// Hash Time Locked Contract (HTLC) の実装
#[derive(Debug)]
pub struct Htlc {
    sender: String,
    recipient: String,
    amount: u64,
    secret_hash: SecretHash,
    timeout: Duration,
    created_at: SystemTime,
    state: HtlcState,
}

impl Htlc {
    /// 新しいHTLCを作成
    pub fn new(
        sender: String,
        recipient: String,
        amount: u64,
        secret_hash: SecretHash,
        timeout: Duration,
    ) -> Result<Self, HtlcError> {
        // 入力検証
        if sender.is_empty() {
            return Err(HtlcError::InvalidInput("Sender cannot be empty".into()));
        }
        if recipient.is_empty() {
            return Err(HtlcError::InvalidInput("Recipient cannot be empty".into()));
        }
        if amount == 0 {
            return Err(HtlcError::InvalidInput("Amount must be positive".into()));
        }

        Ok(Self {
            sender,
            recipient,
            amount,
            secret_hash,
            timeout,
            created_at: SystemTime::now(),
            state: HtlcState::Pending,
        })
    }

    /// 現在の状態を取得
    pub fn state(&self) -> &HtlcState {
        &self.state
    }

    /// 送信者を取得
    pub fn sender(&self) -> &str {
        &self.sender
    }

    /// 受信者を取得
    pub fn recipient(&self) -> &str {
        &self.recipient
    }

    /// 金額を取得
    pub fn amount(&self) -> u64 {
        self.amount
    }

    /// シークレットハッシュを取得
    pub fn secret_hash(&self) -> &SecretHash {
        &self.secret_hash
    }

    /// タイムアウトしているかチェック
    pub fn is_timed_out(&self) -> bool {
        match SystemTime::now().duration_since(self.created_at) {
            Ok(elapsed) => elapsed > self.timeout,
            Err(_) => true, // 時刻が過去の場合もタイムアウトとする
        }
    }

    /// シークレットを提供してクレーム
    pub fn claim(&mut self, secret: &Secret) -> Result<(), HtlcError> {
        // 状態チェック
        if self.state != HtlcState::Pending {
            return Err(HtlcError::InvalidState);
        }

        // シークレットの検証（定数時間比較を使用）
        let provided_hash = hash_secret(secret);
        if provided_hash.ct_eq(&self.secret_hash).unwrap_u8() != 1 {
            return Err(HtlcError::InvalidSecret);
        }

        // 状態を更新
        self.state = HtlcState::Claimed;
        Ok(())
    }

    /// タイムアウト後にリファンド
    pub fn refund(&mut self) -> Result<(), HtlcError> {
        // 状態チェック
        if self.state != HtlcState::Pending {
            return Err(HtlcError::InvalidState);
        }

        // タイムアウトチェック
        if !self.is_timed_out() {
            return Err(HtlcError::NotTimedOut);
        }

        // 状態を更新
        self.state = HtlcState::Refunded;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_secret_generation() {
        let secret1 = generate_secret();
        let secret2 = generate_secret();

        // Secrets should be different
        assert_ne!(secret1, secret2);

        // Secrets should be 32 bytes
        assert_eq!(secret1.len(), 32);
        assert_eq!(secret2.len(), 32);
    }

    #[test]
    fn test_secret_hashing() {
        let secret = generate_secret();
        let hash1 = hash_secret(&secret);
        let hash2 = hash_secret(&secret);

        // Same secret should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should be 32 bytes
        assert_eq!(hash1.len(), 32);

        // Different secrets should produce different hashes
        let different_secret = generate_secret();
        let different_hash = hash_secret(&different_secret);
        assert_ne!(hash1, different_hash);
    }

    #[test]
    fn test_htlc_creation() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        let htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .unwrap();

        assert_eq!(htlc.sender(), "alice.near");
        assert_eq!(htlc.recipient(), "bob.near");
        assert_eq!(htlc.amount(), 1000000);
        assert_eq!(htlc.secret_hash(), &secret_hash);
        assert_eq!(htlc.state(), &HtlcState::Pending);
        assert!(!htlc.is_timed_out());
    }

    #[test]
    fn test_htlc_creation_validation() {
        let secret_hash = hash_secret(&generate_secret());

        // Empty sender should fail
        let result = Htlc::new(
            "".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        );
        assert!(matches!(result, Err(HtlcError::InvalidInput(_))));

        // Empty recipient should fail
        let result = Htlc::new(
            "alice.near".to_string(),
            "".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        );
        assert!(matches!(result, Err(HtlcError::InvalidInput(_))));

        // Zero amount should fail
        let result = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            0,
            secret_hash,
            Duration::from_secs(3600),
        );
        assert!(matches!(result, Err(HtlcError::InvalidInput(_))));
    }

    #[test]
    fn test_htlc_claim_with_correct_secret() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .unwrap();

        // Should be able to claim with correct secret
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
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .unwrap();

        // Should fail with wrong secret
        let result = htlc.claim(&wrong_secret);
        assert!(matches!(result, Err(HtlcError::InvalidSecret)));
        assert_eq!(htlc.state(), &HtlcState::Pending);
    }

    #[test]
    fn test_htlc_double_claim() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_secs(3600),
        )
        .unwrap();

        // First claim should succeed
        htlc.claim(&secret).unwrap();
        assert_eq!(htlc.state(), &HtlcState::Claimed);

        // Second claim should fail
        let result = htlc.claim(&secret);
        assert!(matches!(result, Err(HtlcError::InvalidState)));
    }

    #[test]
    fn test_htlc_timeout() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        let htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_millis(50), // Very short timeout
        )
        .unwrap();

        // Should not be timed out initially
        assert!(!htlc.is_timed_out());

        // Wait for timeout
        thread::sleep(Duration::from_millis(100));

        // Should now be timed out
        assert!(htlc.is_timed_out());
    }

    #[test]
    fn test_htlc_refund_after_timeout() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_millis(50),
        )
        .unwrap();

        // Should not be able to refund before timeout
        let result = htlc.refund();
        assert!(matches!(result, Err(HtlcError::NotTimedOut)));

        // Wait for timeout
        thread::sleep(Duration::from_millis(100));

        // Should now be able to refund
        let result = htlc.refund();
        assert!(result.is_ok());
        assert_eq!(htlc.state(), &HtlcState::Refunded);
    }

    #[test]
    fn test_htlc_refund_after_claim() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_millis(50),
        )
        .unwrap();

        // Claim first
        htlc.claim(&secret).unwrap();

        // Wait for timeout
        thread::sleep(Duration::from_millis(100));

        // Should not be able to refund after claim
        let result = htlc.refund();
        assert!(matches!(result, Err(HtlcError::InvalidState)));
    }

    #[test]
    fn test_htlc_claim_after_timeout() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            secret_hash,
            Duration::from_millis(50),
        )
        .unwrap();

        // Wait for timeout
        thread::sleep(Duration::from_millis(100));

        // Should still be able to claim even after timeout (this is by design)
        let result = htlc.claim(&secret);
        assert!(result.is_ok());
        assert_eq!(htlc.state(), &HtlcState::Claimed);
    }

    #[test]
    fn test_constant_time_secret_comparison() {
        let secret1 = [1u8; 32];
        let secret2 = [2u8; 32];
        let hash1 = hash_secret(&secret1);
        let _hash2 = hash_secret(&secret2);

        let mut htlc = Htlc::new(
            "alice.near".to_string(),
            "bob.near".to_string(),
            1000000,
            hash1,
            Duration::from_secs(3600),
        )
        .unwrap();

        // Wrong secret should fail
        let result = htlc.claim(&secret2);
        assert!(matches!(result, Err(HtlcError::InvalidSecret)));

        // Correct secret should succeed
        let result = htlc.claim(&secret1);
        assert!(result.is_ok());
    }
}
