use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("invalid-command");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

#[test]
fn test_missing_required_args_create_htlc() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("create-htlc");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required arguments"));
}

#[test]
fn test_missing_required_args_claim() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("claim")
        .arg("--htlc-id")
        .arg("some_id");
    // Missing --secret

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_missing_required_args_order_create() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create")
        .arg("--maker-asset")
        .arg("0x4200000000000000000000000000000000000006");
    // Missing many required args

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_invalid_hex_format_in_addresses() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create")
        .arg("--maker-asset")
        .arg("0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG") // Invalid hex
        .arg("--taker-asset")
        .arg("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")
        .arg("--maker")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--making-amount")
        .arg("1000000000000000000")
        .arg("--taking-amount")
        .arg("3000000000")
        .arg("--htlc-secret-hash")
        .arg("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--htlc-timeout")
        .arg("3600")
        .arg("--chain-id")
        .arg("84532")
        .arg("--verifying-contract")
        .arg("0x171C87724E720F2806fc29a010a62897B30fdb62");

    cmd.assert().failure();
}

#[test]
fn test_invalid_amount_format() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("not_a_number")
        .arg("--timeout")
        .arg("3600");

    cmd.assert().failure();
}

#[test]
fn test_negative_amount() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("-1000")
        .arg("--timeout")
        .arg("3600");

    cmd.assert().failure();
}

#[test]
fn test_invalid_secret_hash_length() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create")
        .arg("--maker-asset")
        .arg("0x4200000000000000000000000000000000000006")
        .arg("--taker-asset")
        .arg("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")
        .arg("--maker")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--making-amount")
        .arg("1000000000000000000")
        .arg("--taking-amount")
        .arg("3000000000")
        .arg("--htlc-secret-hash")
        .arg("1234") // Too short
        .arg("--htlc-timeout")
        .arg("3600")
        .arg("--chain-id")
        .arg("84532")
        .arg("--verifying-contract")
        .arg("0x171C87724E720F2806fc29a010a62897B30fdb62");

    cmd.assert().failure();
}

#[test]
fn test_invalid_chain_id() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create")
        .arg("--maker-asset")
        .arg("0x4200000000000000000000000000000000000006")
        .arg("--taker-asset")
        .arg("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")
        .arg("--maker")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--making-amount")
        .arg("1000000000000000000")
        .arg("--taking-amount")
        .arg("3000000000")
        .arg("--htlc-secret-hash")
        .arg("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--htlc-timeout")
        .arg("3600")
        .arg("--chain-id")
        .arg("not_a_number")
        .arg("--verifying-contract")
        .arg("0x171C87724E720F2806fc29a010a62897B30fdb62");

    cmd.assert().failure();
}

#[test]
fn test_invalid_near_amount() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice.near")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("not_a_number")
        .arg("--generate-secret");

    cmd.assert().failure();
}

#[test]
fn test_invalid_slippage_bps() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice.near")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("10.0")
        .arg("--generate-secret")
        .arg("--slippage-bps")
        .arg("not_a_number");

    cmd.assert().failure();
}

#[test]
fn test_conflicting_secret_args() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice.near")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("10.0")
        .arg("--generate-secret")
        .arg("--secret-hash")
        .arg("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");

    // Should still work, but will prefer generate-secret
    cmd.assert().success();
}

#[test]
fn test_invalid_near_network() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("relay-order")
        .arg("--order-hash")
        .arg("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--to-chain")
        .arg("near")
        .arg("--htlc-secret")
        .arg("0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba")
        .arg("--near-network")
        .arg("invalid_network");

    cmd.assert().failure();
}

#[test]
fn test_empty_orderbook_chain() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("orderbook")
        .arg("--chain")
        .arg(""); // Empty chain

    cmd.assert().success(); // Should handle gracefully
}

#[test]
fn test_special_characters_in_strings() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("create-htlc")
        .arg("--sender")
        .arg("Alice@#$%")
        .arg("--recipient")
        .arg("Bob!@#$%^&*()")
        .arg("--amount")
        .arg("1000")
        .arg("--timeout")
        .arg("3600");

    cmd.assert().success(); // Should handle special characters
}

#[test]
fn test_unicode_in_near_account() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice🚀.near") // Unicode emoji
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("10.0")
        .arg("--generate-secret");

    // NEAR accounts might support unicode, but CLI should handle it
    cmd.assert().failure(); // Likely fails validation
}

#[test]
fn test_overflow_timeout() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("1000")
        .arg("--timeout")
        .arg("99999999999999999999"); // Overflow attempt

    cmd.assert().failure();
}

#[test]
fn test_multiple_errors_at_once() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create")
        .arg("--maker-asset")
        .arg("invalid")
        .arg("--taker-asset")
        .arg("also_invalid")
        .arg("--maker")
        .arg("not_an_address")
        .arg("--making-amount")
        .arg("not_a_number")
        .arg("--taking-amount")
        .arg("-100")
        .arg("--htlc-secret-hash")
        .arg("short")
        .arg("--htlc-timeout")
        .arg("negative")
        .arg("--chain-id")
        .arg("text")
        .arg("--verifying-contract")
        .arg("bad");

    cmd.assert().failure();
}

#[test]
fn test_help_for_subcommands() {
    // Test help for order subcommand
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Order commands"));

    // Test help for specific order subcommand
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order").arg("create").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Create a new limit order"));
}

#[test]
fn test_concurrent_command_execution() {
    use std::thread;
    
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
                cmd.arg("create-htlc")
                    .arg("--sender")
                    .arg(format!("Sender{}", i))
                    .arg("--recipient")
                    .arg(format!("Recipient{}", i))
                    .arg("--amount")
                    .arg("1000")
                    .arg("--timeout")
                    .arg("3600")
                    .assert()
                    .success();
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}