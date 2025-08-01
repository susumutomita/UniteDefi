//! 最適実行パス計算モジュール
//!
//! クロスチェーン取引の最適な実行パスを計算し、コストとリスクを最小化します。

use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

/// 実行パス
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionPath {
    /// パスID
    pub id: String,
    /// ステップのリスト
    pub steps: Vec<ExecutionStep>,
    /// 総コスト（USD）
    pub total_cost: f64,
    /// 総実行時間（秒）
    pub total_time: u64,
    /// リスクスコア（0-100）
    pub risk_score: u8,
    /// 期待利益（USD）
    pub expected_profit: f64,
}

/// 実行ステップ
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionStep {
    /// ステップタイプ
    pub step_type: StepType,
    /// ソースチェーン
    pub source_chain: String,
    /// ターゲットチェーン
    pub target_chain: String,
    /// トークン
    pub token: String,
    /// 数量
    pub amount: u128,
    /// 推定コスト（USD）
    pub estimated_cost: f64,
    /// 推定時間（秒）
    pub estimated_time: u64,
}

/// ステップタイプ
#[derive(Debug, Clone, PartialEq)]
pub enum StepType {
    /// ブリッジ転送
    Bridge,
    /// スワップ
    Swap,
    /// HTLC作成
    HTLCCreate,
    /// HTLCクレーム
    HTLCClaim,
    /// リミットオーダー実行
    LimitOrderExecution,
}

/// ルート情報
#[derive(Debug, Clone)]
pub struct Route {
    /// ソースチェーン
    pub source_chain: String,
    /// ターゲットチェーン
    pub target_chain: String,
    /// プロトコル
    pub protocol: String,
    /// 基本コスト（USD）
    pub base_cost: f64,
    /// 基本時間（秒）
    pub base_time: u64,
    /// 流動性（USD）
    pub liquidity: f64,
}

/// 最適化パラメータ
#[derive(Debug, Clone)]
pub struct OptimizationParams {
    /// 最大許容コスト（USD）
    pub max_cost: f64,
    /// 最大許容時間（秒）
    pub max_time: u64,
    /// 最大許容リスクスコア
    pub max_risk_score: u8,
    /// 最小要求利益（USD）
    pub min_profit: f64,
    /// 優先度（コスト、時間、リスク）
    pub priority: OptimizationPriority,
}

/// 最適化優先度
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationPriority {
    /// コスト最小化
    MinimizeCost,
    /// 時間最小化
    MinimizeTime,
    /// リスク最小化
    MinimizeRisk,
    /// 利益最大化
    MaximizeProfit,
}

/// 実行パスオプティマイザー
pub struct ExecutionPathOptimizer {
    /// 利用可能なルート
    routes: Vec<Route>,
    /// チェーン情報
    chain_info: HashMap<String, ChainInfo>,
}

/// チェーン情報
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ChainInfo {
    /// ガス価格（USD）
    gas_price: f64,
    /// 平均ブロック時間（秒）
    block_time: u64,
    /// ネットワーク混雑度（0-1）
    congestion: f64,
}

impl Default for ExecutionPathOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionPathOptimizer {
    /// 新しいオプティマイザーを作成
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            chain_info: Self::default_chain_info(),
        }
    }

    /// デフォルトのチェーン情報
    fn default_chain_info() -> HashMap<String, ChainInfo> {
        let mut info = HashMap::new();

        info.insert(
            "ethereum".to_string(),
            ChainInfo {
                gas_price: 30.0,
                block_time: 12,
                congestion: 0.5,
            },
        );

        info.insert(
            "near".to_string(),
            ChainInfo {
                gas_price: 0.1,
                block_time: 1,
                congestion: 0.2,
            },
        );

        info.insert(
            "bsc".to_string(),
            ChainInfo {
                gas_price: 5.0,
                block_time: 3,
                congestion: 0.3,
            },
        );

        info
    }

    /// ルートを追加
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// 最適な実行パスを計算
    pub fn find_optimal_path(
        &self,
        source_chain: &str,
        target_chain: &str,
        token: &str,
        amount: u128,
        params: &OptimizationParams,
    ) -> Result<Vec<ExecutionPath>> {
        let mut paths = Vec::new();

        // 直接パスを探索
        if let Some(direct_path) = self.find_direct_path(source_chain, target_chain, token, amount)
        {
            if self.is_valid_path(&direct_path, params) {
                paths.push(direct_path);
            }
        }

        // 中継パスを探索
        let relay_paths = self.find_relay_paths(source_chain, target_chain, token, amount);
        for path in relay_paths {
            if self.is_valid_path(&path, params) {
                paths.push(path);
            }
        }

        // パスをソート（優先度に基づく）
        self.sort_paths(&mut paths, &params.priority);

        // 上位5つまでを返す
        paths.truncate(5);

        if paths.is_empty() {
            return Err(anyhow!("No valid execution path found"));
        }

        Ok(paths)
    }

    /// 直接パスを探索
    fn find_direct_path(
        &self,
        source_chain: &str,
        target_chain: &str,
        token: &str,
        amount: u128,
    ) -> Option<ExecutionPath> {
        // 直接ルートを探す
        let route = self
            .routes
            .iter()
            .find(|r| r.source_chain == source_chain && r.target_chain == target_chain)?;

        // 流動性チェック
        let amount_usd = self.estimate_amount_usd(amount, token);
        if route.liquidity < amount_usd {
            return None;
        }

        let steps = vec![ExecutionStep {
            step_type: StepType::Bridge,
            source_chain: source_chain.to_string(),
            target_chain: target_chain.to_string(),
            token: token.to_string(),
            amount,
            estimated_cost: route.base_cost,
            estimated_time: route.base_time,
        }];

        Some(ExecutionPath {
            id: format!("direct_{}_to_{}", source_chain, target_chain),
            steps,
            total_cost: route.base_cost,
            total_time: route.base_time,
            risk_score: self.calculate_risk_score(source_chain, target_chain, 1),
            expected_profit: 15.0, // 簡易的な利益計算（実装すべき）
        })
    }

    /// 中継パスを探索
    fn find_relay_paths(
        &self,
        source_chain: &str,
        target_chain: &str,
        token: &str,
        amount: u128,
    ) -> Vec<ExecutionPath> {
        let mut paths = Vec::new();
        let intermediate_chains = self.get_intermediate_chains();

        for intermediate in intermediate_chains {
            // ソース → 中継
            let route1 = self
                .routes
                .iter()
                .find(|r| r.source_chain == source_chain && r.target_chain == intermediate);

            // 中継 → ターゲット
            let route2 = self
                .routes
                .iter()
                .find(|r| r.source_chain == intermediate && r.target_chain == target_chain);

            if let (Some(r1), Some(r2)) = (route1, route2) {
                let amount_usd = self.estimate_amount_usd(amount, token);

                // 流動性チェック
                if r1.liquidity < amount_usd || r2.liquidity < amount_usd {
                    continue;
                }

                let steps = vec![
                    ExecutionStep {
                        step_type: StepType::Bridge,
                        source_chain: source_chain.to_string(),
                        target_chain: intermediate.clone(),
                        token: token.to_string(),
                        amount,
                        estimated_cost: r1.base_cost,
                        estimated_time: r1.base_time,
                    },
                    ExecutionStep {
                        step_type: StepType::Bridge,
                        source_chain: intermediate.clone(),
                        target_chain: target_chain.to_string(),
                        token: token.to_string(),
                        amount,
                        estimated_cost: r2.base_cost,
                        estimated_time: r2.base_time,
                    },
                ];

                let path = ExecutionPath {
                    id: format!(
                        "relay_{}_{}_to_{}",
                        source_chain, intermediate, target_chain
                    ),
                    steps,
                    total_cost: r1.base_cost + r2.base_cost,
                    total_time: r1.base_time + r2.base_time,
                    risk_score: self.calculate_risk_score(source_chain, target_chain, 2),
                    expected_profit: 12.0, // 簡易的な利益計算（中継パス）
                };

                paths.push(path);
            }
        }

        paths
    }

    /// 中継チェーンのリストを取得
    fn get_intermediate_chains(&self) -> Vec<String> {
        let mut chains = HashSet::new();

        for route in &self.routes {
            chains.insert(route.source_chain.clone());
            chains.insert(route.target_chain.clone());
        }

        chains.into_iter().collect()
    }

    /// パスの妥当性をチェック
    fn is_valid_path(&self, path: &ExecutionPath, params: &OptimizationParams) -> bool {
        path.total_cost <= params.max_cost
            && path.total_time <= params.max_time
            && path.risk_score <= params.max_risk_score
            && path.expected_profit >= params.min_profit
    }

    /// パスをソート
    fn sort_paths(&self, paths: &mut [ExecutionPath], priority: &OptimizationPriority) {
        match priority {
            OptimizationPriority::MinimizeCost => {
                paths.sort_by(|a, b| a.total_cost.partial_cmp(&b.total_cost).unwrap());
            }
            OptimizationPriority::MinimizeTime => {
                paths.sort_by(|a, b| a.total_time.cmp(&b.total_time));
            }
            OptimizationPriority::MinimizeRisk => {
                paths.sort_by(|a, b| a.risk_score.cmp(&b.risk_score));
            }
            OptimizationPriority::MaximizeProfit => {
                paths.sort_by(|a, b| b.expected_profit.partial_cmp(&a.expected_profit).unwrap());
            }
        }
    }

    /// リスクスコアを計算
    fn calculate_risk_score(&self, source_chain: &str, target_chain: &str, hop_count: u8) -> u8 {
        let mut score = 0u8;

        // ホップ数によるリスク
        score += hop_count * 10;

        // チェーンの混雑度によるリスク
        if let Some(source_info) = self.chain_info.get(source_chain) {
            score += (source_info.congestion * 20.0) as u8;
        }
        if let Some(target_info) = self.chain_info.get(target_chain) {
            score += (target_info.congestion * 20.0) as u8;
        }

        score.min(100)
    }

    /// 金額をUSDに変換（簡易版）
    fn estimate_amount_usd(&self, amount: u128, token: &str) -> f64 {
        // TODO: 実際の価格オラクルを使用
        match token {
            "NEAR" => amount as f64 * 5.0 / 1e24,
            "ETH" => amount as f64 * 2000.0 / 1e18,
            "USDC" => amount as f64 / 1e6,
            _ => amount as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = ExecutionPathOptimizer::new();
        assert_eq!(optimizer.routes.len(), 0);
        assert!(optimizer.chain_info.contains_key("ethereum"));
        assert!(optimizer.chain_info.contains_key("near"));
    }

    #[test]
    fn test_add_route() {
        let mut optimizer = ExecutionPathOptimizer::new();

        let route = Route {
            source_chain: "ethereum".to_string(),
            target_chain: "near".to_string(),
            protocol: "rainbow_bridge".to_string(),
            base_cost: 10.0,
            base_time: 300,
            liquidity: 1000000.0,
        };

        optimizer.add_route(route);
        assert_eq!(optimizer.routes.len(), 1);
    }

    #[test]
    fn test_find_direct_path() {
        let mut optimizer = ExecutionPathOptimizer::new();

        optimizer.add_route(Route {
            source_chain: "ethereum".to_string(),
            target_chain: "near".to_string(),
            protocol: "rainbow_bridge".to_string(),
            base_cost: 10.0,
            base_time: 300,
            liquidity: 1000000.0,
        });

        let params = OptimizationParams {
            max_cost: 100.0,
            max_time: 3600,
            max_risk_score: 80,
            min_profit: 0.0,
            priority: OptimizationPriority::MinimizeCost,
        };

        let paths = optimizer
            .find_optimal_path(
                "ethereum",
                "near",
                "USDC",
                1_000_000_000, // 1000 USDC
                &params,
            )
            .unwrap();

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].steps.len(), 1);
        assert_eq!(paths[0].total_cost, 10.0);
    }

    #[test]
    fn test_find_relay_path() {
        let mut optimizer = ExecutionPathOptimizer::new();

        // Ethereum → BSC
        optimizer.add_route(Route {
            source_chain: "ethereum".to_string(),
            target_chain: "bsc".to_string(),
            protocol: "multichain".to_string(),
            base_cost: 5.0,
            base_time: 180,
            liquidity: 2000000.0,
        });

        // BSC → NEAR
        optimizer.add_route(Route {
            source_chain: "bsc".to_string(),
            target_chain: "near".to_string(),
            protocol: "allbridge".to_string(),
            base_cost: 3.0,
            base_time: 120,
            liquidity: 1500000.0,
        });

        let params = OptimizationParams {
            max_cost: 100.0,
            max_time: 3600,
            max_risk_score: 80,
            min_profit: 0.0,
            priority: OptimizationPriority::MinimizeCost,
        };

        let paths = optimizer
            .find_optimal_path("ethereum", "near", "USDC", 1_000_000_000, &params)
            .unwrap();

        assert!(!paths.is_empty());

        // 中継パスをチェック
        let relay_path = paths.iter().find(|p| p.steps.len() == 2);
        assert!(relay_path.is_some());

        if let Some(path) = relay_path {
            assert_eq!(path.total_cost, 8.0); // 5.0 + 3.0
            assert_eq!(path.total_time, 300); // 180 + 120
        }
    }

    #[test]
    fn test_path_validation() {
        let optimizer = ExecutionPathOptimizer::new();

        let path = ExecutionPath {
            id: "test".to_string(),
            steps: vec![],
            total_cost: 50.0,
            total_time: 1800,
            risk_score: 40,
            expected_profit: 10.0,
        };

        let params = OptimizationParams {
            max_cost: 100.0,
            max_time: 3600,
            max_risk_score: 80,
            min_profit: 5.0,
            priority: OptimizationPriority::MinimizeCost,
        };

        assert!(optimizer.is_valid_path(&path, &params));

        // コスト超過
        let expensive_path = ExecutionPath {
            id: "expensive".to_string(),
            steps: vec![],
            total_cost: 150.0,
            total_time: 1800,
            risk_score: 40,
            expected_profit: 10.0,
        };

        assert!(!optimizer.is_valid_path(&expensive_path, &params));
    }

    #[test]
    fn test_path_sorting() {
        let optimizer = ExecutionPathOptimizer::new();

        let mut paths = vec![
            ExecutionPath {
                id: "path1".to_string(),
                steps: vec![],
                total_cost: 20.0,
                total_time: 600,
                risk_score: 30,
                expected_profit: 15.0,
            },
            ExecutionPath {
                id: "path2".to_string(),
                steps: vec![],
                total_cost: 10.0,
                total_time: 1200,
                risk_score: 40,
                expected_profit: 20.0,
            },
            ExecutionPath {
                id: "path3".to_string(),
                steps: vec![],
                total_cost: 15.0,
                total_time: 300,
                risk_score: 20,
                expected_profit: 10.0,
            },
        ];

        // コスト最小化でソート
        optimizer.sort_paths(&mut paths, &OptimizationPriority::MinimizeCost);
        assert_eq!(paths[0].id, "path2");
        assert_eq!(paths[1].id, "path3");
        assert_eq!(paths[2].id, "path1");

        // 時間最小化でソート
        optimizer.sort_paths(&mut paths, &OptimizationPriority::MinimizeTime);
        assert_eq!(paths[0].id, "path3");
        assert_eq!(paths[1].id, "path1");
        assert_eq!(paths[2].id, "path2");
    }
}
