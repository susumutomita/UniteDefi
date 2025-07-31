use anyhow::{anyhow, Result};
use clap::Args;
use fusion_core::{
    eip712::OrderEIP712,
    htlc::{generate_secret, hash_secret},
    limit_order_htlc::create_near_to_ethereum_order,
    price_oracle::{MockPriceOracle, PriceConverter},
};
use serde_json::json;

#[derive(Args)]
pub struct CreateNearOrderArgs {
    /// NEAR account ID (e.g., alice.near)
    #[arg(long)]
    pub near_account: String,

    /// Ethereum recipient address
    #[arg(long)]
    pub ethereum_address: String,

    /// Amount of NEAR to swap
    #[arg(long)]
    pub near_amount: f64,

    /// Generate a new secret (if not provided, must provide secret_hash)
    #[arg(long)]
    pub generate_secret: bool,

    /// HTLC secret hash (32 bytes hex, required if not generating)
    #[arg(long)]
    pub secret_hash: Option<String>,

    /// HTLC timeout in seconds (default: 3600)
    #[arg(long, default_value = "3600")]
    pub timeout: u64,

    /// Slippage tolerance in basis points (100 = 1%)
    #[arg(long, default_value = "100")]
    pub slippage_bps: u16,

    /// Chain ID (default: Base Sepolia)
    #[arg(long, default_value = "84532")]
    pub chain_id: u64,

    /// Limit Order Protocol address (default: Base Sepolia deployment)
    #[arg(long, default_value = "0x171C87724E720F2806fc29a010a62897B30fdb62")]
    pub limit_order_protocol: String,
}

pub async fn handle_create_near_order(args: CreateNearOrderArgs) -> Result<()> {
    // Validate inputs
    if !args.near_account.ends_with(".near") && !args.near_account.ends_with(".testnet") {
        return Err(anyhow!("Invalid NEAR account format"));
    }

    validate_ethereum_address(&args.ethereum_address)?;

    // Handle secret generation or parsing
    let (secret, secret_hash) = if args.generate_secret {
        let secret = generate_secret();
        let hash = hash_secret(&secret);
        (Some(secret), hash)
    } else if let Some(hash_str) = args.secret_hash {
        let hash_bytes = hex::decode(hash_str.trim_start_matches("0x"))
            .map_err(|_| anyhow!("Invalid secret hash format"))?;
        if hash_bytes.len() != 32 {
            return Err(anyhow!("Secret hash must be exactly 32 bytes"));
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hash_bytes);
        (None, hash)
    } else {
        return Err(anyhow!("Must either generate secret or provide secret hash"));
    };

    // Convert NEAR amount to smallest unit
    let near_amount_yocto = (args.near_amount * 10f64.powi(24)) as u128;

    // Setup price oracle and calculate USDC amount
    let oracle = MockPriceOracle::new();
    let converter = PriceConverter::new(oracle);

    let usdc_amount = converter
        .convert_amount(near_amount_yocto, "NEAR", 24, "USDC", 6)
        .await?;

    // Apply slippage
    let slippage_factor = 1.0 - (args.slippage_bps as f64 / 10000.0);
    let usdc_with_slippage = ((usdc_amount as f64) * slippage_factor) as u128;

    // Create order
    let order = create_near_to_ethereum_order(
        &args.near_account,
        &args.ethereum_address,
        near_amount_yocto,
        usdc_with_slippage,
        secret_hash,
        args.timeout,
    )?;

    // Create EIP-712 typed data
    let typed_data = order.to_eip712(args.chain_id, &args.limit_order_protocol);
    let eip712_hash = typed_data.hash();

    // Prepare output
    let mut output = json!({
        "order": {
            "salt": format!("0x{}", hex::encode(order.salt)),
            "makerAsset": order.maker_asset(),
            "takerAsset": order.taker_asset(),
            "maker": order.maker(),
            "receiver": order.receiver,
            "allowedSender": order.allowed_sender,
            "makingAmount": order.making_amount().to_string(),
            "takingAmount": order.taking_amount().to_string(),
            "offsets": "0",
            "interactions": order.interactions,
        },
        "domain": {
            "name": typed_data.domain.name,
            "version": typed_data.domain.version,
            "chainId": typed_data.domain.chain_id,
            "verifyingContract": typed_data.domain.verifying_contract,
        },
        "eip712_hash": format!("0x{}", hex::encode(eip712_hash)),
        "details": {
            "near_amount": args.near_amount,
            "usdc_amount": usdc_with_slippage as f64 / 10f64.powi(6),
            "slippage_bps": args.slippage_bps,
            "timeout_seconds": args.timeout,
            "secret_hash": format!("0x{}", hex::encode(secret_hash)),
        },
        "instructions": {
            "1": "Sign this order using your NEAR wallet",
            "2": "Submit the signed order to 1inch Fusion API",
            "3": "Monitor for order execution",
            "4": "Claim USDC on Ethereum once filled",
        }
    });

    // Add secret to output if generated
    if let Some(secret) = secret {
        output["secret"] = json!({
            "value": format!("0x{}", hex::encode(secret)),
            "warning": "KEEP THIS SECRET! You need it to claim funds",
        });
    }

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn validate_ethereum_address(address: &str) -> Result<()> {
    let addr = address.trim_start_matches("0x");
    if addr.len() != 40 {
        return Err(anyhow!("Invalid Ethereum address format"));
    }
    hex::decode(addr).map_err(|_| anyhow!("Invalid hexadecimal in address"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ethereum_address() {
        assert!(validate_ethereum_address("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0").is_ok());
        assert!(validate_ethereum_address("742d35Cc6634C0532925a3b844Bc9e7595f8b4e0").is_ok());
        assert!(validate_ethereum_address("0x123").is_err());
        assert!(validate_ethereum_address("invalid").is_err());
    }
}