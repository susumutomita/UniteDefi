# UniteSwap Deployment Information

## 🚀 Deployed Contracts

### Base Sepolia (Chain ID: 84532)

#### Limit Order Protocol (1inch Official)
- **Address**: `0x171C87724E720F2806fc29a010a62897B30fdb62`
- **Deployed**: 2025-07-31
- **Transaction**: `0x7f2542bcbba474cd2f32360968be9c59b98dae67873a4a60a1733af355b781cf`
- **Block**: 29102175
- **Explorer**: [View on BaseScan](https://sepolia.basescan.org/address/0x171C87724E720F2806fc29a010a62897B30fdb62)

#### Escrow Factory (UniteSwap)
- **Address**: `0x848285f35044e485BD5F0235c27924b1392144b3`
- **Deployed**: 2025-08-02
- **Transaction**: `0x6d1df7cd80ea24fa09c2fbb6adb90a0797f520b7a4151456883b25cd5f54cc1b`
- **Block**: 29196021
- **Deployer**: `0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950`
- **Explorer**: [View on BaseScan](https://sepolia.basescan.org/address/0x848285f35044e485BD5F0235c27924b1392144b3)

### NEAR Testnet

#### HTLC Contract
- **Contract ID**: `fusion-htlc.testnet` (pending deployment)
- **Status**: To be deployed
- **Explorer**: [View on NEAR Explorer](https://explorer.testnet.near.org/accounts/fusion-htlc.testnet)

## 🔧 Environment Setup

To use UniteSwap with deployed contracts, set the following environment variables:

```bash
# Base Sepolia
export ETHEREUM_RPC_URL=https://base-sepolia.g.alchemy.com/v2/YOUR_KEY
export LIMIT_ORDER_PROTOCOL_ADDRESS=0x171C87724E720F2806fc29a010a62897B30fdb62
export ESCROW_FACTORY_ADDRESS=0x848285f35044e485BD5F0235c27924b1392144b3

# NEAR Testnet
export NEAR_RPC_URL=https://rpc.testnet.near.org
export NEAR_HTLC_CONTRACT=fusion-htlc.testnet
export NEAR_ACCOUNT_ID=your-account.testnet
export NEAR_PRIVATE_KEY=ed25519:YOUR_PRIVATE_KEY
```

## 📝 Quick Test Commands

### Test Limit Order Creation (Base Sepolia)
```bash
./target/release/fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash 0x6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263 \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62
```

### Test NEAR Order Creation
```bash
./target/release/fusion-cli order create-near \
  --near-account alice.testnet \
  --ethereum-address 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --near-amount 1000000000000000000000000 \
  --secret-hash 0x6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263
```

## 🔍 Verification

All deployed contracts can be verified on their respective block explorers:
- Base Sepolia: https://sepolia.basescan.org/
- NEAR Testnet: https://explorer.testnet.near.org/

## 📊 Gas Costs

- Escrow Factory Deployment: 0.002938131052886358 ETH
- Typical Escrow Creation: ~0.001 ETH
- NEAR HTLC Deployment: ~1-2 NEAR