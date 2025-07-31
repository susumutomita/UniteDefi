use crate::chains::ethereum::events::{LimitOrderEvent, OrderCancelledEvent, OrderFilledEvent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub event_type: String,
    pub order_hash: String,
    pub block_number: u64,
    pub timestamp: u64,
    pub data: EventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    OrderFilled {
        remaining_amount: String,
    },
    OrderCancelled,
}

#[async_trait]
pub trait EventStorage: Send + Sync {
    async fn save_event(&self, event: StoredEvent) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_events_by_order_hash(&self, order_hash: &str) -> Result<Vec<StoredEvent>, Box<dyn std::error::Error>>;
    async fn get_all_events(&self) -> Result<Vec<StoredEvent>, Box<dyn std::error::Error>>;
}

// In-memory storage for testing
pub struct InMemoryEventStorage {
    events: Arc<RwLock<Vec<StoredEvent>>>,
    by_order_hash: Arc<RwLock<HashMap<String, Vec<usize>>>>,
}

impl InMemoryEventStorage {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            by_order_hash: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EventStorage for InMemoryEventStorage {
    async fn save_event(&self, event: StoredEvent) -> Result<(), Box<dyn std::error::Error>> {
        let mut events = self.events.write().await;
        let mut by_order_hash = self.by_order_hash.write().await;
        
        let index = events.len();
        let order_hash = event.order_hash.clone();
        
        events.push(event);
        
        by_order_hash
            .entry(order_hash)
            .or_insert_with(Vec::new)
            .push(index);
        
        Ok(())
    }

    async fn get_events_by_order_hash(&self, order_hash: &str) -> Result<Vec<StoredEvent>, Box<dyn std::error::Error>> {
        let events = self.events.read().await;
        let by_order_hash = self.by_order_hash.read().await;
        
        if let Some(indices) = by_order_hash.get(order_hash) {
            let result = indices
                .iter()
                .filter_map(|&idx| events.get(idx).cloned())
                .collect();
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    async fn get_all_events(&self) -> Result<Vec<StoredEvent>, Box<dyn std::error::Error>> {
        let events = self.events.read().await;
        Ok(events.clone())
    }
}

// Helper function to convert LimitOrderEvent to StoredEvent
pub fn convert_to_stored_event(event: LimitOrderEvent, block_number: u64, timestamp: u64) -> StoredEvent {
    match event {
        LimitOrderEvent::OrderFilled(e) => StoredEvent {
            event_type: "OrderFilled".to_string(),
            order_hash: format!("{:?}", e.order_hash),
            block_number,
            timestamp,
            data: EventData::OrderFilled {
                remaining_amount: e.remaining_amount.to_string(),
            },
        },
        LimitOrderEvent::OrderCancelled(e) => StoredEvent {
            event_type: "OrderCancelled".to_string(),
            order_hash: format!("{:?}", e.order_hash),
            block_number,
            timestamp,
            data: EventData::OrderCancelled,
        },
    }
}