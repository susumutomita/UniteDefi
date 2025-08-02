# UniteSwap Demo Script

## Prerequisites
- Base Sepolia ETH in wallet
- USDC tokens on Base Sepolia
- NEAR testnet account (for future demo)

## Demo Flow

### 1. Introduction (30 seconds)
- Show project overview
- Explain 1inch Fusion+ extension concept
- Highlight cross-chain atomic swap capability

### 2. Ethereum HTLC Demo (2 minutes)

#### Create HTLC
```bash
# Create an HTLC on Base Sepolia
./target/release/fusion-cli create-htlc \
  --sender 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --recipient 0x9876543210987654321098765432109876543210 \
  --amount 1000000000000000000 \
  --timeout 3600

# Show the generated secret and HTLC ID
```

#### Show Contract State
- Open Base Sepolia explorer
- Show the Escrow Factory contract: `0x848285f35044e485BD5F0235c27924b1392144b3`
- Demonstrate the HTLC creation transaction

#### Claim HTLC
```bash
# Claim with the secret
./target/release/fusion-cli claim \
  --htlc-id <htlc_id> \
  --secret <secret>
```

### 3. 1inch Limit Order Integration (1.5 minutes)

#### Create Limit Order
```bash
# Create a limit order with HTLC
./target/release/fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash <secret_hash> \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62
```

### 4. Cross-Chain Capability (1 minute)
- Show NEAR contract code
- Explain the HTLC implementation for NEAR
- Mention Cosmos and Stellar implementations
- Note: "NEAR contract deployed but initialization pending due to testnet issues"

### 5. Architecture Benefits (30 seconds)
- Unified Rust implementation
- Modular design for new chains
- Preserves 1inch security guarantees
- CLI-first approach for developers

## Key Points to Emphasize
1. **Security**: All HTLCs maintain hashlock and timelock properties
2. **Extensibility**: Easy to add new non-EVM chains
3. **Integration**: Works with existing 1inch infrastructure
4. **Performance**: Rust implementation for speed and safety

## バックアップ Plan
If live demo fails:
1. Show pre-recorded transaction hashes
2. Use block explorer to demonstrate past transactions
3. Show code architecture and test results