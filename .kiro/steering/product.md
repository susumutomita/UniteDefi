# Product Overview

## Product Description
Fusion+ Universal Rust Gateway is a high-performance Rust CLI implementation that extends the 1inch Fusion+ protocol to enable trustless atomic swaps between Ethereum (EVM) and Rust-native non-EVM chains. Built for ETHGlobal Unite hackathon, it provides a unified gateway for cross-chain token swaps while preserving all security guarantees of the original Fusion+ protocol.

## Core Features
- **Cross-chain Atomic Swaps**: Secure token swaps between EVM and non-EVM chains using HTLC pattern
- **Multi-chain Support**: Native integration with NEAR, Cosmos, and Stellar blockchains
- **Bidirectional Swaps**: Support for both EVM→non-EVM and non-EVM→EVM transfers
- **CLI-First Interface**: Powerful command-line tool for developers and integrators
- **Modular Architecture**: Extensible design for easy addition of new blockchain networks
- **Safety Mechanisms**: Built-in deposit system to prevent griefing attacks
- **Partial Fill Support**: Multiple secrets for gradual order execution
- **Custom Relayer**: Specialized relayer implementation for non-EVM chains

## Target Use Cases
- **DeFi Traders**: Execute cross-chain swaps without centralized exchanges
- **Protocol Integrators**: Build cross-chain DeFi applications using the CLI as a backend
- **Market Makers**: Provide liquidity across different blockchain ecosystems
- **Developers**: Test and validate cross-chain swap mechanisms on testnets
- **Bridge Operators**: Use as a foundation for building production bridges

## Key Value Proposition
- **Security First**: Maintains all HTLC security properties (hashlock and timelock) across all chains
- **Cost Efficiency**: Dramatically lower gas costs on non-EVM chains (<$0.01 vs $5-20 on Ethereum)
- **Speed**: Faster finality on non-EVM chains (2-6 seconds vs 15 seconds on Ethereum)
- **Developer Experience**: Simple CLI interface with clear commands and monitoring capabilities
- **1inch Integration**: Leverages official 1inch escrow factory and contracts for EVM side
- **Open Source**: MIT licensed, enabling community contributions and audits
- **Hackathon Winner Potential**: Addresses Track 1 requirements for cross-chain swap extension