// Simple NEP-141 token contract for testing
// This is a minimal implementation for integration tests only

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TestToken {
    pub total_supply: Balance,
    pub balances: LookupMap<AccountId, Balance>,
    pub allowances: LookupMap<(AccountId, AccountId), Balance>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[near_bindgen]
impl TestToken {
    #[init]
    pub fn new(total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        let mut token = Self {
            total_supply: total_supply.0,
            balances: LookupMap::new(b"b"),
            allowances: LookupMap::new(b"a"),
        };
        
        // Give all tokens to the contract creator
        token.balances.insert(&env::predecessor_account_id(), &total_supply.0);
        
        token
    }

    // NEP-141 standard methods
    pub fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        let sender = env::predecessor_account_id();
        let amount = amount.0;
        
        assert!(amount > 0, "Amount must be positive");
        
        let sender_balance = self.balances.get(&sender).unwrap_or(0);
        assert!(sender_balance >= amount, "Insufficient balance");
        
        // Update balances
        self.balances.insert(&sender, &(sender_balance - amount));
        let receiver_balance = self.balances.get(&receiver_id).unwrap_or(0);
        self.balances.insert(&receiver_id, &(receiver_balance + amount));
        
        env::log_str(&format!(
            "Transfer {} from {} to {} memo: {:?}",
            amount, sender, receiver_id, memo
        ));
    }

    pub fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.ft_transfer(receiver_id.clone(), amount, memo);
        
        // For testing, just return the amount
        PromiseOrValue::Value(amount)
    }

    pub fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.balances.get(&account_id).unwrap_or(0))
    }

    pub fn ft_total_supply(&self) -> U128 {
        U128(self.total_supply)
    }

    // Storage management (simplified for testing)
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) -> U128 {
        let account = account_id.unwrap_or_else(env::predecessor_account_id);
        
        // Just mark as registered
        if self.balances.get(&account).is_none() {
            self.balances.insert(&account, &0);
        }
        
        U128(env::attached_deposit())
    }
}