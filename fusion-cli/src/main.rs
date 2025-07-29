use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use fusion_core::htlc::{generate_secret, hash_secret, Htlc};
use serde_json::json;
use std::time::Duration;

mod storage;
use storage::{HtlcStorage, StoredHtlc};
use once_cell::sync::Lazy;

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
        state: "Pending".to_string(),
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

async fn handle_claim(_args: ClaimArgs) -> Result<()> {
    // TODO: Implement claim logic
    // For now, return a placeholder response
    let output = json!({
        "error": "Claim functionality not yet implemented",
        "message": "This feature will be implemented in Issue #16"
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

async fn handle_refund(args: RefundArgs) -> Result<()> {
    // Get HTLC from storage
    let stored_htlc = STORAGE.get(&args.htlc_id)?;

    // Check if HTLC is already claimed or refunded
    if stored_htlc.state == "Claimed" {
        let output = json!({
            "error": "HTLC already claimed",
            "htlc_id": args.htlc_id,
            "status": stored_htlc.state
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    if stored_htlc.state == "Refunded" {
        let output = json!({
            "error": "HTLC already refunded",
            "htlc_id": args.htlc_id,
            "status": stored_htlc.state
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Recreate HTLC to check timeout
    let mut htlc = Htlc::new(
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
    STORAGE.update_state(&args.htlc_id, "Refunded".to_string())?;

    // Output successful refund
    let output = json!({
        "htlc_id": args.htlc_id,
        "status": "Refunded",
        "refunded_at": chrono::Utc::now().to_rfc3339()
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
