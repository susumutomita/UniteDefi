use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_swap_command_exists() {
    // Test that the swap command is available
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap").arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Integrated cross-chain token swap",
    ));
}

#[test]
fn test_swap_ethereum_to_near() {
    // Test basic Ethereum to NEAR swap parameters
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap")
        .arg("swap")
        .arg("--from-chain")
        .arg("ethereum")
        .arg("--to-chain")
        .arg("near")
        .arg("--from-token")
        .arg("0x4200000000000000000000000000000000000006")
        .arg("--to-token")
        .arg("NEAR")
        .arg("--amount")
        .arg("1.0")
        .arg("--from-address")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--to-address")
        .arg("alice.near")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("swap_plan"));
}

#[test]
fn test_swap_near_to_ethereum() {
    // Test basic NEAR to Ethereum swap parameters
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap")
        .arg("swap")
        .arg("--from-chain")
        .arg("near")
        .arg("--to-chain")
        .arg("ethereum")
        .arg("--from-token")
        .arg("NEAR")
        .arg("--to-token")
        .arg("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")
        .arg("--amount")
        .arg("10.0")
        .arg("--from-address")
        .arg("alice.near")
        .arg("--to-address")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("swap_plan"));
}

#[test]
fn test_swap_requires_from_chain() {
    // Test that from-chain is required
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap").arg("swap").arg("--to-chain").arg("near");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--from-chain"));
}

#[test]
fn test_swap_with_slippage() {
    // Test swap with custom slippage
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap")
        .arg("swap")
        .arg("--from-chain")
        .arg("ethereum")
        .arg("--to-chain")
        .arg("near")
        .arg("--from-token")
        .arg("WETH")
        .arg("--to-token")
        .arg("NEAR")
        .arg("--amount")
        .arg("0.5")
        .arg("--from-address")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--to-address")
        .arg("alice.near")
        .arg("--slippage")
        .arg("0.5")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("swap_plan"));
}

#[test]
fn test_swap_with_timeout() {
    // Test swap with custom timeout
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap")
        .arg("swap")
        .arg("--from-chain")
        .arg("ethereum")
        .arg("--to-chain")
        .arg("near")
        .arg("--from-token")
        .arg("WETH")
        .arg("--to-token")
        .arg("NEAR")
        .arg("--amount")
        .arg("0.5")
        .arg("--from-address")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--to-address")
        .arg("alice.near")
        .arg("--timeout")
        .arg("7200")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("swap_plan"));
}

#[test]
fn test_swap_invalid_chain() {
    // Test invalid chain name
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap")
        .arg("swap")
        .arg("--from-chain")
        .arg("invalid")
        .arg("--to-chain")
        .arg("near")
        .arg("--from-token")
        .arg("WETH")
        .arg("--to-token")
        .arg("NEAR")
        .arg("--amount")
        .arg("1.0")
        .arg("--from-address")
        .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
        .arg("--to-address")
        .arg("alice.near")
        .arg("--dry-run");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid from_chain"));
}

#[test]
fn test_swap_batch_config() {
    // Test batch swap configuration
    use std::fs;
    use std::io::Write;

    // Create a temporary config file
    let config_content = r#"[
        {
            "from_chain": "ethereum",
            "to_chain": "near",
            "from_token": "WETH",
            "to_token": "NEAR",
            "amount": 0.5,
            "from_address": "0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950",
            "to_address": "alice.near"
        },
        {
            "from_chain": "near",
            "to_chain": "ethereum",
            "from_token": "NEAR",
            "to_token": "USDC",
            "amount": 10.0,
            "from_address": "bob.near",
            "to_address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
        }
    ]"#;

    let config_path = "/tmp/test_swaps.json";
    let mut file = fs::File::create(config_path).unwrap();
    file.write_all(config_content.as_bytes()).unwrap();

    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    cmd.arg("swap")
        .arg("batch")
        .arg("--config")
        .arg(config_path)
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("batch_swap_plan"));

    // Clean up
    let _ = fs::remove_file(config_path);
}
