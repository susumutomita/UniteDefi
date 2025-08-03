#!/bin/bash

# UniteSwap Standalone Demo Script
# This script demonstrates the CLI functionality without requiring full environment setup

echo "========================================="
echo "    UniteSwap CLI Demonstration"
echo "========================================="
echo ""

# Check if fusion-cli binary exists
if [ ! -f "./fusion-cli" ] && [ ! -f "./target/release/fusion-cli" ]; then
    echo "Error: fusion-cli binary not found!"
    echo "Please download the binary for your platform from GitHub releases"
    echo "or build it with: cargo build -p fusion-cli --release"
    exit 1
fi

# Set the CLI path
if [ -f "./fusion-cli" ]; then
    CLI="./fusion-cli"
else
    CLI="./target/release/fusion-cli"
fi

# Display help
echo "1. Displaying CLI help:"
echo "----------------------"
$CLI --help
echo ""

# Show HTLC commands
echo "2. HTLC Commands:"
echo "-----------------"
$CLI htlc --help
echo ""

# Show order commands
echo "3. Order Commands:"
echo "------------------"
$CLI order --help
echo ""

# Show swap commands
echo "4. Swap Commands (NEW!):"
echo "------------------------"
$CLI swap --help
echo ""

# Example command formats
echo "5. Example Commands:"
echo "--------------------"
echo ""
echo "Create HTLC:"
echo '$CLI htlc create --sender 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 --recipient 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 --amount 1000000000000000000'
echo ""
echo "Create Limit Order:"
echo '$CLI order create --maker-asset 0x4200000000000000000000000000000000000006 --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 --making-amount 1000000000000000000 --taking-amount 3000000000'
echo ""
echo "Integrated Swap (Ethereum to NEAR):"
echo '$CLI swap --from-chain ethereum --to-chain near --from-token WETH --to-token NEAR --amount 0.1 --from-address 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 --to-address alice.near'
echo ""

# Test HTLC creation (dry run)
echo "6. Test HTLC Creation (simulation):"
echo "-----------------------------------"
echo "Note: This will generate a secret but won't submit to blockchain without PRIVATE_KEY"
$CLI htlc create \
  --sender 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --recipient 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --amount 1000000000000000000 \
  --timeout 3600 || echo "Note: This is expected to show the HTLC data structure"

echo ""
echo "========================================="
echo "Demo complete!"
echo ""
echo "To use with real blockchain transactions:"
echo "1. Set up your .env file with ETHEREUM_RPC_URL and PRIVATE_KEY"
echo "2. Install NEAR CLI for NEAR operations"
echo "3. Run commands with --sign and --submit flags"
echo ""
echo "For more information, see README.md"
echo "========================================="