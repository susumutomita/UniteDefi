# NEAR HTLC for 1inch Fusion+ Cross-Chain Swap

This repository contains a NEAR Protocol implementation of Hash Time Locked Contracts (HTLC) designed specifically for 1inch Fusion+ cross-chain swaps.

## 🎯 Key Findings from Research

### NEAR Protocol Specifics

1. No Solidity Support: NEAR uses WebAssembly runtime, not EVM. Smart contracts must be written in Rust or AssemblyScript.

2. Asynchronous Execution: Cross-contract calls use Promises and execute asynchronously, requiring callback patterns for error handling.

3. Account Model:
   - Human-readable account IDs (e.g., alice.near)
   - Multiple access keys with different permissions
   - Storage requires staking (1 NEAR per 100KB)

4. No Automatic Rollback: Failed transactions don't automatically revert state. Manual rollback logic is required.

5. Gas Limitations: Maximum 300 TGas per transaction (~300ms compute time).

### Cross-Chain Integration Options

1. Direct HTLC Implementation (Recommended)
   - Implement matching HTLCs on NEAR and Ethereum
   - Use relayer service for coordination
   - Lower latency and cost

2. Rainbow Bridge
   - Existing trustless bridge between NEAR and Ethereum
   - Higher latency (6 min ETH→NEAR, 16 hrs NEAR→ETH)
   - Higher cost (~$10-60 per transfer)
   - Not ideal for atomic swaps due to delays

## 📁 Project Structure

```
near-htlc/
├── src/
│   ├── lib.rs              # Main module exports
│   └── fusion_htlc.rs      # 1inch Fusion+ compatible HTLC
├── examples/
│   └── cross_chain_swap.js # JavaScript coordination example
├── scripts/
│   └── deploy_testnet.sh   # Deployment automation
└── docs/
    └── NEAR_HTLC_Implementation_Guide.md
```

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install NEAR CLI
npm install -g near-cli

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Build and Deploy

```bash
# Run automated deployment
./scripts/deploy_testnet.sh

# Or manually:
# 1. Build contract
cargo build --target wasm32-unknown-unknown --release

# 2. Create testnet account
near create-account fusion-htlc.testnet --useFaucet

# 3. Deploy
near deploy fusion-htlc.testnet ./target/wasm32-unknown-unknown/release/near_htlc.wasm

# 4. Initialize
near call fusion-htlc.testnet new '{"owner": "your-account.testnet"}' --accountId fusion-htlc.testnet
```

### Create Escrow

```bash
near call fusion-htlc.testnet create_escrow '{
    "beneficiary": "alice.testnet",
    "secret_hash": "YOUR_SECRET_HASH_BASE58",
    "token_id": null,
    "amount": "1000000000000000000000000",
    "safety_deposit": "100000000000000000000000",
    "safety_deposit_beneficiary": null,
    "finality_period": 1800,
    "cancel_period": 3600,
    "public_cancel_period": 7200
}' --accountId resolver.testnet --deposit 1.1
```

### Claim with Secret

```bash
near call fusion-htlc.testnet claim '{
    "escrow_id": "fusion_0",
    "secret": "your_secret"
}' --accountId alice.testnet
```

## 🔑 Key Features

### Time Locks (1inch Fusion+ Compatible)

1. Finality Period: Only beneficiary can claim with secret
2. Cancel Period: Resolver can cancel after finality
3. Public Cancel Period: Anyone can cancel (cleanup)

### Safety Deposits

- Additional deposit to incentivize proper resolution
- Can be directed to a specific beneficiary
- Returned to resolver on cancel

### Token Support

- Native NEAR transfers
- NEP-141 token support (ERC-20 equivalent)
- Batch operations for efficiency

## 🔐 Security Considerations

1. Storage Attacks: Require storage deposits from users
2. Gas Exhaustion: Split complex operations
3. Callback Security: Use `#[private]` for callbacks
4. State Management: Update state before external calls

## 📊 Comparison with Ethereum Implementation

| Feature | Ethereum | NEAR |
|---------|----------|------|
| Language | Solidity | Rust |
| Execution | Synchronous | Asynchronous (Promises) |
| State Rollback | Automatic | Manual |
| Gas Model | Dynamic pricing | Fixed pricing |
| Storage | Included in gas | Separate staking |
| Account Model | Addresses | Named accounts |

## 🛠 Development Tools

- NEAR Explorer: https://explorer.testnet.near.org/
- NEAR Wallet: https://wallet.testnet.near.org/
- Documentation: https://docs.near.org/

## 📝 Next Steps

1. Testing: Comprehensive unit and integration tests
2. Audit: Security review of HTLC implementation
3. Relayer: Build coordination service for cross-chain swaps
4. Mainnet: Deploy after thorough testing

## 📚 Additional Resources

- [NEAR Protocol Documentation](https://docs.near.org/)
- [1inch Fusion+ Documentation](https://docs.1inch.io/docs/fusion-swap/introduction)
- [Implementation Guide](./docs/NEAR_HTLC_Implementation_Guide.md)

## ⚠️ Important Notes

- NEAR doesn't support Solidity or EVM
- Requires understanding of asynchronous programming
- Storage costs must be considered in contract design
- Manual state management for failed transactions

## License

MIT
