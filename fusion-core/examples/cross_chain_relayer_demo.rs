//! クロスチェーンリレイヤーのデモ

use fusion_core::{
    automated_executor::{AutomatedExecutor, ExecutionTask, RetryConfig, StandardExecutionEngine, TaskStatus},
    cross_chain_executor::CrossChainExecutor,
    enhanced_price_oracle::{AggregationStrategy, EnhancedPriceOracle},
    execution_path_optimizer::{ExecutionPath, ExecutionPathOptimizer, OptimizationParams, OptimizationPriority, Route},
    order_matching_engine::{OrderMatch, OrderMatchingEngine, OrderType, PendingOrder},
    price_oracle::MockPriceOracle,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== クロスチェーンリレイヤーデモ ===\n");

    // 1. オーダーマッチングのデモ
    demo_order_matching()?;
    
    // 2. 価格オラクル統合のデモ
    demo_price_oracle().await?;
    
    // 3. 実行パス最適化のデモ
    demo_path_optimization()?;
    
    // 4. 自動実行のデモ
    demo_automated_execution().await?;

    Ok(())
}

fn demo_order_matching() -> anyhow::Result<()> {
    println!("1. オーダーマッチングエンジンのデモ");
    println!("====================================");

    let mut engine = OrderMatchingEngine::new(50); // 0.5% minimum profit

    // サンプルオーダーを追加
    let orders = vec![
        PendingOrder {
            id: "eth_buy_001".to_string(),
            chain_id: "ethereum".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.5,
            amount: 2000_000_000_000_000_000_000_000_000,
            timestamp: 1234567890,
        },
        PendingOrder {
            id: "near_sell_001".to_string(),
            chain_id: "near".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Sell,
            price: 5.0,
            amount: 1500_000_000_000_000_000_000_000_000,
            timestamp: 1234567891,
        },
        PendingOrder {
            id: "bsc_buy_001".to_string(),
            chain_id: "bsc".to_string(),
            token_pair: "NEAR/USDC".to_string(),
            order_type: OrderType::Buy,
            price: 5.3,
            amount: 1000_000_000_000_000_000_000_000_000,
            timestamp: 1234567892,
        },
    ];

    for order in orders {
        println!("  追加: {} {} {} @ ${}", 
            order.chain_id, 
            match order.order_type {
                OrderType::Buy => "買い",
                OrderType::Sell => "売り",
            },
            order.amount / 1_000_000_000_000_000_000_000_000,
            order.price
        );
        engine.add_order(order)?;
    }

    let matches = engine.find_matches("NEAR/USDC");
    println!("\n  マッチング結果:");
    for m in &matches {
        println!("    {} <-> {}: 価格 ${:.2}, 数量 {}, 利益 {:.2}%",
            m.buy_order_id,
            m.sell_order_id,
            m.match_price,
            m.match_amount / 1_000_000_000_000_000_000_000_000,
            m.profit_bps as f64 / 100.0
        );
    }

    println!("\n");
    Ok(())
}

async fn demo_price_oracle() -> anyhow::Result<()> {
    println!("2. 価格オラクル統合のデモ");
    println!("========================");

    let mut oracle = EnhancedPriceOracle::new(
        AggregationStrategy::WeightedAverage,
        300,
    );

    // 複数の価格ソースをシミュレート
    for i in 0..3 {
        let mut mock = MockPriceOracle::new();
        mock.set_price("NEAR", 5.0 + (i as f64 * 0.05));
        oracle.add_source(format!("Source{}", i + 1), Box::new(mock));
    }

    let price = oracle.get_price("NEAR").await?;
    println!("  NEAR価格（加重平均）: ${:.2}", price.price);
    println!("  信頼度: {:.2}%", price.confidence * 100.0);

    // 複数トークンの価格を取得
    let tokens = ["NEAR", "ETH", "USDC"];
    let prices = oracle.get_prices(&tokens).await?;
    
    println!("\n  複数トークンの価格:");
    for (token, price_data) in prices {
        println!("    {}: ${:.2}", token, price_data.price);
    }

    println!("\n");
    Ok(())
}

fn demo_path_optimization() -> anyhow::Result<()> {
    println!("3. 実行パス最適化のデモ");
    println!("======================");

    let mut optimizer = ExecutionPathOptimizer::new();

    // 複数のブリッジルートを追加
    let routes = vec![
        Route {
            source_chain: "ethereum".to_string(),
            target_chain: "near".to_string(),
            protocol: "Rainbow Bridge".to_string(),
            base_cost: 25.0,
            base_time: 900, // 15分
            liquidity: 10_000_000.0,
        },
        Route {
            source_chain: "ethereum".to_string(),
            target_chain: "bsc".to_string(),
            protocol: "Multichain".to_string(),
            base_cost: 10.0,
            base_time: 300, // 5分
            liquidity: 20_000_000.0,
        },
        Route {
            source_chain: "bsc".to_string(),
            target_chain: "near".to_string(),
            protocol: "Allbridge".to_string(),
            base_cost: 8.0,
            base_time: 300, // 5分
            liquidity: 5_000_000.0,
        },
    ];

    for route in routes {
        optimizer.add_route(route);
    }

    let params = OptimizationParams {
        max_cost: 100.0,
        max_time: 3600,
        max_risk_score: 70,
        min_profit: 0.0,
        priority: OptimizationPriority::MinimizeCost,
    };

    let paths = optimizer.find_optimal_path(
        "ethereum",
        "near",
        "USDC",
        5000_000_000, // 5000 USDC
        &params,
    )?;

    println!("  最適な実行パス（コスト優先）:");
    for (i, path) in paths.iter().take(3).enumerate() {
        println!("    {}. {}", i + 1, path.id);
        println!("       コスト: ${:.2}", path.total_cost);
        println!("       時間: {}分", path.total_time / 60);
        println!("       リスクスコア: {}/100", path.risk_score);
        
        for step in &path.steps {
            println!("       - {} → {}", step.source_chain, step.target_chain);
        }
    }

    println!("\n");
    Ok(())
}

async fn demo_automated_execution() -> anyhow::Result<()> {
    println!("4. 自動実行エンジンのデモ");
    println!("========================");

    let cross_chain_executor = CrossChainExecutor::new(
        "https://eth-sepolia.example.com",
        "0x0000000000000000000000000000000000000000",
        "https://rpc.testnet.near.org",
    )?;

    let engine = Box::new(StandardExecutionEngine::new(cross_chain_executor));
    let retry_config = RetryConfig {
        max_retries: 3,
        retry_delay: 60,
        exponential_backoff: true,
    };

    let mut executor = AutomatedExecutor::new(engine, 5, retry_config);

    // サンプルタスクを作成
    let task = ExecutionTask {
        id: "demo_task_001".to_string(),
        order_match: OrderMatch {
            buy_order_id: "eth_buy_001".to_string(),
            sell_order_id: "near_sell_001".to_string(),
            match_price: 5.25,
            match_amount: 1000_000_000_000_000_000_000_000_000,
            profit_bps: 250,
        },
        execution_path: ExecutionPath {
            id: "relay_ethereum_bsc_to_near".to_string(),
            steps: vec![],
            total_cost: 18.0,
            total_time: 600,
            risk_score: 35,
            expected_profit: 50.0,
        },
        status: TaskStatus::Pending,
        created_at: 1234567900,
        updated_at: 1234567900,
        error_message: None,
    };

    executor.add_task(task)?;

    println!("  タスクを追加しました");
    
    // ステータスサマリーを表示
    let summary = executor.get_status_summary();
    println!("\n  実行ステータス:");
    println!("    待機中: {}", summary.get("pending").unwrap_or(&0));
    println!("    実行中: {}", summary.get("executing").unwrap_or(&0));
    println!("    完了: {}", summary.get("completed").unwrap_or(&0));
    println!("    失敗: {}", summary.get("failed").unwrap_or(&0));

    // 実際の実行ループはここでは開始しない（デモのため）
    println!("\n  ※ 実際の環境では executor.start_execution_loop() で自動実行が開始されます");

    Ok(())
}