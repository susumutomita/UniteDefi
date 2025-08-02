# UniteDefi Cross-Chain Atomic Swap Demo Guide

## Overview

UniteDefi enables trustless atomic swaps between Ethereum and NEAR.
It uses HTLCs integrated with 1inch Fusion+ protocol.

## Architecture

```
┌─────────────────┐                    ┌─────────────────┐
│   Ethereum      │                    │      NEAR       │
│ (Base Sepolia)  │                    │   (Testnet)     │
├─────────────────┤                    ├─────────────────┤
│ Escrow Factory  │◄──── Hash ────────►│  HTLC Contract  │
│   + 1inch       │      Locked        │                 │
│   Fusion+       │      Swaps         │                 │
└─────────────────┘                    └─────────────────┘
```

## Deployed Contracts

- Ethereum (Base Sepolia):
  - Escrow Factory: `0x848285f35044e485BD5F0235c27924b1392144b3`
  - 1inch Limit Order Protocol: Integrated

- NEAR (Testnet):
  - HTLC Contract: `htlc-v2.testnet`
  - Owner: `uniteswap.testnet`

## Demo Scenarios

### 1. Quick NEAR HTLC Demo

Tests the NEAR HTLC functionality:

```bash
./demo/quick-demo.sh
```

This demonstrates:
- Creating an HTLC escrow
- Viewing escrow details
- Claiming with the correct secret

### 2. Full Cross-Chain Swap Demo

Simulates a complete cross-chain atomic swap:

```bash
./demo/cross-chain-swap-demo.sh
```

This demonstrates:
- Bob locks NEAR in HTLC
- Alice locks ETH in Escrow (simulated)
- Bob claims ETH revealing the secret
- Alice claims NEAR using the revealed secret

### 3. Ethereum Demo

Tests the Ethereum side functionality:

```bash
./demo/ethereum-demo.sh
```

## How Atomic Swaps Work

1. **Initiation**: 
   - Bob creates HTLC on NEAR with secret hash
   - Alice sees this and creates matching HTLC on Ethereum

2. **Execution**:
   - Bob claims Ethereum funds by revealing the secret
   - Alice uses the revealed secret to claim NEAR funds

3. **Security**:
   - If Bob doesn't reveal the secret, both parties can refund after timeout
   - Atomic property ensures both swaps complete or neither does

## Key Features

- Trustless - No intermediary required
- Atomic - All-or-nothing execution
- Cross-chain - Ethereum <-> NEAR bridging
- 1inch Integration - Advanced order matching on Ethereum
- Time-locked - Automatic refunds on timeout

## Testing the System

### Prerequisites

1. NEAR CLI installed and configured
2. Test accounts with funds:
   - NEAR: `uniteswap.testnet`
   - Ethereum: Wallet with Base Sepolia ETH

### Create a Test Swap

```bash
# 1. Generate a secret and hash
SECRET="my-test-secret-$(date +%s)"
HASH=$(cd contracts/near-htlc/hash_test && cargo run --quiet $SECRET)

# 2. Create NEAR escrow
ESCROW_ID=$(near call htlc-v2.testnet create_escrow \
  '{"recipient": "alice.testnet", "secret_hash": "'$HASH'", "timeout_seconds": 3600}' \
  --accountId uniteswap.testnet --deposit 0.1 | head -1 | tr -d '"')

# 3. Claim with secret
near call htlc-v2.testnet claim \
  '{"escrow_id": "'$ESCROW_ID'", "secret": "'$SECRET'"}' \
  --accountId uniteswap.testnet
```

## Production Deployment

For mainnet deployment:

1. Deploy contracts to respective mainnets
2. Configure resolver nodes for cross-chain monitoring
3. Set up limit order interfaces
4. Implement proper key management

## Video Demo Script

For hackathon submission:

1. Introduction (30s)
   - Problem: No trustless swaps between Ethereum and NEAR
   - Solution: UniteDefi with 1inch Fusion+ integration

2. Architecture (45s)
   - Show architecture diagram
   - Explain HTLC mechanism
   - Highlight 1inch integration

3. Live Demo (2min)
   - Run quick-demo.sh
   - Show transaction on explorers
   - Demonstrate atomic property

4. Use Cases (45s)
   - Cross-chain DeFi
   - Liquidity bridging
   - Decentralized trading

## Troubleshooting

- "Escrow not found" - Check escrow ID and that it hasn't been claimed
- "Invalid secret" - Ensure correct secret that generates the stored hash
- "Access key not found" - Run `near login` for the account
- Timeout errors - Increase gas or check network connectivity

## Next Steps

1. Implement Ethereum claim monitoring
2. Add automated resolver nodes
3. Create user-friendly UI
4. Expand to more chains