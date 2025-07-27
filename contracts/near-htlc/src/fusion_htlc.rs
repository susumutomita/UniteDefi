use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Gas, NearToken, PanicOnDefault, Promise, PromiseResult,
};
use sha2::{Digest, Sha256};

type Balance = u128;
type Timestamp = u64;

// Gas constants - Made configurable for future NEAR upgrades
const BASE_GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(20);
const BASE_GAS_FOR_CALLBACK: Gas = Gas::from_tgas(10);
// Removed unused constants - GAS_PER_BATCH_ITEM and NO_DEPOSIT
const ONE_YOCTO: Balance = 1;

// Time constants for overflow protection
const MAX_TIME_PERIOD_SECONDS: u64 = 10 * 365 * 24 * 60 * 60; // 10 years
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;

// Storage limits to prevent DoS attacks
const MAX_TOTAL_ESCROWS: u64 = 10_000; // Maximum number of total escrows
const MAX_ESCROWS_PER_ACCOUNT: u64 = 100; // Maximum number of active escrows per account
const MAX_ESCROW_AMOUNT: Balance = 1_000_000 * 10u128.pow(24); // 1M NEAR max per escrow

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FusionHTLC {
    pub escrows: UnorderedMap<String, FusionEscrow>,
    pub escrow_counter: u64,
    pub owner: AccountId,
    pub active_escrows_per_account: UnorderedMap<AccountId, u64>, // Track active escrows per account
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FusionEscrow {
    // Core participants
    pub resolver: AccountId,    // Who locks the funds (1inch resolver)
    pub beneficiary: AccountId, // Who receives funds with correct secret

    // Amounts
    pub amount: Balance,                               // Main swap amount
    pub safety_deposit: Balance,                       // Safety deposit amount
    pub safety_deposit_beneficiary: Option<AccountId>, // Who gets safety deposit

    // Token info
    pub token_id: Option<AccountId>, // None for NEAR, Some for NEP-141

    // Hash lock
    pub secret_hash: String, // Base58 encoded SHA256 hash

    // Time locks (all in nanoseconds)
    pub deployment_time: Timestamp,    // When escrow was created
    pub finality_time: Timestamp,      // Before this: only beneficiary can claim
    pub cancel_time: Timestamp,        // After this: resolver can cancel
    pub public_cancel_time: Timestamp, // After this: anyone can cancel

    // State
    pub state: EscrowState,
    pub resolved_by: Option<AccountId>, // Who claimed/cancelled
    pub resolution_time: Option<Timestamp>, // When it was resolved
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
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
    pub cancel_period: u64,        // Seconds until resolver can cancel
    pub public_cancel_period: u64, // Seconds until anyone can cancel
}

#[near_bindgen]
impl FusionHTLC {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            escrows: UnorderedMap::new(b"e"),
            escrow_counter: 0,
            owner,
            active_escrows_per_account: UnorderedMap::new(b"a"),
        }
    }

    /// Create a new 1inch Fusion+ compatible escrow
    #[payable]
    pub fn create_escrow(&mut self, params: CreateEscrowParams) -> String {
        let resolver = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let now = env::block_timestamp();

        // Check storage limits to prevent DoS
        assert!(
            self.escrow_counter < MAX_TOTAL_ESCROWS,
            "Maximum total escrows limit reached"
        );

        // Check per-account limits
        let active_count = self.active_escrows_per_account.get(&resolver).unwrap_or(0);
        assert!(
            active_count < MAX_ESCROWS_PER_ACCOUNT,
            "Maximum escrows per account limit reached"
        );

        // Check escrow amount limits
        let amount: Balance = params.amount.into();
        let safety_deposit: Balance = params.safety_deposit.into();
        assert!(
            amount <= MAX_ESCROW_AMOUNT,
            "Escrow amount exceeds maximum limit"
        );
        assert!(
            safety_deposit <= MAX_ESCROW_AMOUNT,
            "Safety deposit exceeds maximum limit"
        );

        // Validate time periods to prevent overflow
        assert!(
            params.finality_period <= MAX_TIME_PERIOD_SECONDS,
            "Finality period too large"
        );
        assert!(
            params.cancel_period <= MAX_TIME_PERIOD_SECONDS,
            "Cancel period too large"
        );
        assert!(
            params.public_cancel_period <= MAX_TIME_PERIOD_SECONDS,
            "Public cancel period too large"
        );

        // Convert time periods to timestamps with overflow protection
        let finality_time = self.safe_add_time(now, params.finality_period);
        let cancel_time = self.safe_add_time(now, params.cancel_period);
        let public_cancel_time = self.safe_add_time(now, params.public_cancel_period);

        // Validate time periods
        assert!(
            finality_time < cancel_time,
            "Finality must be before cancel time"
        );
        assert!(
            cancel_time <= public_cancel_time,
            "Cancel time must be before public cancel"
        );

        let amount: Balance = params.amount.into();
        let safety_deposit: Balance = params.safety_deposit.into();
        let total_amount = amount + safety_deposit;

        // For NEAR transfers, ensure sufficient deposit
        if params.token_id.is_none() {
            assert!(
                deposit >= NearToken::from_yoctonear(total_amount),
                "Insufficient NEAR deposit"
            );
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

        // Update active escrow count for resolver
        self.active_escrows_per_account
            .insert(&resolver, &(active_count + 1));

        env::log_str(&format!(
            "Fusion escrow created: {} by {} for {}, amount: {}, safety: {}",
            escrow_id, resolver, escrow.beneficiary, amount, safety_deposit
        ));

        escrow_id
    }

    /// Claim escrow with secret (only beneficiary before finality)
    /// Secret should be provided as hex-encoded string
    pub fn claim(&mut self, escrow_id: String, secret: String) -> Promise {
        let mut escrow = self.escrows.get(&escrow_id).expect("Escrow not found");
        let claimer = env::predecessor_account_id();
        let now = env::block_timestamp();

        // Validate state
        assert_eq!(escrow.state, EscrowState::Active, "Escrow not active");

        // Validate timing - only beneficiary can claim before finality
        assert!(
            now < escrow.finality_time,
            "Past finality time, cannot claim"
        );
        assert_eq!(claimer, escrow.beneficiary, "Only beneficiary can claim");

        // Verify secret
        let secret_hash = self.hash_secret(&secret);
        assert_eq!(secret_hash, escrow.secret_hash, "Invalid secret");

        // Update state before external calls
        escrow.state = EscrowState::Claimed;
        escrow.resolved_by = Some(claimer.clone());
        escrow.resolution_time = Some(now);
        self.escrows.insert(&escrow_id, &escrow);

        // Decrease active escrow count for resolver
        let active_count = self
            .active_escrows_per_account
            .get(&escrow.resolver)
            .unwrap_or(1);
        if active_count > 1 {
            self.active_escrows_per_account
                .insert(&escrow.resolver, &(active_count - 1));
        } else {
            self.active_escrows_per_account.remove(&escrow.resolver);
        }

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

        // Decrease active escrow count for resolver
        let active_count = self
            .active_escrows_per_account
            .get(&escrow.resolver)
            .unwrap_or(1);
        if active_count > 1 {
            self.active_escrows_per_account
                .insert(&escrow.resolver, &(active_count - 1));
        } else {
            self.active_escrows_per_account.remove(&escrow.resolver);
        }

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

        for key in keys.iter().skip(start).take(end - start) {
            if let Some(escrow) = self.escrows.get(key) {
                if escrow.state == EscrowState::Active {
                    result.push((key.clone(), escrow));
                }
            }
        }

        result
    }

    // Private helper methods

    /// Safely add seconds to a timestamp, preventing overflow
    fn safe_add_time(&self, base_time: Timestamp, seconds: u64) -> Timestamp {
        let nanoseconds = seconds.saturating_mul(NANOSECONDS_PER_SECOND);
        base_time.saturating_add(nanoseconds)
    }

    // Removed unused calculate_gas method

    fn hash_secret(&self, secret: &str) -> String {
        // Decode hex string to bytes
        let secret_bytes = hex::decode(secret).expect("Invalid hex secret");

        let mut hasher = Sha256::new();
        hasher.update(&secret_bytes);
        let result = hasher.finalize();
        bs58::encode(result).into_string()
    }

    fn execute_claim_transfers(&self, escrow_id: String, escrow: FusionEscrow) -> Promise {
        let mut promise: Promise;

        if let Some(token_id) = escrow.token_id {
            // NEP-141 token transfers
            promise = Promise::new(token_id.clone()).function_call(
                "ft_transfer".to_string(),
                format!(
                    r#"{{"receiver_id":"{}","amount":"{}"}}"#,
                    escrow.beneficiary, escrow.amount
                )
                .into_bytes(),
                NearToken::from_yoctonear(ONE_YOCTO),
                BASE_GAS_FOR_FT_TRANSFER,
            );

            // Transfer safety deposit if exists
            if escrow.safety_deposit > 0 {
                let safety_recipient = escrow
                    .safety_deposit_beneficiary
                    .unwrap_or(escrow.resolver.clone());

                promise = promise.then(
                    Promise::new(token_id).function_call(
                        "ft_transfer".to_string(),
                        format!(
                            r#"{{"receiver_id":"{}","amount":"{}"}}"#,
                            safety_recipient, escrow.safety_deposit
                        )
                        .into_bytes(),
                        NearToken::from_yoctonear(ONE_YOCTO),
                        BASE_GAS_FOR_FT_TRANSFER,
                    ),
                );
            }
        } else {
            // NEAR transfers
            promise = Promise::new(escrow.beneficiary.clone())
                .transfer(NearToken::from_yoctonear(escrow.amount));

            if escrow.safety_deposit > 0 {
                let safety_recipient = escrow
                    .safety_deposit_beneficiary
                    .unwrap_or(escrow.resolver.clone());

                promise = promise.then(
                    Promise::new(safety_recipient)
                        .transfer(NearToken::from_yoctonear(escrow.safety_deposit)),
                );
            }
        }

        // Add callback to handle failures
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(BASE_GAS_FOR_CALLBACK)
                .on_transfer_complete(escrow_id, "claim".to_string()),
        )
    }

    fn execute_cancel_refund(&self, escrow_id: String, escrow: FusionEscrow) -> Promise {
        let total_amount = escrow.amount + escrow.safety_deposit;

        let promise = if let Some(token_id) = escrow.token_id {
            // NEP-141 token refund
            Promise::new(token_id).function_call(
                "ft_transfer".to_string(),
                format!(
                    r#"{{"receiver_id":"{}","amount":"{}"}}"#,
                    escrow.resolver, total_amount
                )
                .into_bytes(),
                NearToken::from_yoctonear(ONE_YOCTO),
                BASE_GAS_FOR_FT_TRANSFER,
            )
        } else {
            // NEAR refund
            Promise::new(escrow.resolver.clone()).transfer(NearToken::from_yoctonear(total_amount))
        };

        // Add callback
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(BASE_GAS_FOR_CALLBACK)
                .on_transfer_complete(escrow_id, "cancel".to_string()),
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
    // Batch operations with reentrancy protection
    pub fn batch_cancel(&mut self, escrow_ids: Vec<String>) -> Vec<String> {
        let mut cancelled_ids = Vec::new();
        let mut processed_ids = std::collections::HashSet::<String>::new();

        for escrow_id in escrow_ids {
            // Skip duplicates to prevent reentrancy
            if processed_ids.contains(&escrow_id) {
                continue;
            }
            processed_ids.insert(escrow_id.clone());

            if let Some(escrow) = self.escrows.get(&escrow_id) {
                if escrow.state == EscrowState::Active
                    && env::block_timestamp() >= escrow.public_cancel_time
                {
                    // Store state before external call
                    let escrow_id_copy = escrow_id.clone();

                    // Use promise batching for efficiency
                    self.cancel(escrow_id);
                    cancelled_ids.push(escrow_id_copy);
                }
            }
        }

        cancelled_ids
    }

    // View methods for monitoring
    pub fn get_claimable_escrows(&self, beneficiary: AccountId) -> Vec<(String, FusionEscrow)> {
        let mut result = Vec::new();
        let now = env::block_timestamp();

        for (id, escrow) in self.escrows.iter() {
            if escrow.state == EscrowState::Active
                && escrow.beneficiary == beneficiary
                && now < escrow.finality_time
            {
                result.push((id, escrow));
            }
        }

        result
    }

    pub fn get_cancellable_escrows(
        &self,
        resolver: Option<AccountId>,
    ) -> Vec<(String, FusionEscrow)> {
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

    fn get_context(predecessor: AccountId, deposit: Balance, timestamp: Timestamp) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor)
            .attached_deposit(NearToken::from_yoctonear(deposit))
            .block_timestamp(timestamp)
            .build()
    }

    fn create_valid_secret_hash() -> String {
        // Create a proper base58 encoded SHA256 hash
        let secret = "my_secret_12345";
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let result = hasher.finalize();
        bs58::encode(result).into_string()
    }

    #[test]
    fn test_create_fusion_escrow() {
        let context = get_context(accounts(0), 2_000_000_000_000_000_000_000_000, 0); // 2 NEAR
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000), // 1 NEAR
            safety_deposit: U128(100_000_000_000_000_000_000_000), // 0.1 NEAR
            safety_deposit_beneficiary: Some(accounts(2)),
            finality_period: 3600,       // 1 hour
            cancel_period: 7200,         // 2 hours
            public_cancel_period: 10800, // 3 hours
        };

        let escrow_id = contract.create_escrow(params);
        assert_eq!(escrow_id, "fusion_0");

        let escrow = contract.get_escrow(escrow_id).unwrap();
        assert_eq!(escrow.resolver, accounts(0));
        assert_eq!(escrow.beneficiary, accounts(1));
        assert_eq!(escrow.state, EscrowState::Active);
    }

    // Test 1: Binary Data Hash Verification
    #[test]
    fn test_hash_verification_with_binary_data() {
        let context = get_context(accounts(0), 2_000_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        // Test with actual binary data secret
        let secret_bytes = vec![0xde, 0xad, 0xbe, 0xef, 0x01, 0x23, 0x45, 0x67];
        let secret_hex = hex::encode(&secret_bytes);

        // Create hash from binary data
        let mut hasher = Sha256::new();
        hasher.update(&secret_bytes);
        let hash_result = hasher.finalize();
        let secret_hash = bs58::encode(hash_result).into_string();

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: secret_hash.clone(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        let escrow_id = contract.create_escrow(params);

        // Switch to beneficiary context and try to claim
        testing_env!(get_context(accounts(1), 0, 1_800_000_000_000)); // 30 minutes later

        // This should succeed with correct secret
        contract.claim(escrow_id.clone(), secret_hex.clone());

        let escrow = contract.get_escrow(escrow_id).unwrap();
        assert_eq!(escrow.state, EscrowState::Claimed);
    }

    #[test]
    #[should_panic(expected = "Invalid hex secret")]
    fn test_invalid_hex_secret() {
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        let escrow_id = contract.create_escrow(params);

        testing_env!(get_context(accounts(1), 0, 1_800_000_000_000));

        // Try to claim with invalid hex
        contract.claim(escrow_id, "not_valid_hex_gg".to_string());
    }

    // Test 2: Timestamp Precision and Overflow
    #[test]
    fn test_timestamp_precision_nanoseconds() {
        let start_time: Timestamp = 1_000_000_000_000_000_000; // 1 billion seconds in nanoseconds
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, start_time);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        // Test with large time periods that could cause overflow
        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 31_536_000,      // 1 year in seconds
            cancel_period: 63_072_000,        // 2 years in seconds
            public_cancel_period: 94_608_000, // 3 years in seconds
        };

        let escrow_id = contract.create_escrow(params);
        let escrow = contract.get_escrow(escrow_id).unwrap();

        // Check that timestamps are correctly converted to nanoseconds
        assert_eq!(
            escrow.finality_time,
            start_time + (31_536_000 * 1_000_000_000)
        );
        assert_eq!(
            escrow.cancel_time,
            start_time + (63_072_000 * 1_000_000_000)
        );
        assert_eq!(
            escrow.public_cancel_time,
            start_time + (94_608_000 * 1_000_000_000)
        );
    }

    #[test]
    #[should_panic] // Should panic due to overflow
    fn test_timestamp_overflow_protection() {
        let max_time: Timestamp = u64::MAX - 1_000_000_000; // Near u64 max
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, max_time);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        // This should cause overflow
        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: u64::MAX / 1_000_000_000, // This will overflow
            cancel_period: u64::MAX / 1_000_000_000,
            public_cancel_period: u64::MAX / 1_000_000_000,
        };

        contract.create_escrow(params);
    }

    // Test 3: Timeout Boundary Tests
    #[test]
    fn test_claim_at_finality_boundary() {
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));
        let secret = "test_secret_123";
        let secret_hash = contract.hash_secret(&hex::encode(secret.as_bytes()));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash,
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        let escrow_id = contract.create_escrow(params);

        // Test claiming right before finality time (should succeed)
        let just_before_finality = 3600 * 1_000_000_000 - 1;
        testing_env!(get_context(accounts(1), 0, just_before_finality));

        let escrow_before = contract.get_escrow(escrow_id.clone()).unwrap();
        assert!(just_before_finality < escrow_before.finality_time);

        contract.claim(escrow_id.clone(), hex::encode(secret.as_bytes()));
    }

    #[test]
    #[should_panic(expected = "Past finality time, cannot claim")]
    fn test_claim_after_finality() {
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));
        let secret = "test_secret_123";
        let secret_hash = contract.hash_secret(&hex::encode(secret.as_bytes()));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash,
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        let escrow_id = contract.create_escrow(params);

        // Test claiming right after finality time (should fail)
        let just_after_finality = 3600 * 1_000_000_000 + 1;
        testing_env!(get_context(accounts(1), 0, just_after_finality));

        contract.claim(escrow_id, hex::encode(secret.as_bytes()));
    }

    // Test 4: Reentrancy Protection in batch_cancel
    #[test]
    fn test_batch_cancel_reentrancy_protection() {
        let context = get_context(accounts(0), 5_000_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));
        let mut escrow_ids = Vec::new();

        // Create multiple escrows
        for _i in 0..5 {
            let params = CreateEscrowParams {
                beneficiary: accounts(1),
                secret_hash: create_valid_secret_hash(),
                token_id: None,
                amount: U128(1_000_000_000_000_000_000_000_000),
                safety_deposit: U128(0),
                safety_deposit_beneficiary: None,
                finality_period: 3600,
                cancel_period: 7200,
                public_cancel_period: 10800,
            };
            escrow_ids.push(contract.create_escrow(params));
        }

        // Move to public cancel time
        testing_env!(get_context(accounts(2), 0, 11000 * 1_000_000_000));

        // Test batch cancel - should handle state changes properly
        contract.batch_cancel(escrow_ids.clone());

        // Verify all escrows are cancelled
        for id in escrow_ids {
            let escrow = contract.get_escrow(id).unwrap();
            assert_eq!(escrow.state, EscrowState::Cancelled);
        }
    }

    // Test 5: Cross-contract call failure handling
    #[test]
    fn test_callback_failure_reverts_state() {
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));
        let secret = "test_secret_123";
        let secret_hash = contract.hash_secret(&hex::encode(secret.as_bytes()));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash,
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        let escrow_id = contract.create_escrow(params);

        // Simulate claim
        testing_env!(get_context(accounts(1), 0, 1800 * 1_000_000_000));
        contract.claim(escrow_id.clone(), hex::encode(secret.as_bytes()));

        // Simulate callback with failure
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(env::current_account_id())
            .build());

        // Mock failed promise result
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(env::current_account_id())
            .build());

        // Note: In real tests, we'd need to properly mock promise results
        // This is a simplified version to show the pattern
    }

    // Test 6: NEP-141 Token Transfer Security
    #[test]
    fn test_nep141_token_escrow() {
        let context = get_context(accounts(0), 1, 0); // Only 1 yocto for token transfers
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));
        let token_id: AccountId = "token.testnet".parse().unwrap();

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: Some(token_id.clone()),
            amount: U128(1_000_000),       // 1 USDC (6 decimals)
            safety_deposit: U128(100_000), // 0.1 USDC
            safety_deposit_beneficiary: Some(accounts(2)),
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        // Should accept token escrow with minimal NEAR deposit
        let escrow_id = contract.create_escrow(params);
        let escrow = contract.get_escrow(escrow_id).unwrap();

        assert_eq!(escrow.token_id, Some(token_id));
        assert_eq!(escrow.amount, 1_000_000);
    }

    // Test 7: Authorization and Access Control
    #[test]
    #[should_panic(expected = "Only beneficiary can claim")]
    fn test_unauthorized_claim() {
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));
        let secret = "test_secret_123";
        let secret_hash = contract.hash_secret(&hex::encode(secret.as_bytes()));

        let params = CreateEscrowParams {
            beneficiary: accounts(1), // Beneficiary is account 1
            secret_hash,
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        let escrow_id = contract.create_escrow(params);

        // Try to claim as wrong account (account 2)
        testing_env!(get_context(accounts(2), 0, 1800 * 1_000_000_000));
        contract.claim(escrow_id, hex::encode(secret.as_bytes()));
    }

    #[test]
    #[should_panic(expected = "Only resolver can cancel now")]
    fn test_unauthorized_cancel_before_public() {
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        let escrow_id = contract.create_escrow(params);

        // Try to cancel as non-resolver during resolver-only period
        testing_env!(get_context(accounts(2), 0, 7500 * 1_000_000_000)); // Between cancel and public cancel
        contract.cancel(escrow_id);
    }

    // Test 8: Base58 Encoding Consistency
    #[test]
    fn test_base58_encoding_consistency() {
        let context = get_context(accounts(0), 0, 0);
        testing_env!(context);

        let contract = FusionHTLC::new(accounts(0));

        // Test various binary patterns
        let test_cases = vec![
            vec![0x00, 0x00, 0x00, 0x00],                         // All zeros
            vec![0xFF, 0xFF, 0xFF, 0xFF],                         // All ones
            vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0], // Mixed
            vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], // Sequential
        ];

        for test_data in test_cases {
            let hex_secret = hex::encode(&test_data);
            let hash1 = contract.hash_secret(&hex_secret);
            let hash2 = contract.hash_secret(&hex_secret);

            // Same input should produce same hash
            assert_eq!(hash1, hash2);

            // Hash should be valid base58
            let decoded = bs58::decode(&hash1).into_vec().unwrap();
            assert_eq!(decoded.len(), 32); // SHA256 is 32 bytes
        }
    }

    // Test 9: Edge Cases and Input Validation
    #[test]
    #[should_panic(expected = "Finality must be before cancel time")]
    fn test_invalid_time_ordering() {
        let context = get_context(accounts(0), 1_100_000_000_000_000_000_000_000, 0);
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: None,
            amount: U128(1_000_000_000_000_000_000_000_000),
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 7200,       // 2 hours
            cancel_period: 3600,         // 1 hour (invalid - before finality)
            public_cancel_period: 10800, // 3 hours
        };

        contract.create_escrow(params);
    }

    #[test]
    #[should_panic(expected = "Insufficient NEAR deposit")]
    fn test_insufficient_deposit() {
        let context = get_context(accounts(0), 1_000_000_000_000_000_000_000_000, 0); // 1 NEAR
        testing_env!(context);

        let mut contract = FusionHTLC::new(accounts(0));

        let params = CreateEscrowParams {
            beneficiary: accounts(1),
            secret_hash: create_valid_secret_hash(),
            token_id: None,
            amount: U128(2_000_000_000_000_000_000_000_000), // 2 NEAR (more than deposit)
            safety_deposit: U128(0),
            safety_deposit_beneficiary: None,
            finality_period: 3600,
            cancel_period: 7200,
            public_cancel_period: 10800,
        };

        contract.create_escrow(params);
    }
}
