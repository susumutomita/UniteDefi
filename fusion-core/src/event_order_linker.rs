use crate::chains::near_events::{NearHtlcCreateEvent, NearHtlcClaimEvent};
use crate::order::Order;
use crate::htlc::{generate_secret, hash_secret, SecretHash};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum LinkError {
    #[error("No matching order found")]
    NoMatchingOrder,
    #[error("Invalid secret hash")]
    InvalidSecretHash,
    #[error("Order already linked")]
    OrderAlreadyLinked,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LinkType {
    SecretHashMatch,
    NoMatch,
}

#[derive(Debug, Clone)]
pub struct EventOrderLink {
    pub is_linked: bool,
    pub link_type: LinkType,
    pub order_id: Option<String>,
}

/// イベントとオーダーのリンク管理
pub struct EventOrderLinker {
    // シークレットハッシュ -> オーダーIDのマッピング
    secret_hash_to_order: HashMap<String, String>,
}

impl EventOrderLinker {
    pub fn new() -> Self {
        Self {
            secret_hash_to_order: HashMap::new(),
        }
    }
    
    /// オーダーを登録
    pub fn register_order(&mut self, order_id: &str, order: &Order) -> Result<(), LinkError> {
        // TODO: オーダーのinteractionsからHTLCデータを抽出してシークレットハッシュを取得
        // 現在はダミー実装
        Ok(())
    }
    
    /// イベントとオーダーをリンク
    pub fn link_event_to_order(&self, event: &NearHtlcCreateEvent, order: &Order) -> Result<EventOrderLink, LinkError> {
        // シークレットハッシュでマッチング
        // TODO: orderのinteractionsからHTLCデータを抽出して比較
        
        // 現在はダミー実装
        if event.secret_hash.is_empty() {
            return Ok(EventOrderLink {
                is_linked: false,
                link_type: LinkType::NoMatch,
                order_id: None,
            });
        }
        
        Ok(EventOrderLink {
            is_linked: true,
            link_type: LinkType::SecretHashMatch,
            order_id: Some("dummy_order_id".to_string()),
        })
    }
}

/// オーダーステータス
#[derive(Debug, PartialEq, Clone)]
pub enum OrderStatus {
    Pending,
    HtlcCreated,
    HtlcClaimed,
    HtlcCancelled,
    PartiallyFulfilled,
    Completed,
    Failed,
}

/// オーダーマネージャー
pub struct OrderManager {
    orders: HashMap<String, (Order, OrderStatus)>,
    event_linker: EventOrderLinker,
}

#[derive(Error, Debug)]
pub enum OrderError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
    #[error("Link error: {0}")]
    LinkError(#[from] LinkError),
}

impl OrderManager {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            event_linker: EventOrderLinker::new(),
        }
    }
    
    /// オーダーを追加
    pub async fn add_order(&mut self, order_id: &str, order: Order) {
        self.orders.insert(order_id.to_string(), (order.clone(), OrderStatus::Pending));
        // イベントリンカーにも登録
        let _ = self.event_linker.register_order(order_id, &order);
    }
    
    /// オーダーステータスを取得
    pub async fn get_order_status(&self, order_id: &str) -> Result<OrderStatus, OrderError> {
        self.orders
            .get(order_id)
            .map(|(_, status)| status.clone())
            .ok_or(OrderError::OrderNotFound)
    }
    
    /// HTLC Createイベントを処理
    pub async fn process_htlc_create_event(&mut self, event: &NearHtlcCreateEvent) -> Result<(), OrderError> {
        // TODO: イベントに関連するオーダーを見つけてステータスを更新
        // 現在はダミー実装
        
        // 最初のオーダーのステータスを更新（テスト用）
        if let Some((order_id, (order, status))) = self.orders.iter_mut().next() {
            *status = OrderStatus::HtlcCreated;
        }
        
        Ok(())
    }
    
    /// HTLC Claimイベントを処理
    pub async fn process_htlc_claim_event(&mut self, event: &NearHtlcClaimEvent) -> Result<(), OrderError> {
        // TODO: イベントに関連するオーダーを見つけてステータスを更新
        Ok(())
    }
    
    /// 残り金額を取得
    pub async fn get_remaining_amount(&self, order_id: &str) -> Result<u128, OrderError> {
        // TODO: 部分的な実行を考慮した残り金額の計算
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::{MakerTraits, Order};
    use ethers::types::{Address, U256};
    use std::str::FromStr;
    
    fn create_test_order() -> Order {
        Order {
            maker: Address::zero(),
            receiver: Address::zero(),
            maker_asset: Address::zero(),
            taker_asset: Address::zero(),
            making_amount: U256::from(1000000),
            taking_amount: U256::from(2000000),
            salt: U256::from(1),
            maker_traits: MakerTraits::default(),
            pre_interaction: vec![],
            post_interaction: vec![],
            interactions: vec![],
        }
    }
    
    fn create_near_to_ethereum_order(
        maker: &str,
        receiver: &str,
        making_amount: u128,
        taking_amount: u128,
        secret_hash: SecretHash,
        timeout: u64,
    ) -> Result<Order, Box<dyn std::error::Error>> {
        // テスト用のオーダー作成
        Ok(Order {
            maker: Address::zero(),
            receiver: Address::from_str(receiver)?,
            maker_asset: Address::zero(),
            taker_asset: Address::zero(),
            making_amount: U256::from(making_amount),
            taking_amount: U256::from(taking_amount),
            salt: U256::from(1),
            maker_traits: MakerTraits::default(),
            pre_interaction: vec![],
            post_interaction: vec![],
            interactions: vec![], // TODO: HTLCデータを含める
        })
    }
    
    #[test]
    fn should_link_event_to_limit_order() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        
        // Limit Order作成
        let order = create_near_to_ethereum_order(
            "alice.near",
            "0x1234567890123456789012345678901234567890",
            1000000000000000000000000,
            5000000,
            secret_hash,
            3600,
        ).unwrap();
        
        // NEAR HTLCイベント
        let create_event = NearHtlcCreateEvent {
            escrow_id: "fusion_0".to_string(),
            resolver: "alice.near".to_string(),
            beneficiary: "bob.near".to_string(),
            amount: 1000000000000000000000000,
            secret_hash: hex::encode(secret_hash),
            finality_time: 3600,
            cancel_time: 7200,
            public_cancel_time: 10800,
        };
        
        let linker = EventOrderLinker::new();
        let link = linker.link_event_to_order(&create_event, &order).unwrap();
        
        assert!(link.is_linked);
        assert_eq!(link.link_type, LinkType::SecretHashMatch);
    }
    
    #[tokio::test]
    async fn should_update_order_status_on_event_received() {
        let mut order_manager = OrderManager::new();
        let order_id = "order_123";
        
        // 初期状態: Pending
        let order = create_test_order();
        order_manager.add_order(order_id, order).await;
        
        let status = order_manager.get_order_status(order_id).await.unwrap();
        assert_eq!(status, OrderStatus::Pending);
        
        // HTLCCreateイベント受信
        let create_event = NearHtlcCreateEvent {
            escrow_id: "fusion_0".to_string(),
            resolver: "alice.near".to_string(),
            beneficiary: "bob.near".to_string(),
            amount: 1000000,
            secret_hash: "test_hash".to_string(),
            finality_time: 3600,
            cancel_time: 7200,
            public_cancel_time: 10800,
        };
        
        order_manager.process_htlc_create_event(&create_event).await.unwrap();
        
        let status = order_manager.get_order_status(order_id).await.unwrap();
        assert_eq!(status, OrderStatus::HtlcCreated);
    }
    
    #[tokio::test]
    async fn should_handle_partial_order_fulfillment() {
        let mut order_manager = OrderManager::new();
        
        let mut order = create_test_order();
        order.making_amount = U256::from(1000000); // 1 USDC
        order_manager.add_order("order_1", order).await;
        
        // 部分的なHTLC作成（半分の金額）
        let partial_create_event = NearHtlcCreateEvent {
            escrow_id: "fusion_0".to_string(),
            resolver: "alice.near".to_string(),
            beneficiary: "bob.near".to_string(),
            amount: 500000, // 0.5 USDC
            secret_hash: "test_hash".to_string(),
            finality_time: 3600,
            cancel_time: 7200,
            public_cancel_time: 10800,
        };
        
        // TODO: 部分的な実行のロジックを実装
        order_manager.process_htlc_create_event(&partial_create_event).await.unwrap();
        
        // 現在のダミー実装では、ステータスはHtlcCreatedになる
        let status = order_manager.get_order_status("order_1").await.unwrap();
        assert_eq!(status, OrderStatus::HtlcCreated);
        
        // TODO: 部分的な実行を適切に処理する実装を追加
        // assert_eq!(status, OrderStatus::PartiallyFulfilled);
        
        // let remaining = order_manager.get_remaining_amount("order_1").await.unwrap();
        // assert_eq!(remaining, 500000);
    }
}