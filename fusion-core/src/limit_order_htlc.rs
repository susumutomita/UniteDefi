use crate::htlc::SecretHash;
use crate::near_limit_order::HTLCData;
use crate::order::{Order, OrderBuilder};
use anyhow::{anyhow, Result};

/// Limit OrderとHTLCを統合するための拡張トレイト
pub trait OrderHTLCExt {
    /// OrderからHTLC情報を抽出
    fn extract_htlc_data(&self) -> Result<HTLCData>;
    
    /// HTLC情報を含むかチェック
    fn has_htlc_data(&self) -> bool;
}

impl OrderHTLCExt for Order {
    fn extract_htlc_data(&self) -> Result<HTLCData> {
        // interactionsフィールドからHTLCデータを抽出
        if self.interactions.len() < 2 || !self.interactions.starts_with("0x") {
            return Err(anyhow!("Invalid interactions format"));
        }
        
        HTLCData::from_hex(&self.interactions)
    }
    
    fn has_htlc_data(&self) -> bool {
        // interactionsフィールドの長さをチェックして、HTLCデータが含まれているか判定
        if self.interactions.len() < 2 || !self.interactions.starts_with("0x") {
            return false;
        }
        
        // HTLCデータの最小サイズ（32 + 8 + 1 + 1 + 1 + 1 = 44バイト）
        // Hex文字列なので、0xプレフィックス + 88文字以上
        self.interactions.len() >= 90
    }
}

/// HTLC対応のOrderBuilderを作成するヘルパー構造体
pub struct HTLCOrderBuilder {
    builder: OrderBuilder,
    htlc_data: Option<HTLCData>,
}

impl HTLCOrderBuilder {
    pub fn new() -> Self {
        Self {
            builder: OrderBuilder::new(),
            htlc_data: None,
        }
    }

    /// HTLCデータを設定
    pub fn htlc_data(mut self, htlc_data: HTLCData) -> Self {
        self.htlc_data = Some(htlc_data);
        self
    }

    /// 基本的なOrder設定をプロキシ
    pub fn maker_asset(mut self, maker_asset: &str) -> Self {
        self.builder = self.builder.maker_asset(maker_asset);
        self
    }

    pub fn taker_asset(mut self, taker_asset: &str) -> Self {
        self.builder = self.builder.taker_asset(taker_asset);
        self
    }

    pub fn maker(mut self, maker: &str) -> Self {
        self.builder = self.builder.maker(maker);
        self
    }

    pub fn receiver(mut self, receiver: &str) -> Self {
        self.builder = self.builder.receiver(receiver);
        self
    }

    pub fn allowed_sender(mut self, allowed_sender: &str) -> Self {
        self.builder = self.builder.allowed_sender(allowed_sender);
        self
    }

    pub fn making_amount(mut self, amount: u128) -> Self {
        self.builder = self.builder.making_amount(amount);
        self
    }

    pub fn taking_amount(mut self, amount: u128) -> Self {
        self.builder = self.builder.taking_amount(amount);
        self
    }

    /// Orderをビルド（HTLCデータをinteractionsに埋め込む）
    pub fn build(self) -> Result<Order> {
        let mut builder = self.builder;
        
        // HTLCデータがある場合は、interactionsフィールドに埋め込む
        if let Some(htlc_data) = self.htlc_data {
            builder = builder.interactions(&htlc_data.to_hex());
        }
        
        builder.build()
    }
}

/// NEAR特有のオーダー作成関数
pub fn create_near_to_ethereum_order(
    near_account: &str,
    ethereum_address: &str,
    near_amount: u128,
    usdc_amount: u128,
    secret_hash: SecretHash,
    timeout: u64,
) -> Result<Order> {
    // NEAR testnetのトークンアドレス（仮）
    const NEAR_TOKEN: &str = "near.testnet";
    // Base SepoliaのUSDCアドレス（仮）
    const USDC_TOKEN: &str = "0x036CbD53842c5426634e7929541eC2318f3dCF7e";
    
    // HTLCデータを作成
    let htlc_data = HTLCData::new(
        secret_hash,
        timeout,
        "near".to_string(),
        near_account.to_string(),
    )?;
    
    // Orderを作成
    HTLCOrderBuilder::new()
        .maker_asset(NEAR_TOKEN)
        .taker_asset(USDC_TOKEN)
        .maker(near_account)
        .receiver(ethereum_address)
        .making_amount(near_amount)
        .taking_amount(usdc_amount)
        .htlc_data(htlc_data)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::htlc::{generate_secret, hash_secret};

    #[test]
    fn test_htlc_order_builder() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        
        let htlc_data = HTLCData::new(
            secret_hash,
            3600,
            "near".to_string(),
            "alice.near".to_string(),
        ).unwrap();
        
        let order = HTLCOrderBuilder::new()
            .maker_asset("near.testnet")
            .taker_asset("0x036CbD53842c5426634e7929541eC2318f3dCF7e")
            .maker("alice.near")
            .receiver("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
            .making_amount(1_000_000_000_000_000_000_000_000) // 1 NEAR
            .taking_amount(5_000_000) // 5 USDC
            .htlc_data(htlc_data)
            .build()
            .unwrap();
        
        // HTLCデータが含まれていることを確認
        assert!(order.has_htlc_data());
        
        // HTLCデータを抽出して検証
        let extracted = order.extract_htlc_data().unwrap();
        assert_eq!(extracted.secret_hash, secret_hash);
        assert_eq!(extracted.timeout, 3600);
        assert_eq!(extracted.recipient_chain, "near");
        assert_eq!(extracted.recipient_address, "alice.near");
    }

    #[test]
    fn test_order_without_htlc_data() {
        let order = OrderBuilder::new()
            .maker_asset("near.testnet")
            .taker_asset("0x036CbD53842c5426634e7929541eC2318f3dCF7e")
            .maker("alice.near")
            .making_amount(1_000_000_000_000_000_000_000_000)
            .taking_amount(5_000_000)
            .build()
            .unwrap();
        
        // HTLCデータが含まれていないことを確認
        assert!(!order.has_htlc_data());
        
        // HTLCデータの抽出は失敗するはず
        assert!(order.extract_htlc_data().is_err());
    }

    #[test]
    fn test_create_near_to_ethereum_order() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        
        let order = create_near_to_ethereum_order(
            "alice.near",
            "0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0",
            1_000_000_000_000_000_000_000_000, // 1 NEAR
            5_000_000, // 5 USDC
            secret_hash,
            3600,
        ).unwrap();
        
        assert_eq!(order.maker(), "alice.near");
        assert_eq!(order.receiver, "0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0");
        assert_eq!(order.making_amount(), 1_000_000_000_000_000_000_000_000);
        assert_eq!(order.taking_amount(), 5_000_000);
        
        // HTLCデータを検証
        let htlc_data = order.extract_htlc_data().unwrap();
        assert_eq!(htlc_data.secret_hash, secret_hash);
        assert_eq!(htlc_data.timeout, 3600);
        assert_eq!(htlc_data.recipient_chain, "near");
        assert_eq!(htlc_data.recipient_address, "alice.near");
    }

    #[test]
    fn test_order_with_custom_interactions() {
        // 既存のinteractionsデータがある場合の動作確認
        let order = OrderBuilder::new()
            .maker_asset("token1")
            .taker_asset("token2")
            .maker("maker")
            .making_amount(100)
            .taking_amount(200)
            .interactions("0x1234567890") // カスタムinteractions
            .build()
            .unwrap();
        
        // HTLCデータとして解釈できないことを確認
        assert!(!order.has_htlc_data());
        assert!(order.extract_htlc_data().is_err());
    }
}