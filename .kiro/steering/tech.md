# Technology Stack

## Architecture Overview
- **Type**: Modular CLI application with chain-specific implementations
- **Pattern**: Trait-based architecture with async/await support
- **Core Design**: HTLC (Hash Time Lock Contract) pattern for atomic cross-chain swaps
- **Integration**: 1inch Fusion+ protocol extension for non-EVM chains

## Core Technologies

### Backend (Rust)
- **Language**: Rust 1.75+
- **Async Runtime**: Tokio for async operations
- **CLI Framework**: Clap for command-line parsing
- **Error Handling**: Result<T, E> pattern with custom error types
- **Serialization**: Serde for JSON/binary serialization
- **Cryptography**: ring/sha2 for secure hash generation

### Blockchain Integrations
- **Ethereum**: ethers-rs for Web3 interactions
- **NEAR**: near-sdk-rs for smart contract development
- **Cosmos**: cosmwasm-std for CosmWasm contracts
- **Stellar**: soroban-sdk for Stellar smart contracts

### Testing
- **Unit Tests**: Built-in Rust testing framework
- **Integration Tests**: Custom test harness with feature flags
- **Test Coverage**: cargo-tarpaulin for coverage reports

### Development Tools
- **Build System**: Cargo (Rust package manager)
- **Linting**: 
  - Rust: clippy for Rust code
  - Markdown: textlint for documentation
- **Formatting**: rustfmt for consistent code style
- **Git Hooks**: Husky for pre-commit checks
- **CI/CD**: GitHub Actions for automated testing and deployment

## Development Environment

### Required Tools
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Node.js (for Ethereum interactions and textlint)
# Install via nvm or package manager
node --version  # Should be 18+

# Chain-specific CLIs
cargo install near-cli-rs  # NEAR CLI
# Install gaiad for Cosmos
# Install stellar for Stellar
```

### Environment Variables
```bash
# Chain RPC endpoints
ETHEREUM_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
NEAR_RPC_URL=https://rpc.testnet.near.org
COSMOS_RPC_URL=https://rpc.testnet.cosmos.network
STELLAR_RPC_URL=https://horizon-testnet.stellar.org

# Account keys (for testing)
ETHEREUM_PRIVATE_KEY=0x...
NEAR_ACCOUNT_ID=your-account.testnet
COSMOS_MNEMONIC="your twelve word mnemonic..."
STELLAR_SECRET_KEY=S...
```

## Common Commands

### Development
```bash
# Build the project
cargo build
cargo build --release  # Optimized build

# Run tests
cargo test
cargo test --features integration  # Include integration tests
cargo test --package fusion-core  # Test specific package

# Format and lint
cargo fmt
cargo clippy -- -D warnings
pnpm lint  # Lint markdown files
pnpm lint:fix  # Auto-fix markdown issues

# Run the CLI
cargo run -- --help
cargo run -- swap create --from ethereum --to near --amount 100
```

### Testing Specific Chains
```bash
# NEAR tests
cargo test --package near-htlc

# Cosmos tests
cargo test --package cosmos-htlc

# Stellar tests
cargo test --package stellar-htlc
```

### Documentation
```bash
# Generate Rust docs
cargo doc --open

# Build and serve docs locally
cargo doc --no-deps
```

## Port Configuration
- **Local RPC Proxy**: 8545 (optional, for caching RPC calls)
- **Test Network Ports**: Uses default ports for each chain's testnet

## Performance Considerations
- **Async Operations**: All blockchain interactions are async
- **Connection Pooling**: Reuse RPC connections where possible
- **Caching**: Consider implementing RPC response caching for development
- **Parallel Execution**: Use rayon for CPU-intensive operations

## Security Standards
- **No Private Keys in Code**: Use environment variables
- **Secure Random Generation**: Use cryptographically secure RNG
- **Input Validation**: Validate all user inputs and chain responses
- **Error Handling**: Never expose sensitive information in errors