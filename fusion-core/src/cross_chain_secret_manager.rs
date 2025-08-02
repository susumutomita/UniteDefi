use crate::chains::ethereum::event_monitor::EthereumEventMonitor;
use crate::chains::near::event_monitor::NearEventMonitor;
use crate::htlc::SecretHash;
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

#[derive(Debug, Clone)]
pub struct SecretMapping {
    pub secret: [u8; 32],
    pub secret_hash: SecretHash,
    pub evm_order_hash: String,
    pub near_htlc_id: String,
    pub revealed_on: Option<String>, // "ethereum" or "near"
    pub revelation_timestamp: Option<u64>,
}

pub struct CrossChainSecretManager {
    secret_mappings: Arc<RwLock<HashMap<SecretHash, SecretMapping>>>,
    eth_monitor: Option<Arc<EthereumEventMonitor>>,
    near_monitor: Option<Arc<NearEventMonitor>>,
}

impl Default for CrossChainSecretManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossChainSecretManager {
    pub fn new() -> Self {
        Self {
            secret_mappings: Arc::new(RwLock::new(HashMap::new())),
            eth_monitor: None,
            near_monitor: None,
        }
    }

    pub fn with_ethereum_monitor(mut self, monitor: EthereumEventMonitor) -> Self {
        self.eth_monitor = Some(Arc::new(monitor));
        self
    }

    pub fn with_near_monitor(mut self, monitor: NearEventMonitor) -> Self {
        self.near_monitor = Some(Arc::new(monitor));
        self
    }

    pub async fn register_cross_chain_swap(
        &self,
        secret: [u8; 32],
        evm_order_hash: String,
        near_htlc_id: String,
    ) -> Result<SecretHash> {
        // Calculate secret hash
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let hash_result = hasher.finalize();
        let mut secret_hash = [0u8; 32];
        secret_hash.copy_from_slice(&hash_result);

        // Store mapping
        let mapping = SecretMapping {
            secret,
            secret_hash,
            evm_order_hash,
            near_htlc_id,
            revealed_on: None,
            revelation_timestamp: None,
        };

        let mut mappings = self.secret_mappings.write().await;
        mappings.insert(secret_hash, mapping);

        Ok(secret_hash)
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        let mappings = self.secret_mappings.clone();

        // Monitor Ethereum for secret revelations
        if let Some(eth_monitor) = &self.eth_monitor {
            let (eth_tx, mut eth_rx) = mpsc::channel::<Vec<u8>>(100);
            let mappings_eth = mappings.clone();
            let eth_monitor = eth_monitor.clone();

            tokio::spawn(async move {
                eth_monitor
                    .monitor_secret_revealed_events(eth_tx)
                    .await
                    .ok();
            });

            tokio::spawn(async move {
                while let Some(secret) = eth_rx.recv().await {
                    if secret.len() == 32 {
                        let mut secret_array = [0u8; 32];
                        secret_array.copy_from_slice(&secret);

                        // Calculate hash
                        let mut hasher = Sha256::new();
                        hasher.update(secret_array);
                        let hash_result = hasher.finalize();
                        let mut secret_hash = [0u8; 32];
                        secret_hash.copy_from_slice(&hash_result);

                        // Update mapping
                        let mut mappings = mappings_eth.write().await;
                        if let Some(mapping) = mappings.get_mut(&secret_hash) {
                            mapping.revealed_on = Some("ethereum".to_string());
                            mapping.revelation_timestamp = Some(
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            );
                        }
                    }
                }
            });
        }

        // Monitor NEAR for secret revelations
        if let Some(near_monitor) = &self.near_monitor {
            let (near_tx, mut near_rx) = mpsc::channel::<Vec<u8>>(100);
            let mappings_near = mappings.clone();
            let near_monitor = near_monitor.clone();

            tokio::spawn(async move {
                near_monitor
                    .monitor_secret_revealed_events(near_tx)
                    .await
                    .ok();
            });

            tokio::spawn(async move {
                while let Some(secret) = near_rx.recv().await {
                    if secret.len() == 32 {
                        let mut secret_array = [0u8; 32];
                        secret_array.copy_from_slice(&secret);

                        // Calculate hash
                        let mut hasher = Sha256::new();
                        hasher.update(secret_array);
                        let hash_result = hasher.finalize();
                        let mut secret_hash = [0u8; 32];
                        secret_hash.copy_from_slice(&hash_result);

                        // Update mapping
                        let mut mappings = mappings_near.write().await;
                        if let Some(mapping) = mappings.get_mut(&secret_hash) {
                            mapping.revealed_on = Some("near".to_string());
                            mapping.revelation_timestamp = Some(
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            );
                        }
                    }
                }
            });
        }

        Ok(())
    }

    pub async fn get_revealed_secret(&self, secret_hash: &SecretHash) -> Result<Option<[u8; 32]>> {
        let mappings = self.secret_mappings.read().await;

        if let Some(mapping) = mappings.get(secret_hash) {
            if mapping.revealed_on.is_some() {
                return Ok(Some(mapping.secret));
            }
        }

        Ok(None)
    }

    pub async fn is_secret_revealed(&self, secret_hash: &SecretHash) -> bool {
        let mappings = self.secret_mappings.read().await;

        mappings
            .get(secret_hash)
            .map(|m| m.revealed_on.is_some())
            .unwrap_or(false)
    }

    pub async fn get_revelation_details(
        &self,
        secret_hash: &SecretHash,
    ) -> Result<Option<(String, u64)>> {
        let mappings = self.secret_mappings.read().await;

        if let Some(mapping) = mappings.get(secret_hash) {
            if let (Some(chain), Some(timestamp)) =
                (&mapping.revealed_on, mapping.revelation_timestamp)
            {
                return Ok(Some((chain.clone(), timestamp)));
            }
        }

        Ok(None)
    }
}
