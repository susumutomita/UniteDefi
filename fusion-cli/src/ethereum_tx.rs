use anyhow::{anyhow, Result};
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use std::str::FromStr;
use std::sync::Arc;

/// Submit a limit order to the 1inch Limit Order Protocol
pub async fn submit_limit_order(
    order_data: &serde_json::Value,
    rpc_url: &str,
    chain_id: u64,
    private_key: Option<String>,
) -> Result<TransactionReceipt> {
    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);

    // Get signer
    let wallet = if let Some(key) = private_key {
        // Security: Private key should come from secure environment variable
        let wallet: LocalWallet = key
            .parse()
            .map_err(|_| anyhow!("Invalid private key format"))?;
        wallet.with_chain_id(chain_id)
    } else {
        return Err(anyhow!("Private key required for transaction submission"));
    };

    let client = SignerMiddleware::new(provider.clone(), wallet);

    // Extract order details
    let _order = order_data["order"]
        .as_object()
        .ok_or_else(|| anyhow!("Invalid order data"))?;

    let verifying_contract = order_data["domain"]["verifyingContract"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing verifying contract"))?;

    // 1inch Limit Order Protocol ABI for fillOrder function
    abigen!(
        LimitOrderProtocol,
        r#"[
            {
                "inputs": [
                    {
                        "components": [
                            {"internalType": "uint256", "name": "salt", "type": "uint256"},
                            {"internalType": "address", "name": "makerAsset", "type": "address"},
                            {"internalType": "address", "name": "takerAsset", "type": "address"},
                            {"internalType": "address", "name": "maker", "type": "address"},
                            {"internalType": "address", "name": "receiver", "type": "address"},
                            {"internalType": "address", "name": "allowedSender", "type": "address"},
                            {"internalType": "uint256", "name": "makingAmount", "type": "uint256"},
                            {"internalType": "uint256", "name": "takingAmount", "type": "uint256"},
                            {"internalType": "uint256", "name": "offsets", "type": "uint256"},
                            {"internalType": "bytes", "name": "interactions", "type": "bytes"}
                        ],
                        "internalType": "struct Order",
                        "name": "order",
                        "type": "tuple"
                    },
                    {"internalType": "bytes", "name": "signature", "type": "bytes"},
                    {"internalType": "bytes", "name": "interaction", "type": "bytes"},
                    {"internalType": "uint256", "name": "makingAmount", "type": "uint256"},
                    {"internalType": "uint256", "name": "takingAmount", "type": "uint256"},
                    {"internalType": "uint256", "name": "skipPermitAndThresholdAmount", "type": "uint256"}
                ],
                "name": "fillOrder",
                "outputs": [
                    {"internalType": "uint256", "name": "", "type": "uint256"},
                    {"internalType": "uint256", "name": "", "type": "uint256"},
                    {"internalType": "bytes32", "name": "", "type": "bytes32"}
                ],
                "stateMutability": "payable",
                "type": "function"
            }
        ]"#
    );

    let contract_address = Address::from_str(verifying_contract)?;
    let _contract = LimitOrderProtocol::new(contract_address, Arc::new(client.clone()));

    // For now, we'll create the order but not fill it immediately
    // In a real implementation, this would be posted to 1inch API
    // or another order book for matching

    println!("Order created and ready for submission to 1inch");
    println!("Contract address: {}", verifying_contract);
    println!(
        "Order hash: {}",
        order_data["eip712_hash"].as_str().unwrap_or("unknown")
    );

    // Get network ID from chain ID
    let network_id = match chain_id {
        1 => 1,         // Ethereum mainnet
        137 => 137,     // Polygon
        42161 => 42161, // Arbitrum
        10 => 10,       // Optimism
        8453 => 8453,   // Base mainnet
        84532 => 8453,  // Base Sepolia uses Base mainnet API
        _ => {
            return Err(anyhow!(
                "Unsupported network for 1inch API: chain ID {}",
                chain_id
            ))
        }
    };

    // Get API key from environment
    let api_key = std::env::var("ONEINCH_API_KEY").ok();

    // Create 1inch client
    let oneinch_client = crate::oneinch_api::OneInchClient::new(network_id, api_key);

    // Convert order data to 1inch format
    let oneinch_order = crate::oneinch_api::convert_to_oneinch_format(order_data)?;

    // Submit order to 1inch
    match oneinch_client.submit_order(&oneinch_order).await {
        Ok(response) => {
            println!("Order submitted to 1inch successfully!");
            println!("Order hash: {}", response.order_hash);
            println!("Status: {}", response.status);
            println!("Created at: {}", response.created_at);

            // Return a mock receipt for now since 1inch doesn't return a transaction receipt
            // In a real implementation, you would monitor for the order execution
            Err(anyhow!(
                "Order submitted to 1inch. Monitor order hash {} for execution status",
                response.order_hash
            ))
        }
        Err(e) => Err(anyhow!("Failed to submit order to 1inch: {}", e)),
    }
}

/// Sign an order using EIP-712 hash
pub async fn sign_order_hash(eip712_hash: &[u8; 32], private_key: &str) -> Result<Signature> {
    let wallet: LocalWallet = private_key
        .parse()
        .map_err(|_| anyhow!("Invalid private key format"))?;

    // Convert hash to H256
    let message = H256::from_slice(eip712_hash);

    // Sign the hash directly
    let signature = wallet.sign_hash(message)?;

    Ok(signature)
}

/// Get the current gas price
#[allow(dead_code)]
pub async fn get_gas_price(rpc_url: &str) -> Result<U256> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let gas_price = provider.get_gas_price().await?;
    Ok(gas_price)
}

/// Estimate gas for a transaction
#[allow(dead_code)]
pub async fn estimate_gas(
    rpc_url: &str,
    from: Address,
    to: Address,
    data: Bytes,
    value: U256,
) -> Result<U256> {
    let provider = Provider::<Http>::try_from(rpc_url)?;

    let tx = TransactionRequest::new()
        .from(from)
        .to(to)
        .data(data)
        .value(value);

    let tx_typed: TypedTransaction = tx.into();
    let gas_estimate = provider.estimate_gas(&tx_typed, None).await?;
    Ok(gas_estimate)
}
