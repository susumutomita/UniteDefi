use crate::htlc::SecretHash;
use near_crypto::{InMemorySigner, SecretKey};
use near_jsonrpc_client::{JsonRpcClient, methods};
use near_primitives::types::{AccountId, Balance};
use near_primitives::views::{ExecutionOutcomeWithIdView, FinalExecutionOutcomeView, QueryRequest};
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use serde_json::json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FusionEscrowView {
    pub resolver: String,
    pub beneficiary: String,
    pub amount: String,
    pub safety_deposit: String,
    pub safety_deposit_beneficiary: Option<String>,
    pub token_id: Option<String>,
    pub secret_hash: String,
    pub deployment_time: u64,
    pub finality_time: u64,
    pub cancel_time: u64,
    pub public_cancel_time: u64,
    pub state: String,
    pub resolved_by: Option<String>,
    pub resolution_time: Option<u64>,
}

pub struct NearConnector {
    rpc_url: String,
    contract_id: Option<String>,
    account_id: Option<String>,
    signer: Option<InMemorySigner>,
}

impl NearConnector {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            contract_id: None,
            account_id: None,
            signer: None,
        }
    }

    pub fn with_contract(mut self, contract_id: &str) -> Self {
        self.contract_id = Some(contract_id.to_string());
        self
    }

    pub fn with_account_id(mut self, account_id: String) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn with_private_key(mut self, private_key: &str) -> Self {
        if let (Some(account_id), Ok(secret_key)) = (&self.account_id, private_key.parse::<SecretKey>()) {
            if let Ok(account_id) = account_id.parse::<AccountId>() {
                self.signer = Some(InMemorySigner::from_secret_key(account_id, secret_key));
            }
        }
        self
    }

    pub fn account_id(&self) -> &String {
        self.account_id.as_ref().expect("Account ID not set")
    }

    pub fn has_private_key(&self) -> bool {
        self.signer.is_some()
    }

    pub async fn create_htlc(
        &self,
        amount: u128,
        secret_hash: SecretHash,
        timeout_seconds: u64,
        recipient: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let contract_id = self.contract_id.as_ref()
            .ok_or("Contract ID not set")?;
        let signer = self.signer.as_ref()
            .ok_or("Signer not configured")?;

        let client = JsonRpcClient::connect(&self.rpc_url);
        
        // Prepare the function call arguments
        let args = json!({
            "beneficiary": recipient,
            "secret_hash": bs58::encode(secret_hash.as_bytes()).into_string(),
            "amount": amount.to_string(),
            "safety_deposit": "0",
            "finality_period": timeout_seconds / 3,
            "cancel_period": timeout_seconds,
            "public_cancel_period": timeout_seconds * 2,
        });

        // Create the function call action
        let args_vec = serde_json::to_vec(&args)?;
        let action = Action::FunctionCall(Box::new(FunctionCallAction {
            method_name: "create_escrow".to_string(),
            args: args_vec,
            gas: 100_000_000_000_000, // 100 TGas
            deposit: amount, // Deposit the NEAR amount
        }));

        // Get the latest block hash
        let access_key_query_request = methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        };

        let access_key_response = client.call(access_key_query_request).await?;
        let access_key = match access_key_response.kind {
            near_primitives::views::QueryResponseKind::AccessKey(ak) => ak,
            _ => return Err("Failed to get access key".into()),
        };

        // Create the transaction
        let transaction = Transaction {
            signer_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
            nonce: access_key.nonce + 1,
            receiver_id: contract_id.parse()?,
            block_hash: access_key_response.block_hash,
            actions: vec![action],
        };

        // Sign and send the transaction
        let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction: transaction.sign(signer),
        };

        let response = client.call(request).await?;
        
        // Extract the escrow ID from the response
        if let Some(outcome) = response.final_execution_outcome {
            for receipt_outcome in outcome.receipts_outcome {
                for log in receipt_outcome.outcome.logs {
                    if log.contains("Fusion escrow created:") {
                        if let Some(escrow_id) = log.split_whitespace().nth(3) {
                            return Ok(escrow_id.to_string());
                        }
                    }
                }
            }
        }

        Ok("fusion_0".to_string()) // Fallback if log parsing fails
    }

    pub async fn claim(
        &self,
        escrow_id: &str,
        secret: [u8; 32],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let contract_id = self.contract_id.as_ref()
            .ok_or("Contract ID not set")?;
        let signer = self.signer.as_ref()
            .ok_or("Signer not configured")?;

        let client = JsonRpcClient::connect(&self.rpc_url);
        
        // Prepare the function call arguments
        let args = json!({
            "escrow_id": escrow_id,
            "secret": hex::encode(&secret),
        });

        // Create the function call action
        let args_vec = serde_json::to_vec(&args)?;
        let action = Action::FunctionCall(Box::new(FunctionCallAction {
            method_name: "claim".to_string(),
            args: args_vec,
            gas: 100_000_000_000_000, // 100 TGas
            deposit: 0, // No deposit needed for claiming
        }));

        // Get the latest block hash
        let access_key_query_request = methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        };

        let access_key_response = client.call(access_key_query_request).await?;
        let access_key = match access_key_response.kind {
            near_primitives::views::QueryResponseKind::AccessKey(ak) => ak,
            _ => return Err("Failed to get access key".into()),
        };

        // Create the transaction
        let transaction = Transaction {
            signer_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
            nonce: access_key.nonce + 1,
            receiver_id: contract_id.parse()?,
            block_hash: access_key_response.block_hash,
            actions: vec![action],
        };

        // Sign and send the transaction
        let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction: transaction.sign(signer),
        };

        let response = client.call(request).await?;
        
        // Check if the transaction succeeded
        if let Some(outcome) = response.final_execution_outcome {
            match outcome.status {
                near_primitives::views::FinalExecutionStatus::SuccessValue(_) => Ok("success".to_string()),
                _ => Err("Claim transaction failed".into()),
            }
        } else {
            Err("No execution outcome received".into())
        }
    }

    pub async fn refund(
        &self,
        escrow_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let contract_id = self.contract_id.as_ref()
            .ok_or("Contract ID not set")?;
        let signer = self.signer.as_ref()
            .ok_or("Signer not configured")?;

        let client = JsonRpcClient::connect(&self.rpc_url);
        
        // Prepare the function call arguments
        let args = json!({
            "escrow_id": escrow_id,
        });

        // Create the function call action
        let args_vec = serde_json::to_vec(&args)?;
        let action = Action::FunctionCall(Box::new(FunctionCallAction {
            method_name: "cancel".to_string(),
            args: args_vec,
            gas: 100_000_000_000_000, // 100 TGas
            deposit: 0, // No deposit needed for cancelling
        }));

        // Get the latest block hash
        let access_key_query_request = methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        };

        let access_key_response = client.call(access_key_query_request).await?;
        let access_key = match access_key_response.kind {
            near_primitives::views::QueryResponseKind::AccessKey(ak) => ak,
            _ => return Err("Failed to get access key".into()),
        };

        // Create the transaction
        let transaction = Transaction {
            signer_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
            nonce: access_key.nonce + 1,
            receiver_id: contract_id.parse()?,
            block_hash: access_key_response.block_hash,
            actions: vec![action],
        };

        // Sign and send the transaction
        let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction: transaction.sign(signer),
        };

        let response = client.call(request).await?;
        
        // Check if the transaction succeeded
        if let Some(outcome) = response.final_execution_outcome {
            match outcome.status {
                near_primitives::views::FinalExecutionStatus::SuccessValue(_) => Ok("success".to_string()),
                _ => Err("Refund transaction failed".into()),
            }
        } else {
            Err("No execution outcome received".into())
        }
    }

    pub async fn get_escrow(
        &self,
        escrow_id: &str,
    ) -> Result<FusionEscrowView, Box<dyn std::error::Error>> {
        let contract_id = self.contract_id.as_ref()
            .ok_or("Contract ID not set")?;

        let client = JsonRpcClient::connect(&self.rpc_url);
        
        // Prepare the view call arguments
        let args = json!({
            "escrow_id": escrow_id,
        });

        let request = methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: QueryRequest::CallFunction {
                account_id: contract_id.parse()?,
                method_name: "get_escrow".to_string(),
                args: serde_json::to_vec(&args)?.into(),
            },
        };

        let response = client.call(request).await?;
        
        match response.kind {
            near_primitives::views::QueryResponseKind::CallResult(result) => {
                let escrow: FusionEscrowView = serde_json::from_slice(&result.result)?;
                Ok(escrow)
            }
            _ => Err("Unexpected query response".into()),
        }
    }

    pub fn hash_secret(secret: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub async fn get_escrow_state(
        &self,
        escrow_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let escrow = self.get_escrow(escrow_id).await?;
        Ok(escrow.state)
    }
}

// For testing on NEAR testnet
pub const NEAR_TESTNET_RPC: &str = "https://rpc.testnet.near.org";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_near_connector_creation() {
        let connector = NearConnector::new(NEAR_TESTNET_RPC);
        assert_eq!(connector.rpc_url, NEAR_TESTNET_RPC);
    }

    #[test]
    fn test_with_contract() {
        let connector = NearConnector::new(NEAR_TESTNET_RPC).with_contract("htlc.testnet");
        assert_eq!(connector.contract_id, Some("htlc.testnet".to_string()));
    }

    #[test]
    fn test_with_account_id() {
        let account_id = "test.near".to_string();
        let connector = NearConnector::new(NEAR_TESTNET_RPC)
            .with_account_id(account_id.clone());
        assert_eq!(connector.account_id(), &account_id);
    }

    #[test]
    fn test_with_private_key() {
        let connector = NearConnector::new(NEAR_TESTNET_RPC)
            .with_account_id("test.near".to_string())
            .with_private_key("ed25519:5JueXZhErbqtSERSQCXVDqwNwz3eXHmB8x8XmJvZZim6ssUg8aQVZjFu1gNbJDnJqZsx7U7kcSCnBpbQTfUnL6Hq");
        assert!(connector.has_private_key());
    }
}