#!/bin/bash

# NEAR Fusion HTLC Deployment Script for Testnet

set -e

echo "🚀 NEAR Fusion HTLC Deployment Script"
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CONTRACT_NAME="fusion-htlc"
TESTNET_SUFFIX=".testnet"
WASM_FILE="./target/near/fusion_htlc.wasm"

# Check if near-cli is installed
if ! command -v near &> /dev/null; then
    echo -e "${RED}Error: near-cli is not installed${NC}"
    echo "Install it with: npm install -g near-cli"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Install it from: https://rustup.rs/"
    exit 1
fi

# Function to create account
create_account() {
    local account_id="$1"
    echo -e "${YELLOW}Creating account: $account_id${NC}"

    near create-account "$account_id" --useFaucet

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Account created successfully${NC}"
    else
        echo -e "${RED}✗ Failed to create account${NC}"
        exit 1
    fi
}

# Function to build contract
build_contract() {
    echo -e "${YELLOW}Building contract...${NC}"

    # Add wasm target if not already added
    rustup target add wasm32-unknown-unknown

    # Build the contract
    cargo build --target wasm32-unknown-unknown --release

    # Create target directory
    mkdir -p target/near

    # Copy and optimize wasm file
    cp target/wasm32-unknown-unknown/release/*.wasm "$WASM_FILE"

    if [ -f "$WASM_FILE" ]; then
        echo -e "${GREEN}✓ Contract built successfully${NC}"
        echo "WASM size: $(du -h $WASM_FILE | cut -f1)"
    else
        echo -e "${RED}✗ Failed to build contract${NC}"
        exit 1
    fi
}

# Function to deploy contract
deploy_contract() {
    local account_id="$1"
    echo -e "${YELLOW}Deploying contract to: $account_id${NC}"

    near deploy --accountId "$account_id" --wasmFile "$WASM_FILE"

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Contract deployed successfully${NC}"
    else
        echo -e "${RED}✗ Failed to deploy contract${NC}"
        exit 1
    fi
}

# Function to initialize contract
initialize_contract() {
    local account_id="$1"
    local owner_id="$2"

    echo -e "${YELLOW}Initializing contract...${NC}"

    near call "$account_id" new "{\"owner\": \"$owner_id\"}" --accountId "$account_id"

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Contract initialized successfully${NC}"
    else
        echo -e "${RED}✗ Failed to initialize contract${NC}"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    echo -e "${YELLOW}Running contract tests...${NC}"

    cargo test

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ All tests passed${NC}"
    else
        echo -e "${RED}✗ Tests failed${NC}"
        exit 1
    fi
}

# Function to create test escrow
create_test_escrow() {
    local contract_id="$1"
    local resolver_id="$2"
    local beneficiary_id="$3"

    echo -e "${YELLOW}Creating test escrow...${NC}"

    local secret="test_secret_12345"
    local secret_hash=$(echo -n "$secret" | sha256sum | cut -d' ' -f1)
    local secret_hash_base58=$(echo -n "$secret_hash" | xxd -r -p | base58)

    echo "Secret: $secret"
    echo "Hash: $secret_hash"
    echo "Base58 Hash: $secret_hash_base58"

    near call "$contract_id" create_escrow '{
        "beneficiary": "'$beneficiary_id'",
        "secret_hash": "'$secret_hash_base58'",
        "token_id": null,
        "amount": "1000000000000000000000000",
        "safety_deposit": "100000000000000000000000",
        "safety_deposit_beneficiary": null,
        "finality_period": 3600,
        "cancel_period": 7200,
        "public_cancel_period": 10800
    }' --accountId "$resolver_id" --deposit 1.1

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Test escrow created successfully${NC}"
    else
        echo -e "${RED}✗ Failed to create test escrow${NC}"
    fi
}

# Main deployment flow
main() {
    echo "Starting deployment process..."
    echo

    # Get account names
    read -p "Enter contract account name (without .testnet): " CONTRACT_NAME
    CONTRACT_ACCOUNT="${CONTRACT_NAME}${TESTNET_SUFFIX}"

    read -p "Enter owner account name (without .testnet): " OWNER_NAME
    OWNER_ACCOUNT="${OWNER_NAME}${TESTNET_SUFFIX}"

    # Step 1: Run tests
    echo
    echo "Step 1: Running tests"
    run_tests

    # Step 2: Build contract
    echo
    echo "Step 2: Building contract"
    build_contract

    # Step 3: Create account (optional)
    echo
    read -p "Do you want to create a new account? (y/n): " create_new
    if [[ $create_new == "y" ]]; then
        create_account "$CONTRACT_ACCOUNT"
    fi

    # Step 4: Deploy contract
    echo
    echo "Step 4: Deploying contract"
    deploy_contract "$CONTRACT_ACCOUNT"

    # Step 5: Initialize contract
    echo
    echo "Step 5: Initializing contract"
    initialize_contract "$CONTRACT_ACCOUNT" "$OWNER_ACCOUNT"

    # Step 6: Create test escrow (optional)
    echo
    read -p "Do you want to create a test escrow? (y/n): " create_test
    if [[ $create_test == "y" ]]; then
        read -p "Enter resolver account (without .testnet): " RESOLVER_NAME
        read -p "Enter beneficiary account (without .testnet): " BENEFICIARY_NAME

        RESOLVER_ACCOUNT="${RESOLVER_NAME}${TESTNET_SUFFIX}"
        BENEFICIARY_ACCOUNT="${BENEFICIARY_NAME}${TESTNET_SUFFIX}"

        create_test_escrow "$CONTRACT_ACCOUNT" "$RESOLVER_ACCOUNT" "$BENEFICIARY_ACCOUNT"
    fi

    echo
    echo -e "${GREEN}🎉 Deployment complete!${NC}"
    echo
    echo "Contract deployed at: $CONTRACT_ACCOUNT"
    echo "View on explorer: https://explorer.testnet.near.org/accounts/$CONTRACT_ACCOUNT"
    echo
    echo "Example commands:"
    echo "  View escrows:    near view $CONTRACT_ACCOUNT get_active_escrows '{\"from_index\": 0, \"limit\": 10}'"
    echo "  Claim escrow:    near call $CONTRACT_ACCOUNT claim '{\"escrow_id\": \"fusion_0\", \"secret\": \"test_secret_12345\"}' --accountId $BENEFICIARY_ACCOUNT"
    echo "  Cancel escrow:   near call $CONTRACT_ACCOUNT cancel '{\"escrow_id\": \"fusion_0\"}' --accountId $RESOLVER_ACCOUNT"
}

# Run main function
main
