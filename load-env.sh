#!/bin/bash
# Load environment variables from .env file

if [ -f .env ]; then
  # Export each non-empty line that doesn't start with #
  export $(grep -v '^#' .env | xargs)
  echo "✅ Environment variables loaded from .env"
  
  # Show loaded variables (hiding sensitive data)
  echo "📋 Loaded variables:"
  echo "  - ETHEREUM_RPC_URL: $ETHEREUM_RPC_URL"
  echo "  - ETHEREUM_ADDRESS: $ETHEREUM_ADDRESS"
  echo "  - ETHEREUM_CHAIN_ID: $ETHEREUM_CHAIN_ID"
  echo "  - NEAR_ACCOUNT_ID: $NEAR_ACCOUNT_ID"
  echo "  - NEAR_CONTRACT_ID: $NEAR_CONTRACT_ID"
  echo "  - LIMIT_ORDER_CONTRACT: $LIMIT_ORDER_CONTRACT"
  echo "  - PRIVATE_KEY: [HIDDEN]"
else
  echo "❌ .env file not found"
  exit 1
fi