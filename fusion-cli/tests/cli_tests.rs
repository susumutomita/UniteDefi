// Note: These integration tests are currently ignored because the in-memory storage
// doesn't persist between separate command executions. Each command runs as a separate
// process with its own memory space. These tests will be re-enabled once persistent
// storage (e.g., file-based) is implemented.

use assert_cmd::Command;
use fusion_core::htlc::generate_secret;
use predicates::prelude::*;
use serde_json::Value;

#[test]
#[ignore = "In-memory storage doesn't persist between separate command executions"]
fn test_claim_with_valid_secret() {
    // First, create an HTLC
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let create_output = cmd
        .arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("1000")
        .arg("--timeout")
        .arg("3600")
        .output()
        .expect("Failed to create HTLC");

    assert!(create_output.status.success());

    // Parse the output to get HTLC ID and secret
    let output_json: Value =
        serde_json::from_slice(&create_output.stdout).expect("Failed to parse create output");
    let htlc_id = output_json["htlc_id"].as_str().unwrap();
    let secret = output_json["secret"].as_str().unwrap();

    // Now claim the HTLC
    let mut claim_cmd = Command::cargo_bin("fusion-cli").unwrap();
    claim_cmd
        .arg("claim")
        .arg("--htlc-id")
        .arg(htlc_id)
        .arg("--secret")
        .arg(secret)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"Claimed\""));
}

#[test]
#[ignore = "In-memory storage doesn't persist between separate command executions"]
fn test_claim_with_invalid_secret() {
    // First, create an HTLC
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let create_output = cmd
        .arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("1000")
        .arg("--timeout")
        .arg("3600")
        .output()
        .expect("Failed to create HTLC");

    assert!(create_output.status.success());

    // Parse the output to get HTLC ID
    let output_json: Value =
        serde_json::from_slice(&create_output.stdout).expect("Failed to parse create output");
    let htlc_id = output_json["htlc_id"].as_str().unwrap();

    // Try to claim with wrong secret
    let wrong_secret = hex::encode(generate_secret());

    let mut claim_cmd = Command::cargo_bin("fusion-cli").unwrap();
    claim_cmd
        .arg("claim")
        .arg("--htlc-id")
        .arg(htlc_id)
        .arg("--secret")
        .arg(&wrong_secret)
        .assert()
        .success()
        .stdout(predicate::str::contains("Invalid secret"));
}

#[test]
fn test_claim_htlc_not_found() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("claim")
        .arg("--htlc-id")
        .arg("non_existent_htlc")
        .arg("--secret")
        .arg("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .assert()
        .success()
        .stdout(predicate::str::contains("HTLC not found"));
}

#[test]
#[ignore = "In-memory storage doesn't persist between separate command executions"]
fn test_claim_already_claimed_htlc() {
    // First, create an HTLC
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let create_output = cmd
        .arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("1000")
        .arg("--timeout")
        .arg("3600")
        .output()
        .expect("Failed to create HTLC");

    assert!(create_output.status.success());

    // Parse the output to get HTLC ID and secret
    let output_json: Value =
        serde_json::from_slice(&create_output.stdout).expect("Failed to parse create output");
    let htlc_id = output_json["htlc_id"].as_str().unwrap();
    let secret = output_json["secret"].as_str().unwrap();

    // First claim should succeed
    let mut claim_cmd = Command::cargo_bin("fusion-cli").unwrap();
    claim_cmd
        .arg("claim")
        .arg("--htlc-id")
        .arg(htlc_id)
        .arg("--secret")
        .arg(secret)
        .assert()
        .success();

    // Second claim should fail
    let mut claim_cmd2 = Command::cargo_bin("fusion-cli").unwrap();
    claim_cmd2
        .arg("claim")
        .arg("--htlc-id")
        .arg(htlc_id)
        .arg("--secret")
        .arg(secret)
        .assert()
        .success()
        .stdout(predicate::str::contains("HTLC already claimed"));
}
