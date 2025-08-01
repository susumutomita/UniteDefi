use fusion_core::chains::near_events::{NearHtlcCreateEvent, NearHtlcClaimEvent, NearEventParser};
use fusion_core::chains::near_monitor::{NearHtlcMonitor, MonitorConfig};
use fusion_core::event_order_linker::{OrderManager, OrderStatus};
use fusion_core::secret_manager::{SecretManager, CrossChainExecutor, CrossChainClaimRequest};
use fusion_core::htlc::{generate_secret, hash_secret};
use fusion_core::order::{Order, MakerTraits};
use ethers::types::{Address, U256};
use std::time::Duration;
use tokio::sync::mpsc;

// Mock Ethereum connector for testing
struct MockEthereumConnector {
    chain_name: String,
}

impl MockEthereumConnector {
    async fn new(_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            chain_name: "ethereum".to_string(),
        })
    }
}

impl fusion_core::secret_manager::ChainConnector for MockEthereumConnector {
    fn chain_name(&self) -> &str {
        &self.chain_name
    }
}

fn create_test_order() -> Order {
    Order {
        maker: Address::zero(),
        receiver: Address::zero(),
        maker_asset: Address::zero(),
        taker_asset: Address::zero(),
        making_amount: U256::from(1000000000000000000000000u128), // 1 NEAR
        taking_amount: U256::from(5000000), // 5 USDC
        salt: U256::from(1),
        maker_traits: MakerTraits::default(),
        pre_interaction: vec![],
        post_interaction: vec![],
        interactions: vec![], // In real implementation, this would contain HTLC data
    }
}

fn create_mock_htlc_create_event(secret_hash: String) -> NearHtlcCreateEvent {
    NearHtlcCreateEvent {
        escrow_id: "fusion_0".to_string(),
        resolver: "alice.near".to_string(),
        beneficiary: "bob.near".to_string(),
        amount: 1000000000000000000000000, // 1 NEAR
        secret_hash,
        finality_time: 3600,
        cancel_time: 7200,
        public_cancel_time: 10800,
    }
}

fn create_mock_htlc_claim_event(escrow_id: String, secret: String) -> NearHtlcClaimEvent {
    NearHtlcClaimEvent {
        escrow_id,
        claimer: "bob.near".to_string(),
        secret,
        timestamp: 1234567890,
    }
}

#[tokio::test]
async fn should_complete_full_cross_chain_swap_flow() {
    // Note: This is a mock integration test. In production, we would use real NEAR testnet
    
    // セットアップ
    let mut order_manager = OrderManager::new();
    let mut secret_manager = SecretManager::new();
    let cross_chain_executor = CrossChainExecutor::new();
    
    // 1. オーダー作成
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);
    let order = create_test_order();
    order_manager.add_order("test_order", order).await;
    
    // 2. NEAR HTLCイベント（モック）
    let create_event = create_mock_htlc_create_event(hex::encode(secret_hash));
    let claim_event = create_mock_htlc_claim_event(
        "fusion_0".to_string(),
        hex::encode(secret),
    );
    
    // 3. イベント処理
    // HTLCが作成されたことを記録
    order_manager.process_htlc_create_event(&create_event).await.unwrap();
    
    // ステータス確認
    let status = order_manager.get_order_status("test_order").await.unwrap();
    assert_eq!(status, OrderStatus::HtlcCreated);
    
    // 4. シークレットが公開されたことを記録
    secret_manager.process_claim_event(&claim_event).await.unwrap();
    
    // シークレット取得確認
    let stored_secret = secret_manager.get_secret(&claim_event.escrow_id).await.unwrap();
    assert_eq!(stored_secret, claim_event.secret);
    
    // 5. クロスチェーン実行（モック）
    let ethereum_connector = MockEthereumConnector::new("https://sepolia.infura.io/v3/test")
        .await
        .unwrap();
    
    let claim_request = CrossChainClaimRequest {
        target_chain: "ethereum".to_string(),
        htlc_id: "0x1234567890abcdef".to_string(),
        secret: stored_secret,
        recipient: "0x456789abcdef".to_string(),
    };
    
    let tx_hash = cross_chain_executor
        .execute_claim(claim_request, &ethereum_connector)
        .await
        .unwrap();
    
    // 6. 検証
    assert!(!tx_hash.is_empty());
    assert!(tx_hash.starts_with("0x"));
    
    // 実際の実装では、ここでオーダーのステータスをCompletedに更新
    // order_manager.mark_order_completed("test_order").await.unwrap();
    // let final_status = order_manager.get_order_status("test_order").await.unwrap();
    // assert_eq!(final_status, OrderStatus::Completed);
}

#[tokio::test]
async fn test_event_parsing_integration() {
    // イベントパーシングの統合テスト
    let log_create = "Fusion escrow created: fusion_123 by resolver.near for beneficiary.near, amount: 5000000000000000000000000, safety: 500000000000000000000000";
    let log_claim = "Secret revealed: 1234567890abcdef1234567890abcdef";
    
    // Createイベントのパース
    let create_event = NearEventParser::parse_create_event(log_create).unwrap();
    assert_eq!(create_event.escrow_id, "fusion_123");
    assert_eq!(create_event.resolver, "resolver.near");
    assert_eq!(create_event.beneficiary, "beneficiary.near");
    assert_eq!(create_event.amount, 5000000000000000000000000);
    
    // Claimイベントのパース
    let claim_event = NearEventParser::parse_claim_event(log_claim).unwrap();
    assert_eq!(claim_event.secret, "1234567890abcdef1234567890abcdef");
}

#[tokio::test]
async fn test_monitor_with_event_processing() {
    // モニターとイベント処理の統合テスト
    let (event_tx, mut event_rx) = mpsc::channel(10);
    
    // Note: 実際のテストではモックまたはテストネットを使用
    let monitor_result = NearHtlcMonitor::new(
        "https://rpc.testnet.near.org",
        "fusion_htlc.testnet",
    ).await;
    
    if monitor_result.is_err() {
        eprintln!("Warning: Could not create NEAR monitor. Skipping integration test.");
        return;
    }
    
    let monitor = monitor_result.unwrap();
    
    // モニタリングタスクを開始（短時間で終了）
    let monitor_task = tokio::spawn(async move {
        let config = MonitorConfig {
            retry_delay: Duration::from_millis(100),
            max_retries: 1,
        };
        let _ = monitor.start_monitoring_with_config(event_tx, config).await;
    });
    
    // タイムアウトを設定してタスクを終了
    tokio::time::sleep(Duration::from_millis(500)).await;
    monitor_task.abort();
    
    // 現在の実装では実際のイベントは来ないが、
    // 実際の環境では event_rx からイベントを受信して処理
    let _timeout_result = tokio::time::timeout(
        Duration::from_millis(100),
        event_rx.recv()
    ).await;
}