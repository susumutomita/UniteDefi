use crate::htlc::SecretHash;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionReceipt, U256};
use std::sync::Arc;

pub mod abi;
pub mod events;
pub mod event_storage;

pub struct EthereumConnector {
    provider: Arc<Provider<Http>>,
    factory_address: Address,
    signer: Option<LocalWallet>,
}

impl EthereumConnector {
    pub fn new(rpc_url: &str, factory_address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let factory_address = factory_address.parse::<Address>()?;

        Ok(Self {
            provider: Arc::new(provider),
            factory_address,
            signer: None,
        })
    }

    pub fn with_signer(mut self, private_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let wallet = private_key.parse::<LocalWallet>()?;
        self.signer = Some(wallet);
        Ok(self)
    }

    pub async fn create_escrow(
        &self,
        token: Address,
        amount: U256,
        secret_hash: SecretHash,
        timeout: U256,
        recipient: Address,
    ) -> Result<Address, Box<dyn std::error::Error>> {
        let signer = self.signer.as_ref().ok_or("Signer not configured")?;

        let client =
            SignerMiddleware::new(self.provider.clone(), signer.clone().with_chain_id(1u64));

        // Foundryで生成されたABIバインディングを使用
        let factory = abi::factory::IEscrowFactory::new(self.factory_address, Arc::new(client));

        // secret_hashを[u8; 32]から[u8; 32]に変換（すでに同じ型）
        let secret_hash_bytes = secret_hash;

        // create_escrowを呼び出し
        let tx = factory.create_escrow(token, amount, secret_hash_bytes, timeout, recipient);

        // ETHを送る場合はvalueを設定
        let tx = if token == Address::zero() {
            tx.value(amount)
        } else {
            tx
        };

        let pending_tx = tx.send().await?;
        let receipt = pending_tx.await?.ok_or("Transaction failed")?;

        // イベントからescrowアドレスを取得
        // EscrowCreatedイベントの2番目のトピックがescrowアドレス
        for log in receipt.logs {
            if log.topics.len() >= 3 {
                // topic[0]: イベントシグネチャ
                // topic[1]: escrowId (indexed)
                // topic[2]: escrow address (indexed)
                let escrow_address = Address::from(log.topics[2]);
                return Ok(escrow_address);
            }
        }

        Err("Escrow address not found in logs".into())
    }

    pub async fn claim_escrow(
        &self,
        escrow_address: Address,
        secret: [u8; 32],
    ) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        let signer = self.signer.as_ref().ok_or("Signer not configured")?;

        let client =
            SignerMiddleware::new(self.provider.clone(), signer.clone().with_chain_id(1u64));

        // Escrowコントラクトに接続
        let escrow = abi::escrow::IEscrow::new(escrow_address, Arc::new(client));

        // claimを実行
        let tx = escrow.claim(secret);
        let pending_tx = tx.send().await?;
        let receipt = pending_tx.await?.ok_or("Transaction failed")?;

        Ok(receipt)
    }

    pub async fn refund_escrow(
        &self,
        escrow_address: Address,
    ) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        let signer = self.signer.as_ref().ok_or("Signer not configured")?;

        let client =
            SignerMiddleware::new(self.provider.clone(), signer.clone().with_chain_id(1u64));

        // Escrowコントラクトに接続
        let escrow = abi::escrow::IEscrow::new(escrow_address, Arc::new(client));

        // refundを実行
        let tx = escrow.refund();
        let pending_tx = tx.send().await?;
        let receipt = pending_tx.await?.ok_or("Transaction failed")?;

        Ok(receipt)
    }
}

// For testing on Sepolia
pub const SEPOLIA_RPC: &str = "https://sepolia.infura.io/v3/YOUR_INFURA_KEY";
pub const ESCROW_FACTORY_SEPOLIA: &str = "0x0000000000000000000000000000000000000000"; // TODO: Get actual address after deployment

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethereum_connector_creation() {
        let connector = EthereumConnector::new(
            "https://sepolia.infura.io/v3/test",
            "0x0000000000000000000000000000000000000000",
        );
        assert!(connector.is_ok());
    }

    #[test]
    fn test_with_signer() {
        let connector = EthereumConnector::new(
            "https://sepolia.infura.io/v3/test",
            "0x0000000000000000000000000000000000000000",
        )
        .unwrap();

        // Test private key (NOT FOR PRODUCTION)
        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let result = connector.with_signer(private_key);
        assert!(result.is_ok());
    }
}
