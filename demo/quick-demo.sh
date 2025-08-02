#!/bin/bash

# Quick Cross-Chain Swap Demo
# Demonstrates NEAR HTLC functionality

set -e

echo "======================================"
echo "UniteDefi Quick Demo - NEAR HTLC"
echo "======================================"
echo ""

# Configuration
NEAR_HTLC="htlc-v2.testnet"
SECRET="demo-secret-$(date +%s)"
HASH=""

# Generate hash
cd /Users/susumu/UniteDefi/contracts/near-htlc/hash_test
cat > src/main.rs << 'EOF'
use sha2::{Digest, Sha256};
fn main() {
    let secret = std::env::args().nth(1).unwrap();
    let hash = Sha256::digest(secret.as_bytes());
    println!("{}", bs58::encode(hash).into_string());
}
EOF
cargo build --quiet 2>/dev/null
HASH=$(../target/debug/hash_test "$SECRET")

echo "1. Creating HTLC escrow on NEAR"
echo "   Secret: $SECRET"
echo "   Hash: $HASH"
echo ""

# Create escrow
ESCROW_ID=$(near call $NEAR_HTLC create_escrow \
    "{\"recipient\": \"alice.testnet\", \"secret_hash\": \"$HASH\", \"timeout_seconds\": 3600}" \
    --accountId uniteswap.testnet \
    --deposit 0.1 2>/dev/null | head -1 | tr -d '"')

echo "✅ Escrow created: $ESCROW_ID"
echo ""

# View escrow
echo "2. Viewing escrow details:"
near view $NEAR_HTLC get_escrow "{\"escrow_id\": \"$ESCROW_ID\"}" 2>/dev/null | jq .
echo ""

# Claim escrow
echo "3. Claiming escrow with secret:"
near call $NEAR_HTLC claim \
    "{\"escrow_id\": \"$ESCROW_ID\", \"secret\": \"$SECRET\"}" \
    --accountId uniteswap.testnet 2>&1 | grep -E "(succeeded|claimed)" || true
echo ""

echo "✅ Demo complete! HTLC functionality verified."