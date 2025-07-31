use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use crate::order::Order;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EIP712Domain {
    pub name: String,
    pub version: String,
    pub chain_id: u64,
    pub verifying_contract: String,
}

impl EIP712Domain {
    pub fn separator(&self) -> [u8; 32] {
        let type_hash = keccak256(b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
        
        let mut hasher = Keccak256::new();
        hasher.update(type_hash);
        hasher.update(keccak256(self.name.as_bytes()));
        hasher.update(keccak256(self.version.as_bytes()));
        hasher.update(encode_uint256(self.chain_id));
        hasher.update(encode_address(&self.verifying_contract));
        
        let result = hasher.finalize();
        let mut separator = [0u8; 32];
        separator.copy_from_slice(&result);
        separator
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedData {
    pub domain: EIP712Domain,
    pub primary_type: String,
    pub message: serde_json::Value,
}

impl TypedData {
    pub fn hash(&self) -> [u8; 32] {
        let domain_separator = self.domain.separator();
        let message_hash = hash_struct(&self.primary_type, &self.message);
        
        let mut hasher = Keccak256::new();
        hasher.update(b"\x19\x01");
        hasher.update(domain_separator);
        hasher.update(message_hash);
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

pub trait OrderEIP712 {
    fn to_eip712(&self, chain_id: u64, verifying_contract: &str) -> TypedData;
}

impl OrderEIP712 for Order {
    fn to_eip712(&self, chain_id: u64, verifying_contract: &str) -> TypedData {
        let domain = EIP712Domain {
            name: "1inch Limit Order Protocol".to_string(),
            version: "3".to_string(),
            chain_id,
            verifying_contract: verifying_contract.to_string(),
        };

        let message = serde_json::json!({
            "salt": format!("0x{}", hex::encode(&self.salt)),
            "makerAsset": self.maker_asset,
            "takerAsset": self.taker_asset,
            "maker": self.maker,
            "receiver": self.receiver,
            "allowedSender": self.allowed_sender,
            "makingAmount": self.making_amount.to_string(),
            "takingAmount": self.taking_amount.to_string(),
            "offsets": self.offsets.to_string(),
            "interactions": self.interactions,
        });

        TypedData {
            domain,
            primary_type: "Order".to_string(),
            message,
        }
    }
}

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

fn encode_uint256(value: u64) -> [u8; 32] {
    let mut encoded = [0u8; 32];
    encoded[24..].copy_from_slice(&value.to_be_bytes());
    encoded
}

fn encode_address(address: &str) -> [u8; 32] {
    let mut encoded = [0u8; 32];
    let address_bytes = hex::decode(address.trim_start_matches("0x"))
        .unwrap_or_else(|_| vec![0u8; 20]);
    encoded[12..].copy_from_slice(&address_bytes[..20.min(address_bytes.len())]);
    encoded
}

fn hash_struct(type_name: &str, message: &serde_json::Value) -> [u8; 32] {
    // Simplified implementation for Order type
    if type_name == "Order" {
        let type_hash = keccak256(b"Order(uint256 salt,address makerAsset,address takerAsset,address maker,address receiver,address allowedSender,uint256 makingAmount,uint256 takingAmount,uint256 offsets,bytes interactions)");
        
        let mut hasher = Keccak256::new();
        hasher.update(type_hash);
        
        // Hash each field according to its type
        if let Some(salt) = message.get("salt").and_then(|v| v.as_str()) {
            let salt_bytes = hex::decode(salt.trim_start_matches("0x")).unwrap_or([0u8; 32].to_vec());
            let mut salt_array = [0u8; 32];
            salt_array.copy_from_slice(&salt_bytes[..32.min(salt_bytes.len())]);
            hasher.update(salt_array);
        }
        
        // Address fields
        for field in ["makerAsset", "takerAsset", "maker", "receiver", "allowedSender"] {
            if let Some(addr) = message.get(field).and_then(|v| v.as_str()) {
                hasher.update(encode_address(addr));
            }
        }
        
        // Amount fields
        for field in ["makingAmount", "takingAmount", "offsets"] {
            if let Some(amount_str) = message.get(field).and_then(|v| v.as_str()) {
                let amount = amount_str.parse::<u128>().unwrap_or(0);
                let mut encoded = [0u8; 32];
                encoded[16..].copy_from_slice(&amount.to_be_bytes());
                hasher.update(encoded);
            }
        }
        
        // Interactions (bytes)
        if let Some(interactions) = message.get("interactions").and_then(|v| v.as_str()) {
            let bytes = hex::decode(interactions.trim_start_matches("0x")).unwrap_or_default();
            hasher.update(keccak256(&bytes));
        }
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    } else {
        [0u8; 32]
    }
}