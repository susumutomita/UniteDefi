use crate::chains::near_events::NearHtlcClaimEvent;
use crate::htlc::{generate_secret, hash_secret, Secret, SecretHash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use tracing::{debug, info};

#[derive(Error, Debug)]
pub enum SecretError {
    #[error("Secret not found")]
    SecretNotFound,
    #[error("Invalid secret format")]
    InvalidSecretFormat,
    #[error("Secret already exists")]
    SecretAlreadyExists,
    #[error("Secret generation failed")]
    GenerationFailed,
    #[error("Secret has expired")]
    SecretExpired,
    #[error("Invalid secret hash")]
    InvalidSecretHash,
}

/// Secret data with metadata for lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretData {
    pub secret: Secret,
    pub secret_hash: SecretHash,
    pub swap_id: String,
    pub created_at: SystemTime,
    pub revealed_at: Option<SystemTime>,
    pub expires_at: SystemTime,
    pub revealed_on_chain: Option<String>, // "ethereum" or "near"
    pub revelation_tx_hash: Option<String>,
    pub auto_cleanup: bool,
}

impl SecretData {
    pub fn new(swap_id: String, ttl: Duration) -> Result<Self, SecretError> {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        let now = SystemTime::now();

        Ok(Self {
            secret,
            secret_hash,
            swap_id,
            created_at: now,
            revealed_at: None,
            expires_at: now + ttl,
            revealed_on_chain: None,
            revelation_tx_hash: None,
            auto_cleanup: true,
        })
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed_at.is_some()
    }

    pub fn mark_revealed(&mut self, chain: String, tx_hash: Option<String>) {
        self.revealed_at = Some(SystemTime::now());
        self.revealed_on_chain = Some(chain);
        self.revelation_tx_hash = tx_hash;
    }
}

/// Enhanced secret manager with lifecycle management and automatic cleanup
pub struct SecretManager {
    secrets: HashMap<String, SecretData>, // swap_id -> secret_data
    secrets_by_hash: HashMap<SecretHash, String>, // secret_hash -> swap_id
    default_ttl: Duration,
    cleanup_interval: Duration,
    last_cleanup: SystemTime,
}

impl Default for SecretManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretManager {
    pub fn new() -> Self {
        Self {
            secrets: HashMap::new(),
            secrets_by_hash: HashMap::new(),
            default_ttl: Duration::from_secs(24 * 60 * 60), // 24 hours default
            cleanup_interval: Duration::from_secs(60 * 60), // 1 hour cleanup interval
            last_cleanup: SystemTime::now(),
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = ttl;
        self
    }

    pub fn with_cleanup_interval(mut self, interval: Duration) -> Self {
        self.cleanup_interval = interval;
        self
    }

    /// Generate a new cryptographically secure secret for a swap
    pub fn generate_secret(&mut self, swap_id: &str) -> Result<SecretHash, SecretError> {
        // Check if secret already exists
        if self.secrets.contains_key(swap_id) {
            return Err(SecretError::SecretAlreadyExists);
        }

        // Generate new secret data
        let secret_data = SecretData::new(swap_id.to_string(), self.default_ttl)?;
        let secret_hash = secret_data.secret_hash;

        // Store the secret
        self.secrets_by_hash
            .insert(secret_hash, swap_id.to_string());
        self.secrets.insert(swap_id.to_string(), secret_data);

        info!("Generated secret for swap {}", swap_id);
        debug!("Secret hash: {:?}", hex::encode(secret_hash));

        Ok(secret_hash)
    }

    /// Get secret by swap ID
    pub fn get_secret(&self, swap_id: &str) -> Result<&Secret, SecretError> {
        let secret_data = self
            .secrets
            .get(swap_id)
            .ok_or(SecretError::SecretNotFound)?;

        if secret_data.is_expired() {
            return Err(SecretError::SecretExpired);
        }

        Ok(&secret_data.secret)
    }

    /// Get secret by hash
    pub fn get_secret_by_hash(&self, secret_hash: &SecretHash) -> Result<&Secret, SecretError> {
        let swap_id = self
            .secrets_by_hash
            .get(secret_hash)
            .ok_or(SecretError::SecretNotFound)?;
        self.get_secret(swap_id)
    }

    /// Get secret data with metadata
    pub fn get_secret_data(&self, swap_id: &str) -> Result<&SecretData, SecretError> {
        let secret_data = self
            .secrets
            .get(swap_id)
            .ok_or(SecretError::SecretNotFound)?;

        if secret_data.is_expired() {
            return Err(SecretError::SecretExpired);
        }

        Ok(secret_data)
    }

    /// Mark a secret as revealed from blockchain transaction
    pub fn mark_secret_revealed(
        &mut self,
        swap_id: &str,
        chain: String,
        tx_hash: Option<String>,
    ) -> Result<(), SecretError> {
        let secret_data = self
            .secrets
            .get_mut(swap_id)
            .ok_or(SecretError::SecretNotFound)?;

        if secret_data.is_expired() {
            return Err(SecretError::SecretExpired);
        }

        secret_data.mark_revealed(chain.clone(), tx_hash.clone());

        info!("Secret revealed for swap {} on chain {}", swap_id, chain);
        if let Some(tx) = tx_hash {
            debug!("Revelation transaction: {}", tx);
        }

        Ok(())
    }

    /// Extract secret from blockchain transaction data
    pub async fn extract_secret_from_transaction(
        &mut self,
        tx_data: &[u8],
        expected_hash: &SecretHash,
    ) -> Result<Secret, SecretError> {
        // Try to find a 32-byte sequence that hashes to the expected hash
        for i in 0..tx_data.len().saturating_sub(31) {
            let potential_secret: [u8; 32] = tx_data[i..i + 32]
                .try_into()
                .map_err(|_| SecretError::InvalidSecretFormat)?;

            let computed_hash = hash_secret(&potential_secret);
            if computed_hash == *expected_hash {
                info!("Successfully extracted secret from transaction data");
                return Ok(potential_secret);
            }
        }

        Err(SecretError::SecretNotFound)
    }

    /// Process claim event and extract secret
    pub async fn process_claim_event(
        &mut self,
        event: &NearHtlcClaimEvent,
    ) -> Result<(), SecretError> {
        // Validate secret format
        if event.secret.is_empty() {
            return Err(SecretError::InvalidSecretFormat);
        }

        // Convert hex string to bytes
        let secret_bytes =
            hex::decode(&event.secret).map_err(|_| SecretError::InvalidSecretFormat)?;

        if secret_bytes.len() != 32 {
            return Err(SecretError::InvalidSecretFormat);
        }

        let mut secret = [0u8; 32];
        secret.copy_from_slice(&secret_bytes);

        // Find the corresponding swap by secret hash
        let secret_hash = hash_secret(&secret);
        let swap_id = self
            .secrets_by_hash
            .get(&secret_hash)
            .ok_or(SecretError::SecretNotFound)?
            .clone();

        // Mark as revealed
        self.mark_secret_revealed(&swap_id, "near".to_string(), None)?;

        Ok(())
    }

    /// Check if a secret is revealed
    pub fn is_secret_revealed(&self, swap_id: &str) -> bool {
        self.secrets
            .get(swap_id)
            .map(|data| data.is_revealed())
            .unwrap_or(false)
    }

    /// Get revelation details
    pub fn get_revelation_details(
        &self,
        swap_id: &str,
    ) -> Option<(String, SystemTime, Option<String>)> {
        self.secrets.get(swap_id).and_then(|data| {
            if let (Some(chain), Some(revealed_at)) = (&data.revealed_on_chain, data.revealed_at) {
                Some((chain.clone(), revealed_at, data.revelation_tx_hash.clone()))
            } else {
                None
            }
        })
    }

    /// Perform automatic cleanup of expired secrets
    pub fn cleanup_expired_secrets(&mut self) -> usize {
        let now = SystemTime::now();

        // Check if cleanup is needed
        if now
            .duration_since(self.last_cleanup)
            .unwrap_or(Duration::ZERO)
            < self.cleanup_interval
        {
            return 0;
        }

        let mut removed_count = 0;
        let mut to_remove = Vec::new();

        // Find expired secrets
        for (swap_id, secret_data) in &self.secrets {
            if secret_data.auto_cleanup && secret_data.is_expired() {
                to_remove.push((swap_id.clone(), secret_data.secret_hash));
                removed_count += 1;
            }
        }

        // Remove expired secrets
        for (swap_id, secret_hash) in to_remove {
            self.secrets.remove(&swap_id);
            self.secrets_by_hash.remove(&secret_hash);
            info!("Cleaned up expired secret for swap {}", swap_id);
        }

        self.last_cleanup = now;

        if removed_count > 0 {
            info!("Cleaned up {} expired secrets", removed_count);
        }

        removed_count
    }

    /// Force cleanup of a specific secret (secure disposal)
    pub fn dispose_secret(&mut self, swap_id: &str) -> Result<(), SecretError> {
        let secret_data = self
            .secrets
            .remove(swap_id)
            .ok_or(SecretError::SecretNotFound)?;
        self.secrets_by_hash.remove(&secret_data.secret_hash);

        info!("Securely disposed secret for swap {}", swap_id);
        Ok(())
    }

    /// Get statistics about managed secrets
    pub fn get_stats(&self) -> SecretManagerStats {
        let total = self.secrets.len();
        let revealed = self.secrets.values().filter(|s| s.is_revealed()).count();
        let expired = self.secrets.values().filter(|s| s.is_expired()).count();

        SecretManagerStats {
            total_secrets: total,
            revealed_secrets: revealed,
            expired_secrets: expired,
            active_secrets: total - expired,
        }
    }

    /// Clear all secrets (for testing)
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.secrets.clear();
        self.secrets_by_hash.clear();
    }
}

#[derive(Debug, Clone)]
pub struct SecretManagerStats {
    pub total_secrets: usize,
    pub revealed_secrets: usize,
    pub expired_secrets: usize,
    pub active_secrets: usize,
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
    use std::thread;

    #[tokio::test]
    async fn test_secret_generation() {
        let mut secret_manager = SecretManager::new();

        // Generate a secret
        let secret_hash = secret_manager.generate_secret("swap_1").unwrap();

        // Verify secret exists and can be retrieved
        let secret = secret_manager.get_secret("swap_1").unwrap();
        assert_eq!(secret.len(), 32);

        // Verify hash matches
        let computed_hash = hash_secret(secret);
        assert_eq!(computed_hash, secret_hash);

        // Verify secret can be retrieved by hash
        let secret_by_hash = secret_manager.get_secret_by_hash(&secret_hash).unwrap();
        assert_eq!(secret, secret_by_hash);
    }

    #[tokio::test]
    async fn test_secret_lifecycle() {
        let mut secret_manager = SecretManager::new();

        // Generate secret
        let _secret_hash = secret_manager.generate_secret("swap_1").unwrap();

        // Initially not revealed
        assert!(!secret_manager.is_secret_revealed("swap_1"));

        // Mark as revealed
        secret_manager
            .mark_secret_revealed("swap_1", "ethereum".to_string(), Some("0x123".to_string()))
            .unwrap();

        // Now should be revealed
        assert!(secret_manager.is_secret_revealed("swap_1"));

        // Check revelation details
        let (chain, _timestamp, tx_hash) = secret_manager.get_revelation_details("swap_1").unwrap();
        assert_eq!(chain, "ethereum");
        assert_eq!(tx_hash, Some("0x123".to_string()));
    }

    #[tokio::test]
    async fn test_secret_expiration() {
        let mut secret_manager = SecretManager::new().with_ttl(Duration::from_millis(100)); // Very short TTL for testing

        // Generate secret
        secret_manager.generate_secret("swap_1").unwrap();

        // Should be accessible initially
        assert!(secret_manager.get_secret("swap_1").is_ok());

        // Wait for expiration
        thread::sleep(Duration::from_millis(150));

        // Should now be expired
        let result = secret_manager.get_secret("swap_1");
        assert!(matches!(result, Err(SecretError::SecretExpired)));
    }

    #[tokio::test]
    async fn test_automatic_cleanup() {
        let mut secret_manager = SecretManager::new()
            .with_ttl(Duration::from_millis(50))
            .with_cleanup_interval(Duration::from_millis(10));

        // Generate multiple secrets
        secret_manager.generate_secret("swap_1").unwrap();
        secret_manager.generate_secret("swap_2").unwrap();
        secret_manager.generate_secret("swap_3").unwrap();

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 3);

        // Wait for expiration
        thread::sleep(Duration::from_millis(100));

        // Trigger cleanup
        let cleaned = secret_manager.cleanup_expired_secrets();
        assert_eq!(cleaned, 3);

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 0);
    }

    #[tokio::test]
    async fn test_secret_extraction_from_transaction() {
        let mut secret_manager = SecretManager::new();

        // Generate a known secret
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);

        // Create transaction data containing the secret
        let mut tx_data = vec![0u8; 100];
        tx_data[50..82].copy_from_slice(&secret);

        // Extract secret from transaction
        let extracted_secret = secret_manager
            .extract_secret_from_transaction(&tx_data, &secret_hash)
            .await
            .unwrap();

        assert_eq!(extracted_secret, secret);
    }

    #[tokio::test]
    async fn test_process_claim_event() {
        let mut secret_manager = SecretManager::new();

        // Generate a secret first
        let _secret_hash = secret_manager.generate_secret("swap_1").unwrap();
        let secret = *secret_manager.get_secret("swap_1").unwrap();

        // Create claim event with the secret
        let claim_event = NearHtlcClaimEvent {
            escrow_id: "fusion_0".to_string(),
            claimer: "bob.near".to_string(),
            secret: hex::encode(secret),
            timestamp: 1234567890,
        };

        // Process the claim event
        secret_manager
            .process_claim_event(&claim_event)
            .await
            .unwrap();

        // Secret should now be marked as revealed
        assert!(secret_manager.is_secret_revealed("swap_1"));

        let (chain, _timestamp, _tx_hash) =
            secret_manager.get_revelation_details("swap_1").unwrap();
        assert_eq!(chain, "near");
    }

    #[tokio::test]
    async fn test_prevent_duplicate_secrets() {
        let mut secret_manager = SecretManager::new();

        // Generate first secret
        secret_manager.generate_secret("swap_1").unwrap();

        // Try to generate another secret with same ID
        let result = secret_manager.generate_secret("swap_1");
        assert!(matches!(result, Err(SecretError::SecretAlreadyExists)));
    }

    #[tokio::test]
    async fn test_secure_disposal() {
        let mut secret_manager = SecretManager::new();

        // Generate secret
        secret_manager.generate_secret("swap_1").unwrap();

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 1);

        // Dispose of secret
        secret_manager.dispose_secret("swap_1").unwrap();

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 0);

        // Should not be able to retrieve disposed secret
        let result = secret_manager.get_secret("swap_1");
        assert!(matches!(result, Err(SecretError::SecretNotFound)));
    }

    #[tokio::test]
    async fn test_secret_manager_stats() {
        let mut secret_manager = SecretManager::new().with_ttl(Duration::from_millis(100));

        // Generate secrets
        secret_manager.generate_secret("swap_1").unwrap();
        secret_manager.generate_secret("swap_2").unwrap();
        secret_manager.generate_secret("swap_3").unwrap();

        // Mark one as revealed
        secret_manager
            .mark_secret_revealed("swap_1", "ethereum".to_string(), None)
            .unwrap();

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 3);
        assert_eq!(stats.revealed_secrets, 1);
        assert_eq!(stats.active_secrets, 3);

        // Wait for expiration
        thread::sleep(Duration::from_millis(150));

        let stats = secret_manager.get_stats();
        assert_eq!(stats.total_secrets, 3);
        assert_eq!(stats.expired_secrets, 3);
        assert_eq!(stats.active_secrets, 0);
    }

    #[tokio::test]
    async fn test_invalid_secret_format_in_claim_event() {
        let mut secret_manager = SecretManager::new();

        let invalid_event = NearHtlcClaimEvent {
            escrow_id: "fusion_0".to_string(),
            claimer: "bob.near".to_string(),
            secret: "invalid_hex".to_string(),
            timestamp: 1234567890,
        };

        let result = secret_manager.process_claim_event(&invalid_event).await;
        assert!(matches!(result, Err(SecretError::InvalidSecretFormat)));
    }

    #[tokio::test]
    async fn test_secret_not_found_in_transaction() {
        let mut secret_manager = SecretManager::new();

        // Create random hash that won't be found
        let random_hash = [1u8; 32];
        let tx_data = vec![0u8; 100];

        let result = secret_manager
            .extract_secret_from_transaction(&tx_data, &random_hash)
            .await;

        assert!(matches!(result, Err(SecretError::SecretNotFound)));
    }

    // Mock connector for cross-chain executor tests
    struct MockEthereumConnector;

    impl ChainConnector for MockEthereumConnector {
        fn chain_name(&self) -> &str {
            "ethereum"
        }
    }

    #[tokio::test]
    async fn test_cross_chain_executor_with_valid_request() {
        let cross_chain_executor = CrossChainExecutor::new();
        let ethereum_connector = MockEthereumConnector;

        let claim_request = CrossChainClaimRequest {
            target_chain: "ethereum".to_string(),
            htlc_id: "0x1234567890abcdef".to_string(),
            secret: hex::encode(generate_secret()),
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
    async fn test_cross_chain_executor_with_invalid_parameters() {
        let cross_chain_executor = CrossChainExecutor::new();
        let ethereum_connector = MockEthereumConnector;

        let invalid_request = CrossChainClaimRequest {
            target_chain: "ethereum".to_string(),
            htlc_id: "".to_string(), // Empty HTLC ID
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
