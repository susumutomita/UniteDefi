use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use fusion_core::htlc::{generate_secret, hash_secret, Htlc, HtlcState};
use serde_json::json;
use std::time::Duration;

mod near_order_handler;
mod order_handler;
mod order_management;
mod relay_order_handler;
mod storage;
mod swap_handler;
use once_cell::sync::Lazy;
use storage::{HtlcStorage, StoredHtlc};

static STORAGE: Lazy<HtlcStorage> = Lazy::new(HtlcStorage::new);

#[derive(Parser)]
#[command(name = "fusion-cli")]
#[command(about = "UniteSwap CLI")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new HTLC
    CreateHtlc(CreateHtlcArgs),
    /// Claim an existing HTLC with secret
    Claim(ClaimArgs),
    /// Refund an HTLC after timeout
    Refund(RefundArgs),
    /// Order commands
    Order(Box<OrderCommands>),
    /// Relay an order from EVM to another chain
    RelayOrder(relay_order_handler::RelayOrderArgs),
    /// Display orderbook for a specific chain
    Orderbook(order_management::OrderbookArgs),
    /// Integrated cross-chain token swap
    #[command(subcommand)]
    Swap(swap_handler::SwapCommands),
}

#[derive(Args)]
struct OrderCommands {
    #[command(subcommand)]
    command: OrderSubcommands,
}

#[derive(Subcommand)]
enum OrderSubcommands {
    /// Create a new limit order
    Create(order_handler::CreateOrderArgs),
    /// Create a NEAR to Ethereum order
    CreateNear(near_order_handler::CreateNearOrderArgs),
    /// Check order status
    Status(order_management::StatusArgs),
    /// Cancel an order
    Cancel(order_management::CancelArgs),
}

#[derive(Args)]
struct CreateHtlcArgs {
    /// Sender address
    #[arg(long)]
    sender: String,
    /// Recipient address
    #[arg(long)]
    recipient: String,
    /// Amount to transfer
    #[arg(long)]
    amount: u64,
    /// Timeout duration in seconds
    #[arg(long, default_value = "3600")]
    timeout: u64,
}

#[derive(Args)]
struct ClaimArgs {
    /// HTLC identifier
    #[arg(long)]
    htlc_id: String,
    /// Secret to claim the HTLC
    #[arg(long)]
    secret: String,
}

#[derive(Args)]
struct RefundArgs {
    /// HTLC identifier
    #[arg(long)]
    htlc_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateHtlc(args) => handle_create_htlc(args).await,
        Commands::Claim(args) => handle_claim(args).await,
        Commands::Refund(args) => handle_refund(args).await,
        Commands::Order(order_cmd) => match order_cmd.command {
            OrderSubcommands::Create(args) => order_handler::handle_create_order(args).await,
            OrderSubcommands::CreateNear(args) => {
                near_order_handler::handle_create_near_order(args).await
            }
            OrderSubcommands::Status(args) => order_management::handle_order_status(args).await,
            OrderSubcommands::Cancel(args) => order_management::handle_order_cancel(args).await,
        },
        Commands::RelayOrder(args) => relay_order_handler::handle_relay_order(args).await,
        Commands::Orderbook(args) => order_management::handle_orderbook(args).await,
        Commands::Swap(swap_cmd) => match swap_cmd {
            swap_handler::SwapCommands::Execute(args) => swap_handler::handle_swap(args).await,
            swap_handler::SwapCommands::Batch(args) => swap_handler::handle_batch_swap(args).await,
        },
    }
}

async fn handle_create_htlc(args: CreateHtlcArgs) -> Result<()> {
    // Generate secret and hash
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);

    // Create HTLC
    let htlc = Htlc::new(
        args.sender.clone(),
        args.recipient.clone(),
        args.amount,
        secret_hash,
        Duration::from_secs(args.timeout),
    )?;

    // Generate a simple HTLC ID (in real implementation, this would be a proper hash)
    let htlc_id = format!("htlc_{}", hex::encode(&secret_hash[..8]));

    // Store HTLC in storage
    let stored_htlc = StoredHtlc {
        sender: args.sender,
        recipient: args.recipient,
        amount: args.amount,
        secret_hash,
        timeout: Duration::from_secs(args.timeout),
        created_at: std::time::SystemTime::now(),
        state: HtlcState::Pending,
        secret: Some(secret.to_vec()),
    };
    STORAGE.store(htlc_id.clone(), stored_htlc)?;

    // Output result as JSON
    let output = json!({
        "htlc_id": htlc_id,
        "secret": hex::encode(secret),
        "secret_hash": hex::encode(secret_hash),
        "sender": htlc.sender(),
        "recipient": htlc.recipient(),
        "amount": htlc.amount(),
        "timeout_seconds": args.timeout,
        "status": "Pending"
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

async fn handle_claim(args: ClaimArgs) -> Result<()> {
    // Get HTLC from storage
    let stored_htlc = match STORAGE.get(&args.htlc_id) {
        Ok(htlc) => htlc,
        Err(_) => {
            let output = json!({
                "error": "HTLC not found",
                "htlc_id": args.htlc_id
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(());
        }
    };

    // Check if HTLC is already claimed
    if stored_htlc.state == HtlcState::Claimed {
        let output = json!({
            "error": "HTLC already claimed",
            "htlc_id": args.htlc_id,
            "status": stored_htlc.state
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Check if HTLC is refunded
    if stored_htlc.state == HtlcState::Refunded {
        let output = json!({
            "error": "HTLC already refunded",
            "htlc_id": args.htlc_id,
            "status": stored_htlc.state
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Parse the secret from hex string
    let secret_bytes = match hex::decode(&args.secret) {
        Ok(bytes) => bytes,
        Err(_) => {
            let output = json!({
                "error": "Invalid secret format",
                "message": "Secret must be a valid hex string"
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(());
        }
    };

    // Convert to Secret type (32 bytes)
    let secret: fusion_core::htlc::Secret = match secret_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => {
            let output = json!({
                "error": "Invalid secret length",
                "message": "Secret must be exactly 32 bytes (64 hex characters)"
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(());
        }
    };

    // Create a mutable HTLC to validate the claim
    let mut htlc = Htlc::new(
        stored_htlc.sender.clone(),
        stored_htlc.recipient.clone(),
        stored_htlc.amount,
        stored_htlc.secret_hash,
        stored_htlc.timeout,
    )?;

    // Try to claim with the provided secret
    match htlc.claim(&secret) {
        Ok(_) => {
            // Update state in storage
            STORAGE.update_state(&args.htlc_id, HtlcState::Claimed)?;

            // Output successful claim
            let output = json!({
                "htlc_id": args.htlc_id,
                "status": "Claimed",
                "claimed_at": chrono::Utc::now().to_rfc3339()
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        Err(fusion_core::htlc::HtlcError::InvalidSecret) => {
            let output = json!({
                "error": "Invalid secret",
                "htlc_id": args.htlc_id
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        Err(e) => {
            let output = json!({
                "error": format!("Claim failed: {}", e),
                "htlc_id": args.htlc_id
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}

async fn handle_refund(args: RefundArgs) -> Result<()> {
    // Get HTLC from storage
    let stored_htlc = STORAGE.get(&args.htlc_id)?;

    // Check if HTLC is already claimed or refunded
    if stored_htlc.state == HtlcState::Claimed {
        let output = json!({
            "error": "HTLC already claimed",
            "htlc_id": args.htlc_id,
            "status": stored_htlc.state
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    if stored_htlc.state == HtlcState::Refunded {
        let output = json!({
            "error": "HTLC already refunded",
            "htlc_id": args.htlc_id,
            "status": stored_htlc.state
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Recreate HTLC to validate construction
    let _htlc = Htlc::new(
        stored_htlc.sender.clone(),
        stored_htlc.recipient.clone(),
        stored_htlc.amount,
        stored_htlc.secret_hash,
        stored_htlc.timeout,
    )?;

    // Check if HTLC has timed out
    let elapsed = std::time::SystemTime::now()
        .duration_since(stored_htlc.created_at)
        .unwrap_or(Duration::from_secs(0));

    if elapsed <= stored_htlc.timeout {
        let output = json!({
            "error": "HTLC has not timed out yet",
            "htlc_id": args.htlc_id,
            "timeout_remaining_seconds": (stored_htlc.timeout.as_secs() - elapsed.as_secs())
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Update state to refunded
    STORAGE.update_state(&args.htlc_id, HtlcState::Refunded)?;

    // Output successful refund
    let output = json!({
        "htlc_id": args.htlc_id,
        "status": "Refunded",
        "refunded_at": chrono::Utc::now().to_rfc3339()
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
