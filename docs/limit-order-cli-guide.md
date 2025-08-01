# Limit Order CLI Guide

## Overview

The `fusion-cli` tool provides commands to create and manage limit orders.
These orders are compatible with the 1inch Limit Order Protocol v3.
This guide explains how to use the `order create` command.
It shows how to generate orders with embedded HTLC information.

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

The HTLC information is embedded in the order's `interactions` field.
This field is also referred to as `makerAssetData`:

- Bytes 0-31: HTLC secret hash (32 bytes)
- Bytes 32-63: HTLC timeout as uint256 (32 bytes, big-endian)

This encoding allows the order to carry cross-chain swap information.
It remains compatible with the 1inch protocol.

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

## Relay Order Command

The `relay-order` command enables manual relaying of limit orders from EVM chains to NEAR. This is essential for cross-chain atomic swaps before automated relayers are implemented.

### Usage

```bash
fusion-cli relay-order [OPTIONS]
```

### Required Options

- `--order-hash <HASH>` - The hash of the order created on EVM
- `--to-chain <CHAIN>` - Target chain for relaying (currently only "near" is supported)
- `--htlc-secret <SECRET>` - The HTLC secret (32 bytes in hex format)

### Optional Options

- `--near-account <ACCOUNT>` - NEAR account ID (defaults to environment variable)
- `--evm-rpc <URL>` - EVM RPC endpoint
- `--near-network <NETWORK>` - NEAR network: testnet or mainnet (default: testnet)

### Example

```bash
fusion-cli relay-order \
  --order-hash 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890 \
  --to-chain near \
  --htlc-secret 0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba \
  --near-account alice.testnet \
  --near-network testnet
```

### Output Format

```json
{
  "status": "success",
  "relay_details": {
    "from_chain": "ethereum",
    "to_chain": "near",
    "order_hash": "0xabcdef..."
  },
  "htlc_info": {
    "htlc_id": "htlc_12345678",
    "secret_hash": "0x...",
    "timeout_seconds": 3600,
    "recipient": "alice.testnet"
  },
  "transactions": {
    "near_htlc_creation": "0x...",
    "explorer_url": "https://explorer.testnet.near.org/transactions/0x..."
  },
  "next_steps": [
    "Monitor the order execution on Ethereum",
    "Once the order is filled, the secret will be revealed",
    "Use the secret to claim funds from the NEAR HTLC"
  ]
}
```

### Workflow

1. **Create Order**: First create a limit order on EVM using `fusion-cli order create`
2. **Relay Order**: Use this command to create a corresponding HTLC on NEAR
3. **Monitor**: Watch for order execution on Ethereum
4. **Claim**: Once filled, use the revealed secret to claim from NEAR HTLC

## Error Handling

The CLI provides clear error messages for common issues:

- Invalid address format
- Incorrect secret hash length
- Missing required parameters
- Invalid hex encoding
- Unsupported target chain
- Invalid order hash format

## Related Commands

- `fusion-cli create-htlc` - Create HTLC contracts
- `fusion-cli claim` - Claim HTLC with secret
- `fusion-cli refund` - Refund expired HTLC
- `fusion-cli relay-order` - Relay orders from EVM to NEAR