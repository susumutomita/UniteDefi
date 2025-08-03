#!/bin/bash

# Test script to verify release build locally
# This simulates what GitHub Actions will do

echo "Testing release build locally..."
echo "==============================="

# Test macOS ARM build (current platform)
TARGET="aarch64-apple-darwin"
VERSION="v0.1.0-test"

echo "Building for $TARGET..."
cargo build -p fusion-cli --release --target $TARGET

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    
    # Create test staging directory
    STAGING="fusion-cli-$VERSION-$TARGET"
    mkdir -p "$STAGING"
    
    # Copy files
    cp "target/$TARGET/release/fusion-cli" "$STAGING/"
    cp README.md "$STAGING/"
    cp LICENSE "$STAGING/" 2>/dev/null || echo "Note: LICENSE file not found"
    
    # Create .env.example
    cat > "$STAGING/.env.example" << 'EOF'
# Ethereum Configuration
ETHEREUM_RPC_URL=https://sepolia.base.org
PRIVATE_KEY=your_private_key_here

# NEAR Configuration  
NEAR_ACCOUNT=your_account.testnet
NEAR_NETWORK=testnet
EOF
    
    # Create run script
    cat > "$STAGING/run.sh" << 'EOF'
#!/bin/bash
echo "UniteSwap CLI - Cross-chain Atomic Swaps"
echo "========================================"
echo ""
echo "Please ensure you have set up your .env file with:"
echo "- ETHEREUM_RPC_URL"
echo "- PRIVATE_KEY" 
echo "- NEAR_ACCOUNT (for NEAR operations)"
echo ""
./fusion-cli "$@"
EOF
    chmod +x "$STAGING/run.sh"
    chmod +x "$STAGING/fusion-cli"
    
    # Create archive
    tar czf "$STAGING.tar.gz" "$STAGING"
    
    echo ""
    echo "✅ Test archive created: $STAGING.tar.gz"
    echo ""
    echo "Archive contents:"
    tar -tzf "$STAGING.tar.gz" | head -10
    
    # Clean up
    rm -rf "$STAGING"
    
    echo ""
    echo "✅ Release build test completed successfully!"
    echo ""
    echo "To create an actual release:"
    echo "1. Commit and push all changes"
    echo "2. Create a new tag: git tag v0.1.0"
    echo "3. Push the tag: git push origin v0.1.0"
    echo "4. Create a release on GitHub with this tag"
    echo "5. The workflow will automatically build and upload binaries"
else
    echo "❌ Build failed!"
    exit 1
fi