use crate::chains::Chain;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub chain_id: u64,
    pub escrow_factory: Option<String>,
    pub explorer_url: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub chains: HashMap<Chain, ChainConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let mut chains = HashMap::new();

        // Ethereum Sepolia testnet
        chains.insert(
            Chain::Ethereum,
            ChainConfig {
                rpc_url: "https://sepolia.infura.io/v3/YOUR_INFURA_KEY".to_string(),
                chain_id: 11155111,
                escrow_factory: None, // Will be set after deployment
                explorer_url: "https://sepolia.etherscan.io".to_string(),
            },
        );

        // NEAR testnet
        chains.insert(
            Chain::NEAR,
            ChainConfig {
                rpc_url: "https://rpc.testnet.near.org".to_string(),
                chain_id: 0,          // NEAR doesn't use chain IDs
                escrow_factory: None, // Contract account ID will be set after deployment
                explorer_url: "https://explorer.testnet.near.org".to_string(),
            },
        );

        // Base Sepolia testnet
        chains.insert(
            Chain::BaseSepolia,
            ChainConfig {
                rpc_url: "https://sepolia.base.org".to_string(),
                chain_id: 84532,
                escrow_factory: None, // Will be set after deployment
                explorer_url: "https://sepolia.basescan.org".to_string(),
            },
        );

        Self { chains }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Override with environment variables if available
        if let Ok(eth_rpc) = std::env::var("ETH_SEPOLIA_RPC_URL") {
            if let Some(eth_config) = config.chains.get_mut(&Chain::Ethereum) {
                eth_config.rpc_url = eth_rpc;
            }
        }

        if let Ok(eth_factory) = std::env::var("ETH_ESCROW_FACTORY_ADDRESS") {
            if let Some(eth_config) = config.chains.get_mut(&Chain::Ethereum) {
                eth_config.escrow_factory = Some(eth_factory);
            }
        }

        if let Ok(near_rpc) = std::env::var("NEAR_TESTNET_RPC_URL") {
            if let Some(near_config) = config.chains.get_mut(&Chain::NEAR) {
                near_config.rpc_url = near_rpc;
            }
        }

        if let Ok(near_contract) = std::env::var("NEAR_HTLC_CONTRACT_ID") {
            if let Some(near_config) = config.chains.get_mut(&Chain::NEAR) {
                near_config.escrow_factory = Some(near_contract);
            }
        }

        // Base Sepolia configuration
        if let Ok(base_rpc) = std::env::var("BASE_SEPOLIA_RPC_URL") {
            if let Some(base_config) = config.chains.get_mut(&Chain::BaseSepolia) {
                base_config.rpc_url = base_rpc;
            }
        }

        if let Ok(base_factory) = std::env::var("BASE_ESCROW_FACTORY_ADDRESS") {
            if let Some(base_config) = config.chains.get_mut(&Chain::BaseSepolia) {
                base_config.escrow_factory = Some(base_factory);
            }
        }

        config
    }

    pub fn get_chain_config(&self, chain: Chain) -> Option<&ChainConfig> {
        self.chains.get(&chain)
    }
}
