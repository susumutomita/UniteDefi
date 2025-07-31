use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub salt: [u8; 32],
    pub maker_asset: String,
    pub taker_asset: String,
    pub maker: String,
    pub receiver: String,
    pub allowed_sender: String,
    pub making_amount: u128,
    pub taking_amount: u128,
    pub offsets: U256,
    pub interactions: String,
}

type U256 = u64; // Simplified for now

impl Order {
    pub fn maker_asset(&self) -> &str {
        &self.maker_asset
    }

    pub fn taker_asset(&self) -> &str {
        &self.taker_asset
    }

    pub fn maker(&self) -> &str {
        &self.maker
    }

    pub fn making_amount(&self) -> u128 {
        self.making_amount
    }

    pub fn taking_amount(&self) -> u128 {
        self.taking_amount
    }
}

#[derive(Default)]
pub struct OrderBuilder {
    salt: Option<[u8; 32]>,
    maker_asset: Option<String>,
    taker_asset: Option<String>,
    maker: Option<String>,
    receiver: Option<String>,
    allowed_sender: Option<String>,
    making_amount: Option<u128>,
    taking_amount: Option<u128>,
    offsets: Option<U256>,
    interactions: Option<String>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn salt(mut self, salt: [u8; 32]) -> Self {
        self.salt = Some(salt);
        self
    }

    pub fn maker_asset(mut self, maker_asset: &str) -> Self {
        self.maker_asset = Some(maker_asset.to_string());
        self
    }

    pub fn taker_asset(mut self, taker_asset: &str) -> Self {
        self.taker_asset = Some(taker_asset.to_string());
        self
    }

    pub fn maker(mut self, maker: &str) -> Self {
        self.maker = Some(maker.to_string());
        self
    }

    pub fn receiver(mut self, receiver: &str) -> Self {
        self.receiver = Some(receiver.to_string());
        self
    }

    pub fn allowed_sender(mut self, allowed_sender: &str) -> Self {
        self.allowed_sender = Some(allowed_sender.to_string());
        self
    }

    pub fn making_amount(mut self, amount: u128) -> Self {
        self.making_amount = Some(amount);
        self
    }

    pub fn taking_amount(mut self, amount: u128) -> Self {
        self.taking_amount = Some(amount);
        self
    }

    pub fn offsets(mut self, offsets: U256) -> Self {
        self.offsets = Some(offsets);
        self
    }

    pub fn interactions(mut self, interactions: &str) -> Self {
        self.interactions = Some(interactions.to_string());
        self
    }

    pub fn build(self) -> Result<Order> {
        // Generate random salt if not provided
        let salt = self.salt.unwrap_or_else(|| {
            let mut salt = [0u8; 32];
            use rand::Rng;
            rand::thread_rng().fill(&mut salt);
            salt
        });

        Ok(Order {
            salt,
            maker_asset: self
                .maker_asset
                .ok_or_else(|| anyhow!("maker_asset is required"))?,
            taker_asset: self
                .taker_asset
                .ok_or_else(|| anyhow!("taker_asset is required"))?,
            maker: self.maker.ok_or_else(|| anyhow!("maker is required"))?,
            receiver: self
                .receiver
                .unwrap_or_else(|| "0x0000000000000000000000000000000000000000".to_string()),
            allowed_sender: self
                .allowed_sender
                .unwrap_or_else(|| "0x0000000000000000000000000000000000000000".to_string()),
            making_amount: self
                .making_amount
                .ok_or_else(|| anyhow!("making_amount is required"))?,
            taking_amount: self
                .taking_amount
                .ok_or_else(|| anyhow!("taking_amount is required"))?,
            offsets: self.offsets.unwrap_or(0),
            interactions: self.interactions.unwrap_or_else(|| "0x".to_string()),
        })
    }
}
