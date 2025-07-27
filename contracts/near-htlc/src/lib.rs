use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Timestamp};
use sha2::{Digest, Sha256};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct HTLCContract {
    pub escrows: UnorderedMap<String, Escrow>,
    pub escrow_counter: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Escrow {
    pub sender: AccountId,
    pub recipient: AccountId,
    pub amount: Balance,
    pub token_id: Option<AccountId>, // None for NEAR, Some for NEP-141
    pub secret_hash: String,         // Base58 encoded
    pub timeout: Timestamp,
    pub state: EscrowState,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum EscrowState {
    Pending,
    Claimed,
    Refunded,
}

#[near_bindgen]
impl HTLCContract {
    #[init]
    pub fn new() -> Self {
        Self {
            escrows: UnorderedMap::new(b"e"),
            escrow_counter: 0,
        }
    }

    /// Create a new HTLC escrow
    #[payable]
    pub fn create_escrow(
        &mut self,
        recipient: AccountId,
        secret_hash: String,
        timeout_seconds: u64,
    ) -> String {
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Deposit required");
        
        let timeout = env::block_timestamp() + (timeout_seconds * 1_000_000_000); // Convert to nanoseconds
        
        let escrow_id = format!("escrow_{}", self.escrow_counter);
        self.escrow_counter += 1;
        
        let escrow = Escrow {
            sender: env::predecessor_account_id(),
            recipient,
            amount: deposit,
            token_id: None, // NEAR transfer
            secret_hash,
            timeout,
            state: EscrowState::Pending,
        };
        
        self.escrows.insert(&escrow_id, &escrow);
        
        env::log_str(&format!(
            "Escrow created: {} NEAR from {} to {}, timeout: {}",
            deposit, escrow.sender, escrow.recipient, timeout
        ));
        
        escrow_id
    }

    /// Claim the escrow with the secret
    pub fn claim(&mut self, escrow_id: String, secret: String) {
        let mut escrow = self.escrows.get(&escrow_id).expect("Escrow not found");
        
        assert_eq!(escrow.state, EscrowState::Pending, "Escrow not pending");
        assert!(env::block_timestamp() < escrow.timeout, "Escrow expired");
        
        // Verify secret
        let secret_hash = self.hash_secret(&secret);
        assert_eq!(secret_hash, escrow.secret_hash, "Invalid secret");
        
        // Update state
        escrow.state = EscrowState::Claimed;
        self.escrows.insert(&escrow_id, &escrow);
        
        // Transfer to recipient
        Promise::new(escrow.recipient.clone()).transfer(escrow.amount);
        
        env::log_str(&format!(
            "Escrow claimed: {} NEAR to {}",
            escrow.amount, escrow.recipient
        ));
    }

    /// Refund the escrow after timeout
    pub fn refund(&mut self, escrow_id: String) {
        let mut escrow = self.escrows.get(&escrow_id).expect("Escrow not found");
        
        assert_eq!(escrow.state, EscrowState::Pending, "Escrow not pending");
        assert!(env::block_timestamp() >= escrow.timeout, "Escrow not expired");
        
        // Update state
        escrow.state = EscrowState::Refunded;
        self.escrows.insert(&escrow_id, &escrow);
        
        // Transfer back to sender
        Promise::new(escrow.sender.clone()).transfer(escrow.amount);
        
        env::log_str(&format!(
            "Escrow refunded: {} NEAR to {}",
            escrow.amount, escrow.sender
        ));
    }

    /// Get escrow details
    pub fn get_escrow(&self, escrow_id: String) -> Option<Escrow> {
        self.escrows.get(&escrow_id)
    }

    /// Helper function to hash a secret
    fn hash_secret(&self, secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let result = hasher.finalize();
        bs58::encode(result).into_string()
    }
}

// Import Promise for transfers
use near_sdk::Promise;

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor: AccountId, deposit: Balance) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor)
            .attached_deposit(deposit)
            .build()
    }

    #[test]
    fn test_create_escrow() {
        let mut context = get_context(accounts(0), 1_000_000_000_000_000_000_000_000); // 1 NEAR
        testing_env!(context.clone());
        
        let mut contract = HTLCContract::new();
        let escrow_id = contract.create_escrow(
            accounts(1),
            "test_hash".to_string(),
            3600, // 1 hour
        );
        
        assert_eq!(escrow_id, "escrow_0");
        let escrow = contract.get_escrow(escrow_id).unwrap();
        assert_eq!(escrow.sender, accounts(0));
        assert_eq!(escrow.recipient, accounts(1));
        assert_eq!(escrow.state, EscrowState::Pending);
    }
}