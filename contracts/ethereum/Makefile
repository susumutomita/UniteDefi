.PHONY: help build clean test deploy deploy-sepolia deploy-base-sepolia deploy-monad verify deploy-limitorder-base-sepolia

# Default target - show help
help:
	@echo "UniteDefi EVM Contracts - Available commands:"
	@echo ""
	@echo "  make build              - Build contracts using Forge"
	@echo "  make clean              - Remove build artifacts"
	@echo "  make test               - Run all tests"
	@echo "  make test-gas           - Run tests with gas reporting"
	@echo ""
	@echo "Deployment commands:"
	@echo "  make deploy-sepolia     - Deploy EscrowFactory to Ethereum Sepolia testnet"
	@echo "  make deploy-base-sepolia - Deploy EscrowFactory to Base Sepolia testnet"
	@echo "  make deploy-monad       - Deploy EscrowFactory to Monad testnet (when available)"
	@echo "  make deploy             - Deploy EscrowFactory using custom environment variables"
	@echo ""
	@echo "1inch Limit Order Protocol deployment:"
	@echo "  make deploy-limitorder-base-sepolia - Deploy 1inch Limit Order Protocol to Base Sepolia"
	@echo ""
	@echo "Verification:"
	@echo "  make verify             - Verify contract on Etherscan"
	@echo ""
	@echo "Environment setup:"
	@echo "  1. Copy .env.example to .env"
	@echo "  2. Fill in your API keys and addresses"
	@echo "  3. Run deployment commands"
	@echo ""

# Load environment variables from .env file
-include .env
export $(shell sed 's/=.*//' .env 2>/dev/null)

# Build the project using Forge
build:
	forge build

# Clean the project using Forge (removes build artifacts)
clean:
	forge clean

# Run tests
test:
	forge test

# Run tests with gas reporting
test-gas:
	forge test --gas-report

# Default deployment settings with the ability to override from the command line
RPC_URL ?= $(SEPOLIA_RPC_URL)
SENDER_ADDRESS ?= $(SEPOLIA_SENDER_ADDRESS)
PRIVATE_KEY ?= $(SEPOLIA_PRIVATE_KEY)

# Deploy EscrowFactory to Sepolia
deploy-sepolia: build
	forge script script/DeployEscrowFactory.s.sol:DeployEscrowFactory \
		--rpc-url $(SEPOLIA_RPC_URL) \
		--broadcast \
		--sender $(SEPOLIA_SENDER_ADDRESS) \
		--private-key $(SEPOLIA_PRIVATE_KEY) \
		--verify \
		--etherscan-api-key $(ETHERSCAN_API_KEY) \
		-vvvv

# Deploy EscrowFactory to Base Sepolia
deploy-base-sepolia: build
	forge script script/DeployEscrowFactory.s.sol:DeployEscrowFactory \
		--rpc-url $(BASE_SEPOLIA_RPC_URL) \
		--broadcast \
		--sender $(BASE_SENDER_ADDRESS) \
		--private-key $(BASE_PRIVATE_KEY) \
		-vvvv

# Deploy EscrowFactory to Monad Testnet (when available)
deploy-monad: build
	forge script script/DeployEscrowFactory.s.sol:DeployEscrowFactory \
		--rpc-url $(MONAD_RPC_URL) \
		--broadcast \
		--sender $(MONAD_SENDER_ADDRESS) \
		--private-key $(MONAD_PRIVATE_KEY) \
		-vvvv

# Deploy EscrowFactory using custom environment variables
deploy: build
	forge script script/DeployEscrowFactory.s.sol:DeployEscrowFactory \
		--rpc-url $(RPC_URL) \
		--broadcast \
		--sender $(SENDER_ADDRESS) \
		--private-key $(PRIVATE_KEY) \
		-vvvv

# Verify contract on Etherscan
verify:
	forge verify-contract \
		--chain-id $(CHAIN_ID) \
		--etherscan-api-key $(ETHERSCAN_API_KEY) \
		$(CONTRACT_ADDRESS) \
		src/EscrowFactory.sol:EscrowFactory

# Deploy 1inch Limit Order Protocol to Base Sepolia
deploy-limitorder-base-sepolia: build
	@echo "Deploying 1inch Limit Order Protocol to Base Sepolia..."
	@echo ""
	forge script script/DeployLimitOrderProtocol.s.sol:DeployLimitOrderProtocol \
		--rpc-url $(BASE_SEPOLIA_RPC_URL) \
		--broadcast \
		--private-key $(BASE_PRIVATE_KEY) \
		-vvvv
