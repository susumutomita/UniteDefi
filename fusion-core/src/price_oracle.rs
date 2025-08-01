use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 価格データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    /// 価格（USD単位）
    pub price: f64,
    /// タイムスタンプ（Unix時間）
    pub timestamp: u64,
    /// 信頼度（0.0 - 1.0）
    pub confidence: f64,
}

/// 価格オラクルのトレイト
#[async_trait]
pub trait PriceOracle: Send + Sync {
    /// トークンの現在価格を取得
    async fn get_price(&self, token_symbol: &str) -> Result<PriceData>;

    /// 複数トークンの価格を一括取得
    async fn get_prices(&self, token_symbols: &[&str]) -> Result<HashMap<String, PriceData>>;

    /// サポートされているトークンのリストを取得
    async fn supported_tokens(&self) -> Result<Vec<String>>;
}

/// モック価格オラクル（テスト用）
pub struct MockPriceOracle {
    prices: HashMap<String, PriceData>,
}

impl Default for MockPriceOracle {
    fn default() -> Self {
        let mut prices = HashMap::new();

        // デフォルトの価格を設定
        prices.insert(
            "NEAR".to_string(),
            PriceData {
                price: 5.0,
                timestamp: 1700000000,
                confidence: 0.99,
            },
        );

        prices.insert(
            "ETH".to_string(),
            PriceData {
                price: 2000.0,
                timestamp: 1700000000,
                confidence: 0.99,
            },
        );

        prices.insert(
            "USDC".to_string(),
            PriceData {
                price: 1.0,
                timestamp: 1700000000,
                confidence: 1.0,
            },
        );

        Self { prices }
    }
}

impl MockPriceOracle {
    pub fn new() -> Self {
        Self::default()
    }

    /// 価格を更新（テスト用）
    pub fn set_price(&mut self, token: &str, price: f64) {
        if let Some(data) = self.prices.get_mut(token) {
            data.price = price;
            data.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }
}

#[async_trait]
impl PriceOracle for MockPriceOracle {
    async fn get_price(&self, token_symbol: &str) -> Result<PriceData> {
        self.prices
            .get(token_symbol)
            .cloned()
            .ok_or_else(|| anyhow!("Token {} not supported", token_symbol))
    }

    async fn get_prices(&self, token_symbols: &[&str]) -> Result<HashMap<String, PriceData>> {
        let mut result = HashMap::new();

        for symbol in token_symbols {
            if let Some(price) = self.prices.get(*symbol) {
                result.insert(symbol.to_string(), price.clone());
            }
        }

        Ok(result)
    }

    async fn supported_tokens(&self) -> Result<Vec<String>> {
        Ok(self.prices.keys().cloned().collect())
    }
}

/// Chainlink価格オラクル（将来の実装用）
pub struct ChainlinkOracle {
    // TODO: Chainlinkのコントラクトアドレスなどを保持
}

impl ChainlinkOracle {
    pub fn new(_network: &str) -> Self {
        Self {}
    }
}

#[async_trait]
impl PriceOracle for ChainlinkOracle {
    async fn get_price(&self, _token_symbol: &str) -> Result<PriceData> {
        // TODO: Chainlinkから実際の価格を取得
        Err(anyhow!("Chainlink oracle not implemented yet"))
    }

    async fn get_prices(&self, _token_symbols: &[&str]) -> Result<HashMap<String, PriceData>> {
        Err(anyhow!("Chainlink oracle not implemented yet"))
    }

    async fn supported_tokens(&self) -> Result<Vec<String>> {
        Err(anyhow!("Chainlink oracle not implemented yet"))
    }
}

/// 価格変換ユーティリティ
pub struct PriceConverter<O: PriceOracle> {
    oracle: O,
}

impl<O: PriceOracle> PriceConverter<O> {
    pub fn new(oracle: O) -> Self {
        Self { oracle }
    }

    /// トークンAからトークンBへの変換レートを計算
    pub async fn get_conversion_rate(&self, from_token: &str, to_token: &str) -> Result<f64> {
        let from_price = self.oracle.get_price(from_token).await?;
        let to_price = self.oracle.get_price(to_token).await?;

        Ok(from_price.price / to_price.price)
    }

    /// 金額を変換
    pub async fn convert_amount(
        &self,
        amount: u128,
        from_token: &str,
        from_decimals: u8,
        to_token: &str,
        to_decimals: u8,
    ) -> Result<u128> {
        let rate = self.get_conversion_rate(from_token, to_token).await?;

        // デシマルを考慮して変換
        let from_units = amount as f64 / 10f64.powi(from_decimals as i32);
        let to_units = from_units * rate;
        let to_amount = (to_units * 10f64.powi(to_decimals as i32)) as u128;

        Ok(to_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_oracle() {
        let oracle = MockPriceOracle::new();

        // NEARの価格を取得
        let near_price = oracle.get_price("NEAR").await.unwrap();
        assert_eq!(near_price.price, 5.0);

        // サポートされているトークンを確認
        let tokens = oracle.supported_tokens().await.unwrap();
        assert!(tokens.contains(&"NEAR".to_string()));
        assert!(tokens.contains(&"ETH".to_string()));
        assert!(tokens.contains(&"USDC".to_string()));
    }

    #[tokio::test]
    async fn test_price_converter() {
        let oracle = MockPriceOracle::new();
        let converter = PriceConverter::new(oracle);

        // NEAR -> ETH の変換レートを計算
        let rate = converter.get_conversion_rate("NEAR", "ETH").await.unwrap();
        assert_eq!(rate, 5.0 / 2000.0); // 0.0025

        // 1 NEAR を ETH に変換
        let near_amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR (24 decimals)
        let eth_amount = converter
            .convert_amount(near_amount, "NEAR", 24, "ETH", 18)
            .await
            .unwrap();

        // 1 NEAR * 0.0025 = 0.0025 ETH = 2_500_000_000_000_000 wei
        assert_eq!(eth_amount, 2_500_000_000_000_000);
    }

    #[tokio::test]
    async fn test_batch_price_fetch() {
        let oracle = MockPriceOracle::new();

        let prices = oracle.get_prices(&["NEAR", "ETH"]).await.unwrap();
        assert_eq!(prices.len(), 2);
        assert_eq!(prices.get("NEAR").unwrap().price, 5.0);
        assert_eq!(prices.get("ETH").unwrap().price, 2000.0);
    }

    #[tokio::test]
    async fn test_price_update() {
        let mut oracle = MockPriceOracle::new();

        // 価格を更新
        oracle.set_price("NEAR", 6.0);

        let near_price = oracle.get_price("NEAR").await.unwrap();
        assert_eq!(near_price.price, 6.0);
    }

    #[tokio::test]
    async fn test_unsupported_token() {
        let oracle = MockPriceOracle::new();

        let result = oracle.get_price("UNKNOWN").await;
        assert!(result.is_err());
    }
}
