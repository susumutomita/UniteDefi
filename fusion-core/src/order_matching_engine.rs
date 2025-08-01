//! クロスチェーンオーダーマッチングエンジン
//! 
//! 異なるチェーン間のオーダーをマッチングし、最適な実行パスを決定します。

use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// マッチング可能なオーダーのペア
#[derive(Debug, Clone, PartialEq)]
pub struct OrderMatch {
    /// 買い注文ID
    pub buy_order_id: String,
    /// 売り注文ID
    pub sell_order_id: String,
    /// マッチング価格
    pub match_price: f64,
    /// マッチング数量
    pub match_amount: u128,
    /// 想定利益（basis points）
    pub profit_bps: u16,
}

/// オーダーブック
#[derive(Debug, Default)]
pub struct OrderBook {
    /// 買い注文（価格降順）
    buy_orders: Vec<PendingOrder>,
    /// 売り注文（価格昇順）
    sell_orders: Vec<PendingOrder>,
}

/// 保留中のオーダー
#[derive(Debug, Clone)]
pub struct PendingOrder {
    /// オーダーID
    pub id: String,
    /// チェーンID
    pub chain_id: String,
    /// トークンペア（例：NEAR/USDC）
    pub token_pair: String,
    /// オーダータイプ
    pub order_type: OrderType,
    /// 価格
    pub price: f64,
    /// 数量
    pub amount: u128,
    /// タイムスタンプ
    pub timestamp: u64,
}

/// オーダータイプ
#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

/// オーダーマッチングエンジン
pub struct OrderMatchingEngine {
    /// チェーンごとのオーダーブック
    order_books: HashMap<String, OrderBook>,
    /// 最小利益閾値（basis points）
    min_profit_threshold: u16,
}

impl OrderMatchingEngine {
    /// 新しいエンジンを作成
    pub fn new(min_profit_threshold: u16) -> Self {
        Self {
            order_books: HashMap::new(),
            min_profit_threshold,
        }
    }

    /// オーダーを追加
    pub fn add_order(&mut self, order: PendingOrder) -> Result<()> {
        let order_book = self.order_books
            .entry(order.token_pair.clone())
            .or_insert_with(OrderBook::default);

        match order.order_type {
            OrderType::Buy => {
                order_book.buy_orders.push(order);
                // 価格降順でソート
                order_book.buy_orders.sort_by(|a, b| {
                    b.price.partial_cmp(&a.price).unwrap()
                });
            }
            OrderType::Sell => {
                order_book.sell_orders.push(order);
                // 価格昇順でソート
                order_book.sell_orders.sort_by(|a, b| {
                    a.price.partial_cmp(&b.price).unwrap()
                });
            }
        }

        Ok(())
    }

    /// マッチング可能なオーダーを検索
    pub fn find_matches(&self, token_pair: &str) -> Vec<OrderMatch> {
        let mut matches = Vec::new();

        if let Some(order_book) = self.order_books.get(token_pair) {
            for buy_order in &order_book.buy_orders {
                for sell_order in &order_book.sell_orders {
                    if let Some(order_match) = self.try_match(buy_order, sell_order) {
                        matches.push(order_match);
                    }
                }
            }
        }

        matches
    }

    /// 2つのオーダーのマッチングを試行
    fn try_match(&self, buy_order: &PendingOrder, sell_order: &PendingOrder) -> Option<OrderMatch> {
        // 異なるチェーンのオーダーのみマッチング
        if buy_order.chain_id == sell_order.chain_id {
            return None;
        }

        // 価格条件チェック（買値 >= 売値）
        if buy_order.price < sell_order.price {
            return None;
        }

        // マッチング価格は中間値
        let match_price = (buy_order.price + sell_order.price) / 2.0;

        // マッチング数量は小さい方
        let match_amount = buy_order.amount.min(sell_order.amount);

        // 利益計算（スプレッドから）
        let spread = buy_order.price - sell_order.price;
        let profit_bps = ((spread / sell_order.price) * 10000.0) as u16;

        // 最小利益閾値チェック
        if profit_bps < self.min_profit_threshold {
            return None;
        }

        Some(OrderMatch {
            buy_order_id: buy_order.id.clone(),
            sell_order_id: sell_order.id.clone(),
            match_price,
            match_amount,
            profit_bps,
        })
    }

    /// オーダーを削除
    pub fn remove_order(&mut self, token_pair: &str, order_id: &str) -> Result<()> {
        let order_book = self.order_books
            .get_mut(token_pair)
            .ok_or_else(|| anyhow!("Order book not found for {}", token_pair))?;

        // 買い注文から削除を試行
        order_book.buy_orders.retain(|o| o.id != order_id);
        // 売り注文から削除を試行
        order_book.sell_orders.retain(|o| o.id != order_id);

        Ok(())
    }

    /// アクティブなオーダー数を取得
    pub fn get_order_count(&self, token_pair: &str) -> (usize, usize) {
        if let Some(order_book) = self.order_books.get(token_pair) {
            (order_book.buy_orders.len(), order_book.sell_orders.len())
        } else {
            (0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_engine() {
        let engine = OrderMatchingEngine::new(50); // 0.5% minimum profit
        assert_eq!(engine.min_profit_threshold, 50);
        assert!(engine.order_books.is_empty());
    }

    #[test]
    fn test_add_order() {
        let mut engine = OrderMatchingEngine::new(50);
        
        let order = PendingOrder {
            id: "order1".to_string(),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.0,
            amount: 1000,
            timestamp: 1234567890,
        };

        engine.add_order(order).unwrap();
        
        let (buy_count, sell_count) = engine.get_order_count("NEAR/USDC");
        assert_eq!(buy_count, 1);
        assert_eq!(sell_count, 0);
    }

    #[test]
    fn test_order_matching() {
        let mut engine = OrderMatchingEngine::new(50); // 0.5% minimum profit
        
        // 買い注文（高い価格）
        let buy_order = PendingOrder {
            id: "buy1".to_string(),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.1,
            amount: 1000,
            timestamp: 1234567890,
        };

        // 売り注文（低い価格）
        let sell_order = PendingOrder {
            id: "sell1".to_string(),
            chain_id: "near".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Sell,
            price: 5.0,
            amount: 800,
            timestamp: 1234567891,
        };

        engine.add_order(buy_order).unwrap();
        engine.add_order(sell_order).unwrap();

        let matches = engine.find_matches("NEAR/USDC");
        assert_eq!(matches.len(), 1);

        let order_match = &matches[0];
        assert_eq!(order_match.buy_order_id, "buy1");
        assert_eq!(order_match.sell_order_id, "sell1");
        assert_eq!(order_match.match_price, 5.05);
        assert_eq!(order_match.match_amount, 800);
        assert_eq!(order_match.profit_bps, 200); // 2%
    }

    #[test]
    fn test_no_match_same_chain() {
        let mut engine = OrderMatchingEngine::new(50);
        
        // 同じチェーンのオーダー
        let buy_order = PendingOrder {
            id: "buy1".to_string(),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.1,
            amount: 1000,
            timestamp: 1234567890,
        };

        let sell_order = PendingOrder {
            id: "sell1".to_string(),
            chain_id: "ethereum".to_string(), // 同じチェーン
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Sell,
            price: 5.0,
            amount: 800,
            timestamp: 1234567891,
        };

        engine.add_order(buy_order).unwrap();
        engine.add_order(sell_order).unwrap();

        let matches = engine.find_matches("NEAR/USDC");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_no_match_price_condition() {
        let mut engine = OrderMatchingEngine::new(50);
        
        // 買値が売値より低い
        let buy_order = PendingOrder {
            id: "buy1".to_string(),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 4.9,
            amount: 1000,
            timestamp: 1234567890,
        };

        let sell_order = PendingOrder {
            id: "sell1".to_string(),
            chain_id: "near".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Sell,
            price: 5.0,
            amount: 800,
            timestamp: 1234567891,
        };

        engine.add_order(buy_order).unwrap();
        engine.add_order(sell_order).unwrap();

        let matches = engine.find_matches("NEAR/USDC");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_no_match_insufficient_profit() {
        let mut engine = OrderMatchingEngine::new(50); // 0.5% minimum
        
        // 利益が閾値未満
        let buy_order = PendingOrder {
            id: "buy1".to_string(),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.02,
            amount: 1000,
            timestamp: 1234567890,
        };

        let sell_order = PendingOrder {
            id: "sell1".to_string(),
            chain_id: "near".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Sell,
            price: 5.0,
            amount: 800,
            timestamp: 1234567891,
        };

        engine.add_order(buy_order).unwrap();
        engine.add_order(sell_order).unwrap();

        let matches = engine.find_matches("NEAR/USDC");
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_remove_order() {
        let mut engine = OrderMatchingEngine::new(50);
        
        let order = PendingOrder {
            id: "order1".to_string(),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.0,
            amount: 1000,
            timestamp: 1234567890,
        };

        engine.add_order(order).unwrap();
        assert_eq!(engine.get_order_count("NEAR/USDC").0, 1);

        engine.remove_order("NEAR/USDC", "order1").unwrap();
        assert_eq!(engine.get_order_count("NEAR/USDC").0, 0);
    }
}