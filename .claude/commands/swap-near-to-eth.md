# Swap NEAR to ETH

This command swaps NEAR tokens to Ethereum (or ERC20 tokens) using UniteSwap.

## Usage
```
Run the swap command from NEAR to Ethereum
```

## What I'll do:
1. Check your environment variables are set up
2. Create HTLC on NEAR with your tokens
3. Create cross-chain order for Ethereum
4. Monitor the swap progress
5. Automatically claim funds if --auto-claim is enabled

## Required Environment Variables:
- NEAR_ACCOUNT_ID
- ETHEREUM_ADDRESS
- ETHEREUM_RPC_URL
- PRIVATE_KEY (for claiming on Ethereum)

## Example Command:
```bash
./target/release/fusion-cli swap \
  --from-chain near \
  --to-chain ethereum \
  --from-token NEAR \
  --to-token WETH \
  --amount 1.0 \
  --from-address $NEAR_ACCOUNT_ID \
  --to-address $ETHEREUM_ADDRESS \
  --auto-claim
```

## Options:
- `--amount`: Amount to swap (e.g., 1.0)
- `--slippage`: Slippage tolerance (default: 1.0%)
- `--timeout`: HTLC timeout in seconds (default: 3600)
- `--auto-claim`: Automatically claim funds when available
- `--dry-run`: Simulate without executing

## What happens:
1. Lock NEAR tokens in HTLC contract
2. Create cross-chain order
3. Wait for market maker to fill order
4. Claim ETH/tokens on Ethereum