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

## Deployment Instructions

1. Install dependencies:
```bash
cd contracts/ethereum
npm install
```

2. Configure environment:
```bash
cp .env.example .env
# Edit .env with your RPC URL and private key
```

3. Compile contracts:
```bash
npm run compile
```

4. Deploy to Sepolia:
```bash
npm run deploy:sepolia
```

5. Save the deployed EscrowFactory address for use in the Rust connector.

## Testing

Run tests with:
```bash
npm test
```

## Integration with Rust
The deployed factory address should be used in the Ethereum connector configuration.
