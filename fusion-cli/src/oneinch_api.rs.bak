use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const ONEINCH_API_BASE: &str = "https://api.1inch.dev";

#[derive(Debug, Serialize, Deserialize)]
pub struct OneInchOrder {
    pub order_hash: String,
    pub signature: String,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderQuote {
    pub from_token_address: String,
    pub to_token_address: String,
    pub amount: String,
    pub from_address: String,
    pub slippage: f64,
    pub disable_estimate: bool,
    pub allow_partial_fill: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSubmitRequest {
    pub order_hash: String,
    pub signature: String,
    pub order_data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSubmitResponse {
    pub order_hash: String,
    pub status: String,
    pub created_at: u64,
}

pub struct OneInchClient {
    client: Client,
    api_key: Option<String>,
    network_id: u64,
}

impl OneInchClient {
    pub fn new(network_id: u64, api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            network_id,
        }
    }

    /// Get a quote for a limit order
    #[allow(dead_code)]
    pub async fn get_quote(&self, quote_request: &OrderQuote) -> Result<Value> {
        let url = format!(
            "{}/orderbook/v3.0/{}/quote",
            ONEINCH_API_BASE, self.network_id
        );

        let mut request = self.client.get(&url).query(&quote_request);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("1inch API error: {}", error_text));
        }

        let quote: Value = response.json().await?;
        Ok(quote)
    }

    /// Submit a signed order to 1inch
    pub async fn submit_order(&self, order_data: &Value) -> Result<OrderSubmitResponse> {
        let url = format!(
            "{}/orderbook/v3.0/{}/order",
            ONEINCH_API_BASE, self.network_id
        );

        let mut request = self.client.post(&url).json(&order_data);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to submit order to 1inch: {}", error_text));
        }

        let submit_response: OrderSubmitResponse = response.json().await?;
        Ok(submit_response)
    }

    /// Get order status
    #[allow(dead_code)]
    pub async fn get_order_status(&self, order_hash: &str) -> Result<Value> {
        let url = format!(
            "{}/orderbook/v3.0/{}/order/{}/status",
            ONEINCH_API_BASE, self.network_id, order_hash
        );

        let mut request = self.client.get(&url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to get order status: {}", error_text));
        }

        let status: Value = response.json().await?;
        Ok(status)
    }

    /// Get all orders for an address
    #[allow(dead_code)]
    pub async fn get_orders(&self, address: &str, limit: Option<u32>) -> Result<Vec<Value>> {
        let url = format!(
            "{}/orderbook/v3.0/{}/address/{}/orders",
            ONEINCH_API_BASE, self.network_id, address
        );

        let mut request = self.client.get(&url);

        if let Some(limit) = limit {
            request = request.query(&[("limit", limit.to_string())]);
        }

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to get orders: {}", error_text));
        }

        let orders: Vec<Value> = response.json().await?;
        Ok(orders)
    }
}

/// Convert our order format to 1inch API format
pub fn convert_to_oneinch_format(order_data: &Value) -> Result<Value> {
    let order = order_data["order"]
        .as_object()
        .ok_or_else(|| anyhow!("Invalid order data"))?;

    let signature = order_data["signature"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing signature"))?;

    // 1inch expects a specific format for limit orders
    let oneinch_order = json!({
        "orderHash": order_data["eip712_hash"],
        "signature": signature,
        "data": {
            "makerAsset": order["makerAsset"],
            "takerAsset": order["takerAsset"],
            "maker": order["maker"],
            "receiver": order["receiver"],
            "allowedSender": order["allowedSender"],
            "makingAmount": order["makingAmount"],
            "takingAmount": order["takingAmount"],
            "salt": order["salt"],
            "offsets": order["offsets"],
            "interactions": order["interactions"],
        }
    });

    Ok(oneinch_order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_oneinch_format() {
        let order_data = json!({
            "order": {
                "salt": "0x1234",
                "makerAsset": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
                "takerAsset": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                "maker": "0x1234567890123456789012345678901234567890",
                "receiver": "0x0000000000000000000000000000000000000000",
                "allowedSender": "0x0000000000000000000000000000000000000000",
                "makingAmount": "1000000",
                "takingAmount": "500000000000000000",
                "offsets": "0",
                "interactions": "0x"
            },
            "eip712_hash": "0xabcdef",
            "signature": "0x123456"
        });

        let result = convert_to_oneinch_format(&order_data).unwrap();
        assert_eq!(result["orderHash"], "0xabcdef");
        assert_eq!(result["signature"], "0x123456");
        assert_eq!(
            result["data"]["maker"],
            "0x1234567890123456789012345678901234567890"
        );
    }
}
