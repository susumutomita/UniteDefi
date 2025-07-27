use rand::Rng;
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
#[derive(Debug, Clone, PartialEq)]
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
