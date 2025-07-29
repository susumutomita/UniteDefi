use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use fusion_core::htlc::{generate_secret, hash_secret, Htlc};
use fusion_core::storage::HtlcStorage;
use serde_json::json;
use std::time::{Duration, SystemTime};

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

    // Store HTLC
    let storage = HtlcStorage::new()?;
    storage.store_htlc(htlc_id.clone(), &htlc, args.timeout)?;

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
    let storage = HtlcStorage::new()?;
    
    // Get HTLC from storage
    let stored_htlc = storage.get_htlc(&args.htlc_id)?
        .ok_or_else(|| anyhow!("HTLC not found"))?;
    
    // Check if already claimed
    if stored_htlc.state == "Claimed" {
        let output = json!({
            "error": "HTLC already claimed"
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Err(anyhow!("HTLC already claimed"));
    }
    
    // Parse secret from hex
    let secret_bytes = hex::decode(&args.secret)
        .map_err(|_| anyhow!("Invalid secret format"))?;
    
    if secret_bytes.len() != 32 {
        let output = json!({
            "error": "Invalid secret length"
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Err(anyhow!("Invalid secret length"));
    }
    
    let mut secret = [0u8; 32];
    secret.copy_from_slice(&secret_bytes);
    
    // Create HTLC object to verify secret
    let mut htlc = Htlc::new(
        stored_htlc.sender.clone(),
        stored_htlc.recipient.clone(),
        stored_htlc.amount,
        stored_htlc.secret_hash,
        Duration::from_secs(stored_htlc.timeout_seconds),
    )?;
    
    // Try to claim with the provided secret
    match htlc.claim(&secret) {
        Ok(()) => {
            let claimed_at = SystemTime::now();
            // Update storage
            storage.update_htlc_state(&args.htlc_id, "Claimed", Some(claimed_at))?;
            
            // Output success result
            let output = json!({
                "htlc_id": args.htlc_id,
                "status": "Claimed",
                "claimed_at": chrono::DateTime::<chrono::Utc>::from(claimed_at).to_rfc3339()
            });
            
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        Err(e) => {
            let output = json!({
                "error": format!("Invalid secret: {}", e)
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Err(anyhow!("Invalid secret: {}", e))
        }
    }
}

async fn handle_refund(_args: RefundArgs) -> Result<()> {
    // TODO: Implement refund logic
    // For now, return a placeholder response
    let output = json!({
        "error": "Refund functionality not yet implemented",
        "message": "This feature will be implemented in Issue #17"
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
