use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

#[test]
fn test_create_htlc_success() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("1000")
        .arg("--timeout")
        .arg("3600");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"htlc_id\""))
        .stdout(predicate::str::contains("\"secret\""))
        .stdout(predicate::str::contains("\"secret_hash\""))
        .stdout(predicate::str::contains("\"status\": \"Pending\""));
}

#[test]
fn test_create_htlc_with_custom_timeout() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("create-htlc")
        .arg("--sender")
        .arg("Charlie")
        .arg("--recipient")
        .arg("David")
        .arg("--amount")
        .arg("5000")
        .arg("--timeout")
        .arg("7200");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"timeout_seconds\": 7200"));
}

#[test]
fn test_claim_htlc_not_found() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("claim")
        .arg("--htlc-id")
        .arg("non_existent_htlc_id")
        .arg("--secret")
        .arg("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"error\": \"HTLC not found\""));
}

#[test]
fn test_claim_with_invalid_secret_format() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("claim")
        .arg("--htlc-id")
        .arg("some_htlc_id")
        .arg("--secret")
        .arg("invalid_secret");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"error\": \"Invalid secret format\""));
}

#[test]
fn test_claim_with_invalid_secret_length() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("claim")
        .arg("--htlc-id")
        .arg("some_htlc_id")
        .arg("--secret")
        .arg("0x1234");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"error\": \"Invalid secret length\""));
}

#[test]
fn test_refund_htlc_not_found() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("refund")
        .arg("--htlc-id")
        .arg("non_existent_htlc_id");

    cmd.assert().failure();
}

#[test]
fn test_order_create_success() {
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
        .arg("84532")
        .arg("--verifying-contract")
        .arg("0x171C87724E720F2806fc29a010a62897B30fdb62");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"eip712_hash\""))
        .stdout(predicate::str::contains("\"order\""))
        .stdout(predicate::str::contains("\"domain\""))
        .stdout(predicate::str::contains("\"htlc_info\""));
}

#[test]
fn test_order_create_with_optional_fields() {
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
        .arg("84532")
        .arg("--verifying-contract")
        .arg("0x171C87724E720F2806fc29a010a62897B30fdb62")
        .arg("--receiver")
        .arg("0x123456789012345678901234567890123456789a")
        .arg("--allowed-sender")
        .arg("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"receiver\": \"0x123456789012345678901234567890123456789a\""))
        .stdout(predicate::str::contains("\"allowedSender\": \"0xabcdefabcdefabcdefabcdefabcdefabcdefabcd\""));
}

#[test]
fn test_order_create_with_invalid_maker_asset() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create")
        .arg("--maker-asset")
        .arg("invalid_address")
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
fn test_order_create_with_invalid_secret_hash() {
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
        .arg("invalid_hash")
        .arg("--htlc-timeout")
        .arg("3600")
        .arg("--chain-id")
        .arg("84532")
        .arg("--verifying-contract")
        .arg("0x171C87724E720F2806fc29a010a62897B30fdb62");

    cmd.assert().failure();
}

#[test]
fn test_order_create_near_with_secret_generation() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice.near")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("10.5")
        .arg("--generate-secret");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"secret\""))
        .stdout(predicate::str::contains("\"warning\": \"KEEP THIS SECRET!\""))
        .stdout(predicate::str::contains("\"order\""))
        .stdout(predicate::str::contains("\"details\""));
}

#[test]
fn test_order_create_near_with_provided_secret_hash() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("bob.testnet")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("5.0")
        .arg("--secret-hash")
        .arg("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--slippage-bps")
        .arg("200");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"slippage_bps\": 200"))
        .stdout(predicate::str::is_match("\"secret\"").unwrap().not());
}

#[test]
fn test_order_create_near_invalid_account() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("invalid_account")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("10.0")
        .arg("--generate-secret");

    cmd.assert().failure();
}

#[test]
fn test_order_create_near_invalid_ethereum_address() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice.near")
        .arg("--ethereum-address")
        .arg("invalid_address")
        .arg("--near-amount")
        .arg("10.0")
        .arg("--generate-secret");

    cmd.assert().failure();
}

#[test]
fn test_order_create_near_no_secret_provided() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice.near")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("10.0");

    cmd.assert().failure();
}

#[test]
fn test_order_status_not_found() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("status")
        .arg("--order-id")
        .arg("non_existent_order");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"error\": \"Order not found\""));
}

#[test]
fn test_order_cancel_not_found() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order")
        .arg("cancel")
        .arg("--order-id")
        .arg("non_existent_order");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"error\": \"Order not found\""));
}

#[test]
fn test_orderbook_empty() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("orderbook")
        .arg("--chain")
        .arg("ethereum");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"orders\": []"));
}

#[test]
fn test_relay_order_minimal_args() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("relay-order")
        .arg("--order-hash")
        .arg("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--to-chain")
        .arg("near")
        .arg("--htlc-secret")
        .arg("0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"relay_status\""));
}

#[test]
fn test_relay_order_with_all_args() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("relay-order")
        .arg("--order-hash")
        .arg("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--to-chain")
        .arg("near")
        .arg("--htlc-secret")
        .arg("0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba")
        .arg("--near-account")
        .arg("alice.testnet")
        .arg("--evm-rpc")
        .arg("https://sepolia.base.org")
        .arg("--near-network")
        .arg("testnet");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"near_account\": \"alice.testnet\""));
}

#[test]
fn test_relay_order_invalid_order_hash() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("relay-order")
        .arg("--order-hash")
        .arg("invalid_hash")
        .arg("--to-chain")
        .arg("near")
        .arg("--htlc-secret")
        .arg("0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba");

    cmd.assert().failure();
}

#[test]
fn test_relay_order_invalid_secret() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("relay-order")
        .arg("--order-hash")
        .arg("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--to-chain")
        .arg("near")
        .arg("--htlc-secret")
        .arg("invalid_secret");

    cmd.assert().failure();
}

#[test]
fn test_relay_order_unsupported_chain() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("relay-order")
        .arg("--order-hash")
        .arg("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        .arg("--to-chain")
        .arg("unsupported_chain")
        .arg("--htlc-secret")
        .arg("0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba");

    cmd.assert().failure();
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("UniteSwap CLI"))
        .stdout(predicate::str::contains("create-htlc"))
        .stdout(predicate::str::contains("claim"))
        .stdout(predicate::str::contains("refund"))
        .stdout(predicate::str::contains("order"))
        .stdout(predicate::str::contains("relay-order"))
        .stdout(predicate::str::contains("orderbook"));
}

#[test]
fn test_order_help_command() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("order").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("create-near"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("cancel"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn create_test_htlc() -> (String, String) {
        let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
        let output = cmd
            .arg("create-htlc")
            .arg("--sender")
            .arg("TestSender")
            .arg("--recipient")
            .arg("TestRecipient")
            .arg("--amount")
            .arg("5000")
            .arg("--timeout")
            .arg("3600")
            .output()
            .expect("Failed to create HTLC");

        assert!(output.status.success());

        let output_json: Value =
            serde_json::from_slice(&output.stdout).expect("Failed to parse output");
        let htlc_id = output_json["htlc_id"].as_str().unwrap().to_string();
        let secret = output_json["secret"].as_str().unwrap().to_string();

        (htlc_id, secret)
    }

    #[test]
    #[ignore = "Storage doesn't persist between commands in tests"]
    fn test_htlc_full_lifecycle() {
        // Create HTLC
        let (htlc_id, secret) = create_test_htlc();

        // Attempt to claim with valid secret
        let mut claim_cmd = Command::cargo_bin("fusion-cli").unwrap();
        claim_cmd
            .arg("claim")
            .arg("--htlc-id")
            .arg(&htlc_id)
            .arg("--secret")
            .arg(&secret)
            .assert()
            .success()
            .stdout(predicate::str::contains("\"status\": \"Claimed\""));

        // Attempt to claim again (should fail)
        let mut claim_again_cmd = Command::cargo_bin("fusion-cli").unwrap();
        claim_again_cmd
            .arg("claim")
            .arg("--htlc-id")
            .arg(&htlc_id)
            .arg("--secret")
            .arg(&secret)
            .assert()
            .success()
            .stdout(predicate::str::contains("HTLC already claimed"));
    }

    #[test]
    fn test_json_output_format() {
        let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
        let output = cmd
            .arg("create-htlc")
            .arg("--sender")
            .arg("Alice")
            .arg("--recipient")
            .arg("Bob")
            .arg("--amount")
            .arg("1000")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());

        // Verify valid JSON output
        let json_result: Result<Value, _> = serde_json::from_slice(&output.stdout);
        assert!(json_result.is_ok(), "Output is not valid JSON");

        let json = json_result.unwrap();
        assert!(json.is_object(), "Output should be a JSON object");
        assert!(json.get("htlc_id").is_some(), "Missing htlc_id field");
        assert!(json.get("secret").is_some(), "Missing secret field");
        assert!(json.get("secret_hash").is_some(), "Missing secret_hash field");
    }
}