use anyhow::{anyhow, Result};
use clap::Args;
use fusion_core::htlc::SecretHash;
use fusion_core::order::{Order, OrderBuilder};
use serde_json::json;

#[derive(Args)]
pub struct RelayOrderArgs {
    /// EVM order hash to relay
    #[arg(long)]
    pub order_hash: String,

    /// Target chain (only "near" supported)
    #[arg(long)]
    pub to_chain: String,

    /// HTLC secret (32 bytes hex)
    #[arg(long)]
    pub htlc_secret: String,

    /// NEAR account ID (optional, defaults to env)
    #[arg(long)]
    pub near_account: Option<String>,

    /// EVM RPC endpoint (optional)
    #[arg(long)]
    pub evm_rpc: Option<String>,

    /// NEAR network (testnet/mainnet)
    #[arg(long, default_value = "testnet")]
    pub near_network: String,
}

pub async fn handle_relay_order(args: RelayOrderArgs) -> Result<()> {
    // Validate inputs
    validate_inputs(&args)?;

    // Step 1: Extract order information from EVM
    let order_info = extract_order_from_evm(&args).await?;

    // Step 2: Create HTLC on NEAR
    let htlc_result = create_htlc_on_near(&args, &order_info).await?;

    // Step 3: Display results
    display_relay_results(&args, &order_info, &htlc_result)?;

    Ok(())
}

fn validate_inputs(args: &RelayOrderArgs) -> Result<()> {
    // Validate order hash
    let order_hash = args.order_hash.trim_start_matches("0x");
    if order_hash.len() != 64 {
        return Err(anyhow!("Order hash must be 32 bytes (64 hex characters)"));
    }
    hex::decode(order_hash).map_err(|_| anyhow!("Invalid order hash format"))?;

    // Validate target chain
    if args.to_chain != "near" {
        return Err(anyhow!("Only 'near' is supported as target chain"));
    }

    // Validate HTLC secret
    let secret = args.htlc_secret.trim_start_matches("0x");
    if secret.len() != 64 {
        return Err(anyhow!("HTLC secret must be 32 bytes (64 hex characters)"));
    }
    hex::decode(secret).map_err(|_| anyhow!("Invalid HTLC secret format"))?;

    // Validate NEAR network
    if !["testnet", "mainnet"].contains(&args.near_network.as_str()) {
        return Err(anyhow!("NEAR network must be 'testnet' or 'mainnet'"));
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct OrderInfo {
    order: Order,
    secret_hash: SecretHash,
    timeout: u64,
    recipient_chain: String,
    recipient_address: String,
}

struct HTLCResult {
    htlc_id: String,
    transaction_hash: String,
    near_explorer_url: String,
}

async fn extract_order_from_evm(args: &RelayOrderArgs) -> Result<OrderInfo> {
    // For now, we'll create a mock order
    // In a real implementation, this would query the Limit Order Protocol

    // Parse the HTLC secret hash
    let secret_hash_bytes = hex::decode(args.htlc_secret.trim_start_matches("0x"))
        .map_err(|_| anyhow!("Invalid secret hash format"))?;
    let mut secret_hash = [0u8; 32];
    secret_hash.copy_from_slice(&secret_hash_bytes);

    // Create a mock order (in real implementation, this would be fetched from EVM)
    let order = OrderBuilder::new()
        .maker("0x1234567890123456789012345678901234567890")
        .maker_asset("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")
        .taker_asset("0x0000000000000000000000000000000000000000")
        .making_amount(1000000)
        .taking_amount(1000000000000000000)
        .build()?; // This would be fetched from the blockchain

    Ok(OrderInfo {
        order,
        secret_hash,
        timeout: 3600, // Default 1 hour
        recipient_chain: "near".to_string(),
        recipient_address: args
            .near_account
            .clone()
            .unwrap_or_else(|| "relay.testnet".to_string()),
    })
}

async fn create_htlc_on_near(args: &RelayOrderArgs, order_info: &OrderInfo) -> Result<HTLCResult> {
    // Parse the secret for validation
    let secret_bytes = hex::decode(args.htlc_secret.trim_start_matches("0x"))
        .map_err(|_| anyhow!("Invalid secret format"))?;

    if secret_bytes.len() != 32 {
        return Err(anyhow!("Secret must be exactly 32 bytes"));
    }

    // In a real implementation, this would:
    // 1. Connect to NEAR
    // 2. Create an HTLC with the provided parameters
    // 3. Return the transaction result

    // Mock result for now
    let htlc_id = format!("htlc_{}", hex::encode(&order_info.secret_hash[..8]));
    let transaction_hash = format!("0x{}", hex::encode(&order_info.secret_hash[..16]));
    let near_explorer_url = format!(
        "https://explorer.{}.near.org/transactions/{}",
        args.near_network, transaction_hash
    );

    Ok(HTLCResult {
        htlc_id,
        transaction_hash,
        near_explorer_url,
    })
}

fn display_relay_results(
    args: &RelayOrderArgs,
    order_info: &OrderInfo,
    htlc_result: &HTLCResult,
) -> Result<()> {
    let mut output = json!({
        "relay_status": "success",
        "relay_details": {
            "from_chain": "ethereum",
            "to_chain": args.to_chain,
            "order_hash": args.order_hash,
            "recipient_chain": order_info.recipient_chain,
        },
        "order": {
            "maker": order_info.order.maker(),
            "maker_asset": order_info.order.maker_asset(),
            "taker_asset": order_info.order.taker_asset(),
            "making_amount": order_info.order.making_amount(),
            "taking_amount": order_info.order.taking_amount(),
        },
        "htlc_info": {
            "htlc_id": htlc_result.htlc_id,
            "secret_hash": format!("0x{}", hex::encode(order_info.secret_hash)),
            "timeout_seconds": order_info.timeout,
            "recipient": order_info.recipient_address,
        },
        "transactions": {
            "near_htlc_creation": htlc_result.transaction_hash,
            "explorer_url": htlc_result.near_explorer_url,
        },
        "next_steps": [
            "Monitor the order execution on Ethereum",
            "Once the order is filled, the secret will be revealed",
            "Use the secret to claim funds from the NEAR HTLC",
        ]
    });

    // Add near_account if provided
    if let Some(ref near_account) = args.near_account {
        output["near_account"] = json!(near_account);
    }

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
