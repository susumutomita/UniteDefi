//! クロスチェーンリレイヤー統合テスト

use fusion_core::{
    automated_executor::{AutomatedExecutor, ExecutionTask, RetryConfig, StandardExecutionEngine, TaskStatus},
    cross_chain_executor::CrossChainExecutor,
    enhanced_price_oracle::{AggregationStrategy, EnhancedPriceOracle},
    execution_path_optimizer::{ExecutionPath, ExecutionPathOptimizer, OptimizationParams, OptimizationPriority, Route},
    order_matching_engine::{OrderMatchingEngine, OrderType, PendingOrder},
    price_oracle::MockPriceOracle,
};

#[tokio::test]
async fn test_end_to_end_order_matching_and_execution() {
    // 1. オーダーマッチングエンジンのセットアップ
    let mut matching_engine = OrderMatchingEngine::new(50); // 0.5% minimum profit

    // 買い注文を追加（Ethereum上）
    let buy_order = PendingOrder {
        id: "eth_buy_001".to_string(),
        chain_id: "ethereum".to_string(),
        token_pair: "NEAR/USDC".to_string(),
        order_type: OrderType::Buy,
        price: 5.2,
        amount: 1000_000_000_000_000_000_000_000_000, // 1000 NEAR
        timestamp: 1234567890,
    };

    // 売り注文を追加（NEAR上）
    let sell_order = PendingOrder {
        id: "near_sell_001".to_string(),
        chain_id: "near".to_string(),
        token_pair: "NEAR/USDC".to_string(),
        order_type: OrderType::Sell,
        price: 5.0,
        amount: 1000_000_000_000_000_000_000_000_000, // 1000 NEAR
        timestamp: 1234567891,
    };

    matching_engine.add_order(buy_order).unwrap();
    matching_engine.add_order(sell_order).unwrap();

    // マッチングを検索
    let matches = matching_engine.find_matches("NEAR/USDC");
    assert_eq!(matches.len(), 1);
    let order_match = &matches[0];
    assert_eq!(order_match.profit_bps, 400); // 4% profit

    // 2. 価格オラクルのセットアップ
    let mut price_oracle = EnhancedPriceOracle::new(
        AggregationStrategy::WeightedAverage,
        300, // 5分キャッシュ
    );

    let mock_source = MockPriceOracle::new();
    price_oracle.add_source("Mock".to_string(), Box::new(mock_source));

    // 3. 実行パスオプティマイザーのセットアップ
    let mut path_optimizer = ExecutionPathOptimizer::new();
    
    // Ethereum → NEAR ルートを追加
    path_optimizer.add_route(Route {
        source_chain: "ethereum".to_string(),
        target_chain: "near".to_string(),
        protocol: "rainbow_bridge".to_string(),
        base_cost: 15.0,
        base_time: 600, // 10分
        liquidity: 10_000_000.0, // $10M
    });

    let optimization_params = OptimizationParams {
        max_cost: 100.0,
        max_time: 3600,
        max_risk_score: 50,
        min_profit: 10.0,
        priority: OptimizationPriority::MinimizeCost,
    };

    let paths = path_optimizer.find_optimal_path(
        "ethereum",
        "near",
        "USDC",
        5000_000_000, // 5000 USDC
        &optimization_params,
    ).unwrap();

    assert!(!paths.is_empty());
    let best_path = &paths[0];

    // 4. 自動実行エンジンのセットアップ
    let cross_chain_executor = CrossChainExecutor::new(
        "https://eth-sepolia.example.com",
        "0x0000000000000000000000000000000000000000",
        "https://rpc.testnet.near.org",
    ).unwrap();

    let execution_engine = Box::new(StandardExecutionEngine::new(cross_chain_executor));
    let retry_config = RetryConfig::default();

    let mut automated_executor = AutomatedExecutor::new(
        execution_engine,
        5, // 最大5つの並行タスク
        retry_config,
    );

    // 実行タスクを作成
    let execution_task = ExecutionTask {
        id: "task_001".to_string(),
        order_match: order_match.clone(),
        execution_path: best_path.clone(),
        status: TaskStatus::Pending,
        created_at: 1234567900,
        updated_at: 1234567900,
        error_message: None,
    };

    // タスクを追加
    automated_executor.add_task(execution_task).unwrap();

    // ステータスサマリーを確認
    let summary = automated_executor.get_status_summary();
    assert_eq!(summary.get("pending").unwrap_or(&0), &1);
}

#[tokio::test]
async fn test_price_oracle_integration() {
    // 複数のモックオラクルソースを作成
    let mut enhanced_oracle = EnhancedPriceOracle::new(
        AggregationStrategy::Median,
        60, // 1分キャッシュ
    );

    // 3つの価格ソースを追加（異なる価格）
    for i in 0..3 {
        let mut mock = MockPriceOracle::new();
        mock.set_price("NEAR", 4.8 + (i as f64 * 0.2)); // 4.8, 5.0, 5.2
        enhanced_oracle.add_source(format!("Source{}", i), Box::new(mock));
    }

    // 価格を取得（中央値が返される）
    let price_data = enhanced_oracle.get_price("NEAR").await.unwrap();
    assert_eq!(price_data.price, 5.0);
    assert!(price_data.confidence > 0.9); // 高い信頼度
}

#[test]
fn test_order_matching_with_multiple_orders() {
    let mut engine = OrderMatchingEngine::new(100); // 1% minimum profit

    // 複数の買い注文
    for i in 0..3 {
        let order = PendingOrder {
            id: format!("buy_{}", i),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.0 + (i as f64 * 0.1),
            amount: 500_000_000_000_000_000_000_000_000,
            timestamp: 1234567890 + i,
        };
        engine.add_order(order).unwrap();
    }

    // 複数の売り注文
    for i in 0..3 {
        let order = PendingOrder {
            id: format!("sell_{}", i),
            chain_id: "near".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Sell,
            price: 4.8 + (i as f64 * 0.1),
            amount: 500_000_000_000_000_000_000_000_000,
            timestamp: 1234567900 + i,
        };
        engine.add_order(order).unwrap();
    }

    let matches = engine.find_matches("NEAR/USDC");
    
    // 複数のマッチが見つかることを確認
    assert!(matches.len() > 0);
    
    // 最も利益の高いマッチを確認
    let best_match = matches.iter().max_by_key(|m| m.profit_bps).unwrap();
    assert!(best_match.profit_bps >= 200); // 少なくとも2%の利益
}

#[test]
fn test_execution_path_with_constraints() {
    let mut optimizer = ExecutionPathOptimizer::new();

    // 複数のルートを追加
    // 直接ルート（高コスト・低リスク）
    optimizer.add_route(Route {
        source_chain: "ethereum".to_string(),
        target_chain: "near".to_string(),
        protocol: "official_bridge".to_string(),
        base_cost: 50.0,
        base_time: 300,
        liquidity: 50_000_000.0,
    });

    // 中継ルート（低コスト・高リスク）
    optimizer.add_route(Route {
        source_chain: "ethereum".to_string(),
        target_chain: "bsc".to_string(),
        protocol: "multichain".to_string(),
        base_cost: 10.0,
        base_time: 180,
        liquidity: 20_000_000.0,
    });

    optimizer.add_route(Route {
        source_chain: "bsc".to_string(),
        target_chain: "near".to_string(),
        protocol: "allbridge".to_string(),
        base_cost: 8.0,
        base_time: 180,
        liquidity: 15_000_000.0,
    });

    // コスト優先の最適化
    let cost_params = OptimizationParams {
        max_cost: 100.0,
        max_time: 3600,
        max_risk_score: 100,
        min_profit: 0.0,
        priority: OptimizationPriority::MinimizeCost,
    };

    let cost_optimal_paths = optimizer.find_optimal_path(
        "ethereum",
        "near",
        "USDC",
        1000_000_000,
        &cost_params,
    ).unwrap();

    // 最もコストが低いパスが最初に来ることを確認
    assert!(cost_optimal_paths[0].total_cost < 50.0);

    // 時間優先の最適化
    let time_params = OptimizationParams {
        max_cost: 100.0,
        max_time: 3600,
        max_risk_score: 100,
        min_profit: 0.0,
        priority: OptimizationPriority::MinimizeTime,
    };

    let time_optimal_paths = optimizer.find_optimal_path(
        "ethereum",
        "near",
        "USDC",
        1000_000_000,
        &time_params,
    ).unwrap();

    // 最も時間が短いパスが最初に来ることを確認
    assert!(time_optimal_paths[0].total_time <= 360);
}

#[tokio::test]
async fn test_automated_executor_retry_logic() {
    // リトライ設定をカスタマイズ
    let retry_config = RetryConfig {
        max_retries: 2,
        retry_delay: 1, // テスト用に短く設定
        exponential_backoff: false,
    };

    let cross_chain_executor = CrossChainExecutor::new(
        "https://eth.example.com",
        "0x0000000000000000000000000000000000000000",
        "https://near.example.com",
    ).unwrap();

    let execution_engine = Box::new(StandardExecutionEngine::new(cross_chain_executor));
    
    let mut automated_executor = AutomatedExecutor::new(
        execution_engine,
        3,
        retry_config,
    );

    // ダミータスクを作成
    let task = ExecutionTask {
        id: "retry_test".to_string(),
        order_match: OrderMatch {
            buy_order_id: "buy1".to_string(),
            sell_order_id: "sell1".to_string(),
            match_price: 5.0,
            match_amount: 1000,
            profit_bps: 100,
        },
        execution_path: ExecutionPath {
            id: "path1".to_string(),
            steps: vec![],
            total_cost: 10.0,
            total_time: 300,
            risk_score: 20,
            expected_profit: 5.0,
        },
        status: TaskStatus::Pending,
        created_at: 1234567890,
        updated_at: 1234567890,
        error_message: None,
    };

    automated_executor.add_task(task).unwrap();

    // タスクが追加されたことを確認
    let summary = automated_executor.get_status_summary();
    assert_eq!(summary.get("pending").unwrap_or(&0), &1);
}