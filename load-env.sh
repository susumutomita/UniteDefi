#!/bin/bash
# Load environment variables from .env file

if [ -f .env ]; then
  # Export each non-empty line that doesn't start with #
  # Security: Use set -a to avoid exposing variables in command line
  set -a
  source .env
  set +a
  
  echo "✅ Environment variables loaded from .env"
  
  # Security: Only show that variables are loaded, not their values
  echo "📋 Environment variables loaded successfully"
  echo "  - Configuration: .env"
  echo "  - Status: Ready"
  
  # Security: Validate required variables exist without showing values
  required_vars=("ETHEREUM_RPC_URL" "ETHEREUM_ADDRESS" "NEAR_ACCOUNT_ID")
  missing_vars=()
  
  for var in "${required_vars[@]}"; do
    if [ -z "${!var}" ]; then
      missing_vars+=("$var")
    fi
  done
  
  if [ ${#missing_vars[@]} -ne 0 ]; then
    echo "❌ Missing required environment variables:"
    printf '  - %s\n' "${missing_vars[@]}"
    exit 1
  fi
  
  echo "  - All required variables: ✓"
else
  echo "❌ .env file not found"
  exit 1
fi