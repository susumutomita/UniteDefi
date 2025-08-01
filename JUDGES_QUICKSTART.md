# UniteSwap - Judges Quick Start Guide
## ETHGlobal Unite Hackathon Submission

### 🎯 Project Summary
UniteSwap extends 1inch Fusion+ protocol to enable trustless atomic swaps between EVM and non-EVM chains (NEAR, Cosmos, Stellar) using a Rust CLI implementation.

### 🚀 Fastest Demo (< 5 minutes)

#### Option 1: Run Automated Demo
```bash
# Clone and build (one-time setup)
git clone https://github.com/susumutomita/UniteDefi.git
cd UniteDefi
cargo build -p fusion-cli --release

# Run the demo
chmod +x demo_verification.sh
./demo_verification.sh
```

#### Option 2: Manual Quick Test
```bash
# 1. Create an HTLC (atomic swap building block)
./target/release/fusion-cli create-htlc \
  --sender 0x1234567890123456789012345678901234567890 \
  --recipient 0x9876543210987654321098765432109876543210 \
  --amount 1000000000000000000

# 2. Create a cross-chain limit order
./target/release/fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash 6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263 \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62

# 3. View the orderbook
./target/release/fusion-cli orderbook --chain ethereum
```

### ✅ Key Features Demonstrated

1. **HTLC Implementation** ✓
   - Hash time lock contracts for atomic swaps
   - Preserves 1inch Fusion+ security guarantees

2. **Cross-Chain Orders** ✓
   - EVM to non-EVM swaps
   - NEAR integration complete
   - Modular design for adding chains

3. **1inch Integration** ✓
   - Compatible with Limit Order Protocol v3
   - Uses official contract addresses
   - EIP-712 signature support

4. **CLI Interface** ✓
   - Full-featured command line tool
   - JSON output for integration
   - Comprehensive error handling

### 📊 Hackathon Requirements Met

| Requirement | Status | Evidence |
|------------|--------|----------|
| Hashlock/Timelock Preservation | ✅ | See `fusion-core/src/htlc.rs` |
| Bidirectional Swaps | ✅ | EVM↔NEAR implemented |
| On-chain Demo | ✅ | Testnet deployments ready |
| 1inch Escrow | ✅ | Uses official contracts |
| Partial Fills | ✅ | Multiple secrets supported |
| Relayer | ✅ | See `relay-order` command |
| CLI | ✅ | Full implementation |

### 🔍 Code Highlights

#### Core HTLC Implementation
- **File**: `fusion-core/src/htlc.rs`
- **Key Feature**: Generic trait for cross-chain compatibility

#### NEAR Integration
- **File**: `contracts/near-htlc/src/fusion_htlc.rs`
- **Key Feature**: NEAR-native HTLC with Fusion+ compatibility

#### Order Management
- **File**: `fusion-cli/src/order_handler.rs`
- **Key Feature**: EIP-712 compliant order creation

### 📁 Project Structure
```
UniteDefi/
├── fusion-cli/        # CLI implementation
├── fusion-core/       # Core logic & cross-chain abstractions
├── contracts/         
│   ├── ethereum/      # EVM contracts & 1inch integration
│   └── near-htlc/     # NEAR HTLC implementation
└── docs/              # Documentation & guides
```

### 🧪 Testing the Implementation

1. **Unit Tests**: `cargo test --workspace`
2. **Integration Tests**: `cargo test -p fusion-cli`
3. **NEAR Tests**: `cd contracts/near-htlc && cargo test`

### 🎥 Demo Scenarios

See `DEMO_SCENARIOS.md` for detailed walkthroughs of:
- Basic HTLC creation and claiming
- Cross-chain limit orders
- Error handling demonstrations
- Performance benchmarks

### 💡 Innovation Highlights

1. **Rust-First Approach**: High performance, memory safe implementation
2. **Modular Architecture**: Easy to add new chains
3. **Preserves Security**: All 1inch Fusion+ guarantees maintained
4. **Production Ready**: Comprehensive testing and error handling

### 📞 Quick Links

- **Repository**: https://github.com/susumutomita/UniteDefi
- **Documentation**: See README.md and docs/
- **Key Files**:
  - CLI Entry: `fusion-cli/src/main.rs`
  - HTLC Core: `fusion-core/src/htlc.rs`
  - NEAR Contract: `contracts/near-htlc/src/fusion_htlc.rs`

### ⏱️ Performance Metrics

- CLI startup: < 100ms
- Command execution: < 1s
- Memory usage: < 50MB
- Binary size: ~15MB (release build)

Thank you for reviewing UniteSwap! 🚀