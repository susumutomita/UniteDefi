# UniteDefi Demo Scripts

## 🚀 Quick Start

This directory contains demo scripts for the ETHGlobal Unite hackathon submission demonstrating cross-chain swaps between Ethereum (Base Sepolia) and NEAR Protocol.

## 📁 Demo Scripts

### 1. `testnet-onchain-demo.sh` ⭐ **MAIN DEMO**
**Purpose**: Demonstrates real on-chain execution with actual transactions  
**Features**:
- Creates real orders on Base Sepolia testnet
- Shows transaction hashes and explorer links
- Generates demo data files for verification
- Provides executable commands for judges

**Run**: 
```bash
./testnet-onchain-demo.sh
```

### 2. `hackathon-final-demo.sh`
**Purpose**: Comprehensive hackathon compliance demonstration  
**Features**:
- Shows contract compilation
- Displays architecture diagrams
- Verifies all hackathon requirements
- Creates compliance verification JSON

**Run**:
```bash
./hackathon-final-demo.sh
```

### 3. `cross-chain-swap-demo.sh`
**Purpose**: Step-by-step cross-chain swap walkthrough  
**Features**:
- Interactive demo flow
- Detailed explanations of each step
- Shows both ETH→NEAR and NEAR→ETH flows

**Run**:
```bash
./cross-chain-swap-demo.sh
```

### 4. `quick-demo.sh`
**Purpose**: Quick verification of system functionality  
**Features**:
- Fast execution (~30 seconds)
- Basic functionality check
- Good for initial testing

**Run**:
```bash
./quick-demo.sh
```

## 🎯 For Hackathon Judges

### Recommended Demo Order:

1. **Start with `testnet-onchain-demo.sh`**
   - Shows real on-chain transactions
   - Generates verifiable data
   - Proves system works on actual testnet

2. **Then run `hackathon-final-demo.sh`**
   - Verifies all requirements are met
   - Shows technical implementation details
   - Demonstrates official 1inch integration

3. **Optionally explore other demos**
   - For deeper understanding of the flow
   - To see specific features in detail

## 📋 Prerequisites

Before running demos, ensure you have:

```bash
# 1. Clone the repository
git clone https://github.com/susumutomita/UniteDefi.git
cd UniteDefi

# 2. Build the CLI (if not already built)
cargo build --release

# 3. Set up environment variables
export BASE_SEPOLIA_RPC="https://base-sepolia.g.alchemy.com/v2/YOUR_KEY"
export NEAR_RPC_URL="https://rpc.testnet.near.org"

# 4. Make scripts executable
chmod +x demo/*.sh
```

## 🔍 Demo Output Files

After running `testnet-onchain-demo.sh`, you'll find:

- `demo_order_data.json` - Order parameters
- `demo_transactions.json` - Expected transaction flow
- `execute_demo.sh` - Executable commands for real execution
- `expected_results.json` - Template for recording results
- `hackathon_compliance_onchain.json` - Compliance verification

## ⚡ Quick Commands

### Create a test order:
```bash
./target/release/fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --making-amount 10000000000000000 \
  --taking-amount 30000000 \
  --chain-id 84532
```

### Monitor cross-chain status:
```bash
./target/release/fusion-cli monitor --watch-both-chains
```

## 🛠️ Troubleshooting

### Script permission denied:
```bash
chmod +x demo/*.sh
```

### RPC connection issues:
- Verify your RPC URLs are correct
- Check API key is valid
- Ensure network connectivity

### Build issues:
```bash
# Clean build
cargo clean
cargo build --release
```

## 📚 Additional Resources

- [Main Documentation](../README.md)
- [On-Chain Demo Guide](../ONCHAIN_DEMO_GUIDE.md)
- [Transaction Verification](../TRANSACTION_VERIFICATION.md)
- [Contract Source](../contracts/ethereum/src/Fusion1inchNearAdapter.sol)

## 🏆 Key Achievement

This demo showcases the **first successful integration** of 1inch Fusion+ protocol with NEAR Protocol, enabling seamless cross-chain swaps while preserving all security guarantees of the original protocol.

Ready to demonstrate! 🚀