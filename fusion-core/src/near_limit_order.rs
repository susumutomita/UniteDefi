use crate::htlc::SecretHash;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// HTLC情報をLimit Orderに埋め込むための構造体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HTLCData {
    /// シークレットハッシュ（32バイト）
    pub secret_hash: SecretHash,
    /// タイムアウト（秒単位）
    pub timeout: u64,
    /// 受取人のチェーン識別子（例: "near", "ethereum"）
    pub recipient_chain: String,
    /// 受取人のアドレス（チェーン固有のフォーマット）
    pub recipient_address: String,
}

impl HTLCData {
    /// HTLCDataを作成
    pub fn new(
        secret_hash: SecretHash,
        timeout: u64,
        recipient_chain: String,
        recipient_address: String,
    ) -> Result<Self> {
        // 入力検証
        if recipient_chain.is_empty() {
            return Err(anyhow!("Recipient chain cannot be empty"));
        }
        if recipient_address.is_empty() {
            return Err(anyhow!("Recipient address cannot be empty"));
        }
        if timeout == 0 {
            return Err(anyhow!("Timeout must be greater than 0"));
        }

        Ok(Self {
            secret_hash,
            timeout,
            recipient_chain,
            recipient_address,
        })
    }

    /// HTLCDataをバイト配列にエンコード
    /// フォーマット:
    /// - 32 bytes: secret_hash
    /// - 8 bytes: timeout (big endian)
    /// - 1 byte: recipient_chain length
    /// - N bytes: recipient_chain
    /// - 1 byte: recipient_address length
    /// - M bytes: recipient_address
    pub fn encode(&self) -> Vec<u8> {
        let mut data = Vec::new();
        
        // シークレットハッシュ（32バイト）
        data.extend_from_slice(&self.secret_hash);
        
        // タイムアウト（8バイト、big endian）
        data.extend_from_slice(&self.timeout.to_be_bytes());
        
        // チェーン識別子
        data.push(self.recipient_chain.len() as u8);
        data.extend_from_slice(self.recipient_chain.as_bytes());
        
        // 受取人アドレス
        data.push(self.recipient_address.len() as u8);
        data.extend_from_slice(self.recipient_address.as_bytes());
        
        data
    }

    /// バイト配列からHTLCDataをデコード
    pub fn decode(data: &[u8]) -> Result<Self> {
        if data.len() < 41 {
            return Err(anyhow!("Data too short for HTLC info"));
        }

        let mut offset = 0;

        // シークレットハッシュ（32バイト）
        let mut secret_hash = [0u8; 32];
        secret_hash.copy_from_slice(&data[offset..offset + 32]);
        offset += 32;

        // タイムアウト（8バイト）
        let mut timeout_bytes = [0u8; 8];
        timeout_bytes.copy_from_slice(&data[offset..offset + 8]);
        let timeout = u64::from_be_bytes(timeout_bytes);
        offset += 8;

        // チェーン識別子
        let chain_len = data[offset] as usize;
        offset += 1;
        if offset + chain_len > data.len() {
            return Err(anyhow!("Invalid chain length"));
        }
        let recipient_chain = String::from_utf8(data[offset..offset + chain_len].to_vec())
            .map_err(|_| anyhow!("Invalid UTF-8 in chain name"))?;
        offset += chain_len;

        // 受取人アドレス
        if offset >= data.len() {
            return Err(anyhow!("Missing recipient address"));
        }
        let address_len = data[offset] as usize;
        offset += 1;
        if offset + address_len > data.len() {
            return Err(anyhow!("Invalid address length"));
        }
        let recipient_address = String::from_utf8(data[offset..offset + address_len].to_vec())
            .map_err(|_| anyhow!("Invalid UTF-8 in address"))?;

        Self::new(secret_hash, timeout, recipient_chain, recipient_address)
    }

    /// Hex文字列としてエンコード（0xプレフィックス付き）
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.encode()))
    }

    /// Hex文字列からデコード
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let data = hex::decode(hex_str.trim_start_matches("0x"))
            .map_err(|_| anyhow!("Invalid hex string"))?;
        Self::decode(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::htlc::generate_secret;
    use crate::htlc::hash_secret;

    #[test]
    fn test_htlc_data_creation() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        
        let htlc_data = HTLCData::new(
            secret_hash,
            3600,
            "near".to_string(),
            "alice.near".to_string(),
        );
        
        assert!(htlc_data.is_ok());
        let htlc_data = htlc_data.unwrap();
        assert_eq!(htlc_data.secret_hash, secret_hash);
        assert_eq!(htlc_data.timeout, 3600);
        assert_eq!(htlc_data.recipient_chain, "near");
        assert_eq!(htlc_data.recipient_address, "alice.near");
    }

    #[test]
    fn test_htlc_data_validation() {
        let secret_hash = [0u8; 32];
        
        // 空のチェーン識別子
        let result = HTLCData::new(
            secret_hash,
            3600,
            "".to_string(),
            "alice.near".to_string(),
        );
        assert!(result.is_err());
        
        // 空のアドレス
        let result = HTLCData::new(
            secret_hash,
            3600,
            "near".to_string(),
            "".to_string(),
        );
        assert!(result.is_err());
        
        // ゼロタイムアウト
        let result = HTLCData::new(
            secret_hash,
            0,
            "near".to_string(),
            "alice.near".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_htlc_data_encode_decode() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        
        let original = HTLCData::new(
            secret_hash,
            3600,
            "near".to_string(),
            "alice.near".to_string(),
        ).unwrap();
        
        // エンコード
        let encoded = original.encode();
        
        // デコード
        let decoded = HTLCData::decode(&encoded).unwrap();
        
        // 元のデータと一致することを確認
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_htlc_data_hex_encoding() {
        let secret = generate_secret();
        let secret_hash = hash_secret(&secret);
        
        let original = HTLCData::new(
            secret_hash,
            7200,
            "ethereum".to_string(),
            "0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0".to_string(),
        ).unwrap();
        
        // Hex文字列にエンコード
        let hex_str = original.to_hex();
        assert!(hex_str.starts_with("0x"));
        
        // Hex文字列からデコード
        let decoded = HTLCData::from_hex(&hex_str).unwrap();
        
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_long_chain_and_address() {
        let secret_hash = [0u8; 32];
        
        // 最大長のチェーン名とアドレス（255文字まで）
        let long_chain = "ethereum_testnet_sepolia".to_string();
        let long_address = "0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0742d35Cc6634C0532925a3b844Bc9e7595f8b4e0".to_string();
        
        let htlc_data = HTLCData::new(
            secret_hash,
            86400,
            long_chain.clone(),
            long_address.clone(),
        ).unwrap();
        
        let encoded = htlc_data.encode();
        let decoded = HTLCData::decode(&encoded).unwrap();
        
        assert_eq!(decoded.recipient_chain, long_chain);
        assert_eq!(decoded.recipient_address, long_address);
    }

    #[test]
    fn test_decode_invalid_data() {
        // データが短すぎる
        let short_data = vec![0u8; 40];
        assert!(HTLCData::decode(&short_data).is_err());
        
        // 無効なチェーン長
        let mut invalid_data = vec![0u8; 40];
        invalid_data[40] = 255; // チェーン長が255だが、実際のデータがない
        assert!(HTLCData::decode(&invalid_data).is_err());
    }
}