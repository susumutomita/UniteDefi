//! Example of using the claim command
//!
//! This example demonstrates how to create an HTLC and then claim it using the correct secret.

use serde_json::Value;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating HTLC...");

    // Create an HTLC
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "create-htlc",
            "--sender",
            "Alice",
            "--recipient",
            "Bob",
            "--amount",
            "1000",
        ])
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Failed to create HTLC: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(());
    }

    // Parse the output
    let output_str = String::from_utf8(output.stdout)?;
    let json: Value = serde_json::from_str(&output_str)?;

    let htlc_id = json["htlc_id"].as_str().unwrap();
    let secret = json["secret"].as_str().unwrap();

    println!("Created HTLC with ID: {}", htlc_id);
    println!("Secret: {}", secret);
    println!("\nClaiming HTLC...");

    // Claim the HTLC
    let claim_output = Command::new("cargo")
        .args([
            "run",
            "--",
            "claim",
            "--htlc-id",
            htlc_id,
            "--secret",
            secret,
        ])
        .output()?;

    if !claim_output.status.success() {
        eprintln!(
            "Failed to claim HTLC: {}",
            String::from_utf8_lossy(&claim_output.stderr)
        );
        return Ok(());
    }

    let claim_str = String::from_utf8(claim_output.stdout)?;
    let claim_json: Value = serde_json::from_str(&claim_str)?;

    println!(
        "Claim result: {}",
        serde_json::to_string_pretty(&claim_json)?
    );

    Ok(())
}
