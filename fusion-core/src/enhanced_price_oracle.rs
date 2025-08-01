//! 拡張価格オラクル統合
//!
//! 複数の価格ソースを統合し、信頼性の高い価格情報を提供します。

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::price_oracle::{PriceData, PriceOracle};

/// 価格ソース
#[derive(Debug, Clone, PartialEq)]
pub enum PriceSource {
    Chainlink,
    Pyth,
    Band,
    UniswapV3TWAP,
    Custom(String),
}

/// 拡張価格データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedPriceData {
    /// 基本価格データ
    pub base: PriceData,
    /// 価格ソース
    pub source: String,
    /// 24時間変動率
    pub change_24h: f64,
    /// ボラティリティ（標準偏差）
    pub volatility: f64,
    /// 流動性深度（USD）
    pub liquidity_depth: Option<f64>,
}

/// 価格集約戦略
#[derive(Debug, Clone)]
pub enum AggregationStrategy {
    /// 中央値を使用
    Median,
    /// 加重平均（信頼度による）
    WeightedAverage,
    /// 最も信頼性の高いソースを使用
    MostTrusted,
}

/// 拡張価格オラクル
pub struct EnhancedPriceOracle {
    /// 価格ソース
    sources: Vec<Box<dyn PriceOracle>>,
    /// ソース名
    source_names: Vec<String>,
    /// 集約戦略
    aggregation_strategy: AggregationStrategy,
    /// キャッシュ
    cache: Arc<RwLock<HashMap<String, EnhancedPriceData>>>,
    /// キャッシュ有効期間（秒）
    cache_ttl: u64,
}

impl EnhancedPriceOracle {
    /// 新しい拡張オラクルを作成
    pub fn new(aggregation_strategy: AggregationStrategy, cache_ttl: u64) -> Self {
        Self {
            sources: Vec::new(),
            source_names: Vec::new(),
            aggregation_strategy,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl,
        }
    }

    /// 価格ソースを追加
    pub fn add_source(&mut self, name: String, source: Box<dyn PriceOracle>) {
        self.source_names.push(name);
        self.sources.push(source);
    }

    /// 複数ソースから価格を取得し集約
    async fn aggregate_price(&self, token_symbol: &str) -> Result<EnhancedPriceData> {
        let mut prices = Vec::new();

        // 各ソースから価格を取得
        for (i, source) in self.sources.iter().enumerate() {
            match source.get_price(token_symbol).await {
                Ok(price_data) => {
                    prices.push(EnhancedPriceData {
                        base: price_data,
                        source: self.source_names[i].clone(),
                        change_24h: 0.0, // TODO: 実装
                        volatility: 0.0, // TODO: 実装
                        liquidity_depth: None,
                    });
                }
                Err(_) => {
                    // エラーはログに記録するが、他のソースを試す
                    continue;
                }
            }
        }

        if prices.is_empty() {
            return Err(anyhow!("No price data available for {}", token_symbol));
        }

        // 集約戦略に基づいて価格を決定
        let aggregated_price = match &self.aggregation_strategy {
            AggregationStrategy::Median => self.calculate_median(&prices),
            AggregationStrategy::WeightedAverage => self.calculate_weighted_average(&prices),
            AggregationStrategy::MostTrusted => self.select_most_trusted(&prices),
        };

        Ok(EnhancedPriceData {
            base: aggregated_price,
            source: "Aggregated".to_string(),
            change_24h: 0.0,
            volatility: self.calculate_volatility(&prices),
            liquidity_depth: None,
        })
    }

    /// 中央値を計算
    fn calculate_median(&self, prices: &[EnhancedPriceData]) -> PriceData {
        let mut values: Vec<f64> = prices.iter().map(|p| p.base.price).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median_price = if values.len() % 2 == 0 {
            (values[values.len() / 2 - 1] + values[values.len() / 2]) / 2.0
        } else {
            values[values.len() / 2]
        };

        PriceData {
            price: median_price,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            confidence: self.calculate_aggregated_confidence(prices),
        }
    }

    /// 加重平均を計算
    fn calculate_weighted_average(&self, prices: &[EnhancedPriceData]) -> PriceData {
        let total_weight: f64 = prices.iter().map(|p| p.base.confidence).sum();
        let weighted_sum: f64 = prices
            .iter()
            .map(|p| p.base.price * p.base.confidence)
            .sum();

        PriceData {
            price: weighted_sum / total_weight,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            confidence: self.calculate_aggregated_confidence(prices),
        }
    }

    /// 最も信頼性の高いソースを選択
    fn select_most_trusted(&self, prices: &[EnhancedPriceData]) -> PriceData {
        prices
            .iter()
            .max_by(|a, b| a.base.confidence.partial_cmp(&b.base.confidence).unwrap())
            .map(|p| p.base.clone())
            .unwrap()
    }

    /// ボラティリティを計算
    fn calculate_volatility(&self, prices: &[EnhancedPriceData]) -> f64 {
        if prices.len() < 2 {
            return 0.0;
        }

        let mean: f64 = prices.iter().map(|p| p.base.price).sum::<f64>() / prices.len() as f64;
        let variance: f64 = prices
            .iter()
            .map(|p| (p.base.price - mean).powi(2))
            .sum::<f64>()
            / prices.len() as f64;

        variance.sqrt()
    }

    /// 集約された信頼度を計算
    fn calculate_aggregated_confidence(&self, prices: &[EnhancedPriceData]) -> f64 {
        if prices.is_empty() {
            return 0.0;
        }

        let avg_confidence: f64 =
            prices.iter().map(|p| p.base.confidence).sum::<f64>() / prices.len() as f64;

        // 複数ソースがある場合、信頼度を上げる
        let source_multiplier = (1.0 + (prices.len() as f64 - 1.0) * 0.1).min(1.2);

        (avg_confidence * source_multiplier).min(1.0)
    }

    /// キャッシュから価格を取得
    fn get_from_cache(&self, token_symbol: &str) -> Option<EnhancedPriceData> {
        let cache = self.cache.read().unwrap();
        if let Some(cached_data) = cache.get(token_symbol) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if now - cached_data.base.timestamp < self.cache_ttl {
                return Some(cached_data.clone());
            }
        }
        None
    }

    /// キャッシュに価格を保存
    fn save_to_cache(&self, token_symbol: &str, data: EnhancedPriceData) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(token_symbol.to_string(), data);
    }
}

#[async_trait]
impl PriceOracle for EnhancedPriceOracle {
    async fn get_price(&self, token_symbol: &str) -> Result<PriceData> {
        // キャッシュチェック
        if let Some(cached) = self.get_from_cache(token_symbol) {
            return Ok(cached.base);
        }

        // 新しい価格を取得
        let enhanced_data = self.aggregate_price(token_symbol).await?;
        let price_data = enhanced_data.base.clone();

        // キャッシュに保存
        self.save_to_cache(token_symbol, enhanced_data);

        Ok(price_data)
    }

    async fn get_prices(&self, token_symbols: &[&str]) -> Result<HashMap<String, PriceData>> {
        let mut result = HashMap::new();

        for symbol in token_symbols {
            if let Ok(price) = self.get_price(symbol).await {
                result.insert(symbol.to_string(), price);
            }
        }

        Ok(result)
    }

    async fn supported_tokens(&self) -> Result<Vec<String>> {
        // 全ソースのサポートトークンを集約
        let mut all_tokens = std::collections::HashSet::new();

        for source in &self.sources {
            if let Ok(tokens) = source.supported_tokens().await {
                all_tokens.extend(tokens);
            }
        }

        Ok(all_tokens.into_iter().collect())
    }
}

/// 価格妥当性チェッカー
pub struct PriceValidityChecker {
    /// 最大価格変動率（%）
    max_price_change: f64,
    /// 最小流動性（USD）
    min_liquidity: f64,
}

impl PriceValidityChecker {
    pub fn new(max_price_change: f64, min_liquidity: f64) -> Self {
        Self {
            max_price_change,
            min_liquidity,
        }
    }

    /// 価格の妥当性をチェック
    pub fn is_valid(&self, price_data: &EnhancedPriceData) -> bool {
        // 価格変動チェック
        if price_data.change_24h.abs() > self.max_price_change {
            return false;
        }

        // 流動性チェック
        if let Some(liquidity) = price_data.liquidity_depth {
            if liquidity < self.min_liquidity {
                return false;
            }
        }

        // 信頼度チェック
        if price_data.base.confidence < 0.8 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::price_oracle::MockPriceOracle;

    #[tokio::test]
    async fn test_enhanced_oracle_with_single_source() {
        let mut enhanced_oracle = EnhancedPriceOracle::new(
            AggregationStrategy::Median,
            300, // 5分キャッシュ
        );

        let mock_oracle = MockPriceOracle::new();
        enhanced_oracle.add_source("Mock".to_string(), Box::new(mock_oracle));

        let price = enhanced_oracle.get_price("NEAR").await.unwrap();
        assert_eq!(price.price, 5.0);
    }

    #[tokio::test]
    async fn test_aggregation_median() {
        let mut enhanced_oracle = EnhancedPriceOracle::new(AggregationStrategy::Median, 300);

        // 複数のモックオラクルを追加
        for i in 0..3 {
            let mut mock = MockPriceOracle::new();
            mock.set_price("NEAR", 4.0 + i as f64); // 4.0, 5.0, 6.0
            enhanced_oracle.add_source(format!("Mock{}", i), Box::new(mock));
        }

        let price = enhanced_oracle.get_price("NEAR").await.unwrap();
        assert_eq!(price.price, 5.0); // 中央値
    }

    #[tokio::test]
    async fn test_aggregation_weighted_average() {
        let enhanced_oracle = EnhancedPriceOracle::new(AggregationStrategy::WeightedAverage, 300);

        let prices = vec![
            EnhancedPriceData {
                base: PriceData {
                    price: 4.0,
                    timestamp: 0,
                    confidence: 0.8,
                },
                source: "Source1".to_string(),
                change_24h: 0.0,
                volatility: 0.0,
                liquidity_depth: None,
            },
            EnhancedPriceData {
                base: PriceData {
                    price: 6.0,
                    timestamp: 0,
                    confidence: 0.2,
                },
                source: "Source2".to_string(),
                change_24h: 0.0,
                volatility: 0.0,
                liquidity_depth: None,
            },
        ];

        let result = enhanced_oracle.calculate_weighted_average(&prices);
        // (4.0 * 0.8 + 6.0 * 0.2) / (0.8 + 0.2) = 4.4
        assert_eq!(result.price, 4.4);
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        let mut enhanced_oracle = EnhancedPriceOracle::new(
            AggregationStrategy::Median,
            5, // 5秒キャッシュ
        );

        let mock_oracle = MockPriceOracle::new();
        enhanced_oracle.add_source("Mock".to_string(), Box::new(mock_oracle));

        // 初回取得
        let price1 = enhanced_oracle.get_price("NEAR").await.unwrap();

        // キャッシュから取得（同じ値）
        let price2 = enhanced_oracle.get_price("NEAR").await.unwrap();
        assert_eq!(price1.price, price2.price);
    }

    #[tokio::test]
    async fn test_price_validity_checker() {
        let checker = PriceValidityChecker::new(50.0, 100000.0);

        let valid_price = EnhancedPriceData {
            base: PriceData {
                price: 5.0,
                timestamp: 0,
                confidence: 0.95,
            },
            source: "Test".to_string(),
            change_24h: 10.0, // 10% change
            volatility: 0.1,
            liquidity_depth: Some(200000.0),
        };

        assert!(checker.is_valid(&valid_price));

        let invalid_price = EnhancedPriceData {
            base: PriceData {
                price: 5.0,
                timestamp: 0,
                confidence: 0.95,
            },
            source: "Test".to_string(),
            change_24h: 60.0, // 60% change - too high
            volatility: 0.1,
            liquidity_depth: Some(200000.0),
        };

        assert!(!checker.is_valid(&invalid_price));
    }

    #[tokio::test]
    async fn test_volatility_calculation() {
        let enhanced_oracle = EnhancedPriceOracle::new(AggregationStrategy::Median, 300);

        let prices = vec![
            EnhancedPriceData {
                base: PriceData {
                    price: 4.0,
                    timestamp: 0,
                    confidence: 0.9,
                },
                source: "S1".to_string(),
                change_24h: 0.0,
                volatility: 0.0,
                liquidity_depth: None,
            },
            EnhancedPriceData {
                base: PriceData {
                    price: 6.0,
                    timestamp: 0,
                    confidence: 0.9,
                },
                source: "S2".to_string(),
                change_24h: 0.0,
                volatility: 0.0,
                liquidity_depth: None,
            },
        ];

        let volatility = enhanced_oracle.calculate_volatility(&prices);
        assert_eq!(volatility, 1.0); // Standard deviation of [4, 6] is 1
    }
}
