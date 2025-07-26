# Fusion+ Universal Rust Gateway

[![CI](https://github.com/susumutomita/UniteDefi/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/susumutomita/UniteDefi/actions/workflows/ci.yml)
![GitHub last commit (by committer)](https://img.shields.io/github/last-commit/susumutomita/UniteDefi)
![GitHub top language](https://img.shields.io/github/languages/top/susumutomita/UniteDefi)
![GitHub pull requests](https://img.shields.io/github/issues-pr/susumutomita/UniteDefi)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/susumutomita/UniteDefi)
![GitHub repo size](https://img.shields.io/github/repo-size/susumutomita/UniteDefi)

A high-performance Rust CLI implementation of 1inch Fusion+ protocol for cross-chain swaps between EVM and non-EVM chains.

## 🏆 ETHGlobal Unite - Track 1: Cross-chain Swap Extension

This project extends 1inch Fusion+ to enable trustless atomic swaps between Ethereum and Rust-native non-EVM chains (NEAR, Cosmos, Stellar).

## 🎯 Project Overview

**Fusion+ Universal Rust Gateway** provides a unified Rust-based CLI tool that implements the Hash Time Lock Contract (HTLC) pattern for secure cross-chain token swaps. Our implementation preserves the security guarantees of 1inch Fusion+ while extending support to multiple non-EVM chains through a modular, extensible architecture.

### Key Features
- ✅ Bidirectional swaps (EVM ↔ non-EVM)
- ✅ Preserved hashlock and timelock functionality
- ✅ Multi-chain support (NEAR, Cosmos, Stellar)
- ✅ Safety deposit mechanism
- ✅ CLI interface for easy testing and integration
- ✅ Modular architecture for adding new chains

## 🛠️ Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Ethereum      │     │   Fusion+ Core  │     │   Non-EVM       │
│   (Source)      │◄────┤   Rust CLI      ├────►│   (Target)      │
│                 │     │                 │     │                 │
│ - Escrow        │     │ - HTLC Logic    │     │ - NEAR HTLC     │
│ - 1inch Factory │     │ - Secret Mgmt   │     │ - Cosmos HTLC   │
│ - ERC20 Tokens  │     │ - Monitoring    │     │ - Stellar HTLC  │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+
- Node.js 18+ (for Ethereum interaction)
- Chain-specific CLIs (near-cli, gaiad, stellar-cli)

### Installation
```bash
# Clone the repository
git clone https://github.com/susumutomita/UniteDefi.git
cd UniteDefi

# Install dependencies
cargo build --release

# Install the CLI globally
cargo install --path .
```

### Basic Usage
```bash
# Initialize configuration
fusion-cli init

# Create a swap from Ethereum to NEAR
fusion-cli swap create \
  --from ethereum \
  --to near \
  --amount 100 \
  --token USDC \
  --recipient near-account.near

# Monitor swap progress
fusion-cli swap status --id <swap-id>

# Complete swap (automatic when conditions are met)
fusion-cli swap complete --id <swap-id>
```

## 📋 Hackathon Requirements Checklist

### Core Requirements ✅
- [x] **Hashlock and Timelock Preservation**: All non-EVM implementations maintain HTLC security properties
- [x] **Bidirectional Swaps**: Support for both EVM→non-EVM and non-EVM→EVM swaps
- [x] **On-chain Execution Demo**: Testnet demonstrations available for all supported chains
- [x] **1inch Escrow Integration**: Uses official 1inch escrow factory and contracts

### Stretch Goals 🎯
- [x] **Partial Fill Support**: Multiple secrets for partial order filling
- [x] **Relayer Implementation**: Custom relayer for non-EVM chains
- [ ] **UI Implementation**: CLI-first approach, UI planned post-hackathon
- [ ] **Mainnet Deployment**: Testnet validated, mainnet deployment ready

## 🔧 Technical Implementation

### Core HTLC Trait
```rust
#[async_trait]
pub trait HTLCContract {
    async fn create_lock(
        &self,
        secret_hash: [u8; 32],
        recipient: String,
        amount: u128,
        timeout: u64,
    ) -> Result<String>;

    async fn claim_with_secret(
        &self,
        lock_id: String,
        secret: [u8; 32],
    ) -> Result<TransactionHash>;

    async fn refund_after_timeout(
        &self,
        lock_id: String,
    ) -> Result<TransactionHash>;
}
```

### Supported Chains

#### NEAR Protocol
- Smart contract: `contracts/near/htlc.rs`
- Uses NEAR's native timing and storage
- Gas-efficient implementation

#### Cosmos
- CosmWasm contract: `contracts/cosmos/htlc.rs`
- IBC-ready for future expansion
- Supports multiple Cosmos zones

#### Stellar
- Stellar smart contract using Soroban
- Optimized for Stellar's unique architecture
- Low-cost operations

## 🧪 Testing

### Run Tests
```bash
# Unit tests
cargo test

# Integration tests
cargo test --features integration

# Specific chain tests
cargo test --package near-htlc
cargo test --package cosmos-htlc
cargo test --package stellar-htlc
```

### Testnet Deployments
- **Ethereum**: Sepolia testnet
- **NEAR**: Testnet (testnet.near.org)
- **Cosmos**: Cosmos testnet
- **Stellar**: Stellar testnet

## 📊 Performance Metrics

| Metric | Ethereum | NEAR | Cosmos | Stellar |
|--------|----------|------|--------|---------|
| Avg Swap Time | 15s | 2s | 6s | 5s |
| Gas Cost | $5-20 | <$0.01 | <$0.01 | <$0.01 |
| Finality | 12 blocks | 2 blocks | 1 block | 1 ledger |

## 🏗️ Project Structure
```
fusion-plus-rust-gateway/
├── src/
│   ├── core/           # Core HTLC logic
│   ├── chains/         # Chain-specific implementations
│   │   ├── ethereum/
│   │   ├── near/
│   │   ├── cosmos/
│   │   └── stellar/
│   ├── cli/            # CLI interface
│   └── relayer/        # Relayer service
├── contracts/          # Smart contracts
│   ├── ethereum/       # Solidity contracts
│   ├── near/          # NEAR contracts
│   ├── cosmos/        # CosmWasm contracts
│   └── stellar/       # Stellar contracts
├── tests/             # Test suites
└── docs/              # Documentation
```

## 🔐 Security Considerations

1. **Secret Generation**: Uses cryptographically secure random number generation
2. **Timeout Handling**: Automatic refunds after timeout expiration
3. **Safety Deposits**: Prevents griefing attacks through economic incentives
4. **Signature Verification**: All operations require proper authorization

## 🤝 Team

- **Lead Developer**: [Susumu Tomita](https://susumutomita.netlify.app/)
- **Blockchain Engineer**: [Team Member]
- **Security Auditor**: [Team Member]

## 📜 License

MIT License - see LICENSE file for details

## 🔗 Resources

- [1inch Fusion+ Documentation](https://docs.1inch.io/)
- [Demo Video](https://youtube.com/your-demo)
- [Technical Deep Dive](./docs/Fusion-Plus-Technical-Guide.md)
- [Winning Ideas](./docs/優勝アイデア.md)
- [Workshop Notes](https://www.youtube.com/watch?v=W2xCf-TCnwc)
- [DeepWiki](https://deepwiki.com/susumutomita/UniteDefi)

## 🚧 Future Roadmap

1. **Phase 1**: Additional chain support (Aptos, Sui)
2. **Phase 2**: Web interface and SDK
3. **Phase 3**: Integration with 1inch production infrastructure
4. **Phase 4**: Decentralized relayer network
