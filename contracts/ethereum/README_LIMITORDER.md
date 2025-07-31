# 1inch Limit Order Protocol Deployment Guide

## Overview
This guide explains how to deploy the 1inch Limit Order Protocol to Base Sepolia testnet.

## Prerequisites
- Foundry installed
- Base Sepolia ETH (minimum 0.02 ETH recommended)
- Private key with funds

## Deployment Steps

### 1. Set Environment Variables
Create a `.env` file with:
```bash
BASE_SEPOLIA_RPC_URL=https://sepolia.base.org
PRIVATE_KEY=your_private_key_here
```

### 2. Deploy the Contract
```bash
# Deploy to Base Sepolia
forge script script/DeployLimitOrderProtocol.s.sol:DeployLimitOrderProtocol \
  --rpc-url base-sepolia \
  --broadcast \
  --verify
```

### 3. Verify Deployment
The deployment script will output:
- LimitOrderProtocol address
- Owner address
- WETH address used

## Contract Details
- **WETH Address (Base Sepolia)**: `0x4200000000000000000000000000000000000006`
- **Estimated Gas**: ~4,590,430
- **Estimated Cost**: ~0.014 ETH

## Post-Deployment
After successful deployment:
1. Save the deployed contract address
2. Update the `.env` file with `LIMIT_ORDER_PROTOCOL_ADDRESS`
3. Configure the CLI to use this address

## Integration with UniteDefi
The Limit Order Protocol will be used for cross-chain atomic swaps:
- Orders include HTLC data in `makerAssetData` or `interaction` fields
- Relayer monitors orders and coordinates cross-chain execution
- CLI provides `fusion-gateway order create` command for order creation