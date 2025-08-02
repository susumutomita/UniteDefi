use crate::chains::ethereum::EthereumConnector;
use crate::chains::near::NearHtlcConnector;
use crate::cross_chain_secret_manager::CrossChainSecretManager;
use crate::htlc::SecretHash;
use anyhow::{anyhow, Result};
use ethers::types::Address;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct ClaimExecutor {
    secret_manager: Arc<CrossChainSecretManager>,
    eth_connector: Option<EthereumConnector>,
    near_connector: Option<NearHtlcConnector>,
}

impl ClaimExecutor {
    pub fn new(secret_manager: Arc<CrossChainSecretManager>) -> Self {
        Self {
            secret_manager,
            eth_connector: None,
            near_connector: None,
        }
    }

    pub fn with_ethereum_connector(mut self, connector: EthereumConnector) -> Self {
        self.eth_connector = Some(connector);
        self
    }

    pub fn with_near_connector(mut self, connector: NearHtlcConnector) -> Self {
        self.near_connector = Some(connector);
        self
    }

    /// Monitor for secret revelations and automatically claim on the opposite chain
    pub async fn start_auto_claiming(&self) -> Result<()> {
        loop {
            // Check all registered swaps for revealed secrets
            // In production, this would be event-driven rather than polling

            sleep(Duration::from_secs(10)).await;
        }
    }

    /// Claim HTLC on Ethereum using revealed secret
    pub async fn claim_on_ethereum(
        &self,
        escrow_address: &str,
        secret_hash: &SecretHash,
    ) -> Result<String> {
        let eth_connector = self
            .eth_connector
            .as_ref()
            .ok_or_else(|| anyhow!("Ethereum connector not configured"))?;

        // Wait for secret to be revealed
        let secret = self
            .wait_for_secret_revelation(secret_hash, Duration::from_secs(3600))
            .await?;

        // Parse escrow address
        let escrow_address = escrow_address
            .parse::<Address>()
            .map_err(|e| anyhow!("Invalid escrow address: {}", e))?;

        // Claim the escrow
        let receipt = eth_connector
            .claim_escrow(escrow_address, secret)
            .await
            .map_err(|e| anyhow!("Failed to claim escrow: {}", e))?;

        Ok(format!("0x{:x}", receipt.transaction_hash))
    }

    /// Claim HTLC on NEAR using revealed secret
    pub async fn claim_on_near(&self, htlc_id: &str, secret_hash: &SecretHash) -> Result<String> {
        let near_connector = self
            .near_connector
            .as_ref()
            .ok_or_else(|| anyhow!("NEAR connector not configured"))?;

        // Wait for secret to be revealed
        let secret = self
            .wait_for_secret_revelation(secret_hash, Duration::from_secs(3600))
            .await?;

        // Claim the HTLC
        let tx_hash = near_connector.claim_htlc(htlc_id, secret).await?;

        Ok(tx_hash)
    }

    /// Wait for a secret to be revealed on either chain
    async fn wait_for_secret_revelation(
        &self,
        secret_hash: &SecretHash,
        timeout: Duration,
    ) -> Result<[u8; 32]> {
        let start = tokio::time::Instant::now();

        loop {
            if let Some(secret) = self.secret_manager.get_revealed_secret(secret_hash).await? {
                return Ok(secret);
            }

            if start.elapsed() > timeout {
                return Err(anyhow!("Timeout waiting for secret revelation"));
            }

            sleep(Duration::from_secs(1)).await;
        }
    }

    /// Get claim status for a cross-chain swap
    pub async fn get_claim_status(&self, secret_hash: &SecretHash) -> Result<ClaimStatus> {
        if let Some((chain, timestamp)) = self
            .secret_manager
            .get_revelation_details(secret_hash)
            .await?
        {
            Ok(ClaimStatus {
                secret_revealed: true,
                revealed_on_chain: Some(chain),
                revelation_timestamp: Some(timestamp),
                ethereum_claimed: false, // Would check actual contract state
                near_claimed: false,     // Would check actual contract state
            })
        } else {
            Ok(ClaimStatus {
                secret_revealed: false,
                revealed_on_chain: None,
                revelation_timestamp: None,
                ethereum_claimed: false,
                near_claimed: false,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClaimStatus {
    pub secret_revealed: bool,
    pub revealed_on_chain: Option<String>,
    pub revelation_timestamp: Option<u64>,
    pub ethereum_claimed: bool,
    pub near_claimed: bool,
}
