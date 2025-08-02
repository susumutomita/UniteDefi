use crate::htlc::SecretHash;
use anyhow::{anyhow, Result};
use near_crypto::{InMemorySigner, KeyType};
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{AccountId, BlockReference};
use near_primitives::views::QueryRequest;
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

    pub fn with_account(mut self, account_id: &str, _private_key: &str) -> Result<Self> {
        let account_id =
            AccountId::from_str(account_id).map_err(|e| anyhow!("Invalid account ID: {}", e))?;

        // In production, parse the actual key format
        // For now, create a test signer
        let signer =
            InMemorySigner::from_seed(account_id.clone(), KeyType::ED25519, account_id.as_ref());

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
        let _contract_id = self
            .contract_id
            .as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;
        let signer = self
            .signer
            .as_ref()
            .ok_or_else(|| anyhow!("Signer not configured"))?;

        // Prepare the function call arguments
        let _args = json!({
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

        let access_key = self
            .rpc_client
            .call(access_key_query)
            .await
            .map_err(|e| anyhow!("Failed to query access key: {}", e))?;

        let _nonce = match access_key.kind {
            QueryResponseKind::AccessKey(key) => key.nonce + 1,
            _ => return Err(anyhow!("Unexpected response type")),
        };

        // In production, this would create and send a real NEAR transaction
        // For now, return a mock transaction hash
        let mock_tx_hash = [1u8; 32];

        // Extract HTLC ID from transaction hash (simplified)
        let htlc_id = format!("htlc_{}", &hex::encode(&mock_tx_hash[..8]));

        Ok(htlc_id)
    }

    pub async fn claim_htlc(&self, htlc_id: &str, secret: [u8; 32]) -> Result<String> {
        let _contract_id = self
            .contract_id
            .as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;
        let _signer = self
            .signer
            .as_ref()
            .ok_or_else(|| anyhow!("Signer not configured"))?;

        let _args = json!({
            "htlc_id": htlc_id,
            "secret": hex::encode(secret),
        });

        // Similar transaction creation as create_htlc
        // Simplified for brevity
        let tx_hash = format!("0x{}", hex::encode(&secret[..16]));
        Ok(tx_hash)
    }

    pub async fn refund_htlc(&self, htlc_id: &str) -> Result<String> {
        let _contract_id = self
            .contract_id
            .as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;
        let _signer = self
            .signer
            .as_ref()
            .ok_or_else(|| anyhow!("Signer not configured"))?;

        let _args = json!({
            "htlc_id": htlc_id,
        });

        // Similar transaction creation
        let tx_hash = format!("refund_{}", htlc_id);
        Ok(tx_hash)
    }

    pub async fn get_htlc_status(&self, _htlc_id: &str) -> Result<String> {
        let _contract_id = self
            .contract_id
            .as_ref()
            .ok_or_else(|| anyhow!("Contract ID not set"))?;

        // In production, this would query the actual HTLC status from NEAR
        // For now, return mock status
        Ok("active".to_string())
    }
}
