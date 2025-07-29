use fusion_core::htlc::{SecretHash, HtlcState};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredHtlc {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub secret_hash: SecretHash,
    pub timeout: Duration,
    pub created_at: SystemTime,
    pub state: String,
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
        let mut storage = self.htlcs.lock().map_err(|e| anyhow!("Lock error: {}", e))?;
        storage.insert(htlc_id, stored_htlc);
        Ok(())
    }

    pub fn get(&self, htlc_id: &str) -> Result<StoredHtlc> {
        let storage = self.htlcs.lock().map_err(|e| anyhow!("Lock error: {}", e))?;
        storage.get(htlc_id)
            .cloned()
            .ok_or_else(|| anyhow!("HTLC not found: {}", htlc_id))
    }

    pub fn update_state(&self, htlc_id: &str, state: String) -> Result<()> {
        let mut storage = self.htlcs.lock().map_err(|e| anyhow!("Lock error: {}", e))?;
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