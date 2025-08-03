use anyhow::{anyhow, Result};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTLCStatus {
    pub htlc_id: String,
    pub chain: String,
    pub status: String,
    pub secret: Option<String>,
    pub timeout: u64,
    pub recipient: String,
    pub amount: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HTLCMonitor {
    pub ethereum_rpc: String,
    pub near_network: String,
}

impl HTLCMonitor {
    pub fn new(ethereum_rpc: String, near_network: String) -> Self {
        Self {
            ethereum_rpc,
            near_network,
        }
    }

    /// Monitor HTLC status on both chains
    pub async fn monitor_htlc(
        &self,
        htlc_id: &str,
        chain: &str,
        max_attempts: u32,
        interval_secs: u64,
    ) -> Result<HTLCStatus> {
        for attempt in 1..=max_attempts {
            println!(
                "Checking HTLC status... (attempt {}/{})",
                attempt, max_attempts
            );

            match chain {
                "ethereum" => {
                    let status = self.check_ethereum_htlc(htlc_id).await?;
                    if status.status != "pending" {
                        return Ok(status);
                    }
                }
                "near" => {
                    let status = self.check_near_htlc(htlc_id).await?;
                    if status.status != "active" {
                        return Ok(status);
                    }
                }
                _ => return Err(anyhow!("Unsupported chain: {}", chain)),
            }

            if attempt < max_attempts {
                sleep(Duration::from_secs(interval_secs)).await;
            }
        }

        Err(anyhow!(
            "HTLC monitoring timed out after {} attempts",
            max_attempts
        ))
    }

    /// Check HTLC status on Ethereum
    async fn check_ethereum_htlc(&self, htlc_id: &str) -> Result<HTLCStatus> {
        // In a real implementation, this would query the Ethereum HTLC contract
        // For now, return a mock status
        println!("Checking Ethereum HTLC: {}", htlc_id);

        Ok(HTLCStatus {
            htlc_id: htlc_id.to_string(),
            chain: "ethereum".to_string(),
            status: "pending".to_string(),
            secret: None,
            timeout: 3600,
            recipient: "0x0000000000000000000000000000000000000000".to_string(),
            amount: "1000000000000000000".to_string(),
        })
    }

    /// Check HTLC status on NEAR
    async fn check_near_htlc(&self, htlc_id: &str) -> Result<HTLCStatus> {
        use std::process::Command;

        println!("Checking NEAR HTLC: {}", htlc_id);

        // Query NEAR contract for escrow status
        let output = Command::new("near")
            .args([
                "view",
                "htlc-v2.testnet",
                "get_escrow",
                &format!(r#"{{"escrow_id": "{}"}}"#, htlc_id),
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute NEAR command: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        if !output.status.success() {
            let error_str = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to get NEAR HTLC status: {}", error_str));
        }

        // Parse the output to extract status
        let status = if output_str.contains("claimed") {
            "claimed"
        } else if output_str.contains("refunded") {
            "refunded"
        } else if output_str.contains("active") {
            "active"
        } else {
            "unknown"
        };

        Ok(HTLCStatus {
            htlc_id: htlc_id.to_string(),
            chain: "near".to_string(),
            status: status.to_string(),
            secret: None,
            timeout: 3600,
            recipient: "recipient.testnet".to_string(),
            amount: "1000000000000000000000000".to_string(),
        })
    }

    /// Claim HTLC on Ethereum
    pub async fn claim_ethereum_htlc(
        &self,
        htlc_id: &str,
        secret: &str,
        private_key: Option<String>,
    ) -> Result<String> {
        println!("Claiming Ethereum HTLC {} with secret", htlc_id);

        let private_key =
            private_key.ok_or_else(|| anyhow!("Private key required for Ethereum HTLC claim"))?;

        // Create provider and signer
        let provider = Provider::<Http>::try_from(&self.ethereum_rpc)?;
        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|_| anyhow!("Invalid private key format"))?;
        let chain_id = provider.get_chainid().await?;
        let wallet = wallet.with_chain_id(chain_id.as_u64());
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        // Define the HTLC contract ABI for claim function
        abigen!(
            HTLCContract,
            r#"[
                {
                    "inputs": [
                        {"internalType": "bytes32", "name": "htlcId", "type": "bytes32"},
                        {"internalType": "bytes32", "name": "secret", "type": "bytes32"}
                    ],
                    "name": "claim",
                    "outputs": [],
                    "stateMutability": "nonpayable",
                    "type": "function"
                }
            ]"#
        );

        // Get HTLC contract address from environment or use default
        let contract_address = std::env::var("ETH_HTLC_CONTRACT")
            .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000".to_string());
        let contract_address = Address::from_str(&contract_address)?;
        let contract = HTLCContract::new(contract_address, client);

        // Convert htlc_id and secret to bytes32
        let htlc_id_bytes = if let Some(stripped) = htlc_id.strip_prefix("0x") {
            hex::decode(stripped)?
        } else {
            hex::decode(htlc_id)?
        };
        let htlc_id_bytes32 = if htlc_id_bytes.len() == 32 {
            let mut array = [0u8; 32];
            array.copy_from_slice(&htlc_id_bytes);
            array
        } else {
            return Err(anyhow!("HTLC ID must be 32 bytes"));
        };

        let secret_bytes = if let Some(stripped) = secret.strip_prefix("0x") {
            hex::decode(stripped)?
        } else {
            hex::decode(secret)?
        };
        let secret_bytes32 = if secret_bytes.len() == 32 {
            let mut array = [0u8; 32];
            array.copy_from_slice(&secret_bytes);
            array
        } else {
            return Err(anyhow!("Secret must be 32 bytes"));
        };

        // Call claim function
        let tx_call = contract
            .claim(htlc_id_bytes32, secret_bytes32)
            .gas(150000u64);

        let tx = tx_call.send().await?;
        let tx_hash = format!("0x{:x}", tx.tx_hash());
        println!("Claim transaction submitted: {}", tx_hash);

        // Wait for confirmation
        let receipt = tx.await?;
        if let Some(receipt) = receipt {
            println!("HTLC claimed successfully!");
            println!("  Transaction hash: {:?}", receipt.transaction_hash);
            println!("  Block number: {:?}", receipt.block_number);
            println!("  Gas used: {:?}", receipt.gas_used);
        }

        Ok(tx_hash)
    }

    /// Claim HTLC on NEAR
    pub async fn claim_near_htlc(
        &self,
        htlc_id: &str,
        secret: &str,
        account_id: &str,
    ) -> Result<String> {
        use std::process::Command;

        println!("Claiming NEAR HTLC {} with secret", htlc_id);

        // Execute NEAR claim command
        let output = Command::new("near")
            .args([
                "call",
                "htlc-v2.testnet",
                "claim",
                &format!(r#"{{"escrow_id": "{}", "secret": "{}"}}"#, htlc_id, secret),
                "--use-account",
                account_id,
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute NEAR claim: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let error_str = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Err(anyhow!("NEAR HTLC claim failed: {}", error_str));
        }

        // Extract transaction ID from output
        if let Some(tx_start) = output_str.find("Transaction Id: ") {
            let tx_line = &output_str[tx_start..];
            if let Some(tx_end) = tx_line.find('\n') {
                let tx_id = &tx_line[16..tx_end].trim();
                return Ok(tx_id.to_string());
            }
        }

        Ok("claim_successful".to_string())
    }

    /// Automated bidirectional swap flow
    pub async fn execute_bidirectional_swap(
        &self,
        source_chain: &str,
        target_chain: &str,
        source_htlc_id: &str,
        target_htlc_id: &str,
        secret: &str,
        interval_secs: u64,
    ) -> Result<()> {
        println!("Starting bidirectional swap monitoring...");
        println!("Source: {} ({})", source_chain, source_htlc_id);
        println!("Target: {} ({})", target_chain, target_htlc_id);

        // Monitor source chain HTLC
        let _max_attempts = 60; // 30 minutes with 30 second intervals

        loop {
            // Check source chain HTLC status
            let source_status = self
                .monitor_htlc(source_htlc_id, source_chain, 1, 0)
                .await?;

            println!("{} HTLC status: {}", source_chain, source_status.status);

            // Check target chain HTLC status
            let target_status = self
                .monitor_htlc(target_htlc_id, target_chain, 1, 0)
                .await?;

            println!("{} HTLC status: {}", target_chain, target_status.status);

            // If source is claimed, we need to claim target
            if source_status.status == "claimed" && target_status.status == "active" {
                println!("Source HTLC claimed! Claiming target HTLC...");

                match target_chain {
                    "ethereum" => {
                        let private_key = std::env::var("PRIVATE_KEY").ok();
                        let tx_hash = self
                            .claim_ethereum_htlc(target_htlc_id, secret, private_key)
                            .await?;
                        println!("Ethereum HTLC claimed! Transaction: {}", tx_hash);
                        break;
                    }
                    "near" => {
                        let account_id = std::env::var("NEAR_ACCOUNT_ID")
                            .unwrap_or_else(|_| "user.testnet".to_string());
                        let tx_id = self
                            .claim_near_htlc(target_htlc_id, secret, &account_id)
                            .await?;
                        println!("NEAR HTLC claimed! Transaction: {}", tx_id);
                        break;
                    }
                    _ => return Err(anyhow!("Unsupported target chain")),
                }
            }

            // Check for timeout or refund conditions
            if source_status.status == "refunded" || target_status.status == "refunded" {
                return Err(anyhow!("Swap failed: one or both HTLCs were refunded"));
            }

            if source_status.status == "claimed" && target_status.status == "claimed" {
                println!("Swap completed successfully!");
                break;
            }

            sleep(Duration::from_secs(interval_secs)).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htlc_status_serialization() {
        let status = HTLCStatus {
            htlc_id: "escrow_123".to_string(),
            chain: "near".to_string(),
            status: "active".to_string(),
            secret: None,
            timeout: 3600,
            recipient: "alice.testnet".to_string(),
            amount: "1000000000000000000000000".to_string(),
        };

        let json = serde_json::to_string(&status).unwrap();
        let parsed: HTLCStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.htlc_id, status.htlc_id);
        assert_eq!(parsed.chain, status.chain);
        assert_eq!(parsed.status, status.status);
    }
}
