// Example: Cross-chain swap coordination between Ethereum and NEAR
// This demonstrates how to orchestrate a 1inch Fusion+ swap

const nearAPI = require('near-api-js');
const { ethers } = require('ethers');
const crypto = require('crypto');

// NEAR setup
const { keyStores, KeyPair, connect } = nearAPI;

// Configuration
const config = {
    // NEAR testnet config
    near: {
        networkId: 'testnet',
        nodeUrl: 'https://rpc.testnet.near.org',
        walletUrl: 'https://wallet.testnet.near.org',
        helperUrl: 'https://helper.testnet.near.org',
        explorerUrl: 'https://explorer.testnet.near.org',
        contractId: 'fusion-htlc.testnet',
    },
    // Ethereum config (example for Goerli)
    ethereum: {
        rpcUrl: 'https://goerli.infura.io/v3/YOUR_INFURA_KEY',
        contractAddress: '0x...', // 1inch Fusion+ escrow contract
    }
};

class CrossChainSwapCoordinator {
    constructor() {
        this.nearConnection = null;
        this.nearAccount = null;
        this.ethProvider = null;
        this.ethSigner = null;
    }

    async initialize() {
        // Initialize NEAR
        const keyStore = new keyStores.InMemoryKeyStore();
        // Add your key here for testing
        // await keyStore.setKey('testnet', 'your-account.testnet', KeyPair.fromString('...'));
        
        this.nearConnection = await connect({
            ...config.near,
            keyStore,
        });
        
        // Initialize Ethereum
        this.ethProvider = new ethers.providers.JsonRpcProvider(config.ethereum.rpcUrl);
        // this.ethSigner = new ethers.Wallet('YOUR_PRIVATE_KEY', this.ethProvider);
    }

    // Generate a random secret and its hash
    generateSecret() {
        const secret = crypto.randomBytes(32).toString('hex');
        const hash = crypto.createHash('sha256').update(secret).digest('hex');
        // Convert to base58 for NEAR
        const bs58 = require('bs58');
        const hashBase58 = bs58.encode(Buffer.from(hash, 'hex'));
        
        return {
            secret,
            hash,
            hashBase58
        };
    }

    // Create matching escrows on both chains
    async createCrossChainSwap({
        ethAmount,        // Amount of ETH/ERC20 to swap
        nearAmount,       // Amount of NEAR to receive
        ethResolver,      // Ethereum resolver address
        nearResolver,     // NEAR resolver account
        ethBeneficiary,   // Who receives ETH
        nearBeneficiary,  // Who receives NEAR
        timeoutSeconds = 3600
    }) {
        const { secret, hash, hashBase58 } = this.generateSecret();
        
        console.log('Generated secret:', secret);
        console.log('Hash (hex):', hash);
        console.log('Hash (base58):', hashBase58);
        
        // Time calculations
        const now = Date.now();
        const finalityPeriod = timeoutSeconds / 2; // 30 minutes
        const cancelPeriod = timeoutSeconds;       // 60 minutes
        const publicCancelPeriod = timeoutSeconds * 1.5; // 90 minutes
        
        // Step 1: Create NEAR escrow
        console.log('\n1. Creating NEAR escrow...');
        const nearEscrowId = await this.createNearEscrow({
            beneficiary: nearBeneficiary,
            secretHash: hashBase58,
            amount: nearAmount,
            safetyDeposit: nearAmount * 0.1, // 10% safety deposit
            finalityPeriod,
            cancelPeriod,
            publicCancelPeriod
        });
        
        console.log('NEAR escrow created:', nearEscrowId);
        
        // Step 2: Create Ethereum escrow
        console.log('\n2. Creating Ethereum escrow...');
        // This would call the Ethereum 1inch Fusion+ contract
        // const ethTx = await this.createEthereumEscrow({...});
        
        // Return swap details
        return {
            secret,
            nearEscrowId,
            // ethEscrowTx: ethTx.hash,
            status: 'pending'
        };
    }

    async createNearEscrow(params) {
        const account = await this.nearConnection.account(config.near.contractId);
        
        const result = await account.functionCall({
            contractId: config.near.contractId,
            methodName: 'create_escrow',
            args: {
                beneficiary: params.beneficiary,
                secret_hash: params.secretHash,
                token_id: null, // NEAR transfer
                amount: params.amount.toString(),
                safety_deposit: params.safetyDeposit.toString(),
                safety_deposit_beneficiary: null,
                finality_period: params.finalityPeriod,
                cancel_period: params.cancelPeriod,
                public_cancel_period: params.publicCancelPeriod
            },
            gas: '100000000000000', // 100 TGas
            attachedDeposit: (params.amount + params.safetyDeposit).toString()
        });
        
        // Extract escrow ID from logs
        const logs = result.receipts_outcome[0].outcome.logs;
        const escrowId = logs[0].match(/fusion_\d+/)[0];
        return escrowId;
    }

    // Monitor escrows on both chains
    async monitorSwap(swapDetails) {
        console.log('\nMonitoring swap...');
        
        const checkInterval = setInterval(async () => {
            // Check NEAR escrow status
            const nearEscrow = await this.getNearEscrowStatus(swapDetails.nearEscrowId);
            console.log('NEAR escrow state:', nearEscrow.state);
            
            // Check Ethereum escrow status
            // const ethStatus = await this.getEthereumEscrowStatus(...);
            
            // If one side is claimed, reveal secret on the other
            if (nearEscrow.state === 'Claimed') {
                console.log('NEAR side claimed! Claiming Ethereum side...');
                // await this.claimEthereumEscrow(swapDetails.secret);
                clearInterval(checkInterval);
            }
            
            // Handle timeouts
            const now = Date.now() / 1000;
            if (now > nearEscrow.cancel_time && nearEscrow.state === 'Active') {
                console.log('Timeout reached, cancelling escrows...');
                await this.cancelNearEscrow(swapDetails.nearEscrowId);
                clearInterval(checkInterval);
            }
        }, 5000); // Check every 5 seconds
    }

    async getNearEscrowStatus(escrowId) {
        const account = await this.nearConnection.account(config.near.contractId);
        
        const escrow = await account.viewFunction({
            contractId: config.near.contractId,
            methodName: 'get_escrow',
            args: { escrow_id: escrowId }
        });
        
        return escrow;
    }

    async claimNearEscrow(escrowId, secret) {
        const account = await this.nearConnection.account('beneficiary.testnet');
        
        const result = await account.functionCall({
            contractId: config.near.contractId,
            methodName: 'claim',
            args: {
                escrow_id: escrowId,
                secret: secret
            },
            gas: '100000000000000'
        });
        
        return result;
    }

    async cancelNearEscrow(escrowId) {
        const account = await this.nearConnection.account('resolver.testnet');
        
        const result = await account.functionCall({
            contractId: config.near.contractId,
            methodName: 'cancel',
            args: {
                escrow_id: escrowId
            },
            gas: '100000000000000'
        });
        
        return result;
    }

    // Example: Execute a complete swap
    async executeSwap() {
        try {
            await this.initialize();
            
            // Create cross-chain swap
            const swapDetails = await this.createCrossChainSwap({
                ethAmount: ethers.utils.parseEther('1.0'),     // 1 ETH
                nearAmount: '5000000000000000000000000',       // 5 NEAR
                ethResolver: '0x...',                           // Ethereum resolver
                nearResolver: 'resolver.testnet',               // NEAR resolver
                ethBeneficiary: '0x...',                        // Who gets ETH
                nearBeneficiary: 'alice.testnet',               // Who gets NEAR
                timeoutSeconds: 3600                            // 1 hour timeout
            });
            
            console.log('\nSwap initiated:', swapDetails);
            
            // Monitor and coordinate
            await this.monitorSwap(swapDetails);
            
        } catch (error) {
            console.error('Swap failed:', error);
        }
    }
}

// Usage example
async function main() {
    const coordinator = new CrossChainSwapCoordinator();
    
    // Example 1: Create a swap
    console.log('=== Cross-Chain Swap Example ===\n');
    
    // Example 2: Check active escrows
    const account = await coordinator.nearConnection.account(config.near.contractId);
    const activeEscrows = await account.viewFunction({
        contractId: config.near.contractId,
        methodName: 'get_active_escrows',
        args: { from_index: 0, limit: 10 }
    });
    
    console.log('Active escrows:', activeEscrows);
    
    // Example 3: Get claimable escrows for a beneficiary
    const claimable = await account.viewFunction({
        contractId: config.near.contractId,
        methodName: 'get_claimable_escrows',
        args: { beneficiary: 'alice.testnet' }
    });
    
    console.log('Claimable escrows:', claimable);
}

// Export for use in other scripts
module.exports = {
    CrossChainSwapCoordinator,
    config
};

// Run if called directly
if (require.main === module) {
    main().catch(console.error);
}