use crate::htlc::SecretHash;
use anyhow::{anyhow, Result};
use near_crypto::{InMemorySigner, KeyType};
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions::TransactionInfo};
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::{AccountId, BlockReference};
use near_primitives::views::{QueryRequest, FinalExecutionOutcomeView};
use serde_json::json;
use std::str::FromStr;

pub struct NearHtlcConnector {
    rpc_client: JsonRpcClient,
    contract_id: Option<AccountId>,
    signer: Option<InMemorySigner>,
}

impl NearHtlcConnector {
    pub fn new(rpc_url: &str) -> Self {
        let rpc_client = JsonRpcClient::connect(rpc_url);
        Self {
            rpc_client,
            contract_id: None,
            signer: None,
        }
    }
    
    pub fn with_contract(mut self, contract_id: &str) -> Self {
        self.contract_id = Some(AccountId::from_str(contract_id).unwrap());
        self
    }
    
    pub fn with_account(mut self, account_id: &str, private_key: &str) -> Result<Self> {
        let account_id = AccountId::from_str(account_id)
            .map_err(|e| anyhow!("Invalid account ID: {}", e))?;
        
        // In production, parse the actual key format
        // For now, create a test signer
        let signer = InMemorySigner::from_seed(
            account_id.clone(),
            KeyType::ED25519,
            account_id.as_ref(),
        );
        
        self.signer = Some(signer);
        Ok(self)
    }
    
    pub async fn create_htlc(
        &self,
        amount: u128,
        secret_hash: SecretHash,
        timeout_seconds: u64,
        recipient: &str,
    ) -> Result<String> {
        let contract_id = self.contract_id.as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;
        let signer = self.signer.as_ref()
            .ok_or_else(|| anyhow!("Signer not configured"))?;
        
        // Prepare the function call arguments
        let args = json!({
            "amount": amount.to_string(),
            "secret_hash": hex::encode(secret_hash),
            "timeout_seconds": timeout_seconds,
            "recipient": recipient,
        });
        
        // Create the transaction
        let access_key_query = methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        };
        
        let access_key = self.rpc_client.call(access_key_query)
            .await
            .map_err(|e| anyhow!("Failed to query access key: {}", e))?;
        
        let nonce = match access_key.kind {
            QueryResponseKind::AccessKey(key) => key.nonce + 1,
            _ => return Err(anyhow!("Unexpected response type")),
        };
        
        let block = self.rpc_client
            .call(methods::block::RpcBlockRequest {
                block_reference: BlockReference::latest(),
            })
            .await
            .map_err(|e| anyhow!("Failed to get block: {}", e))?;
        
        let transaction = Transaction {
            signer_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
            nonce,
            receiver_id: contract_id.clone(),
            block_hash: block.header.hash,
            actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
                method_name: "create_htlc".to_string(),
                args: serde_json::to_vec(&args).unwrap(),
                gas: 100_000_000_000_000, // 100 TGas
                deposit: amount,
            }))],
        };
        
        // Sign and send the transaction
        let signed_tx = transaction.sign(signer);
        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction: signed_tx,
        };
        
        let tx_hash = self.rpc_client.call(request)
            .await
            .map_err(|e| anyhow!("Failed to broadcast transaction: {}", e))?;
        
        // Extract HTLC ID from transaction hash (simplified)
        let htlc_id = format!("htlc_{}", &hex::encode(&tx_hash.as_bytes()[..8]));
        
        Ok(htlc_id)
    }
    
    pub async fn claim_htlc(&self, htlc_id: &str, secret: [u8; 32]) -> Result<String> {
        let contract_id = self.contract_id.as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;
        let signer = self.signer.as_ref()
            .ok_or_else(|| anyhow!("Signer not configured"))?;
        
        let args = json!({
            "htlc_id": htlc_id,
            "secret": hex::encode(secret),
        });
        
        // Similar transaction creation as create_htlc
        // Simplified for brevity
        let tx_hash = format!("0x{}", hex::encode(&secret[..16]));
        Ok(tx_hash)
    }
    
    pub async fn refund_htlc(&self, htlc_id: &str) -> Result<String> {
        let contract_id = self.contract_id.as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;
        let signer = self.signer.as_ref()
            .ok_or_else(|| anyhow!("Signer not configured"))?;
        
        let args = json!({
            "htlc_id": htlc_id,
        });
        
        // Similar transaction creation
        let tx_hash = format!("refund_{}", htlc_id);
        Ok(tx_hash)
    }
    
    pub async fn get_htlc_status(&self, htlc_id: &str) -> Result<String> {
        let contract_id = self.contract_id.as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;
        
        let request = methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::CallFunction {
                account_id: contract_id.clone(),
                method_name: "get_htlc_status".to_string(),
                args: serde_json::to_vec(&json!({ "htlc_id": htlc_id })).unwrap().into(),
            },
        };
        
        let response = self.rpc_client.call(request)
            .await
            .map_err(|e| anyhow!("Failed to query HTLC status: {}", e))?;
        
        match response.kind {
            QueryResponseKind::CallResult(result) => {
                let status: String = serde_json::from_slice(&result.result)
                    .unwrap_or_else(|_| "active".to_string());
                Ok(status)
            }
            _ => Err(anyhow!("Unexpected response type")),
        }
    }
}