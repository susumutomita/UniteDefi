use crate::chains::ethereum::limit_order_abi::{Order as OneinchOrder, LimitOrderProtocol};
use crate::eip712::{OrderEip712, EIP712_DOMAIN};
use ethers::prelude::*;
use ethers::types::transaction::eip712::{Eip712, TypedData};
use std::sync::Arc;
use anyhow::Result;

/// 1inch Fusion+ integration module
/// Provides proper integration with 1inch Limit Order Protocol
pub struct OneInchFusionAdapter {
    limit_order_protocol: LimitOrderProtocol,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    chain_id: u64,
}

impl OneInchFusionAdapter {
    pub fn new(
        protocol_address: Address,
        provider: Arc<Provider<Http>>,
        signer: LocalWallet,
        chain_id: u64,
    ) -> Self {
        let client = Arc::new(SignerMiddleware::new(provider.clone(), signer));
        let limit_order_protocol = LimitOrderProtocol::new(protocol_address, provider);
        
        Self {
            limit_order_protocol,
            signer: client,
            chain_id,
        }
    }

    /// Create a 1inch Fusion+ order with HTLC extension
    pub async fn create_fusion_order(
        &self,
        maker_asset: Address,
        taker_asset: Address,
        making_amount: U256,
        taking_amount: U256,
        maker: Address,
        receiver: Address,
        secret_hash: [u8; 32],
        timeout: u64,
    ) -> Result<FusionOrder> {
        // Build order with Fusion+ features
        let order = self.build_fusion_order(
            maker_asset,
            taker_asset,
            making_amount,
            taking_amount,
            maker,
            receiver,
            secret_hash,
            timeout,
        )?;

        // Sign order using EIP-712
        let signature = self.sign_order(&order).await?;

        Ok(FusionOrder {
            order,
            signature,
            secret_hash,
            fusion_data: FusionData {
                dutch_auction_start_amount: taking_amount,
                dutch_auction_end_amount: taking_amount * 95 / 100, // 5% slippage
                auction_duration: 300, // 5 minutes
                partial_fill_allowed: true,
                mev_protection: true,
            },
        })
    }

    /// Build order with Fusion+ extensions
    fn build_fusion_order(
        &self,
        maker_asset: Address,
        taker_asset: Address,
        making_amount: U256,
        taking_amount: U256,
        maker: Address,
        receiver: Address,
        secret_hash: [u8; 32],
        timeout: u64,
    ) -> Result<OneinchOrder> {
        // Generate salt with timestamp for uniqueness
        let salt = U256::from(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs());

        // Encode HTLC data in maker_traits
        // Bits 0-63: timeout timestamp
        // Bits 64-255: secret hash
        let timeout_timestamp = U256::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() + timeout
        );
        
        let mut maker_traits = timeout_timestamp;
        maker_traits |= U256::from(secret_hash) << 64;

        // Add Fusion+ flags
        // Bit 255: Dutch auction enabled
        // Bit 254: Partial fill enabled
        // Bit 253: MEV protection enabled
        maker_traits |= U256::from(1) << 255; // Dutch auction
        maker_traits |= U256::from(1) << 254; // Partial fill
        maker_traits |= U256::from(1) << 253; // MEV protection

        Ok(OneinchOrder {
            salt,
            maker,
            receiver,
            maker_asset,
            taker_asset,
            making_amount,
            taking_amount,
            maker_traits,
        })
    }

    /// Sign order using EIP-712
    async fn sign_order(&self, order: &OneinchOrder) -> Result<Bytes> {
        let order_hash = order.hash();
        let domain_separator = self.compute_domain_separator();
        
        // Create EIP-712 typed data
        let typed_data = TypedData {
            domain: EIP712_DOMAIN.clone(),
            primary_type: "Order".to_string(),
            message: serde_json::to_value(&OrderEip712::from(order.clone()))?,
            types: serde_json::from_str(include_str!("../../resources/order_types.json"))?,
        };

        // Sign with wallet
        let signature = self.signer.signer().sign_typed_data(&typed_data).await?;
        
        Ok(signature.to_vec().into())
    }

    /// Submit order to 1inch Fusion API
    pub async fn submit_to_fusion_api(&self, fusion_order: &FusionOrder) -> Result<String> {
        // In production, this would submit to 1inch Fusion API
        // For hackathon, we simulate the submission
        let order_hash = fusion_order.order.hash();
        
        // Log order submission
        log::info!(
            "Submitting Fusion+ order to 1inch API: {:?}",
            hex::encode(order_hash)
        );

        // TODO: Actual API integration
        // let response = reqwest::Client::new()
        //     .post("https://fusion.1inch.io/v3.0/84532/order/submit")
        //     .json(&fusion_order)
        //     .send()
        //     .await?;

        Ok(hex::encode(order_hash))
    }

    /// Monitor order execution
    pub async fn monitor_order_execution(
        &self,
        order_hash: H256,
    ) -> Result<OrderStatus> {
        // Check order status on-chain
        let remaining = self
            .limit_order_protocol
            .remaining_invalidator_for_order(Address::zero(), order_hash)
            .call()
            .await?;

        if remaining == U256::zero() {
            Ok(OrderStatus::Filled)
        } else if remaining == U256::max_value() {
            Ok(OrderStatus::Cancelled)
        } else {
            Ok(OrderStatus::PartiallyFilled(remaining))
        }
    }

    fn compute_domain_separator(&self) -> H256 {
        // Compute EIP-712 domain separator
        let domain = serde_json::json!({
            "name": "1inch Limit Order Protocol",
            "version": "4",
            "chainId": self.chain_id,
            "verifyingContract": self.limit_order_protocol.address
        });

        ethers::utils::keccak256(serde_json::to_vec(&domain).unwrap()).into()
    }
}

#[derive(Debug, Clone)]
pub struct FusionOrder {
    pub order: OneinchOrder,
    pub signature: Bytes,
    pub secret_hash: [u8; 32],
    pub fusion_data: FusionData,
}

#[derive(Debug, Clone)]
pub struct FusionData {
    pub dutch_auction_start_amount: U256,
    pub dutch_auction_end_amount: U256,
    pub auction_duration: u64,
    pub partial_fill_allowed: bool,
    pub mev_protection: bool,
}

#[derive(Debug, Clone)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled(U256),
    Filled,
    Cancelled,
    Expired,
}

/// Extension trait for HTLC integration
pub trait HTLCExtension {
    fn encode_htlc_data(secret_hash: [u8; 32], timeout: u64) -> Vec<u8>;
    fn decode_htlc_data(data: &[u8]) -> Result<(H256, u64)>;
}

impl HTLCExtension for FusionOrder {
    fn encode_htlc_data(secret_hash: [u8; 32], timeout: u64) -> Vec<u8> {
        ethers::abi::encode(&[
            ethers::abi::Token::FixedBytes(secret_hash.to_vec()),
            ethers::abi::Token::Uint(U256::from(timeout)),
        ])
    }

    fn decode_htlc_data(data: &[u8]) -> Result<(H256, u64)> {
        let decoded = ethers::abi::decode(
            &[
                ethers::abi::ParamType::FixedBytes(32),
                ethers::abi::ParamType::Uint(256),
            ],
            data,
        )?;

        let secret_hash = H256::from_slice(&decoded[0].clone().into_fixed_bytes().unwrap());
        let timeout = decoded[1].clone().into_uint().unwrap().as_u64();

        Ok((secret_hash, timeout))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htlc_encoding() {
        let secret_hash = [1u8; 32];
        let timeout = 3600u64;

        let encoded = FusionOrder::encode_htlc_data(secret_hash, timeout);
        let (decoded_hash, decoded_timeout) = FusionOrder::decode_htlc_data(&encoded).unwrap();

        assert_eq!(H256::from(secret_hash), decoded_hash);
        assert_eq!(timeout, decoded_timeout);
    }
}