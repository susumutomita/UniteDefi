# UniteSwap Demo Setup

This command helps you set up and run a demo of UniteSwap for the ETHGlobal Unite hackathon.

## What I'll do:
1. Check all dependencies are installed
2. Build the project
3. Set up environment variables
4. Run a test swap
5. Generate demo materials

## Steps:

### 1. Environment Check
```bash
# Check Rust is installed
rustc --version

# Check NEAR CLI is installed
near --version

# Check Node.js (for 1inch integration)
node --version
```

### 2. Build Project
```bash
# Build the entire project
cargo build --release

# Run tests
cargo test
```

### 3. Environment Setup
```bash
# Load environment variables
source load-env.sh

# Verify variables are set
echo "Ethereum: $ETHEREUM_ADDRESS"
echo "NEAR: $NEAR_ACCOUNT_ID"
```

### 4. Demo Swap Commands

#### Small Test Swap (ETH → NEAR)
```bash
./target/release/fusion-cli swap \
  --from-chain ethereum \
  --to-chain near \
  --from-token WETH \
  --to-token NEAR \
  --amount 0.001 \
  --from-address $ETHEREUM_ADDRESS \
  --to-address $NEAR_ACCOUNT_ID \
  --dry-run
```

#### Production Swap with Auto-claim
```bash
./target/release/fusion-cli swap \
  --from-chain ethereum \
  --to-chain near \
  --from-token WETH \
  --to-token NEAR \
  --amount 0.01 \
  --from-address $ETHEREUM_ADDRESS \
  --to-address $NEAR_ACCOUNT_ID \
  --auto-claim \
  --monitor-interval 30
```

### 5. Monitor Swap Status
The swap will automatically:
- Create orders on both chains
- Monitor for execution
- Claim funds when available
- Show transaction hashes for verification

### 6. Demo Checklist
- [ ] Environment variables loaded
- [ ] Sufficient balance on both chains
- [ ] Test swap completed successfully
- [ ] Transaction links ready for presentation
- [ ] Backup demo video recorded

## Troubleshooting:
- If NEAR commands fail: Check NEAR_ACCOUNT_ID is logged in
- If Ethereum fails: Verify PRIVATE_KEY and RPC_URL
- For 1inch errors: Check ONEINCH_API_KEY is set