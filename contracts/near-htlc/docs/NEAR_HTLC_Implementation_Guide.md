# NEAR HTLC Implementation Guide for 1inch Fusion+ Cross-Chain Swap

## Executive Summary

This guide provides comprehensive technical details for implementing Hash Time Locked Contracts (HTLC) on NEAR Protocol to support 1inch Fusion+ cross-chain swaps. NEAR's unique architecture requires specific approaches different from EVM chains.

## 1. NEAR Smart Contract Development

### Language Selection: Rust (Recommended)

Why Rust over AssemblyScript:
- Financial applications require maximum safety guarantees
- Rust offers memory safety and zero-cost abstractions
- More mature toolchain and ecosystem
- NEAR itself is written in Rust
- Better suited for production-grade financial contracts

AssemblyScript Alternative:
- Only suitable for non-financial use cases or prototypes
- Easier learning curve (hours vs weeks)
- Limited safety guarantees compared to Rust

### Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install NEAR CLI
npm install -g near-cli

# Install cargo-near
cargo install cargo-near
```

## 2. NEAR Account Model vs Ethereum

### Key Differences

| Feature | NEAR | Ethereum |
|---------|------|----------|
| Account IDs | Human-readable (alice.near) | Hex addresses (0x123...) |
| Access Keys | Multiple keys with permissions | Single private key |
| Storage | Requires staking (1 NEAR per 100KB) | Gas covers storage |
| Function Calls | Async with Promises | Synchronous |
| Account Creation | Requires funding | Free (just generate keys) |

### Access Key System

NEAR's multi-key architecture provides enhanced security:

1. Full Access Keys: Complete account control
2. Function Call Keys: Limited permissions
   - Specific contract/methods
   - Gas allowance limits
   - Cannot transfer tokens

Example Function Call Key:
```rust
pub struct FunctionCallPermission {
    allowance: Option<Balance>,
    receiver_id: AccountId,
    method_names: Vec<String>,
}
```

## 3. HTLC Implementation on NEAR

### Core Contract Structure (Existing)

```rust
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, Timestamp};
use sha2::{Digest, Sha256};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct HTLCContract {
    pub escrows: UnorderedMap<String, Escrow>,
    pub escrow_counter: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Escrow {
    pub sender: AccountId,
    pub recipient: AccountId,
    pub amount: Balance,
    pub token_id: Option<AccountId>, // NEP-141 token contract
    pub secret_hash: String,
    pub timeout: Timestamp,
    pub state: EscrowState,
}
```

### Enhanced Implementation for 1inch Fusion+

```rust
// Additional fields needed for 1inch Fusion+ compatibility
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct FusionEscrow {
    // Basic HTLC fields
    pub resolver: AccountId,           // Who locked the funds
    pub beneficiary: AccountId,        // Who can claim with secret
    pub amount: Balance,
    pub token_id: Option<AccountId>,

    // Hash lock
    pub secret_hash: String,           // SHA256 hash

    // Time locks (1inch Fusion+ specific)
    pub deployment_time: Timestamp,
    pub finality_lock: Timestamp,      // Before this: only beneficiary can claim
    pub cancel_time: Timestamp,        // After this: resolver can cancel
    pub public_cancel_time: Timestamp, // After this: anyone can cancel

    // Safety deposit
    pub safety_deposit: Balance,
    pub safety_deposit_beneficiary: Option<AccountId>,

    // State
    pub state: EscrowState,
    pub claimed_by: Option<AccountId>,
}

// Time lock validation
impl FusionEscrow {
    pub fn can_claim(&self, claimer: &AccountId) -> bool {
        let now = env::block_timestamp();
        match self.state {
            EscrowState::Pending => {
                if now < self.finality_lock {
                    // Only beneficiary can claim
                    claimer == &self.beneficiary
                } else {
                    false // Past finality lock, no claims allowed
                }
            }
            _ => false,
        }
    }

    pub fn can_cancel(&self, canceller: &AccountId) -> bool {
        let now = env::block_timestamp();
        match self.state {
            EscrowState::Pending => {
                if now >= self.public_cancel_time {
                    true // Anyone can cancel
                } else if now >= self.cancel_time {
                    canceller == &self.resolver // Only resolver can cancel
                } else {
                    false // Too early to cancel
                }
            }
            _ => false,
        }
    }
}
```

## 4. Cross-Contract Calls and Promise Patterns

### Asynchronous Nature

NEAR uses Promises for cross-contract calls, which execute asynchronously:

```rust
// Transfer NEAR tokens
Promise::new(recipient).transfer(amount);

// Transfer NEP-141 tokens
Promise::new(token_contract)
    .function_call(
        "ft_transfer",
        json!({
            "receiver_id": recipient,
            "amount": amount.to_string(),
        }).to_string().as_bytes(),
        1, // 1 yoctoNEAR attached
        5_000_000_000_000 // 5 TGas
    );
```

### Callback Pattern for Error Handling

```rust
#[near_bindgen]
impl HTLCContract {
    pub fn claim_with_callback(&mut self, escrow_id: String, secret: String) {
        // Validate and update state
        let escrow = self.validate_claim(escrow_id.clone(), secret);

        // Execute transfer with callback
        Promise::new(escrow.token_id.unwrap())
            .function_call(
                "ft_transfer",
                json!({
                    "receiver_id": escrow.beneficiary,
                    "amount": escrow.amount.to_string(),
                }).to_string().as_bytes(),
                1,
                20_000_000_000_000
            )
            .then(
                Self::ext(env::current_account_id())
                    .on_transfer_complete(escrow_id)
            );
    }

    #[private]
    pub fn on_transfer_complete(&mut self, escrow_id: String) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                // Transfer succeeded, finalize
                env::log_str("Transfer completed successfully");
            }
            PromiseResult::Failed => {
                // Transfer failed, revert state
                let mut escrow = self.escrows.get(&escrow_id).unwrap();
                escrow.state = EscrowState::Pending;
                self.escrows.insert(&escrow_id, &escrow);
                env::log_str("Transfer failed, escrow reverted to pending");
            }
        }
    }
}
```

## 5. Storage Staking and Gas Economics

### Storage Costs
- 1 NEAR per 100KB of storage
- Storage stake is locked, not consumed
- Released when data is deleted

### Gas Costs
- Maximum 300 TGas per transaction (~300ms compute time)
- Gas price dynamically adjusted by protocol
- 30% of gas fees go to contract developer
- 70% is burned

### Optimization Strategies

```rust
// Efficient storage structure
#[derive(BorshSerialize, BorshDeserialize)]
pub struct CompactEscrow {
    // Pack data efficiently
    pub parties: [AccountId; 2], // sender, recipient
    pub amounts: [u128; 2],       // amount, safety_deposit
    pub timestamps: [u64; 4],     // deployment, finality, cancel, public_cancel
    pub secret_hash: [u8; 32],    // Fixed size instead of String
    pub state: u8,                // Enum as u8
}
```

## 6. Timeout and Refund Handling

### Manual State Management

NEAR doesn't automatically rollback state on failures:

```rust
impl HTLCContract {
    pub fn handle_timeout(&mut self, escrow_id: String) {
        let escrow = self.escrows.get(&escrow_id).expect("Escrow not found");

        // Check timeout conditions
        let now = env::block_timestamp();
        assert!(now >= escrow.timeout, "Not yet timed out");
        assert_eq!(escrow.state, EscrowState::Pending, "Invalid state");

        // Update state first
        let mut escrow = escrow;
        escrow.state = EscrowState::Refunded;
        self.escrows.insert(&escrow_id, &escrow);

        // Execute refund
        if let Some(token_id) = &escrow.token_id {
            // NEP-141 token refund
            Promise::new(token_id.clone())
                .function_call(
                    "ft_transfer",
                    json!({
                        "receiver_id": escrow.sender,
                        "amount": escrow.amount.to_string(),
                    }).to_string().as_bytes(),
                    1,
                    20_000_000_000_000
                );
        } else {
            // NEAR refund
            Promise::new(escrow.sender.clone()).transfer(escrow.amount);
        }
    }
}
```

## 7. NEAR Testnet Deployment

### Account Creation
```bash
# Create testnet account
near create-account htlc.testnet --useFaucet

# Or use wallet
# Visit https://wallet.testnet.near.org
```

### Build and Deploy
```bash
# Build contract
cargo near build

# Deploy
near deploy htlc.testnet ./target/near/htlc.wasm

# Initialize (if needed)
near call htlc.testnet new '{}' --accountId htlc.testnet
```

### Testing
```bash
# Create escrow
near call htlc.testnet create_escrow '{
    "recipient": "bob.testnet",
    "secret_hash": "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae",
    "timeout_seconds": 3600
}' --accountId alice.testnet --deposit 1

# Claim with secret
near call htlc.testnet claim '{
    "escrow_id": "escrow_0",
    "secret": "foo"
}' --accountId bob.testnet
```

## 8. Cross-Chain Integration Patterns

### Option 1: Direct Implementation
- Implement HTLC on both chains
- Use relayer service to coordinate
- Manual secret revelation

### Option 2: Rainbow Bridge Integration
- Use existing bridge infrastructure
- Higher latency (6 min ETH→NEAR, 16 hrs NEAR→ETH)
- More expensive (~$10-60 per transfer)

### Option 3: Hybrid Approach
- HTLC for atomic swap logic
- Rainbow Bridge for final settlement
- Best of both worlds

## 9. Security Considerations

### NEAR-Specific Risks

1. Storage Attacks: Malicious users can inflate storage costs
   - Solution: Require storage deposit from users

2. Gas Exhaustion: Complex operations may exceed 300 TGas
   - Solution: Split operations across multiple transactions

3. Callback Manipulation: Ensure callbacks are private
   - Solution: Use `#[private]` attribute

4. Front-running: NEAR has deterministic ordering
   - Solution: Use commit-reveal patterns for sensitive operations

### Best Practices

```rust
// Require storage deposit
#[payable]
pub fn create_escrow_with_storage(&mut self, params: CreateEscrowParams) {
    let storage_cost = self.calculate_storage_cost();
    assert!(
        env::attached_deposit() >= params.amount + storage_cost,
        "Insufficient deposit for amount + storage"
    );
    // ... create escrow
}

// Prevent reentrancy
pub fn claim(&mut self, escrow_id: String, secret: String) {
    // Update state BEFORE external calls
    let mut escrow = self.escrows.get(&escrow_id).unwrap();
    assert_eq!(escrow.state, EscrowState::Pending);
    escrow.state = EscrowState::Claimed;
    self.escrows.insert(&escrow_id, &escrow);

    // Then make external calls
    Promise::new(escrow.beneficiary).transfer(escrow.amount);
}
```

## 10. Implementation Roadmap

### Phase 1: Basic HTLC (Completed)
- ✅ Basic escrow creation
- ✅ Secret validation
- ✅ Timeout handling
- ✅ NEAR token support

### Phase 2: 1inch Fusion+ Features
- [ ] Multiple time lock periods
- [ ] Safety deposit mechanism
- [ ] NEP-141 token support
- [ ] Batch operations

### Phase 3: Cross-Chain Integration
- [ ] Relayer service
- [ ] Secret coordination
- [ ] Multi-chain monitoring
- [ ] Emergency procedures

### Phase 4: Production Readiness
- [ ] Comprehensive testing
- [ ] Security audit
- [ ] Gas optimization
- [ ] Mainnet deployment

## Conclusion

Implementing HTLC on NEAR for 1inch Fusion+ requires understanding NEAR's unique architecture:
- Asynchronous execution model
- Manual state management
- Storage staking requirements
- Promise-based cross-contract calls

The existing implementation provides a solid foundation, but needs enhancement for full 1inch Fusion+ compatibility, particularly around multiple timeout periods and safety deposits.
