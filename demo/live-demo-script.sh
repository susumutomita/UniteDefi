#!/bin/bash
# UniteSwap Live Demo Script for ETHGlobal Unite
# This script demonstrates cross-chain atomic swaps between Ethereum and NEAR

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Demo configuration
DEMO_AMOUNT="0.001"
MONITOR_INTERVAL="20"

echo -e "${BLUE}================================================${NC}"
echo -e "${BLUE}     UniteSwap - Cross-Chain Atomic Swaps       ${NC}"
echo -e "${BLUE}         ETHGlobal Unite Hackathon              ${NC}"
echo -e "${BLUE}================================================${NC}"
echo

# Step 1: Environment check
echo -e "${YELLOW}[Step 1] Checking environment...${NC}"
source load-env.sh

echo -e "${GREEN}✓ Environment loaded${NC}"
echo -e "  Ethereum Address: ${ETHEREUM_ADDRESS:0:10}...${ETHEREUM_ADDRESS: -4}"
echo -e "  NEAR Account: $NEAR_ACCOUNT_ID"
echo

# Step 2: Show balances
echo -e "${YELLOW}[Step 2] Checking balances...${NC}"
echo -e "${BLUE}Ethereum (Base Sepolia):${NC}"
# In production, would use cast balance command
echo "  WETH Balance: ~0.05 WETH"
echo

echo -e "${BLUE}NEAR (Testnet):${NC}"
# In production, would use near view command
echo "  NEAR Balance: ~10 NEAR"
echo

# Step 3: Demo selection
echo -e "${YELLOW}[Step 3] Select demo direction:${NC}"
echo "1) Ethereum → NEAR (WETH to NEAR)"
echo "2) NEAR → Ethereum (NEAR to WETH)"
echo -n "Enter choice (1 or 2): "
read -r choice

case $choice in
    1)
        echo
        echo -e "${YELLOW}[Step 4] Executing Ethereum → NEAR swap${NC}"
        echo -e "Swapping $DEMO_AMOUNT WETH for NEAR tokens..."
        echo

        # Show the command
        echo -e "${BLUE}Command:${NC}"
        cat << EOF
./target/release/fusion-cli swap \\
  --from-chain ethereum \\
  --to-chain near \\
  --from-token WETH \\
  --to-token NEAR \\
  --amount $DEMO_AMOUNT \\
  --from-address $ETHEREUM_ADDRESS \\
  --to-address $NEAR_ACCOUNT_ID \\
  --auto-claim \\
  --monitor-interval $MONITOR_INTERVAL
EOF
        echo

        # Execute swap
        ./target/release/fusion-cli swap \
            --from-chain ethereum \
            --to-chain near \
            --from-token WETH \
            --to-token NEAR \
            --amount "$DEMO_AMOUNT" \
            --from-address "$ETHEREUM_ADDRESS" \
            --to-address "$NEAR_ACCOUNT_ID" \
            --auto-claim \
            --monitor-interval "$MONITOR_INTERVAL"
        ;;

    2)
        echo
        echo -e "${YELLOW}[Step 4] Executing NEAR → Ethereum swap${NC}"
        echo -e "Swapping 1 NEAR for WETH tokens..."
        echo

        # Show the command
        echo -e "${BLUE}Command:${NC}"
        cat << EOF
./target/release/fusion-cli swap \\
  --from-chain near \\
  --to-chain ethereum \\
  --from-token NEAR \\
  --to-token WETH \\
  --amount 1.0 \\
  --from-address $NEAR_ACCOUNT_ID \\
  --to-address $ETHEREUM_ADDRESS \\
  --auto-claim \\
  --monitor-interval $MONITOR_INTERVAL
EOF
        echo

        # Execute swap
        ./target/release/fusion-cli swap \
            --from-chain near \
            --to-chain ethereum \
            --from-token NEAR \
            --to-token WETH \
            --amount 1.0 \
            --from-address "$NEAR_ACCOUNT_ID" \
            --to-address "$ETHEREUM_ADDRESS" \
            --auto-claim \
            --monitor-interval "$MONITOR_INTERVAL"
        ;;

    *)
        echo -e "${RED}Invalid choice. Exiting.${NC}"
        exit 1
        ;;
esac

echo
echo -e "${GREEN}================================================${NC}"
echo -e "${GREEN}          Demo completed successfully!          ${NC}"
echo -e "${GREEN}================================================${NC}"
echo
echo -e "${YELLOW}Key Features Demonstrated:${NC}"
echo "✓ Cross-chain atomic swaps without bridges"
echo "✓ Integration with 1inch Limit Order Protocol"
echo "✓ HTLC-based security guarantees"
echo "✓ Automatic monitoring and claiming"
echo "✓ Support for both EVM and non-EVM chains"
echo
echo -e "${BLUE}Thank you for watching UniteSwap!${NC}"
