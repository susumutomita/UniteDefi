#!/bin/bash
# Test script to verify real transaction submission

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Testing Real Transaction Submission ===${NC}"
echo

# Load environment
echo -e "${YELLOW}Loading environment variables...${NC}"
source load-env.sh

# Check if private key is set
if [ -z "$PRIVATE_KEY" ]; then
    echo -e "${RED}ERROR: PRIVATE_KEY environment variable not set${NC}"
    echo "Please set your private key to submit transactions"
    exit 1
fi

# Test 1: Create a simple order with sign and submit
echo -e "${YELLOW}Test 1: Creating and submitting a real order to Base Sepolia...${NC}"
./target/release/fusion-cli order create \
    --maker-asset 0x4200000000000000000000000000000000000006 \
    --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
    --maker "$ETHEREUM_ADDRESS" \
    --making-amount 1000000000000000 \
    --taking-amount 1000000 \
    --htlc-secret-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
    --htlc-timeout 3600 \
    --chain-id 84532 \
    --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62 \
    --sign \
    --submit

echo
echo -e "${GREEN}If you see a transaction hash and explorer URL above, the transaction was successfully submitted!${NC}"
echo

# Test 2: Test swap command
echo -e "${YELLOW}Test 2: Testing swap command with real transaction...${NC}"
echo -e "${YELLOW}This will create real transactions on both Ethereum and NEAR${NC}"
echo -e "Press Ctrl+C to cancel, or Enter to continue..."
read -r

./target/release/fusion-cli swap \
    --from-chain ethereum \
    --to-chain near \
    --from-token WETH \
    --to-token NEAR \
    --amount 0.0001 \
    --from-address "$ETHEREUM_ADDRESS" \
    --to-address "$NEAR_ACCOUNT_ID" \
    --slippage 1.0 \
    --timeout 3600

echo
echo -e "${GREEN}Test completed! Check the output above for transaction hashes and explorer URLs.${NC}"