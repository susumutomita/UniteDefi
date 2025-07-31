#[cfg(test)]
mod order_cli_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_order_create_command() {
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
            .stdout(predicate::str::contains("eip712_hash"))
            .stdout(predicate::str::contains("order"))
            .stdout(predicate::str::contains("domain"));
    }

    #[test]
    fn test_order_create_with_invalid_address() {
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
}
