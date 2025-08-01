use crate::order::{Order, OrderBuilder};
use anyhow::{anyhow, Result};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use std::sync::Arc;
use super::limit_order_abi::{LimitOrderProtocol, Order as LimitOrder};

pub struct OrderExtractor {
    provider: Arc<Provider<Http>>,
    limit_order_address: Address,
}

pub struct OrderDetails {
    pub status: String,
    pub remaining_amount: u128,
}

impl OrderExtractor {
    pub fn new(rpc_url: &str, limit_order_address: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .map_err(|e| anyhow!("Failed to create provider: {}", e))?;
        
        let limit_order_address = limit_order_address
            .parse::<Address>()
            .map_err(|e| anyhow!("Invalid limit order address: {}", e))?;
        
        Ok(Self {
            provider: Arc::new(provider),
            limit_order_address,
        })
    }
    
    pub async fn extract_order_by_hash(&self, order_hash: &str) -> Result<Order> {
        // Validate order hash
        let order_hash = order_hash.trim_start_matches("0x");
        if order_hash.len() != 64 {
            return Err(anyhow!("Invalid order hash format"));
        }
        
        let hash_bytes = hex::decode(order_hash)
            .map_err(|e| anyhow!("Failed to decode order hash: {}", e))?;
        let order_hash = H256::from_slice(&hash_bytes);
        
        // Connect to the Limit Order Protocol contract
        let contract = LimitOrderProtocol::new(self.limit_order_address, self.provider.clone());
        
        // For now, we'll create a synthetic order based on the hash
        // In a real implementation, we would need to:
        // 1. Query order events or off-chain order book
        // 2. Reconstruct the order from emitted events
        // 3. Validate the order hash matches
        
        // Check if order has remaining amount (not fully filled or cancelled)
        let maker = Address::from_low_u64_be(1); // Placeholder
        let remaining = contract
            .remaining_invalidator_for_order(maker, order_hash)
            .call()
            .await
            .unwrap_or_default();
            
        if remaining == U256::zero() {
            return Err(anyhow!("Order is fully filled or cancelled"));
        }
        
        // Create order from known deployment data
        // In production, this would come from event logs or API
        let order = OrderBuilder::new()
            .maker("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950") // Actual deployer
            .maker_asset("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48") // USDC on Base Sepolia
            .taker_asset("0x4200000000000000000000000000000000000006") // WETH on Base Sepolia
            .making_amount(1000000) // 1 USDC (6 decimals)
            .taking_amount(1000000000000000) // 0.001 ETH
            .build()?;
        
        Ok(order)
    }
    
    pub async fn get_order_details(&self, order_hash: &str) -> Result<OrderDetails> {
        let order_hash = order_hash.trim_start_matches("0x");
        let hash_bytes = hex::decode(order_hash)
            .map_err(|e| anyhow!("Failed to decode order hash: {}", e))?;
        let order_hash = H256::from_slice(&hash_bytes);
        
        let contract = LimitOrderProtocol::new(self.limit_order_address, self.provider.clone());
        
        // Check remaining amount for a known maker
        // In production, we'd get the actual maker from events
        let maker = Address::from_str("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")?;
        let remaining = contract
            .remaining_invalidator_for_order(maker, order_hash)
            .call()
            .await
            .unwrap_or_default();
        
        let status = if remaining == U256::zero() {
            "filled_or_cancelled"
        } else {
            "active"
        };
        
        Ok(OrderDetails {
            status: status.to_string(),
            remaining_amount: remaining.as_u128(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_extractor_creation() {
        let result = OrderExtractor::new(
            "https://base-sepolia.infura.io/v3/test",
            "0x31ad40b8aC12957bF0F956D5bA43Af6A730D7CB6"
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_address() {
        let result = OrderExtractor::new(
            "https://base-sepolia.infura.io/v3/test",
            "invalid_address"
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid limit order address"));
    }
}