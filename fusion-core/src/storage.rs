use crate::htlc::{Htlc, SecretHash};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredHtlc {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub secret_hash: SecretHash,
    pub timeout_seconds: u64,
    pub created_at: SystemTime,
    pub state: String,
    pub claimed_at: Option<SystemTime>,
}

pub struct HtlcStorage {
    storage_path: PathBuf,
    htlcs: Arc<Mutex<HashMap<String, StoredHtlc>>>,
}

impl HtlcStorage {
    pub fn new() -> Result<Self> {
        let storage_path = dirs::data_dir()
            .context("Failed to get data directory")?
            .join("fusion-cli")
            .join("htlcs.json");
        
        // Create directory if it doesn't exist
        if let Some(parent) = storage_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut storage = Self {
            storage_path,
            htlcs: Arc::new(Mutex::new(HashMap::new())),
        };
        
        storage.load()?;
        Ok(storage)
    }
    
    fn load(&mut self) -> Result<()> {
        if self.storage_path.exists() {
            let data = fs::read_to_string(&self.storage_path)?;
            let htlcs: HashMap<String, StoredHtlc> = serde_json::from_str(&data)?;
            *self.htlcs.lock().unwrap() = htlcs;
        }
        Ok(())
    }
    
    fn save(&self) -> Result<()> {
        let htlcs = self.htlcs.lock().unwrap();
        let data = serde_json::to_string_pretty(&*htlcs)?;
        fs::write(&self.storage_path, data)?;
        Ok(())
    }
    
    pub fn store_htlc(&self, id: String, htlc: &Htlc, timeout_seconds: u64) -> Result<()> {
        let stored_htlc = StoredHtlc {
            id: id.clone(),
            sender: htlc.sender().to_string(),
            recipient: htlc.recipient().to_string(),
            amount: htlc.amount(),
            secret_hash: *htlc.secret_hash(),
            timeout_seconds,
            created_at: SystemTime::now(),
            state: format!("{:?}", htlc.state()),
            claimed_at: None,
        };
        
        self.htlcs.lock().unwrap().insert(id, stored_htlc);
        self.save()?;
        Ok(())
    }
    
    pub fn get_htlc(&self, id: &str) -> Result<Option<StoredHtlc>> {
        let htlcs = self.htlcs.lock().unwrap();
        Ok(htlcs.get(id).cloned())
    }
    
    pub fn update_htlc_state(&self, id: &str, state: &str, claimed_at: Option<SystemTime>) -> Result<()> {
        let mut htlcs = self.htlcs.lock().unwrap();
        if let Some(htlc) = htlcs.get_mut(id) {
            htlc.state = state.to_string();
            htlc.claimed_at = claimed_at;
        }
        drop(htlcs);
        self.save()?;
        Ok(())
    }
}