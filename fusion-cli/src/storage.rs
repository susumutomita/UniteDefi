use anyhow::{anyhow, Result};
use fusion_core::htlc::{HtlcState, SecretHash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredHtlc {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub secret_hash: SecretHash,
    pub timeout: Duration,
    pub created_at: SystemTime,
    pub state: HtlcState,
    pub secret: Option<Vec<u8>>,
}

#[derive(Clone)]
pub struct HtlcStorage {
    htlcs: Arc<Mutex<HashMap<String, StoredHtlc>>>,
}

impl HtlcStorage {
    pub fn new() -> Self {
        Self {
            htlcs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn store(&self, htlc_id: String, stored_htlc: StoredHtlc) -> Result<()> {
        let mut storage = self
            .htlcs
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        storage.insert(htlc_id, stored_htlc);
        Ok(())
    }

    pub fn get(&self, htlc_id: &str) -> Result<StoredHtlc> {
        let storage = self
            .htlcs
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        storage
            .get(htlc_id)
            .cloned()
            .ok_or_else(|| anyhow!("HTLC not found: {}", htlc_id))
    }

    pub fn update_state(&self, htlc_id: &str, state: HtlcState) -> Result<()> {
        let mut storage = self
            .htlcs
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        if let Some(stored) = storage.get_mut(htlc_id) {
            stored.state = state;
            Ok(())
        } else {
            Err(anyhow!("HTLC not found: {}", htlc_id))
        }
    }
}

impl Default for HtlcStorage {
    fn default() -> Self {
        Self::new()
    }
}
