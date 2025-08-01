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

// Order Management Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Active,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredOrder {
    pub id: String,
    pub maker: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub making_amount: u128,
    pub taking_amount: u128,
    pub status: OrderStatus,
    pub created_at: SystemTime,
    pub chain: String,
    pub order_hash: String,
}

#[derive(Clone)]
pub struct OrderStorage {
    orders: Arc<Mutex<HashMap<String, StoredOrder>>>,
}

impl OrderStorage {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub fn store(&self, order_id: String, stored_order: StoredOrder) -> Result<()> {
        let mut storage = self
            .orders
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        storage.insert(order_id, stored_order);
        Ok(())
    }

    pub fn get(&self, order_id: &str) -> Result<StoredOrder> {
        let storage = self
            .orders
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        storage
            .get(order_id)
            .cloned()
            .ok_or_else(|| anyhow!("Order not found: {}", order_id))
    }

    pub fn update_status(&self, order_id: &str, status: OrderStatus) -> Result<()> {
        let mut storage = self
            .orders
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        if let Some(stored) = storage.get_mut(order_id) {
            stored.status = status;
            Ok(())
        } else {
            Err(anyhow!("Order not found: {}", order_id))
        }
    }

    pub fn get_orders_by_chain(&self, chain: &str) -> Result<Vec<StoredOrder>> {
        let storage = self
            .orders
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;

        let orders: Vec<StoredOrder> = storage
            .values()
            .filter(|order| order.chain == chain)
            .cloned()
            .collect();

        Ok(orders)
    }
}

impl Default for OrderStorage {
    fn default() -> Self {
        Self::new()
    }
}
