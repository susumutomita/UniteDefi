# UniteSwap CLI Releases

## Quick Start with Pre-built Binaries

1. **Download the appropriate binary for your platform from GitHub Releases**
   - Linux: `fusion-cli-{version}-x86_64-unknown-linux-gnu.tar.gz`
   - macOS Intel: `fusion-cli-{version}-x86_64-apple-darwin.tar.gz`
   - macOS ARM (M1/M2): `fusion-cli-{version}-aarch64-apple-darwin.tar.gz`
   - Windows: `fusion-cli-{version}-x86_64-pc-windows-msvc.zip`

2. **Extract the archive**
   ```bash
   # Linux/macOS
   tar -xzf fusion-cli-*.tar.gz
   cd fusion-cli-*
   
   # Windows
   # Extract the .zip file using your preferred tool
   ```

3. **Set up environment variables**
   ```bash
   # Copy the example environment file
   cp .env.example .env
   
   # Edit .env with your settings:
   # - ETHEREUM_RPC_URL: Your Ethereum RPC endpoint (default: Base Sepolia)
   # - PRIVATE_KEY: Your wallet private key (for signing transactions)
   # - NEAR_ACCOUNT: Your NEAR account (for NEAR operations)
   ```

4. **Run the CLI**
   ```bash
   # Linux/macOS
   ./run.sh --help
   
   # Windows
   run.bat --help
   
   # Or directly:
   ./fusion-cli --help
   ```

## Demo Commands (No Private Key Required)

These commands demonstrate the CLI functionality without submitting to blockchain:

```bash
# Display help
./fusion-cli --help

# Create HTLC (generates secret, no blockchain submission)
./fusion-cli htlc create \
  --sender 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --recipient 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --amount 1000000000000000000

# Create order (no signing/submission)
./fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62

# Dry run swap (simulation only)
./fusion-cli swap \
  --from-chain ethereum \
  --to-chain near \
  --from-token WETH \
  --to-token NEAR \
  --amount 0.1 \
  --from-address 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --to-address alice.near \
  --dry-run
```

## Full Usage (With Private Key)

To execute real blockchain transactions:

1. Set up your `.env` file with valid credentials
2. Add `--sign` and `--submit` flags to commands
3. Ensure you have testnet funds

Example:
```bash
# Sign and submit a real order
./fusion-cli order create \
  [... parameters ...] \
  --sign \
  --submit
```

## System Requirements

- No additional dependencies required for the CLI itself
- For NEAR operations: NEAR CLI must be installed separately
- For Ethereum operations: Internet connection to RPC endpoint

## Security Notes

- Never share your private key
- Use testnet for testing
- The `.env.example` file is safe to share
- Your actual `.env` file should be kept private

## Troubleshooting

1. **Permission denied on Linux/macOS**
   ```bash
   chmod +x fusion-cli
   chmod +x run.sh
   ```

2. **Binary not working on macOS**
   - You may need to allow it in System Preferences > Security & Privacy
   - Or remove quarantine: `xattr -d com.apple.quarantine fusion-cli`

3. **Missing environment variables**
   - Check that your `.env` file exists and contains required values
   - Use `.env.example` as a template

## Building from Source

If pre-built binaries don't work for your platform:

```bash
git clone https://github.com/susumutomita/UniteDefi.git
cd UniteDefi
cargo build -p fusion-cli --release
./target/release/fusion-cli --help
```