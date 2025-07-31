use ethers::{
    contract::{EthEvent, Event},
    core::types::{Address, Filter, H256, U256},
    providers::{Middleware, Provider, StreamExt, Ws},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use crate::chains::ethereum::event_storage::{EventStorage, convert_to_stored_event};

#[derive(Debug, Clone, EthEvent, Deserialize, Serialize)]
#[ethevent(name = "OrderFilled", abi = "OrderFilled(bytes32,uint256)")]
pub struct OrderFilledEvent {
    pub order_hash: H256,
    pub remaining_amount: U256,
}

#[derive(Debug, Clone, EthEvent, Deserialize, Serialize)]
#[ethevent(name = "OrderCancelled", abi = "OrderCancelled(bytes32)")]
pub struct OrderCancelledEvent {
    pub order_hash: H256,
}

#[derive(Debug, Clone)]
pub enum LimitOrderEvent {
    OrderFilled(OrderFilledEvent),
    OrderCancelled(OrderCancelledEvent),
}

pub struct LimitOrderEventMonitor {
    provider: Arc<Provider<Ws>>,
    contract_address: Address,
}

#[derive(Debug)]
pub struct MonitorConfig {
    pub retry_delay: Duration,
    pub max_retries: u32,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            retry_delay: Duration::from_secs(5),
            max_retries: 3,
        }
    }
}

impl LimitOrderEventMonitor {
    pub fn new(provider: Arc<Provider<Ws>>, contract_address: Address) -> Self {
        Self {
            provider,
            contract_address,
        }
    }

    pub async fn start_monitoring(&self) -> Result<mpsc::Receiver<LimitOrderEvent>, Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel::<LimitOrderEvent>(100);

        // Start monitoring both events concurrently
        let tx_filled = tx.clone();
        let tx_cancelled = tx;

        let provider_filled = self.provider.clone();
        let provider_cancelled = self.provider.clone();
        let contract_address = self.contract_address;

        // Monitor OrderFilled events
        tokio::spawn(async move {
            let filter = Filter::new()
                .address(contract_address)
                .event("OrderFilled(bytes32,uint256)");

            if let Ok(mut stream) = provider_filled.subscribe_logs(&filter).await {
                while let Some(log) = stream.next().await {
                    if log.topics.len() >= 1 && log.data.len() >= 32 {
                        let order_hash = H256::from_slice(&log.topics[1].as_bytes());
                        let remaining_amount = U256::from_big_endian(&log.data[..32]);
                        
                        let event = OrderFilledEvent {
                            order_hash,
                            remaining_amount,
                        };
                        
                        let _ = tx_filled.send(LimitOrderEvent::OrderFilled(event)).await;
                    }
                }
            }
        });

        // Monitor OrderCancelled events
        tokio::spawn(async move {
            let filter = Filter::new()
                .address(contract_address)
                .event("OrderCancelled(bytes32)");

            if let Ok(mut stream) = provider_cancelled.subscribe_logs(&filter).await {
                while let Some(log) = stream.next().await {
                    if log.topics.len() >= 2 {
                        let order_hash = H256::from_slice(&log.topics[1].as_bytes());
                        
                        let event = OrderCancelledEvent { order_hash };
                        
                        let _ = tx_cancelled.send(LimitOrderEvent::OrderCancelled(event)).await;
                    }
                }
            }
        });

        Ok(rx)
    }

    pub async fn monitor_with_storage(
        &self,
        storage: Arc<dyn EventStorage>,
        config: MonitorConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut retries = 0;
        
        loop {
            match self.start_monitoring().await {
                Ok(mut rx) => {
                    retries = 0; // Reset retries on successful connection
                    
                    while let Some(event) = rx.recv().await {
                        // Get current block information (mock for now)
                        let block_number = 0u64; // TODO: Get from provider
                        let timestamp = chrono::Utc::now().timestamp() as u64;
                        
                        let stored_event = convert_to_stored_event(event, block_number, timestamp);
                        
                        if let Err(e) = storage.save_event(stored_event).await {
                            eprintln!("Failed to save event: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket connection failed: {}", e);
                    retries += 1;
                    
                    if retries >= config.max_retries {
                        return Err("Max retries reached".into());
                    }
                    
                    sleep(config.retry_delay).await;
                }
            }
        }
    }

    pub async fn monitor_order_filled_events(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = Filter::new()
            .address(self.contract_address)
            .event("OrderFilled(bytes32,uint256)");

        let mut stream = self.provider.subscribe_logs(&filter).await?;
        
        while let Some(log) = stream.next().await {
            if log.topics.len() >= 1 && log.data.len() >= 32 {
                let order_hash = H256::from_slice(&log.topics[1].as_bytes());
                let remaining_amount = U256::from_big_endian(&log.data[..32]);
                
                let event = OrderFilledEvent {
                    order_hash,
                    remaining_amount,
                };
                
                // Here we would save to database
                println!("OrderFilled event: {:?}", event);
            }
        }
        
        Ok(())
    }

    pub async fn monitor_order_cancelled_events(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = Filter::new()
            .address(self.contract_address)
            .event("OrderCancelled(bytes32)");

        let mut stream = self.provider.subscribe_logs(&filter).await?;
        
        while let Some(log) = stream.next().await {
            if log.topics.len() >= 2 {
                let order_hash = H256::from_slice(&log.topics[1].as_bytes());
                
                let event = OrderCancelledEvent { order_hash };
                
                // Here we would save to database
                println!("OrderCancelled event: {:?}", event);
            }
        }
        
        Ok(())
    }
}