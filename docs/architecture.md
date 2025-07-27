# UniteDefi Architecture

## Overview

UniteDefi is a cross-chain atomic swap implementation.
That extends 1inch Fusion+ protocol to support non-EVM chains.
This document describes the architecture and implementation status.

## Components

### 1. Ethereum Side (EVM)
- Escrow Factory Contract: Creates individual escrow contracts for each swap
- Escrow Contract: HTLC implementation compatible with 1inch Fusion+ interface
- Ethereum Connector: Rust module to interact with Ethereum contracts

### 2. NEAR Side (Non-EVM)
- NEAR HTLC Contract: Custom escrow implementation in Rust
- NEAR Connector: Rust module to interact with NEAR blockchain

### 3. Cross-chain Orchestrator (CLI)
- Orchestration Logic: Coordinates the atomic swap between chains
- Secret Management: Generates and manages HTLC secrets
- Timeout Handling: Monitors and handles timeout scenarios

## Implementation Status

### ✅ Completed
1. Core HTLC Logic (`fusion-core/src/htlc.rs`)
   - Fixed-size arrays for cryptographic values
   - SystemTime for production time handling
   - Concurrent safety tests

2. 1inch-compatible Escrow Contracts
   - `EscrowFactory.sol`: Factory pattern for creating escrows
   - `Escrow.sol`: HTLC implementation with claim/refund
   - Supports both ETH and ERC20 tokens
   - Ready for Sepolia deployment

3. Basic Chain Connectors
   - Ethereum connector structure (placeholder implementation)
   - NEAR connector structure (placeholder implementation)
   - Configuration management system

### 🚧 In Progress
1. Ethereum Connector Implementation
   - Need to deploy contracts to Sepolia
   - Generate ABI bindings with ethers-rs
   - Implement contract interaction methods

### 📋 TODO
1. NEAR Development
   - Set up NEAR development environment
   - Deploy HTLC contract to NEAR testnet
   - Implement NEAR connector methods

2. Cross-chain Orchestrator
   - CLI command structure
   - Swap initiation flow
   - Secret reveal coordination
   - Status monitoring

3. Integration Testing
   - Sepolia ↔ NEAR testnet demo
   - End-to-end swap testing
   - Timeout scenario testing

## Contract Deployment Process

### Ethereum (Sepolia)
1. Install dependencies: `cd contracts/ethereum && npm install`
2. Configure `.env` with RPC URL and private key
3. Deploy: `npm run deploy:sepolia`
4. Update `ESCROW_FACTORY_SEPOLIA` in Rust code with deployed address

### NEAR (Testnet)
1. Build contract: `cd contracts/near-htlc && cargo build --target wasm32-unknown-unknown --release`
2. Deploy with NEAR CLI
3. Update configuration with contract account ID

## Security Considerations
- Secrets are 32-byte arrays for maximum entropy
- Timeouts use SystemTime for production reliability
- Atomic operations ensure no fund loss
- Refund mechanism for failed swaps
