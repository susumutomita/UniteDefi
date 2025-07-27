# 1inch Reference Implementations

This directory contains Git submodules of 1inch repositories.

## Included Repositories

### 1. fusion-sdk
- Purpose: Official SDK for 1inch Fusion protocol
- Key Features: Order creation, signature handling, Dutch auction implementation
- Language: TypeScript

### 2. cross-chain-swap
- Purpose: Core cross-chain swap protocol implementation
- Key Components: EscrowFactory, EscrowSrc, EscrowDst contracts
- Language: Solidity

### 3. cross-chain-resolver-example
- Purpose: Example resolver implementation for cross-chain swaps
- Key Files: `Resolver.sol` - Shows how resolvers interact with escrow contracts
- Language: Solidity

### 4. fusion-resolver-example
- Purpose: Example resolver for standard Fusion (not cross-chain)
- Key Features: Shows resolver integration with 1inch APIs
- Language: TypeScript/Solidity

## Usage

To update all submodules to latest:
```bash
git submodule update --remote --merge
```

To explore a specific repository:
```bash
cd fusion-sdk
npm install  # or yarn install
```

## Key Learnings

1. Fusion Protocol Flow:
   - インテント-based orders
   - Dutch auction for price discovery
   - Professional resolver network

2. Cross-chain Architecture:
   - Source and destination escrows
   - Hash time-locked contracts
   - Multi-phase execution

3. Resolver Implementation:
   - Safety deposits
   - Competitive bidding
   - MEV protection
