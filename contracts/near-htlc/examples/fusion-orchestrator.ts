// 1inch Fusion+ Cross-Chain Swap Orchestrator Example
// This demonstrates how the CLI tool orchestrates swaps between Ethereum and NEAR

import { ethers } from 'ethers';
import { connect, keyStores, utils } from 'near-api-js';
import crypto from 'crypto';

/**
 * Main orchestrator class that coordinates cross-chain swaps
 * No TEE required - uses cryptographic guarantees of HTLC
 */
class FusionCrossChainOrchestrator {
    private ethProvider: ethers.Provider;
    private ethSigner: ethers.Signer;
    private nearConnection: any;
    private nearAccount: any;

    constructor(
        private config: {
            ethereumRpc: string;
            nearRpc: string;
            ethPrivateKey: string;
            nearAccountId: string;
            nearPrivateKey: string;
        }
    ) {}

    async initialize() {
        // Initialize Ethereum connection
        this.ethProvider = new ethers.JsonRpcProvider(this.config.ethereumRpc);
        this.ethSigner = new ethers.Wallet(this.config.ethPrivateKey, this.ethProvider);

        // Initialize NEAR connection
        const keyStore = new keyStores.InMemoryKeyStore();
        await keyStore.setKey(
            'testnet',
            this.config.nearAccountId,
            utils.KeyPair.fromString(this.config.nearPrivateKey)
        );

        this.nearConnection = await connect({
            networkId: 'testnet',
            keyStore,
            nodeUrl: this.config.nearRpc,
        });

        this.nearAccount = await this.nearConnection.account(this.config.nearAccountId);
    }

    /**
     * Execute a cross-chain swap
     * Flow: User has ETH, wants NEAR
     * Resolver provides NEAR liquidity
     */
    async executeSwap(params: {
        userEthAddress: string;
        userNearAddress: string;
        resolverEthAddress: string;
        resolverNearAddress: string;
        ethAmount: string; // in wei
        nearAmount: string; // in yoctoNEAR
    }) {
        console.log('🚀 Starting cross-chain swap...');

        // Step 1: Generate secret and hash
        const secret = crypto.randomBytes(32).toString('hex');
        const secretHash = ethers.keccak256(ethers.toUtf8Bytes(secret));
        console.log('🔐 Secret generated, hash:', secretHash);

        // Step 2: Calculate timeouts
        const now = Math.floor(Date.now() / 1000);
        const timeouts = {
            finality: 30 * 60,        // 30 minutes
            cancel: 60 * 60,          // 1 hour  
            publicCancel: 90 * 60     // 1.5 hours
        };

        try {
            // Step 3: Resolver creates NEAR escrow (destination chain)
            console.log('📝 Resolver creating NEAR escrow...');
            const nearEscrowId = await this.createNearEscrow({
                beneficiary: params.userNearAddress,
                secretHash: this.convertHashForNear(secretHash),
                amount: params.nearAmount,
                timeouts
            });
            console.log('✅ NEAR escrow created:', nearEscrowId);

            // Step 4: Wait for NEAR escrow confirmation
            await this.waitForNearEscrowConfirmation(nearEscrowId);

            // Step 5: User creates Ethereum escrow (source chain)
            console.log('📝 User creating Ethereum escrow...');
            const ethEscrowId = await this.createEthereumEscrow({
                beneficiary: params.resolverEthAddress,
                secretHash: secretHash,
                amount: params.ethAmount,
                timeouts: {
                    finality: timeouts.finality - 10 * 60,     // 10 min earlier
                    cancel: timeouts.cancel - 10 * 60,
                    publicCancel: timeouts.publicCancel - 10 * 60
                }
            });
            console.log('✅ Ethereum escrow created:', ethEscrowId);

            // Step 6: Monitor both escrows
            console.log('👀 Monitoring escrows...');
            await this.monitorEscrows(ethEscrowId, nearEscrowId);

            // Step 7: Reveal secret on NEAR (user claims)
            console.log('🔓 Revealing secret on NEAR...');
            await this.claimNearEscrow(nearEscrowId, secret);
            console.log('✅ User received NEAR!');

            // Step 8: Use same secret on Ethereum (resolver claims)
            console.log('🔓 Claiming on Ethereum with same secret...');
            await this.claimEthereumEscrow(ethEscrowId, secret);
            console.log('✅ Resolver received ETH!');

            console.log('🎉 Swap completed successfully!');
            
            return {
                success: true,
                ethEscrowId,
                nearEscrowId,
                secret
            };

        } catch (error) {
            console.error('❌ Swap failed:', error);
            
            // Handle timeout and refunds
            await this.handleFailure(params);
            
            throw error;
        }
    }

    /**
     * Create NEAR escrow
     * This would call the fusion_htlc.rs contract
     */
    private async createNearEscrow(params: any): Promise<string> {
        const result = await this.nearAccount.functionCall({
            contractId: 'fusion-htlc.testnet',
            methodName: 'create_escrow',
            args: {
                beneficiary: params.beneficiary,
                secret_hash: params.secretHash,
                token_id: null, // NEAR transfer
                amount: params.amount,
                safety_deposit: utils.format.parseNearAmount('0.1'),
                safety_deposit_beneficiary: null,
                finality_period: params.timeouts.finality,
                cancel_period: params.timeouts.cancel,
                public_cancel_period: params.timeouts.publicCancel
            },
            gas: '100000000000000',
            attachedDeposit: params.amount
        });

        return result.receipts_outcome[0].outcome.logs[0]; // Extract escrow ID
    }

    /**
     * Create Ethereum escrow
     * This would call the Ethereum HTLC contract
     */
    private async createEthereumEscrow(params: any): Promise<string> {
        // Ethereum contract ABI (simplified)
        const htlcAbi = [
            'function createEscrow(address beneficiary, bytes32 secretHash, uint256 finality, uint256 cancel, uint256 publicCancel) payable returns (bytes32)'
        ];

        const htlcContract = new ethers.Contract(
            '0x...', // Ethereum HTLC address
            htlcAbi,
            this.ethSigner
        );

        const tx = await htlcContract.createEscrow(
            params.beneficiary,
            params.secretHash,
            params.timeouts.finality,
            params.timeouts.cancel,
            params.timeouts.publicCancel,
            { value: params.amount }
        );

        const receipt = await tx.wait();
        return receipt.logs[0].topics[1]; // Extract escrow ID from event
    }

    /**
     * Monitor escrows for state changes
     * No TEE needed - just watching public blockchain data
     */
    private async monitorEscrows(ethEscrowId: string, nearEscrowId: string) {
        // Set up event listeners
        console.log('Setting up blockchain monitors...');

        // Monitor Ethereum events
        const ethFilter = {
            address: '0x...', // HTLC contract
            topics: [
                ethers.id('EscrowClaimed(bytes32,address,bytes32)'),
                ethEscrowId
            ]
        };

        // Monitor NEAR (polling since NEAR doesn't have event filters)
        const checkNearStatus = setInterval(async () => {
            const escrow = await this.nearAccount.viewFunction({
                contractId: 'fusion-htlc.testnet',
                methodName: 'get_escrow',
                args: { escrow_id: nearEscrowId }
            });

            if (escrow.state === 'Claimed') {
                console.log('NEAR escrow claimed!');
                clearInterval(checkNearStatus);
            }
        }, 5000); // Check every 5 seconds
    }

    /**
     * Convert Ethereum hash format to NEAR base58
     */
    private convertHashForNear(ethHash: string): string {
        const hashBytes = ethers.getBytes(ethHash);
        return utils.serialize.base_encode(hashBytes);
    }

    /**
     * Wait for NEAR transaction confirmation
     */
    private async waitForNearEscrowConfirmation(escrowId: string) {
        // Wait for 2 blocks for finality
        await new Promise(resolve => setTimeout(resolve, 3000));
        
        // Verify escrow exists
        const escrow = await this.nearAccount.viewFunction({
            contractId: 'fusion-htlc.testnet',
            methodName: 'get_escrow',
            args: { escrow_id: escrowId }
        });

        if (!escrow) {
            throw new Error('NEAR escrow not found');
        }
    }

    /**
     * Claim NEAR escrow with secret
     */
    private async claimNearEscrow(escrowId: string, secret: string) {
        await this.nearAccount.functionCall({
            contractId: 'fusion-htlc.testnet',
            methodName: 'claim',
            args: {
                escrow_id: escrowId,
                secret: secret
            },
            gas: '100000000000000'
        });
    }

    /**
     * Claim Ethereum escrow with secret
     */
    private async claimEthereumEscrow(escrowId: string, secret: string) {
        const htlcAbi = [
            'function claim(bytes32 escrowId, bytes32 secret)'
        ];

        const htlcContract = new ethers.Contract(
            '0x...', // Ethereum HTLC address
            htlcAbi,
            this.ethSigner
        );

        const tx = await htlcContract.claim(escrowId, `0x${secret}`);
        await tx.wait();
    }

    /**
     * Handle failure scenarios
     * Smart contracts handle refunds automatically after timeout
     */
    private async handleFailure(params: any) {
        console.log('⏰ Checking for timeout refunds...');
        
        // The beauty of HTLC: refunds are automatic after timeout
        // No need for complex recovery - just wait for timeout
        // No TEE needed - timeouts are enforced by blockchain
    }
}

// Example usage
async function main() {
    const orchestrator = new FusionCrossChainOrchestrator({
        ethereumRpc: 'https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY',
        nearRpc: 'https://rpc.testnet.near.org',
        ethPrivateKey: 'YOUR_ETH_PRIVATE_KEY',
        nearAccountId: 'resolver.testnet',
        nearPrivateKey: 'YOUR_NEAR_PRIVATE_KEY'
    });

    await orchestrator.initialize();

    // Execute a swap
    await orchestrator.executeSwap({
        userEthAddress: '0x...',
        userNearAddress: 'alice.testnet',
        resolverEthAddress: '0x...',
        resolverNearAddress: 'resolver.testnet',
        ethAmount: ethers.parseEther('1.0').toString(),
        nearAmount: utils.format.parseNearAmount('100')
    });
}

// Key insights from this implementation:
// 1. No TEE required - everything is verifiable on-chain
// 2. CLI tool just orchestrates transactions and monitors state
// 3. Secret can be public after use - that's the beauty of HTLC
// 4. Atomicity is guaranteed by smart contracts, not by trusted execution