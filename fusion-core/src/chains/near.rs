use crate::htlc::SecretHash;

pub mod event_monitor;
pub mod htlc_connector;
pub use htlc_connector::NearHtlcConnector;

pub struct NEARConnector {
    _rpc_url: String,
    contract_id: Option<String>,
    // TODO: Add NEAR-specific fields like account_id, keys, etc.
}

impl NEARConnector {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            _rpc_url: rpc_url.to_string(),
            contract_id: None,
        }
    }

    pub fn with_contract(mut self, contract_id: &str) -> Self {
        self.contract_id = Some(contract_id.to_string());
        self
    }

    // TODO: Implement NEAR-specific methods
    pub async fn create_escrow(
        &self,
        _amount: u128,
        _secret_hash: SecretHash,
        _timeout_seconds: u64,
        _recipient: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder implementation
        Err("NEAR connector not implemented yet".into())
    }

    pub async fn claim_escrow(
        &self,
        _escrow_id: &str,
        _secret: [u8; 32],
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder implementation
        Err("NEAR connector not implemented yet".into())
    }

    pub async fn refund_escrow(
        &self,
        _escrow_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder implementation
        Err("NEAR connector not implemented yet".into())
    }
}

// For testing on NEAR testnet
pub const NEAR_TESTNET_RPC: &str = "https://rpc.testnet.near.org";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_near_connector_creation() {
        let connector = NEARConnector::new(NEAR_TESTNET_RPC);
        assert_eq!(connector._rpc_url, NEAR_TESTNET_RPC);
    }

    #[test]
    fn test_with_contract() {
        let connector = NEARConnector::new(NEAR_TESTNET_RPC).with_contract("htlc.testnet");
        assert_eq!(connector.contract_id, Some("htlc.testnet".to_string()));
    }
}
