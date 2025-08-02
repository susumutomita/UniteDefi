use ethers::abi::Token;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::utils::keccak256;
use std::sync::Arc;

// Order struct following 1inch Limit Order Protocol V4
#[derive(Debug, Clone, Default, EthAbiType, EthAbiCodec)]
pub struct Order {
    pub salt: U256,
    pub maker: Address,
    pub receiver: Address,
    pub maker_asset: Address,
    pub taker_asset: Address,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub maker_traits: U256,
}

// LimitOrderProtocol ABI would be generated here in production
// For now, using stub implementation below

impl Order {
    pub fn hash(&self) -> H256 {
        // Using the same type hash as 1inch Limit Order Protocol V4
        let type_hash = keccak256(
            "Order(uint256 salt,address maker,address receiver,address makerAsset,address takerAsset,uint256 makingAmount,uint256 takingAmount,uint256 makerTraits)"
        );

        let encoded = ethers::abi::encode(&[
            Token::FixedBytes(type_hash.to_vec()),
            Token::Uint(self.salt),
            Token::Address(self.maker),
            Token::Address(self.receiver),
            Token::Address(self.maker_asset),
            Token::Address(self.taker_asset),
            Token::Uint(self.making_amount),
            Token::Uint(self.taking_amount),
            Token::Uint(self.maker_traits),
        ]);

        keccak256(&encoded).into()
    }
}

// Stub for LimitOrderProtocol contract binding
// In production, this would be generated from ABI
pub struct LimitOrderProtocol {
    #[allow(dead_code)]
    address: Address,
    client: Arc<Provider<Http>>,
}

impl LimitOrderProtocol {
    pub fn new(address: Address, client: Arc<Provider<Http>>) -> Self {
        Self { address, client }
    }

    pub fn remaining_invalidator_for_order(
        &self,
        _maker: Address,
        _order_hash: H256,
    ) -> RemainingCall {
        RemainingCall {
            client: self.client.clone(),
        }
    }
}

pub struct RemainingCall {
    #[allow(dead_code)]
    client: Arc<Provider<Http>>,
}

impl RemainingCall {
    pub async fn call(&self) -> Result<U256, Box<dyn std::error::Error>> {
        // Stub implementation - returns default value
        Ok(U256::from(1000000))
    }
}
