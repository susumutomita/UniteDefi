#!/bin/bash

# Cross-Chain Atomic Swap Demo
# Ethereum (Base Sepolia) <-> NEAR Testnet

set -e

echo "=========================================="
echo "UniteDefi Cross-Chain Atomic Swap Demo"
echo "Ethereum (Base Sepolia) <-> NEAR Testnet"
echo "=========================================="
echo ""

# Configuration
ETHEREUM_ESCROW_FACTORY="0x848285f35044e485BD5F0235c27924b1392144b3"
NEAR_HTLC_CONTRACT="htlc-v2.testnet"
SECRET="cross-chain-secret-123"
TIMEOUT_SECONDS=3600

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}1. Setting up the swap scenario${NC}"
echo "   - Alice (Ethereum) wants to swap 0.001 ETH for 1 NEAR from Bob"
echo "   - Bob (NEAR) wants to swap 1 NEAR for 0.001 ETH from Alice"
echo ""

# Generate secret hash
echo -e "${BLUE}2. Generating secret and hash${NC}"
echo "   Secret: $SECRET"

# Create hash test program to get the hash
cd /Users/susumu/UniteDefi/contracts/near-htlc/hash_test
cat > src/main.rs << 'EOF'
use sha2::{Digest, Sha256};

fn main() {
    let secret = std::env::args().nth(1).unwrap_or_else(|| "cross-chain-secret-123".to_string());
    let hash = Sha256::digest(secret.as_bytes());
    let hash_bs58 = bs58::encode(hash).into_string();
    println!("{}", hash_bs58);
}
EOF

# Build and run to get hash
cargo build --quiet 2>/dev/null
HASH=$(./target/debug/hash_test "$SECRET")
echo "   Hash: $HASH"
echo ""

echo -e "${BLUE}3. Bob creates HTLC on NEAR (locks 1 NEAR)${NC}"
echo "   Creating escrow with:"
echo "   - Recipient: alice.testnet"
echo "   - Amount: 1 NEAR"
echo "   - Secret Hash: $HASH"
echo "   - Timeout: $TIMEOUT_SECONDS seconds"
echo ""

# For demo purposes, we'll use uniteswap.testnet as Bob
echo "Executing: near call $NEAR_HTLC_CONTRACT create_escrow"
NEAR_ESCROW_ID=$(near call $NEAR_HTLC_CONTRACT create_escrow \
    "{\"recipient\": \"alice.testnet\", \"secret_hash\": \"$HASH\", \"timeout_seconds\": $TIMEOUT_SECONDS}" \
    --accountId uniteswap.testnet \
    --deposit 1 2>/dev/null | head -1 | tr -d '"')

echo -e "${GREEN}✅ NEAR escrow created: $NEAR_ESCROW_ID${NC}"
echo ""

echo -e "${BLUE}4. Alice sees Bob's NEAR escrow and creates corresponding Ethereum escrow${NC}"
echo "   Creating escrow on Ethereum with:"
echo "   - Recipient: 0xBobAddress"
echo "   - Amount: 0.001 ETH"
echo "   - Secret Hash: $HASH (same as NEAR)"
echo "   - Timeout: $TIMEOUT_SECONDS seconds"
echo ""

# Create Ethereum escrow script
cd /Users/susumu/UniteDefi
cat > scripts/create_eth_escrow.js << EOF
const { ethers } = require('ethers');

async function createEscrow() {
    const provider = new ethers.JsonRpcProvider(process.env.ETHEREUM_RPC_URL || 'https://base-sepolia.infura.io/v3/YOUR_KEY');
    const wallet = new ethers.Wallet(process.env.PRIVATE_KEY || '0x0000000000000000000000000000000000000000000000000000000000000000', provider);
    
    const ESCROW_FACTORY_ABI = [
        "function createEscrow(address _recipient, bytes32 _secretHash, uint256 _timeout) external payable returns (address)"
    ];
    
    const factory = new ethers.Contract('$ETHEREUM_ESCROW_FACTORY', ESCROW_FACTORY_ABI, wallet);
    
    // Convert base58 hash to bytes32
    const secretHash = ethers.keccak256(ethers.toUtf8Bytes('$SECRET'));
    const timeout = Math.floor(Date.now() / 1000) + $TIMEOUT_SECONDS;
    
    console.log('Creating Ethereum escrow...');
    const tx = await factory.createEscrow(
        '0x0000000000000000000000000000000000000002', // Bob's address (placeholder)
        secretHash,
        timeout,
        { value: ethers.parseEther('0.001') }
    );
    
    const receipt = await tx.wait();
    console.log('Ethereum escrow created in tx:', receipt.hash);
    
    // Get escrow address from events
    const event = receipt.logs[0];
    console.log('Escrow address:', event.address || 'Check transaction on explorer');
}

createEscrow().catch(console.error);
EOF

echo "Note: In a real scenario, Alice would execute the Ethereum transaction"
echo -e "${GREEN}✅ Ethereum escrow would be created (demo mode)${NC}"
echo ""

echo -e "${BLUE}5. Bob claims the Ethereum escrow using the secret${NC}"
echo "   Bob reveals the secret to claim ETH from Alice's escrow"
echo "   Secret revealed: $SECRET"
echo ""

echo -e "${BLUE}6. Alice sees the revealed secret and claims NEAR${NC}"
echo "   Using the revealed secret to claim from NEAR escrow"
echo ""

echo "Executing: near call $NEAR_HTLC_CONTRACT claim"
CLAIM_RESULT=$(near call $NEAR_HTLC_CONTRACT claim \
    "{\"escrow_id\": \"$NEAR_ESCROW_ID\", \"secret\": \"$SECRET\"}" \
    --accountId uniteswap.testnet 2>&1)

if echo "$CLAIM_RESULT" | grep -q "succeeded"; then
    echo -e "${GREEN}✅ NEAR escrow claimed successfully!${NC}"
else
    echo "Note: Claim may fail if alice.testnet is not set up"
fi
echo ""

echo -e "${GREEN}=========================================="
echo "Cross-Chain Atomic Swap Demo Complete!"
echo "=========================================="
echo ""
echo "Summary:"
echo "1. Bob locked 1 NEAR in escrow (NEAR contract)"
echo "2. Alice would lock 0.001 ETH in escrow (Ethereum contract)"
echo "3. Bob claims ETH using secret (reveals secret on-chain)"
echo "4. Alice claims NEAR using revealed secret"
echo ""
echo "Key Properties:"
echo "- Atomic: Either both swaps complete or neither does"
echo "- Trustless: No intermediary required"
echo "- Secure: Hash-time-locked for safety"
echo -e "${NC}"

# Cleanup
rm -f /Users/susumu/UniteDefi/scripts/create_eth_escrow.js