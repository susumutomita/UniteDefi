#!/bin/bash

# Deploy EscrowFactory to Base Sepolia

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Deploying EscrowFactory to Base Sepolia...${NC}"

# Check if .env file exists
if [ ! -f .env ]; then
    echo -e "${RED}Error: .env file not found. Please copy .env.example to .env and configure it.${NC}"
    exit 1
fi

# Source environment variables
source .env

# Check required environment variables
if [ -z "$BASE_SEPOLIA_RPC_URL" ] || [ -z "$PRIVATE_KEY" ]; then
    echo -e "${RED}Error: BASE_SEPOLIA_RPC_URL and PRIVATE_KEY must be set in .env${NC}"
    exit 1
fi

# Run deployment
echo -e "${GREEN}Running deployment script...${NC}"
forge script script/DeployEscrowFactory.s.sol:DeployEscrowFactory \
    --rpc-url $BASE_SEPOLIA_RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    -vvvv

# Check if deployment was successful
if [ $? -eq 0 ]; then
    echo -e "${GREEN}Deployment successful!${NC}"
    
    # If BASESCAN_API_KEY is set, verify the contract
    if [ ! -z "$BASESCAN_API_KEY" ]; then
        echo -e "${YELLOW}Verifying contract on Basescan...${NC}"
        forge script script/DeployEscrowFactory.s.sol:DeployEscrowFactory \
            --rpc-url $BASE_SEPOLIA_RPC_URL \
            --private-key $PRIVATE_KEY \
            --verify \
            --verifier-url https://api-sepolia.basescan.org/api \
            --etherscan-api-key $BASESCAN_API_KEY \
            -vvvv
    else
        echo -e "${YELLOW}Skipping verification (BASESCAN_API_KEY not set)${NC}"
    fi
    
    echo -e "${GREEN}Deployment complete!${NC}"
    echo -e "${YELLOW}Please update the deployed address in:${NC}"
    echo "  - contracts/ethereum/README.md"
    echo "  - Root .env file (BASE_ESCROW_FACTORY_ADDRESS)"
else
    echo -e "${RED}Deployment failed!${NC}"
    exit 1
fi