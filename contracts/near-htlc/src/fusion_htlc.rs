use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise, PromiseResult, Timestamp};
use sha2::{Digest, Sha256};

// Gas constants
const GAS_FOR_FT_TRANSFER: Gas = Gas(20_000_000_000_000);
const GAS_FOR_CALLBACK: Gas = Gas(10_000_000_000_000);
const NO_DEPOSIT: Balance = 0;
const ONE_YOCTO: Balance = 1;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FusionHTLC {
    pub escrows: UnorderedMap<String, FusionEscrow>,
    pub escrow_counter: u64,
    pub owner: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FusionEscrow {
    // Core participants
    pub resolver: AccountId,              // Who locks the funds (1inch resolver)
    pub beneficiary: AccountId,           // Who receives funds with correct secret
    
    // Amounts
    pub amount: Balance,                  // Main swap amount
    pub safety_deposit: Balance,          // Safety deposit amount
    pub safety_deposit_beneficiary: Option<AccountId>, // Who gets safety deposit
    
    // Token info
    pub token_id: Option<AccountId>,      // None for NEAR, Some for NEP-141
    
    // Hash lock
    pub secret_hash: String,              // Base58 encoded SHA256 hash
    
    // Time locks (all in nanoseconds)
    pub deployment_time: Timestamp,       // When escrow was created
    pub finality_time: Timestamp,         // Before this: only beneficiary can claim
    pub cancel_time: Timestamp,           // After this: resolver can cancel
    pub public_cancel_time: Timestamp,    // After this: anyone can cancel
    
    // State
    pub state: EscrowState,
    pub resolved_by: Option<AccountId>,   // Who claimed/cancelled
    pub resolution_time: Option<Timestamp>, // When it was resolved
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum EscrowState {
    Active,
    Claimed,
    Cancelled,
    Refunded,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateEscrowParams {
    pub beneficiary: AccountId,
    pub secret_hash: String,
    pub token_id: Option<AccountId>,
    pub amount: U128,
    pub safety_deposit: U128,
    pub safety_deposit_beneficiary: Option<AccountId>,
    pub finality_period: u64,      // Seconds until finality lock
    pub cancel_period: u64,         // Seconds until resolver can cancel
    pub public_cancel_period: u64,  // Seconds until anyone can cancel
}

#[near_bindgen]
impl FusionHTLC {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            escrows: UnorderedMap::new(b"e"),
            escrow_counter: 0,
            owner,
        }
    }

    /// Create a new 1inch Fusion+ compatible escrow
    #[payable]
    pub fn create_escrow(&mut self, params: CreateEscrowParams) -> String {
        let resolver = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let now = env::block_timestamp();
        
        // Convert time periods to timestamps
        let finality_time = now + (params.finality_period * 1_000_000_000);
        let cancel_time = now + (params.cancel_period * 1_000_000_000);
        let public_cancel_time = now + (params.public_cancel_period * 1_000_000_000);
        
        // Validate time periods
        assert!(finality_time < cancel_time, "Finality must be before cancel time");
        assert!(cancel_time <= public_cancel_time, "Cancel time must be before public cancel");
        
        let amount: Balance = params.amount.into();
        let safety_deposit: Balance = params.safety_deposit.into();
        let total_amount = amount + safety_deposit;
        
        // For NEAR transfers, ensure sufficient deposit
        if params.token_id.is_none() {
            assert!(deposit >= total_amount, "Insufficient NEAR deposit");
        }
        
        let escrow_id = format!("fusion_{}", self.escrow_counter);
        self.escrow_counter += 1;
        
        let escrow = FusionEscrow {
            resolver: resolver.clone(),
            beneficiary: params.beneficiary,
            amount,
            safety_deposit,
            safety_deposit_beneficiary: params.safety_deposit_beneficiary,
            token_id: params.token_id,
            secret_hash: params.secret_hash,
            deployment_time: now,
            finality_time,
            cancel_time,
            public_cancel_time,
            state: EscrowState::Active,
            resolved_by: None,
            resolution_time: None,
        };
        
        self.escrows.insert(&escrow_id, &escrow);
        
        env::log_str(&format!(
            "Fusion escrow created: {} by {} for {}, amount: {}, safety: {}",
            escrow_id, resolver, escrow.beneficiary, amount, safety_deposit
        ));
        
        escrow_id
    }

    /// Claim escrow with secret (only beneficiary before finality)
    pub fn claim(&mut self, escrow_id: String, secret: String) -> Promise {
        let mut escrow = self.escrows.get(&escrow_id).expect("Escrow not found");
        let claimer = env::predecessor_account_id();
        let now = env::block_timestamp();
        
        // Validate state
        assert_eq!(escrow.state, EscrowState::Active, "Escrow not active");
        
        // Validate timing - only beneficiary can claim before finality
        assert!(now < escrow.finality_time, "Past finality time, cannot claim");
        assert_eq!(claimer, escrow.beneficiary, "Only beneficiary can claim");
        
        // Verify secret
        let secret_hash = self.hash_secret(&secret);
        assert_eq!(secret_hash, escrow.secret_hash, "Invalid secret");
        
        // Update state before external calls
        escrow.state = EscrowState::Claimed;
        escrow.resolved_by = Some(claimer.clone());
        escrow.resolution_time = Some(now);
        self.escrows.insert(&escrow_id, &escrow);
        
        // Store secret for cross-chain verification
        env::log_str(&format!("Secret revealed: {}", secret));
        
        // Execute transfers
        self.execute_claim_transfers(escrow_id, escrow)
    }

    /// Cancel escrow (resolver after cancel_time, anyone after public_cancel_time)
    pub fn cancel(&mut self, escrow_id: String) -> Promise {
        let mut escrow = self.escrows.get(&escrow_id).expect("Escrow not found");
        let canceller = env::predecessor_account_id();
        let now = env::block_timestamp();
        
        // Validate state
        assert_eq!(escrow.state, EscrowState::Active, "Escrow not active");
        
        // Validate timing and permissions
        if now >= escrow.public_cancel_time {
            // Anyone can cancel
        } else if now >= escrow.cancel_time {
            // Only resolver can cancel
            assert_eq!(canceller, escrow.resolver, "Only resolver can cancel now");
        } else {
            panic!("Too early to cancel");
        }
        
        // Update state before external calls
        escrow.state = EscrowState::Cancelled;
        escrow.resolved_by = Some(canceller.clone());
        escrow.resolution_time = Some(now);
        self.escrows.insert(&escrow_id, &escrow);
        
        // Execute refund
        self.execute_cancel_refund(escrow_id, escrow)
    }

    /// Get escrow details
    pub fn get_escrow(&self, escrow_id: String) -> Option<FusionEscrow> {
        self.escrows.get(&escrow_id)
    }

    /// Get all active escrows
    pub fn get_active_escrows(&self, from_index: u64, limit: u64) -> Vec<(String, FusionEscrow)> {
        let mut result = Vec::new();
        let keys: Vec<String> = self.escrows.keys_as_vector().iter().collect();
        
        let start = from_index as usize;
        let end = std::cmp::min(start + limit as usize, keys.len());
        
        for i in start..end {
            if let Some(escrow) = self.escrows.get(&keys[i]) {
                if escrow.state == EscrowState::Active {
                    result.push((keys[i].clone(), escrow));
                }
            }
        }
        
        result
    }

    // Private helper methods

    fn hash_secret(&self, secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let result = hasher.finalize();
        bs58::encode(result).into_string()
    }

    fn execute_claim_transfers(&self, escrow_id: String, escrow: FusionEscrow) -> Promise {
        let mut promise = Promise::new(env::current_account_id());
        
        if let Some(token_id) = escrow.token_id {
            // NEP-141 token transfers
            promise = Promise::new(token_id.clone())
                .function_call(
                    "ft_transfer".to_string(),
                    format!(
                        r#"{{"receiver_id":"{}","amount":"{}"}}"#,
                        escrow.beneficiary, escrow.amount
                    ).into_bytes(),
                    ONE_YOCTO,
                    GAS_FOR_FT_TRANSFER,
                );
            
            // Transfer safety deposit if exists
            if escrow.safety_deposit > 0 {
                let safety_recipient = escrow.safety_deposit_beneficiary
                    .unwrap_or(escrow.resolver.clone());
                
                promise = promise.then(
                    Promise::new(token_id)
                        .function_call(
                            "ft_transfer".to_string(),
                            format!(
                                r#"{{"receiver_id":"{}","amount":"{}"}}"#,
                                safety_recipient, escrow.safety_deposit
                            ).into_bytes(),
                            ONE_YOCTO,
                            GAS_FOR_FT_TRANSFER,
                        )
                );
            }
        } else {
            // NEAR transfers
            promise = Promise::new(escrow.beneficiary.clone())
                .transfer(escrow.amount);
            
            if escrow.safety_deposit > 0 {
                let safety_recipient = escrow.safety_deposit_beneficiary
                    .unwrap_or(escrow.resolver.clone());
                
                promise = promise.then(
                    Promise::new(safety_recipient).transfer(escrow.safety_deposit)
                );
            }
        }
        
        // Add callback to handle failures
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_CALLBACK)
                .on_transfer_complete(escrow_id, "claim".to_string())
        )
    }

    fn execute_cancel_refund(&self, escrow_id: String, escrow: FusionEscrow) -> Promise {
        let total_amount = escrow.amount + escrow.safety_deposit;
        
        let promise = if let Some(token_id) = escrow.token_id {
            // NEP-141 token refund
            Promise::new(token_id)
                .function_call(
                    "ft_transfer".to_string(),
                    format!(
                        r#"{{"receiver_id":"{}","amount":"{}"}}"#,
                        escrow.resolver, total_amount
                    ).into_bytes(),
                    ONE_YOCTO,
                    GAS_FOR_FT_TRANSFER,
                )
        } else {
            // NEAR refund
            Promise::new(escrow.resolver.clone()).transfer(total_amount)
        };
        
        // Add callback
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_CALLBACK)
                .on_transfer_complete(escrow_id, "cancel".to_string())
        )
    }

    #[private]
    pub fn on_transfer_complete(&mut self, escrow_id: String, operation: String) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                env::log_str(&format!(
                    "Transfer completed successfully for {} operation on escrow {}",
                    operation, escrow_id
                ));
            }
            PromiseResult::Failed => {
                // Revert state on failure
                if let Some(mut escrow) = self.escrows.get(&escrow_id) {
                    escrow.state = EscrowState::Active;
                    escrow.resolved_by = None;
                    escrow.resolution_time = None;
                    self.escrows.insert(&escrow_id, &escrow);
                    
                    env::log_str(&format!(
                        "Transfer failed for {} operation on escrow {}, reverted to active",
                        operation, escrow_id
                    ));
                }
            }
        }
    }
}

// Extension trait for cross-contract calls
#[near_bindgen]
impl FusionHTLC {
    // Batch operations for efficiency
    pub fn batch_cancel(&mut self, escrow_ids: Vec<String>) {
        for escrow_id in escrow_ids {
            if let Some(escrow) = self.escrows.get(&escrow_id) {
                if escrow.state == EscrowState::Active && 
                   env::block_timestamp() >= escrow.public_cancel_time {
                    self.cancel(escrow_id);
                }
            }
        }
    }
    
    // View methods for monitoring
    pub fn get_claimable_escrows(&self, beneficiary: AccountId) -> Vec<(String, FusionEscrow)> {
        let mut result = Vec::new();
        let now = env::block_timestamp();
        
        for (id, escrow) in self.escrows.iter() {
            if escrow.state == EscrowState::Active &&
               escrow.beneficiary == beneficiary &&
               now < escrow.finality_time {
                result.push((id, escrow));
            }
        }
        
        result
    }
    
    pub fn get_cancellable_escrows(&self, resolver: Option<AccountId>) -> Vec<(String, FusionEscrow)> {
        let mut result = Vec::new();
        let now = env::block_timestamp();
        
        for (id, escrow) in self.escrows.iter() {
            if escrow.state == EscrowState::Active {
                if now >= escrow.public_cancel_time {
                    result.push((id, escrow));
                } else if now >= escrow.cancel_time {
                    if let Some(ref r) = resolver {
                        if &escrow.resolver == r {
                            result.push((id, escrow));
                        }
                    }
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor: AccountId, deposit: Balance) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor)
            .attached_deposit(deposit)
            .block_timestamp(0)
            .build()
    }

    #[test]
    fn test_create_fusion_escrow() {
        let context = get_context(accounts(0), 2_000_000_000_000_000_000_000_000); // 2 NEAR
        testing_env!(context);
        
        let mut contract = FusionHTLC::new(accounts(0));
        
        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: "test_hash".to_string(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000), // 1 NEAR
            safety_deposit: U128(100_000_000_000_000_000_000_000), // 0.1 NEAR
            safety_deposit_beneficiary: Some(accounts(2)),
            finality_period: 3600,      // 1 hour
            cancel_period: 7200,        // 2 hours
            public_cancel_period: 10800, // 3 hours
        };
        
        let escrow_id = contract.create_escrow(params);
        assert_eq!(escrow_id, "fusion_0");
        
        let escrow = contract.get_escrow(escrow_id).unwrap();
        assert_eq!(escrow.resolver, accounts(0));
        assert_eq!(escrow.beneficiary, accounts(1));
        assert_eq!(escrow.state, EscrowState::Active);
    }
}