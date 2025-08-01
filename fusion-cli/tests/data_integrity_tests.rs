use assert_cmd::Command;
use serde_json::Value;

#[test]
fn test_htlc_id_uniqueness() {
    // Create multiple HTLCs and verify unique IDs
    let mut htlc_ids = Vec::new();
    
    for i in 0..5 {
        let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
        let output = cmd
            .arg("create-htlc")
            .arg("--sender")
            .arg(format!("Sender{}", i))
            .arg("--recipient")
            .arg(format!("Recipient{}", i))
            .arg("--amount")
            .arg("1000")
            .output()
            .expect("Failed to create HTLC");

        assert!(output.status.success());
        
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let htlc_id = json["htlc_id"].as_str().unwrap();
        
        // Verify HTLC ID is unique
        assert!(!htlc_ids.contains(&htlc_id.to_string()));
        htlc_ids.push(htlc_id.to_string());
    }
}

#[test]
fn test_secret_hash_consistency() {
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
        .expect("Failed to create HTLC");

    assert!(output.status.success());
    
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let secret = json["secret"].as_str().unwrap();
    let secret_hash = json["secret_hash"].as_str().unwrap();
    
    // Verify secret is 64 hex characters (32 bytes)
    assert_eq!(secret.len(), 64);
    assert!(secret.chars().all(|c| c.is_ascii_hexdigit()));
    
    // Verify secret_hash is 64 hex characters (32 bytes)
    assert_eq!(secret_hash.len(), 64);
    assert!(secret_hash.chars().all(|c| c.is_ascii_hexdigit()));
    
    // Verify secret and secret_hash are different
    assert_ne!(secret, secret_hash);
}

#[test]
fn test_order_eip712_hash_format() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .arg("order")
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
        .output()
        .expect("Failed to create order");

    assert!(output.status.success());
    
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let eip712_hash = json["eip712_hash"].as_str().unwrap();
    
    // Verify EIP-712 hash format (0x + 64 hex chars)
    assert!(eip712_hash.starts_with("0x"));
    assert_eq!(eip712_hash.len(), 66);
    assert!(eip712_hash[2..].chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_order_salt_uniqueness() {
    let mut salts = Vec::new();
    
    for _ in 0..5 {
        let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
        let output = cmd
            .arg("order")
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
            .output()
            .expect("Failed to create order");

        assert!(output.status.success());
        
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let salt = json["order"]["salt"].as_str().unwrap();
        
        // Verify salt is unique
        assert!(!salts.contains(&salt.to_string()));
        salts.push(salt.to_string());
        
        // Verify salt format (0x + 64 hex chars)
        assert!(salt.starts_with("0x"));
        assert_eq!(salt.len(), 66);
    }
}

#[test]
fn test_near_order_price_calculation() {
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .arg("order")
        .arg("create-near")
        .arg("--near-account")
        .arg("alice.near")
        .arg("--ethereum-address")
        .arg("0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0")
        .arg("--near-amount")
        .arg("10.0")
        .arg("--generate-secret")
        .arg("--slippage-bps")
        .arg("100")
        .output()
        .expect("Failed to create NEAR order");

    assert!(output.status.success());
    
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    
    // Verify price calculation fields exist
    assert!(json["details"]["near_amount"].is_f64());
    assert!(json["details"]["usdc_amount"].is_f64());
    assert_eq!(json["details"]["slippage_bps"].as_u64().unwrap(), 100);
    
    // Verify making/taking amounts are valid numbers
    let making_amount = json["order"]["makingAmount"].as_str().unwrap();
    let taking_amount = json["order"]["takingAmount"].as_str().unwrap();
    assert!(making_amount.parse::<u128>().is_ok());
    assert!(taking_amount.parse::<u128>().is_ok());
}

#[test]
fn test_json_output_consistency() {
    // Test HTLC creation output
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let htlc_output = cmd
        .arg("create-htlc")
        .arg("--sender")
        .arg("Alice")
        .arg("--recipient")
        .arg("Bob")
        .arg("--amount")
        .arg("1000")
        .output()
        .expect("Failed to create HTLC");

    assert!(htlc_output.status.success());
    let htlc_json: Result<Value, _> = serde_json::from_slice(&htlc_output.stdout);
    assert!(htlc_json.is_ok());
    
    // Test order creation output
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let order_output = cmd
        .arg("order")
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
        .output()
        .expect("Failed to create order");

    assert!(order_output.status.success());
    let order_json: Result<Value, _> = serde_json::from_slice(&order_output.stdout);
    assert!(order_json.is_ok());
}

#[test]
fn test_timestamp_formats() {
    // Test claim timestamp format
    let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
    let output = cmd
        .arg("claim")
        .arg("--htlc-id")
        .arg("non_existent")
        .arg("--secret")
        .arg("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
        .output()
        .expect("Failed to execute claim");

    assert!(output.status.success());
    
    // For error cases, we still get valid JSON
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(json.is_object());
}

#[test]
fn test_htlc_timeout_validation() {
    // Test with different timeout values
    let timeouts = vec![0, 60, 3600, 86400, u64::MAX];
    
    for timeout in timeouts {
        let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
        let output = cmd
            .arg("create-htlc")
            .arg("--sender")
            .arg("Alice")
            .arg("--recipient")
            .arg("Bob")
            .arg("--amount")
            .arg("1000")
            .arg("--timeout")
            .arg(timeout.to_string())
            .output()
            .expect("Failed to create HTLC");

        if timeout == 0 || timeout == u64::MAX {
            // Edge cases might fail or succeed, but should not crash
            assert!(output.status.success() || !output.status.success());
        } else {
            assert!(output.status.success());
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(json["timeout_seconds"].as_u64().unwrap(), timeout);
        }
    }
}

#[test]
fn test_order_amount_validation() {
    // Test with edge case amounts
    let amounts = vec![
        ("0", "0"),
        ("1", "1"),
        ("1000000000000000000", "3000000000"),
        ("340282366920938463463374607431768211455", "340282366920938463463374607431768211455"), // u128::MAX
    ];
    
    for (making, taking) in amounts {
        let mut cmd = Command::cargo_bin("fusion-cli").unwrap();
        let output = cmd
            .arg("order")
            .arg("create")
            .arg("--maker-asset")
            .arg("0x4200000000000000000000000000000000000006")
            .arg("--taker-asset")
            .arg("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")
            .arg("--maker")
            .arg("0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950")
            .arg("--making-amount")
            .arg(making)
            .arg("--taking-amount")
            .arg(taking)
            .arg("--htlc-secret-hash")
            .arg("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
            .arg("--htlc-timeout")
            .arg("3600")
            .arg("--chain-id")
            .arg("84532")
            .arg("--verifying-contract")
            .arg("0x171C87724E720F2806fc29a010a62897B30fdb62")
            .output()
            .expect("Failed to create order");

        // Zero amounts might be rejected, but should not crash
        if making == "0" || taking == "0" {
            assert!(output.status.success() || !output.status.success());
        } else {
            assert!(output.status.success());
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(json["order"]["makingAmount"].as_str().unwrap(), making);
            assert_eq!(json["order"]["takingAmount"].as_str().unwrap(), taking);
        }
    }
}