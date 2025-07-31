use crate::chains::ethereum::EthereumConnector;
use crate::chains::near::NEARConnector;
use crate::htlc::{Secret, SecretHash};
use crate::limit_order_htlc::OrderHTLCExt;
use crate::order::Order;
use anyhow::{anyhow, Result};
use ethers::types::{Address, U256};
use std::str::FromStr;

/// クロスチェーン実行フローを管理する構造体
pub struct CrossChainExecutor {
    ethereum_connector: EthereumConnector,
    near_connector: NEARConnector,
}

/// 実行フローの状態
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    /// 初期状態
    Initialized,
    /// Ethereumでオーダーがフィルされた
    OrderFilled {
        tx_hash: String,
        block_number: u64,
    },
    /// NEARでHTLCが作成された
    HTLCCreated {
        escrow_id: String,
        secret_hash: SecretHash,
    },
    /// シークレットが公開された
    SecretRevealed {
        secret: Secret,
        ethereum_claim_tx: String,
    },
    /// NEARでクレームされた
    Completed {
        near_claim_tx: String,
    },
    /// エラーが発生した
    Failed {
        reason: String,
    },
}

/// クロスチェーン実行のパラメータ
pub struct ExecutionParams {
    /// 実行するLimit Order
    pub order: Order,
    /// Ethereum上のLimit Orderプロトコルアドレス
    pub limit_order_protocol: String,
    /// NEAR上のHTLCコントラクトID
    pub near_htlc_contract: String,
}

impl CrossChainExecutor {
    pub fn new(
        ethereum_rpc: &str,
        ethereum_factory: &str,
        near_rpc: &str,
    ) -> Result<Self> {
        let ethereum_connector = EthereumConnector::new(ethereum_rpc, ethereum_factory)?;
        let near_connector = NEARConnector::new(near_rpc);
        
        Ok(Self {
            ethereum_connector,
            near_connector,
        })
    }

    /// 秘密鍵を設定（Ethereum用）
    pub fn with_ethereum_signer(mut self, private_key: &str) -> Result<Self> {
        self.ethereum_connector = self.ethereum_connector.with_signer(private_key)?;
        Ok(self)
    }

    /// NEARコントラクトを設定
    pub fn with_near_contract(mut self, contract_id: &str) -> Self {
        self.near_connector = self.near_connector.with_contract(contract_id);
        self
    }

    /// オーダーのフィル状態を監視
    pub async fn monitor_order_fill(
        &self,
        order: &Order,
        limit_order_protocol: &str,
    ) -> Result<(String, u64)> {
        // TODO: 実際のイベント監視を実装
        // ここでは、Limit OrderプロトコルのOrderFilledイベントを監視する
        
        // 仮の実装
        Ok(("0x1234567890abcdef".to_string(), 12345))
    }

    /// NEARでHTLCを作成
    pub async fn create_near_htlc(
        &self,
        order: &Order,
        secret_hash: SecretHash,
    ) -> Result<String> {
        // HTLCデータを抽出
        let htlc_data = order.extract_htlc_data()?;
        
        // NEARでHTLCを作成
        let escrow_id = self.near_connector.create_escrow(
            order.taking_amount(),
            secret_hash,
            htlc_data.timeout,
            htlc_data.recipient_address,
        ).await?;
        
        Ok(escrow_id)
    }

    /// Ethereumでシークレットを監視
    pub async fn monitor_secret_reveal(
        &self,
        escrow_address: &str,
    ) -> Result<Secret> {
        // TODO: 実際のイベント監視を実装
        // EthereumのEscrowコントラクトでClaimedイベントを監視し、
        // シークレットを取得する
        
        // 仮の実装
        let secret = [0u8; 32];
        Ok(secret)
    }

    /// NEARでクレーム実行
    pub async fn claim_near_htlc(
        &self,
        escrow_id: &str,
        secret: Secret,
    ) -> Result<String> {
        // Hex形式に変換
        let secret_hex = hex::encode(secret);
        
        // NEARでクレーム
        let tx_id = self.near_connector.claim_escrow(escrow_id, secret).await?;
        
        Ok(tx_id)
    }

    /// クロスチェーン実行フローを実行
    pub async fn execute_cross_chain_swap(
        &mut self,
        params: ExecutionParams,
    ) -> Result<ExecutionState> {
        // HTLCデータを抽出
        let htlc_data = params.order.extract_htlc_data()
            .map_err(|e| anyhow!("Failed to extract HTLC data: {}", e))?;
        
        // 1. Ethereumでオーダーのフィルを監視
        let (tx_hash, block_number) = self.monitor_order_fill(
            &params.order,
            &params.limit_order_protocol,
        ).await?;
        
        let state = ExecutionState::OrderFilled {
            tx_hash: tx_hash.clone(),
            block_number,
        };
        
        // 2. NEARでHTLCを作成
        let escrow_id = self.create_near_htlc(
            &params.order,
            htlc_data.secret_hash,
        ).await?;
        
        let state = ExecutionState::HTLCCreated {
            escrow_id: escrow_id.clone(),
            secret_hash: htlc_data.secret_hash,
        };
        
        // 3. Ethereumでシークレットの公開を監視
        // 実際の実装では、オーダーフィルトランザクションから
        // Escrowアドレスを取得する必要がある
        let escrow_address = "0x0000000000000000000000000000000000000000"; // 仮のアドレス
        let secret = self.monitor_secret_reveal(escrow_address).await?;
        
        let state = ExecutionState::SecretRevealed {
            secret,
            ethereum_claim_tx: tx_hash,
        };
        
        // 4. NEARでクレーム実行
        let near_claim_tx = self.claim_near_htlc(&escrow_id, secret).await?;
        
        Ok(ExecutionState::Completed { near_claim_tx })
    }
}

/// 価格変換ユーティリティ
pub mod price_conversion {
    use anyhow::Result;

    /// NEAR/USDC価格をETH/USDC価格に変換
    pub fn convert_near_to_eth_price(
        near_amount: u128,
        near_usdc_price: f64,
        eth_usdc_price: f64,
    ) -> Result<u128> {
        // NEAR量をUSDC価値に変換
        let usdc_value = (near_amount as f64) * near_usdc_price;
        
        // USDC価値をETH量に変換
        let eth_amount = usdc_value / eth_usdc_price;
        
        Ok(eth_amount as u128)
    }
    
    /// 価格レートを適用してオーダー量を計算
    pub fn calculate_order_amounts(
        near_amount: u128,
        near_decimals: u8,
        usdc_decimals: u8,
        near_usdc_price: f64,
        slippage_bps: u16, // basis points (100 = 1%)
    ) -> (u128, u128) {
        // NEARをUSDCに変換（デシマル調整込み）
        let near_in_units = near_amount as f64 / 10f64.powi(near_decimals as i32);
        let usdc_value = near_in_units * near_usdc_price;
        
        // スリッページを適用
        let slippage_factor = 1.0 - (slippage_bps as f64 / 10000.0);
        let usdc_with_slippage = usdc_value * slippage_factor;
        
        // USDCデシマルに変換
        let usdc_amount = (usdc_with_slippage * 10f64.powi(usdc_decimals as i32)) as u128;
        
        (near_amount, usdc_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::htlc::{generate_secret, hash_secret};
    use crate::limit_order_htlc::{create_near_to_ethereum_order};

    #[test]
    fn test_execution_state_transitions() {
        let state = ExecutionState::Initialized;
        assert_eq!(state, ExecutionState::Initialized);
        
        let state = ExecutionState::OrderFilled {
            tx_hash: "0x123".to_string(),
            block_number: 100,
        };
        
        match state {
            ExecutionState::OrderFilled { tx_hash, block_number } => {
                assert_eq!(tx_hash, "0x123");
                assert_eq!(block_number, 100);
            }
            _ => panic!("Unexpected state"),
        }
    }

    #[test]
    fn test_cross_chain_executor_creation() {
        let executor = CrossChainExecutor::new(
            "https://eth-sepolia.example.com",
            "0x0000000000000000000000000000000000000000",
            "https://rpc.testnet.near.org",
        );
        
        assert!(executor.is_ok());
    }

    #[test]
    fn test_price_conversion() {
        use price_conversion::*;
        
        // 1 NEAR = $5, 1 ETH = $2000
        let near_amount = 1_000_000_000_000_000_000_000_000; // 1 NEAR (24 decimals)
        let near_usdc_price = 5.0;
        let eth_usdc_price = 2000.0;
        
        let eth_amount = convert_near_to_eth_price(
            near_amount,
            near_usdc_price,
            eth_usdc_price,
        ).unwrap();
        
        // 1 NEAR * $5 / $2000 = 0.0025 ETH
        // 0.0025 ETH = 2_500_000_000_000_000 wei
        assert_eq!(eth_amount, 2_500_000_000_000_000);
    }

    #[test]
    fn test_calculate_order_amounts() {
        use price_conversion::*;
        
        // 10 NEAR at $5 per NEAR with 1% slippage
        let near_amount = 10_000_000_000_000_000_000_000_000; // 10 NEAR
        let (making_amount, taking_amount) = calculate_order_amounts(
            near_amount,
            24, // NEAR decimals
            6,  // USDC decimals
            5.0, // $5 per NEAR
            100, // 1% slippage
        );
        
        assert_eq!(making_amount, near_amount);
        // 10 NEAR * $5 * 0.99 = $49.50 = 49,500,000 (6 decimals)
        assert_eq!(taking_amount, 49_500_000);
    }

    #[test]
    fn test_extract_htlc_from_order() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        
        let order = create_near_to_ethereum_order(
            "alice.near",
            "0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0",
            1_000_000_000_000_000_000_000_000,
            5_000_000,
            secret_hash,
            3600,
        ).unwrap();
        
        let executor = CrossChainExecutor::new(
            "https://eth-sepolia.example.com",
            "0x0000000000000000000000000000000000000000",
            "https://rpc.testnet.near.org",
        ).unwrap();
        
        // HTLCデータを抽出できることを確認
        let htlc_data = order.extract_htlc_data().unwrap();
        assert_eq!(htlc_data.secret_hash, secret_hash);
        assert_eq!(htlc_data.timeout, 3600);
    }
}