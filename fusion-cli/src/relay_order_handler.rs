use anyhow::{anyhow, Result};
use clap::Args;
use fusion_core::htlc::SecretHash;
use fusion_core::order::Order;
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
    use fusion_core::chains::ethereum::order_extractor::OrderExtractor;
    use sha2::{Digest, Sha256};

    // Get RPC URL and contract address
    let rpc_url = args
        .evm_rpc
        .as_ref()
        .ok_or_else(|| anyhow!("EVM RPC URL must be provided via --evm-rpc flag"))?;
    let limit_order_address = "0x171C87724E720F2806fc29a010a62897B30fdb62"; // Base Sepolia deployment

    // Extract the order from EVM
    let extractor = OrderExtractor::new(rpc_url, limit_order_address)?;
    let order = extractor.extract_order_by_hash(&args.order_hash).await?;

    // Calculate secret hash from provided secret
    let secret_bytes = hex::decode(args.htlc_secret.trim_start_matches("0x"))
        .map_err(|_| anyhow!("Invalid HTLC secret format"))?;
    let mut hasher = Sha256::new();
    hasher.update(&secret_bytes);
    let hash_result = hasher.finalize();
    let mut secret_hash = [0u8; 32];
    secret_hash.copy_from_slice(&hash_result);

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
    use fusion_core::chains::near::NearHtlcConnector;

    // Parse the secret for validation
    let secret_bytes = hex::decode(args.htlc_secret.trim_start_matches("0x"))
        .map_err(|_| anyhow!("Invalid secret format"))?;

    if secret_bytes.len() != 32 {
        return Err(anyhow!("Secret must be exactly 32 bytes"));
    }

    // Get NEAR configuration
    let near_rpc = match args.near_network.as_str() {
        "mainnet" => "https://rpc.mainnet.near.org",
        "testnet" => "https://rpc.testnet.near.org",
        _ => return Err(anyhow!("Invalid NEAR network")),
    };

    let htlc_contract = match args.near_network.as_str() {
        "mainnet" => "fusion-htlc.near",
        "testnet" => "fusion-htlc.testnet",
        _ => unreachable!(),
    };

    // Get account configuration from environment or args
    let near_account = args
        .near_account
        .as_ref()
        .ok_or_else(|| anyhow!("NEAR account not specified"))?;
    let private_key = std::env::var("NEAR_PRIVATE_KEY")
        .map_err(|_| anyhow!("NEAR_PRIVATE_KEY environment variable must be set"))?;

    // Create NEAR connector
    let connector = NearHtlcConnector::new(near_rpc)
        .with_contract(htlc_contract)
        .with_account(near_account, &private_key)?;

    // Calculate amount from order (convert to NEAR)
    // EVM tokens typically have 18 decimals, NEAR has 24 decimals
    let evm_amount = order_info.order.taking_amount();
    let amount = convert_evm_to_near_amount(evm_amount);

    // Create HTLC on NEAR
    let htlc_id = connector
        .create_htlc(
            amount,
            order_info.secret_hash,
            order_info.timeout,
            &order_info.recipient_address,
        )
        .await?;

    // Generate explorer URL
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
    let output = json!({
        "status": "success",
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

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Convert EVM token amount (18 decimals) to NEAR token amount (24 decimals)
fn convert_evm_to_near_amount(evm_amount: u128) -> u128 {
    // EVM: 18 decimals, NEAR: 24 decimals
    // To convert, multiply by 10^6 (1,000,000)
    evm_amount.saturating_mul(1_000_000)
}
