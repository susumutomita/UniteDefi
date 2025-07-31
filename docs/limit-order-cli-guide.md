# Limit Order CLI Guide

## Overview

The `fusion-cli` tool provides commands to create and manage limit orders compatible with the 1inch Limit Order Protocol v3. This guide explains how to use the `order create` command to generate orders with embedded HTLC (Hash Time-Locked Contract) information.

## Order Create Command

The `order create` command generates a limit order with EIP-712 signature data.

### Usage

```bash
fusion-cli order create [OPTIONS]
```

### Required Options

- `--maker-asset <ADDRESS>` - The address of the asset the maker is offering
- `--taker-asset <ADDRESS>` - The address of the asset the maker wants to receive
- `--maker <ADDRESS>` - The address of the order creator (maker)
- `--making-amount <AMOUNT>` - The amount of maker asset to trade
- `--taking-amount <AMOUNT>` - The amount of taker asset to receive
- `--htlc-secret-hash <HASH>` - The HTLC secret hash (32 bytes in hex format)
- `--htlc-timeout <SECONDS>` - The HTLC timeout duration in seconds
- `--chain-id <ID>` - The blockchain chain ID
- `--verifying-contract <ADDRESS>` - The 1inch Limit Order Protocol contract address

### Optional Options

- `--receiver <ADDRESS>` - Custom receiver address (defaults to zero address)
- `--allowed-sender <ADDRESS>` - Restrict who can fill the order (defaults to anyone)

### Example

```bash
fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62
```

### Output Format

The command outputs a JSON object containing:

```json
{
  "order": {
    "salt": "0x...",
    "makerAsset": "0x4200000000000000000000000000000000000006",
    "takerAsset": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
    "maker": "0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950",
    "receiver": "0x0000000000000000000000000000000000000000",
    "allowedSender": "0x0000000000000000000000000000000000000000",
    "makingAmount": "1000000000000000000",
    "takingAmount": "3000000000",
    "offsets": "0",
    "interactions": "0x1234567890abcdef..."
  },
  "domain": {
    "name": "1inch Limit Order Protocol",
    "version": "3",
    "chainId": 84532,
    "verifyingContract": "0x171C87724E720F2806fc29a010a62897B30fdb62"
  },
  "eip712_hash": "0x...",
  "htlc_info": {
    "secret_hash": "0x1234567890abcdef...",
    "timeout_seconds": 3600
  }
}
```

## HTLC Information Encoding

The HTLC information is embedded in the order's `interactions` field (also referred to as `makerAssetData`):

- **Bytes 0-31**: HTLC secret hash (32 bytes)
- **Bytes 32-63**: HTLC timeout as uint256 (32 bytes, big-endian)

This encoding allows the order to carry cross-chain swap information while remaining compatible with the 1inch protocol.

## Signing the Order

To create a valid order, you need to sign the `eip712_hash` with the maker's private key. This can be done using:

1. **Web3 wallets** (MetaMask, etc.)
2. **Ethers.js** or **Web3.js** libraries
3. **Hardware wallets** (Ledger, Trezor)

Example using ethers.js:

```javascript
const signature = await signer.signTypedData(
  domain,
  types,
  order
);
```

## Integration with 1inch Protocol

Once signed, the order can be submitted to:

1. **1inch Limit Order API** for public orderbook listing
2. **Direct settlement** through the Limit Order Protocol contract
3. **Cross-chain bridges** that support HTLC-based atomic swaps

## Security Considerations

1. **Secret Hash**: Never reveal the preimage until claiming funds
2. **Timeout**: Set appropriate timeout values for cross-chain operations
3. **Address Validation**: The CLI validates all addresses are properly formatted
4. **Amount Precision**: Ensure correct decimal places for token amounts

## Error Handling

The CLI provides clear error messages for common issues:

- Invalid address format
- Incorrect secret hash length
- Missing required parameters
- Invalid hex encoding

## Related Commands

- `fusion-cli create-htlc` - Create HTLC contracts
- `fusion-cli claim` - Claim HTLC with secret
- `fusion-cli refund` - Refund expired HTLC