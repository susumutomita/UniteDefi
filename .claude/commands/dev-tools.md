# UniteSwap Developer Tools

Quick commands for development and debugging UniteSwap.

## Common Development Tasks

### Run Quality Checks
```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all

# Security audit
cargo audit
```

### Create and Sign Order (without swap)
```bash
./target/release/fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x0000000000000000000000000000000000000000 \
  --maker $ETHEREUM_ADDRESS \
  --making-amount 1000000000000000 \
  --taking-amount 5000000000000000000 \
  --htlc-secret-hash 0x$(openssl rand -hex 32) \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62 \
  --sign \
  --submit
```

### Check HTLC Status on NEAR
```bash
# View specific escrow
near view htlc-v2.testnet get_escrow '{"escrow_id": "escrow_15"}'

# List all escrows for an account
near view htlc-v2.testnet get_escrows_by_recipient '{"recipient": "'$NEAR_ACCOUNT_ID'"}'
```

### Monitor Ethereum Events
```bash
# Watch for order fills (requires cast from Foundry)
cast logs \
  --rpc-url $ETHEREUM_RPC_URL \
  --address 0x171C87724E720F2806fc29a010a62897B30fdb62 \
  --from-block latest
```

### Debug Transaction
```bash
# Get transaction details
cast tx <TX_HASH> --rpc-url $ETHEREUM_RPC_URL

# Decode transaction data
cast 4byte-decode <FUNCTION_SELECTOR>
```

### Generate Test Secrets
```bash
# Generate secret and hash for testing
SECRET=$(openssl rand -hex 32)
HASH=$(echo -n $SECRET | xxd -r -p | sha256sum | cut -d' ' -f1)
echo "Secret: $SECRET"
echo "Hash: $HASH"
```

### Environment Debugging
```bash
# Check all required variables
./fusion-cli debug env

# Test connections
./fusion-cli debug connections
```

### Performance Testing
```bash
# Run benchmarks
cargo bench

# Profile with flamegraph
cargo flamegraph --bin fusion-cli -- swap --dry-run \
  --from-chain ethereum --to-chain near \
  --from-token WETH --to-token NEAR \
  --amount 0.001 \
  --from-address $ETHEREUM_ADDRESS \
  --to-address $NEAR_ACCOUNT_ID
```

## Quick Fixes

### Reset NEAR HTLC
```bash
# Refund expired HTLC
near call htlc-v2.testnet refund '{"escrow_id": "escrow_X"}' \
  --use-account $NEAR_ACCOUNT_ID
```

### Cancel Ethereum Order
```bash
# Cancel order (if supported by contract)
cast send 0x171C87724E720F2806fc29a010a62897B30fdb62 \
  "cancelOrder(bytes32)" <ORDER_HASH> \
  --private-key $PRIVATE_KEY \
  --rpc-url $ETHEREUM_RPC_URL
```