use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, Value};

#[test]
fn test_claim_with_valid_secret() {
    // Create HTLC first
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "create-htlc",
            "--sender", "Alice",
            "--recipient", "Bob",
            "--amount", "1000",
            "--timeout", "3600"
        ])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    let create_response: Value = serde_json::from_slice(&output.stdout).unwrap();
    let htlc_id = create_response["htlc_id"].as_str().unwrap();
    let secret = create_response["secret"].as_str().unwrap();
    
    // Test claim with valid secret
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "claim",
            "--htlc-id", htlc_id,
            "--secret", secret
        ])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    let claim_response: Value = serde_json::from_slice(&output.stdout).unwrap();
    
    assert_eq!(claim_response["htlc_id"], htlc_id);
    assert_eq!(claim_response["status"], "Claimed");
    assert!(claim_response["claimed_at"].is_string());
}

#[test]
fn test_claim_with_invalid_secret() {
    // Create HTLC first
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "create-htlc",
            "--sender", "Alice",
            "--recipient", "Bob",
            "--amount", "1000",
            "--timeout", "3600"
        ])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    let create_response: Value = serde_json::from_slice(&output.stdout).unwrap();
    let htlc_id = create_response["htlc_id"].as_str().unwrap();
    
    // Test claim with wrong secret
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "claim",
            "--htlc-id", htlc_id,
            "--secret", "0000000000000000000000000000000000000000000000000000000000000000"
        ])
        .output()
        .unwrap();
    
    // Should fail
    assert!(!output.status.success());
    let error_response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(error_response["error"].as_str().unwrap().contains("Invalid secret"));
}

#[test]
fn test_claim_nonexistent_htlc() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "claim",
            "--htlc-id", "nonexistent_htlc",
            "--secret", "0000000000000000000000000000000000000000000000000000000000000000"
        ])
        .output()
        .unwrap();
    
    // Should fail
    assert!(!output.status.success());
    let error_response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(error_response["error"].as_str().unwrap().contains("HTLC not found"));
}

#[test]
fn test_claim_already_claimed() {
    // Create HTLC first
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "create-htlc",
            "--sender", "Alice",
            "--recipient", "Bob",
            "--amount", "1000",
            "--timeout", "3600"
        ])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    let create_response: Value = serde_json::from_slice(&output.stdout).unwrap();
    let htlc_id = create_response["htlc_id"].as_str().unwrap();
    let secret = create_response["secret"].as_str().unwrap();
    
    // First claim should succeed
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "claim",
            "--htlc-id", htlc_id,
            "--secret", secret
        ])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    
    // Second claim should fail
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .args(&[
            "claim",
            "--htlc-id", htlc_id,
            "--secret", secret
        ])
        .output()
        .unwrap();
    
    assert!(!output.status.success());
    let error_response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(error_response["error"].as_str().unwrap().contains("already claimed"));
}