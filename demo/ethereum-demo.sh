#!/bin/bash

# UniteSwap Ethereum Demo Script

echo "🚀 UniteSwap Demo - Cross-Chain Atomic Swaps"
echo "=========================================="
echo ""

# Configuration
ESCROW_FACTORY="0x848285f35044e485BD5F0235c27924b1392144b3"
FUSION_CLI="./target/release/fusion-cli"

# Step 1: Show deployed contracts
echo "1️⃣ Deployed Contracts on Base Sepolia:"
echo "   - Escrow Factory: $ESCROW_FACTORY"
echo "   - Explorer: https://sepolia.basescan.org/address/$ESCROW_FACTORY"
echo ""

# Step 2: Create HTLC
echo "2️⃣ Creating HTLC with auto-generated secret..."
$FUSION_CLI create-htlc \
  --sender 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --recipient 0x9876543210987654321098765432109876543210 \
  --amount 1000000000000000000 \
  --timeout 3600

echo ""
echo "⏸️  Please note the HTLC ID and secret from above output"
echo ""

# Step 3: Show cross-chain capability
echo "3️⃣ Cross-Chain Architecture:"
echo "   - Ethereum: Escrow with 1inch Fusion+ integration ✅"
echo "   - NEAR: HTLC contract (deployed at htlc.uniteswap.testnet) 🔧"
echo "   - Status: NEAR initialization pending due to testnet issues"
echo ""

# Step 4: Claim HTLC
read -p "Enter HTLC ID to claim: " HTLC_ID
read -p "Enter secret: " SECRET

echo ""
echo "4️⃣ Claiming HTLC..."
$FUSION_CLI claim \
  --htlc-id "$HTLC_ID" \
  --secret "$SECRET"

echo ""
echo "5️⃣ Demo Complete!"
echo ""
echo "Key Features Demonstrated:"
echo "✅ HTLC creation with auto-generated secrets"
echo "✅ Hash-time lock security model"
echo "✅ 1inch Fusion+ protocol integration"
echo "🔧 NEAR integration (code complete, deployment pending)"
echo ""
echo "View full codebase: https://github.com/susumutomita/UniteDefi"