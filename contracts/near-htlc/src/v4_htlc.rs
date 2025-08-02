use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise, PanicOnDefault};
use sha2::{Digest, Sha256};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SimpleHTLC {
    pub owner: AccountId,
    pub escrows: UnorderedMap<String, Escrow>,
    pub escrow_counter: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Escrow {
    pub sender: AccountId,
    pub recipient: AccountId,
    pub amount: Balance,
    pub secret_hash: String,
    pub timeout: u64,
    pub is_active: bool,
}

#[near_bindgen]
impl SimpleHTLC {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            escrows: UnorderedMap::new(b"e"),
            escrow_counter: 0,
        }
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn get_escrow_count(&self) -> u64 {
        self.escrow_counter
    }

    #[payable]
    pub fn create_escrow(
        &mut self,
        recipient: AccountId,
        secret_hash: String,
        timeout_seconds: u64,
    ) -> String {
        let amount = env::attached_deposit();
        assert!(amount > 0, "Must attach deposit");
        
        let sender = env::predecessor_account_id();
        let timeout = env::block_timestamp() + (timeout_seconds * 1_000_000_000);
        
        let escrow_id = format!("escrow_{}", self.escrow_counter);
        let escrow = Escrow {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount,
            secret_hash: secret_hash.clone(),
            timeout,
            is_active: true,
        };
        
        self.escrows.insert(&escrow_id, &escrow);
        self.escrow_counter += 1;
        
        log!("Created escrow {} from {} to {} with amount {}", 
             escrow_id, sender, recipient, amount);
        
        escrow_id
    }

    pub fn claim(&mut self, escrow_id: String, secret: String) -> bool {
        let escrow = self.escrows.get(&escrow_id).expect("Escrow not found");
        assert!(escrow.is_active, "Escrow not active");
        
        // Verify secret
        let hash = Self::hash_secret(&secret);
        assert_eq!(hash, escrow.secret_hash, "Invalid secret");
        
        // Update escrow
        let mut updated_escrow = escrow.clone();
        updated_escrow.is_active = false;
        self.escrows.insert(&escrow_id, &updated_escrow);
        
        // Transfer funds
        Promise::new(escrow.recipient.clone()).transfer(escrow.amount);
        
        log!("Escrow {} claimed by {}", escrow_id, escrow.recipient);
        true
    }

    pub fn refund(&mut self, escrow_id: String) -> bool {
        let escrow = self.escrows.get(&escrow_id).expect("Escrow not found");
        assert!(escrow.is_active, "Escrow not active");
        assert!(
            env::block_timestamp() > escrow.timeout,
            "Timeout not reached"
        );
        
        // Update escrow
        let mut updated_escrow = escrow.clone();
        updated_escrow.is_active = false;
        self.escrows.insert(&escrow_id, &updated_escrow);
        
        // Refund to sender
        Promise::new(escrow.sender.clone()).transfer(escrow.amount);
        
        log!("Escrow {} refunded to {}", escrow_id, escrow.sender);
        true
    }

    pub fn get_escrow(&self, escrow_id: String) -> Option<Escrow> {
        self.escrows.get(&escrow_id)
    }

    fn hash_secret(secret: &str) -> String {
        let hash = Sha256::digest(secret.as_bytes());
        bs58::encode(hash).into_string()
    }
}