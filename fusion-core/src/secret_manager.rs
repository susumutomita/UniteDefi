use crate::chains::near_events::NearHtlcClaimEvent;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecretError {
    #[error("Secret not found")]
    SecretNotFound,
    #[error("Invalid secret format")]
    InvalidSecretFormat,
    #[error("Secret already exists")]
    SecretAlreadyExists,
}

/// シークレット管理
#[derive(Default)]
pub struct SecretManager {
    secrets: HashMap<String, String>, // escrow_id -> secret
}

impl SecretManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Claimイベントを処理してシークレットを保存
    pub async fn process_claim_event(
        &mut self,
        event: &NearHtlcClaimEvent,
    ) -> Result<(), SecretError> {
        // シークレットのフォーマットを検証
        if event.secret.is_empty() {
            return Err(SecretError::InvalidSecretFormat);
        }

        // 既存のシークレットがある場合はエラー（重複防止）
        if self.secrets.contains_key(&event.escrow_id) {
            return Err(SecretError::SecretAlreadyExists);
        }

        self.secrets
            .insert(event.escrow_id.clone(), event.secret.clone());
        Ok(())
    }

    /// エスクローIDからシークレットを取得
    pub async fn get_secret(&self, escrow_id: &str) -> Result<String, SecretError> {
        self.secrets
            .get(escrow_id)
            .cloned()
            .ok_or(SecretError::SecretNotFound)
    }

    /// すべてのシークレットをクリア（テスト用）
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.secrets.clear();
    }
}

/// クロスチェーン実行リクエスト
#[derive(Debug, Clone)]
pub struct CrossChainClaimRequest {
    pub target_chain: String,
    pub htlc_id: String,
    pub secret: String,
    pub recipient: String,
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Chain not supported: {0}")]
    ChainNotSupported(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid parameters")]
    InvalidParameters,
}

/// チェーン接続インターフェース
pub trait ChainConnector: Send + Sync {
    fn chain_name(&self) -> &str;
    // TODO: 実際の実装では非同期メソッドを追加
}

/// クロスチェーン実行エンジン
#[derive(Default)]
pub struct CrossChainExecutor;

impl CrossChainExecutor {
    pub fn new() -> Self {
        Self
    }

    /// クロスチェーンクレームを実行
    pub async fn execute_claim(
        &self,
        request: CrossChainClaimRequest,
        _connector: &dyn ChainConnector,
    ) -> Result<String, ExecutionError> {
        // パラメータ検証
        if request.htlc_id.is_empty() || request.secret.is_empty() {
            return Err(ExecutionError::InvalidParameters);
        }

        // TODO: 実際のチェーン呼び出しを実装
        // 現在はダミーのトランザクションハッシュを返す
        Ok(format!("0x{}", "a".repeat(64)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_store_secret_on_claim_event() {
        let mut secret_manager = SecretManager::new();

        let claim_event = NearHtlcClaimEvent {
            escrow_id: "fusion_0".to_string(),
            claimer: "bob.near".to_string(),
            secret: "deadbeef1234567890abcdef".to_string(),
            timestamp: 1234567890,
        };

        secret_manager
            .process_claim_event(&claim_event)
            .await
            .unwrap();

        let stored_secret = secret_manager.get_secret("fusion_0").await.unwrap();
        assert_eq!(stored_secret, "deadbeef1234567890abcdef");
    }

    #[tokio::test]
    async fn should_prevent_duplicate_secrets() {
        let mut secret_manager = SecretManager::new();

        let claim_event = NearHtlcClaimEvent {
            escrow_id: "fusion_0".to_string(),
            claimer: "bob.near".to_string(),
            secret: "secret1".to_string(),
            timestamp: 1234567890,
        };

        // 最初の保存は成功
        secret_manager
            .process_claim_event(&claim_event)
            .await
            .unwrap();

        // 同じエスクローIDで再度保存しようとするとエラー
        let duplicate_event = NearHtlcClaimEvent {
            escrow_id: "fusion_0".to_string(),
            claimer: "alice.near".to_string(),
            secret: "secret2".to_string(),
            timestamp: 1234567891,
        };

        let result = secret_manager.process_claim_event(&duplicate_event).await;
        assert!(result.is_err());

        match result {
            Err(SecretError::SecretAlreadyExists) => {}
            _ => panic!("Expected SecretAlreadyExists error"),
        }
    }

    #[tokio::test]
    async fn should_return_error_for_nonexistent_secret() {
        let secret_manager = SecretManager::new();

        let result = secret_manager.get_secret("nonexistent").await;
        assert!(result.is_err());

        match result {
            Err(SecretError::SecretNotFound) => {}
            _ => panic!("Expected SecretNotFound error"),
        }
    }

    // Mockコネクター（テスト用）
    struct MockEthereumConnector;

    impl ChainConnector for MockEthereumConnector {
        fn chain_name(&self) -> &str {
            "ethereum"
        }
    }

    #[tokio::test]
    async fn should_execute_cross_chain_claim_with_revealed_secret() {
        let cross_chain_executor = CrossChainExecutor::new();
        let ethereum_connector = MockEthereumConnector;

        let claim_request = CrossChainClaimRequest {
            target_chain: "ethereum".to_string(),
            htlc_id: "0x1234567890abcdef".to_string(),
            secret: "deadbeef1234567890abcdef".to_string(),
            recipient: "0x456789abcdef".to_string(),
        };

        let result = cross_chain_executor
            .execute_claim(claim_request, &ethereum_connector)
            .await;

        assert!(result.is_ok());
        let tx_hash = result.unwrap();
        assert!(!tx_hash.is_empty());
        assert!(tx_hash.starts_with("0x"));
    }

    #[tokio::test]
    async fn should_fail_on_invalid_parameters() {
        let cross_chain_executor = CrossChainExecutor::new();
        let ethereum_connector = MockEthereumConnector;

        let invalid_request = CrossChainClaimRequest {
            target_chain: "ethereum".to_string(),
            htlc_id: "".to_string(), // 空のHTLC ID
            secret: "deadbeef".to_string(),
            recipient: "0x456".to_string(),
        };

        let result = cross_chain_executor
            .execute_claim(invalid_request, &ethereum_connector)
            .await;

        assert!(result.is_err());
        match result {
            Err(ExecutionError::InvalidParameters) => {}
            _ => panic!("Expected InvalidParameters error"),
        }
    }
}
