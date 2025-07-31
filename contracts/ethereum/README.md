# Escrow Contract Integration

## Contract Overview

Since 1inch doesn't provide official testnet deployments, we've created our own escrow contracts that follow the 1inch interface pattern for the hackathon demo.

### Escrow Factory

- Interface: IEscrowFactory
- Deploys individual escrow contracts for each swap
- Tracks escrows by unique ID

### Escrow Contract

- HTLC-based atomic swap implementation
- Supports both ETH and ERC20 tokens
- Claim with secret reveal or refund after timeout

### Required Functions

```solidity
interface IEscrowFactory {
    function createEscrow(
        address token,
        uint256 amount,
        bytes32 secretHash,
        uint256 timeout,
        address recipient
    ) external payable returns (address escrow);

    function getEscrow(bytes32 escrowId) external view returns (address);
}

interface IEscrow {
    function claim(bytes32 secret) external;
    function refund() external;
    function getDetails() external view returns (
        address sender,
        address recipient,
        uint256 amount,
        bytes32 secretHash,
        uint256 deadline,
        uint8 state
    );
}
```

## Foundry Development

This project uses Foundry for development, testing, and deployment.

### Build

```shell
forge build
```

### Test

```shell
forge test
```

### Format

```shell
forge fmt
```

### Gas Snapshots

```shell
forge snapshot
```

### Deploy

```shell
forge script script/DeployEscrowFactory.s.sol --rpc-url <your_rpc_url> --private-key <your_private_key> --broadcast
```

### Local Development with Anvil

```shell
anvil
```

## Deployment Instructions

1. Install Foundry if not already installed:

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

2. Build contracts:

```bash
forge build
```

3. Deploy to local testnet:

```bash
# Start Anvil in another terminal
anvil

# Deploy using the provided script
forge script script/DeployEscrowFactory.s.sol --fork-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast
```

4. For testnet deployment, set your environment variables and use:

```bash
# Deploy to Base Sepolia
forge script script/DeployEscrowFactory.s.sol --rpc-url $BASE_SEPOLIA_RPC_URL --private-key $PRIVATE_KEY --broadcast --verify --verifier-url https://api-sepolia.basescan.org/api --etherscan-api-key $BASESCAN_API_KEY

# Deploy to Ethereum Sepolia
forge script script/DeployEscrowFactory.s.sol --rpc-url $SEPOLIA_RPC_URL --private-key $PRIVATE_KEY --broadcast --verify --etherscan-api-key $ETHERSCAN_API_KEY
```

## Base Sepolia Deployment

Base Sepolia is the testnet for Base (Coinbase's Layer 2). To deploy:

1. Set up your environment variables in `.env`:
```bash
cp .env.example .env
# Edit .env with your actual values
```

2. Deploy to Base Sepolia:
```bash
source .env
forge script script/DeployEscrowFactory.s.sol:DeployEscrowFactory \
  --rpc-url $BASE_SEPOLIA_RPC_URL \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --verify \
  --verifier-url https://api-sepolia.basescan.org/api \
  --etherscan-api-key $BASESCAN_API_KEY \
  -vvvv
```

3. Deployed Contract Addresses:
- **Base Sepolia EscrowFactory**: `[To be deployed]`

### Base Sepolia Network Details
- Chain ID: 84532
- RPC URL: https://sepolia.base.org
- Block Explorer: https://sepolia.basescan.org/
- Faucet: https://www.coinbase.com/faucets/base-ethereum-goerli-faucet

## Testing

Run tests with:

```bash
forge test
```

Run tests with gas reporting:

```bash
forge test --gas-report
```

## Integration with Rust

The deployed factory address should be used in the Ethereum connector configuration.