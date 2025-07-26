# 1inch Fusion+ Technical Guide

## 📺 Workshop Summary

Source: [1inch Cross-chain Swap Workshop by Tanner Moore](https://www.youtube.com/watch?v=W2xCf-TCnwc)

## 🔧 1inch Product Suite Overview

### 1. Classic Swap

- Purpose: AMM/PMM aggregation
- Features:
  - Aggregates liquidity from multiple protocols (Uniswap, Curve, SushiSwap, etc.)
  - Optimizes swap output and gas fees
  - Single transaction execution
  - Example: USDC→DAI finds optimal route across multiple DEXs

### 2. 1inch Fusion (インテント-based Swap)

- Problems Solved: MEV attacks, gas fees, failed transactions
- How it Works:
  1. User signs swap request
  2. Dutch auction starts (price starts above market, gradually decreases)
  3. Resolvers monitor and compete for best execution
  4. Winner executes swap when price is optimal
- Benefits:
  - MEV protection
  - Gasless transactions for users
  - Better rates through resolver competition

### 3. 1inch Fusion+ (Cross-chain Swap)

- Built on: Fusion technology
- Key Features:
  - Cross-blockchain swaps
  - Trustless execution
  - Simple UX
  - Good swap rates

## 🔐 Hash Time Lock Contract (HTLC) Architecture

### Core Concept
HTLC is a smart contract that:
- Holds funds in escrow
- Releases funds only when correct "secret" is provided
- Returns funds to sender if timeout expires

### Fusion+ Implementation Flow

```
1. User Request: Ethereum USDC → Base DAI
   ↓
2. 1inch API starts Dutch auction
   ↓
3. Resolver wins auction and creates escrow contracts
   ↓
4. Safety Deposits:
   - Resolver deposits on BOTH chains (Ethereum & Base)
   - Ensures transaction completion incentive
   ↓
5. Fund Movement:
   - User funds → Source chain escrow (Ethereum)
   - Resolver funds → Destination chain escrow (Base)
   ↓
6. Relayer Service:
   - Monitors both escrow contracts
   - Verifies fund deposits
   - Facilitates secret sharing
   ↓
7. Secret Sharing:
   - User shares secret with relayer
   - Relayer shares with resolver
   - Secret becomes public for all resolvers
   ↓
8. Completion:
   - Original resolver completes swap OR
   - Any other resolver can complete (earning safety deposit)
```

### Safety Mechanism
- If original resolver fails, other resolvers can complete the swap
- Safety deposits incentivize completion
- No single point of failure

## 🎯 Hackathon Requirements

### Primary Challenge
Manage HTLC and communication between EVM and non-EVM chains

### Core Requirements
1. Preserve hashlock and timelock functionality in non-EVM implementation
2. Bidirectional swaps: EVM↔non-EVM
3. On-chain token transfer demo (mainnet or testnet)
4. Use 1inch escrow factory and official contracts

### Stretch Goals (Optional but Recommended)
1. UI Creation: User interface for swap interaction
2. Partial Fills: Enable multiple secrets for partial order filling
3. Relayer/Resolver Implementation: For non-EVM chains matching 1inch design
4. Mainnet Deployment: Production-ready implementation

## ⚠️ Important Hackathon Notes

### DO NOT:
- Post orders to official REST API (requires KYC/whitelist)
- Rely on 1inch's production resolvers

### DO:
- Work at smart contract level
- Complete transactions yourself
- Use provided example repositories
- Test thoroughly on testnet first

## 🛠️ Technical Resources

### Example Repository
- Name: `crosschain-resolver-example`
- Language: TypeScript
- Purpose: Simulates Ethereum↔BNB Chain Fusion+ swap
- Contents:
  - Smart contract interaction examples
  - Safety deposit implementation
  - Escrow contract usage

### Hackathon Guide
- URL: hackathon.1inch.community (Notion)
- Contents:
  - Cheat sheets
  - Documentation links
  - API key acquisition
  - Discord support

### API Access
- Promo Code Available: For hackathon participants
- Benefits:
  - No KYC required during hackathon
  - 60 RPM (requests per minute)
  - 500,000 Web3 RPC calls
  - Free Dev portal access

## 🔑 Key Technical Insights

### Dutch Auction Configuration
- Fully controllable at creation time
- Options:
  - Static price
  - Time-based price decay
  - Custom price curves

### Relayer Service
- Currently managed by 1inch
- Future plans for decentralization
- Critical for cross-chain coordination

### Current Limitations
- API supports mainnet only (no testnet)
- Resolver whitelist required for production
- KYC needed for official API usage

## 💡 Implementation Strategy for Rust CLI

### Recommended Approach
1. Study TypeScript Example: Understand EVM-side implementation
2. Port to Rust: Create Rust equivalent of core logic
3. Add Non-EVM Support: Implement HTLC for target chains
4. Build CLI Interface: Command-line tools for testing
5. Create Demo Flow: End-to-end swap demonstration

### Key Components to Implement
```rust
// Core HTLC trait for all chains
trait HTLCContract {
    fn create_lock(&self, secret_hash: Hash, timeout: u64) -> Result<()>;
    fn claim_with_secret(&self, secret: Secret) -> Result<()>;
    fn refund_after_timeout(&self) -> Result<()>;
}

// Chain-specific implementations
impl HTLCContract for EthereumHTLC { ... }
impl HTLCContract for NearHTLC { ... }
impl HTLCContract for CosmosHTLC { ... }
```

### Testing Strategy
1. Local blockchain testing (Ganache, Near sandbox, etc.)
2. Testnet deployment and verification
3. Cross-chain swap simulation
4. Error case handling (timeouts, failed claims)

## 🎪 Demo Preparation

### Required Demo Elements
1. Live Transaction: Show actual cross-chain swap
2. Secret Management: Demonstrate HTLC mechanics
3. Bidirectional: Both EVM→non-EVM and non-EVM→EVM
4. Error Recovery: Show timeout/refund mechanism

### Recommended Demo Flow
```bash
# 1. Setup accounts
fusion-cli setup --source ethereum --destination near

# 2. Check balances
fusion-cli balance --chain ethereum
fusion-cli balance --chain near

# 3. Initiate swap
fusion-cli swap --from ethereum --to near --amount 100 --token USDC

# 4. Monitor progress
fusion-cli status --swap-id 0x123...

# 5. Complete swap (with secret reveal)
fusion-cli complete --swap-id 0x123... --secret 0xabc...

# 6. Verify completion
fusion-cli balance --chain near
```

## 📚 Additional Resources
- [1inch Developer Portal](https://docs.1inch.io/)
- [1inch GitHub](https://github.com/1inch)
- [Fusion+ Whitepaper](TBD)
- [Discord Support](via hackathon.1inch.community)
