# Swap ETH to NEAR

This command swaps Ethereum (or ERC20 tokens) to NEAR tokens using UniteSwap.

## Usage
```
Run the swap command with appropriate parameters
```

## What I'll do:
1. Check your environment variables are set up
2. Execute the swap from Ethereum to NEAR
3. Monitor the swap progress
4. Automatically claim funds if --auto-claim is enabled

## Required Environment Variables:
- ETHEREUM_RPC_URL
- ETHEREUM_ADDRESS
- NEAR_ACCOUNT_ID
- PRIVATE_KEY (for signing)

## Example Command:
```bash
./target/release/fusion-cli swap \
  --from-chain ethereum \
  --to-chain near \
  --from-token WETH \
  --to-token NEAR \
  --amount 0.001 \
  --from-address $ETHEREUM_ADDRESS \
  --to-address $NEAR_ACCOUNT_ID \
  --auto-claim
```

## Options:
- `--amount`: Amount to swap (e.g., 0.001)
- `--slippage`: Slippage tolerance (default: 1.0%)
- `--timeout`: HTLC timeout in seconds (default: 3600)
- `--auto-claim`: Automatically claim funds when available
- `--dry-run`: Simulate without executing

## What happens:
1. Create a limit order on Ethereum
2. Create HTLC on NEAR
3. Monitor for order execution
4. Claim funds when available