use anyhow::{anyhow, Result};
use clap::Args;
use fusion_core::eip712::OrderEIP712;
use fusion_core::order::OrderBuilder;
use serde_json::json;

#[derive(Args)]
pub struct CreateOrderArgs {
    /// Maker asset address
    #[arg(long)]
    pub maker_asset: String,

    /// Taker asset address
    #[arg(long)]
    pub taker_asset: String,

    /// Maker address
    #[arg(long)]
    pub maker: String,

    /// Making amount
    #[arg(long)]
    pub making_amount: u128,

    /// Taking amount
    #[arg(long)]
    pub taking_amount: u128,

    /// HTLC secret hash (32 bytes hex)
    #[arg(long)]
    pub htlc_secret_hash: String,

    /// HTLC timeout in seconds
    #[arg(long)]
    pub htlc_timeout: u64,

    /// Chain ID
    #[arg(long)]
    pub chain_id: u64,

    /// Verifying contract address
    #[arg(long)]
    pub verifying_contract: String,

    /// Receiver address (optional)
    #[arg(long)]
    pub receiver: Option<String>,

    /// Allowed sender address (optional)
    #[arg(long)]
    pub allowed_sender: Option<String>,
}

pub async fn handle_create_order(args: CreateOrderArgs) -> Result<()> {
    // Validate addresses
    validate_address(&args.maker_asset)?;
    validate_address(&args.taker_asset)?;
    validate_address(&args.maker)?;
    validate_address(&args.verifying_contract)?;

    if let Some(ref receiver) = args.receiver {
        validate_address(receiver)?;
    }

    if let Some(ref allowed_sender) = args.allowed_sender {
        validate_address(allowed_sender)?;
    }

    // Parse HTLC secret hash
    let secret_hash_bytes = hex::decode(args.htlc_secret_hash.trim_start_matches("0x"))
        .map_err(|_| anyhow!("Invalid HTLC secret hash format"))?;

    if secret_hash_bytes.len() != 32 {
        return Err(anyhow!("HTLC secret hash must be exactly 32 bytes"));
    }

    // Create maker asset data with embedded HTLC info
    let maker_asset_data = encode_htlc_data(&secret_hash_bytes, args.htlc_timeout);

    // Build order
    let mut builder = OrderBuilder::new()
        .maker_asset(&args.maker_asset)
        .taker_asset(&args.taker_asset)
        .maker(&args.maker)
        .making_amount(args.making_amount)
        .taking_amount(args.taking_amount)
        .interactions(&maker_asset_data);

    if let Some(receiver) = args.receiver {
        builder = builder.receiver(&receiver);
    }

    if let Some(allowed_sender) = args.allowed_sender {
        builder = builder.allowed_sender(&allowed_sender);
    }

    let order = builder.build()?;

    // Create EIP-712 typed data
    let typed_data = order.to_eip712(args.chain_id, &args.verifying_contract);
    let eip712_hash = typed_data.hash();

    // Output result
    let output = json!({
        "order": {
            "salt": format!("0x{}", hex::encode(order.salt)),
            "makerAsset": order.maker_asset,
            "takerAsset": order.taker_asset,
            "maker": order.maker,
            "receiver": order.receiver,
            "allowedSender": order.allowed_sender,
            "makingAmount": order.making_amount.to_string(),
            "takingAmount": order.taking_amount.to_string(),
            "offsets": order.offsets.to_string(),
            "interactions": order.interactions,
        },
        "domain": {
            "name": typed_data.domain.name,
            "version": typed_data.domain.version,
            "chainId": typed_data.domain.chain_id,
            "verifyingContract": typed_data.domain.verifying_contract,
        },
        "eip712_hash": format!("0x{}", hex::encode(eip712_hash)),
        "htlc_info": {
            "secret_hash": format!("0x{}", hex::encode(secret_hash_bytes)),
            "timeout_seconds": args.htlc_timeout,
        }
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn validate_address(address: &str) -> Result<()> {
    let addr = address.trim_start_matches("0x");
    if addr.len() != 40 {
        return Err(anyhow!(
            "Invalid address: must be 40 hex characters (excluding 0x prefix)"
        ));
    }

    hex::decode(addr).map_err(|_| anyhow!("Invalid address: must be valid hexadecimal"))?;

    Ok(())
}

fn encode_htlc_data(secret_hash: &[u8], timeout: u64) -> String {
    // Encode HTLC data into makerAssetData field
    // Format: 32 bytes secret hash + 32 bytes timeout
    let mut data = Vec::new();
    data.extend_from_slice(secret_hash);

    // Encode timeout as 32 bytes
    let mut timeout_bytes = [0u8; 32];
    timeout_bytes[24..].copy_from_slice(&timeout.to_be_bytes());
    data.extend_from_slice(&timeout_bytes);

    format!("0x{}", hex::encode(data))
}

#[cfg(test)]
pub fn extract_htlc_info(maker_asset_data: &str) -> Result<(Vec<u8>, u64)> {
    let data = hex::decode(maker_asset_data.trim_start_matches("0x"))
        .map_err(|_| anyhow!("Invalid maker asset data format"))?;

    if data.len() < 64 {
        return Err(anyhow!("Maker asset data too short for HTLC info"));
    }

    let secret_hash = data[0..32].to_vec();
    let timeout_bytes = &data[32..64];

    // Extract timeout from last 8 bytes of the 32-byte field
    let mut timeout_array = [0u8; 8];
    timeout_array.copy_from_slice(&timeout_bytes[24..32]);
    let timeout = u64::from_be_bytes(timeout_array);

    Ok((secret_hash, timeout))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htlc_info_extraction() {
        let secret_hash = vec![
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab,
            0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78,
            0x90, 0xab, 0xcd, 0xef,
        ];
        let timeout = 3600u64;

        // Create encoded data
        let mut data = Vec::new();
        data.extend_from_slice(&secret_hash);

        let mut timeout_bytes = [0u8; 32];
        timeout_bytes[24..].copy_from_slice(&timeout.to_be_bytes());
        data.extend_from_slice(&timeout_bytes);

        let encoded = format!("0x{}", hex::encode(&data));

        // Extract and verify
        let (extracted_hash, extracted_timeout) = extract_htlc_info(&encoded).unwrap();

        assert_eq!(extracted_hash, secret_hash);
        assert_eq!(extracted_timeout, timeout);
    }

    #[test]
    fn test_validate_address() {
        // Valid addresses
        assert!(validate_address("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950").is_ok());
        assert!(validate_address("7aD8317e9aB4837AEF734e23d1C62F4938a6D950").is_ok());

        // Invalid addresses
        assert!(validate_address("0x123").is_err());
        assert!(validate_address("invalid_address").is_err());
        assert!(validate_address("0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG").is_err());
    }
}
