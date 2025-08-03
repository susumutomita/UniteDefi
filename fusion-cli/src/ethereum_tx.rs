use anyhow::{anyhow, Result};
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::Bytes;
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

    // Submit the order directly to 1inch Limit Order Protocol contract
    println!("Submitting order to 1inch Limit Order Protocol contract...");

    // Extract order data
    let order = order_data["order"]
        .as_object()
        .ok_or_else(|| anyhow!("Invalid order data"))?;

    let signature = order_data
        .get("signature")
        .and_then(|s| s.as_str())
        .ok_or_else(|| anyhow!("Order must be signed before submission"))?;

    // Parse signature bytes
    let signature_bytes = Bytes::from(hex::decode(signature.trim_start_matches("0x"))?);

    // Create the fillOrder transaction
    let contract_address = Address::from_str(verifying_contract)?;
    let contract = LimitOrderProtocol::new(contract_address, Arc::new(client.clone()));

    // Prepare order struct for contract call
    let order_struct = Order {
        salt: U256::from_str(order["salt"].as_str().unwrap_or("0"))?,
        maker_asset: Address::from_str(order["makerAsset"].as_str().unwrap_or("0x0"))?,
        taker_asset: Address::from_str(order["takerAsset"].as_str().unwrap_or("0x0"))?,
        maker: Address::from_str(order["maker"].as_str().unwrap_or("0x0"))?,
        receiver: Address::from_str(order["receiver"].as_str().unwrap_or("0x0"))?,
        allowed_sender: Address::from_str(order["allowedSender"].as_str().unwrap_or("0x0"))?,
        making_amount: U256::from_str(order["makingAmount"].as_str().unwrap_or("0"))?,
        taking_amount: U256::from_str(order["takingAmount"].as_str().unwrap_or("0"))?,
        offsets: U256::from_str(order["offsets"].as_str().unwrap_or("0"))?,
        interactions: Bytes::from(hex::decode(
            order["interactions"]
                .as_str()
                .unwrap_or("0x")
                .trim_start_matches("0x"),
        )?),
    };

    // Empty bytes for interaction parameter
    let interaction = Bytes::default();

    // Call fillOrder on the contract
    let tx_call = contract
        .fill_order(
            order_struct,
            signature_bytes,
            interaction,
            U256::zero(), // making amount (0 for full order)
            U256::zero(), // taking amount (0 for full order)
            U256::zero(), // threshold amount
        )
        .gas(500000u64); // Set gas limit

    let tx = tx_call.send().await?;
    let tx_hash = format!("{:?}", tx.tx_hash());
    println!("Transaction submitted: {}", tx_hash);

    // Wait for confirmation
    let receipt = tx.await?;

    if let Some(receipt) = receipt {
        println!("Order submitted successfully!");
        println!("Transaction hash: {:?}", receipt.transaction_hash);
        println!("Block number: {:?}", receipt.block_number);
        println!("Gas used: {:?}", receipt.gas_used);
        Ok(receipt)
    } else {
        Err(anyhow!("Failed to get transaction receipt"))
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
