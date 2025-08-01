# UniteSwap Troubleshooting Guide

## Common Issues and Solutions

### Build Issues

#### Issue: `fusion-cli` not found
**Solution:**
```bash
# Build the CLI
cargo build -p fusion-cli --release

# The binary will be at:
./target/release/fusion-cli
```

#### Issue: Cargo build fails
**Solution:**
```bash
# Ensure Rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Runtime Issues

#### Issue: "HTLC not found" error
**Cause:** The HTLC ID doesn't exist in local storage
**Solution:** Create a new HTLC first using `create-htlc` command

#### Issue: "Invalid secret" error when claiming
**Cause:** The provided secret doesn't match the HTLC's secret hash
**Solution:** Use the exact secret that was generated when creating the HTLC

#### Issue: "HTLC has not timed out yet" when refunding
**Cause:** Attempting to refund before the timeout period expires
**Solution:** Wait for the timeout period to pass, or create HTLCs with shorter timeouts for testing

#### Issue: Invalid hex format errors
**Cause:** Hex strings must be properly formatted (even length, valid hex characters)
**Solution:** 
- Ensure hex strings don't have '0x' prefix
- Ensure even number of characters
- Use only 0-9 and a-f characters

### Order-Related Issues

#### Issue: Order creation fails
**Possible causes:**
1. Invalid address format
2. Incorrect chain ID
3. Wrong contract address

**Solution:**
- Verify all addresses are 40 hex characters (without 0x prefix)
- Use correct chain IDs:
  - Ethereum Mainnet: 1
  - Sepolia: 11155111
  - Base Sepolia: 84532
- Check contract deployments in docs

#### Issue: Orderbook shows empty
**Cause:** No active orders for the specified chain
**Solution:** Create some orders first, then check the orderbook

### Performance Issues

#### Issue: Commands running slowly
**Possible causes:**
1. Debug build instead of release build
2. Large storage files

**Solution:**
```bash
# Use release build
cargo build --release

# Clear old storage if needed
rm -rf ~/.fusion-cli/
```

### Error Messages Reference

| Error Message | Meaning | Solution |
|--------------|---------|----------|
| "HTLC not found" | HTLC ID doesn't exist | Check HTLC ID is correct |
| "Invalid secret format" | Secret is not valid hex | Ensure 64 hex characters |
| "Order already cancelled" | Order is no longer active | Cannot modify cancelled orders |
| "HTLC already claimed" | HTLC was already used | Cannot claim twice |
| "Invalid address format" | Ethereum address malformed | Use 40 hex chars without 0x |

## Getting Help

1. Check the main documentation: [README.md](README.md)
2. Review CLI help: `fusion-cli --help`
3. Check specific command help: `fusion-cli <command> --help`
4. Review code examples in [docs/limit-order-cli-guide.md](docs/limit-order-cli-guide.md)

## Debug Mode

For verbose output, set the `RUST_LOG` environment variable:
```bash
RUST_LOG=debug fusion-cli <command>
```

## Testing in Isolation

To test without affecting existing data:
```bash
# Use a different storage directory
FUSION_CLI_DATA_DIR=/tmp/test-fusion fusion-cli <command>
```