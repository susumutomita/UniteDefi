//! NEAR-Limit Order統合の使用例

use anyhow::Result;
use fusion_core::{
    cross_chain_executor::{CrossChainExecutor, ExecutionParams},
    htlc::{generate_secret, hash_secret},
    limit_order_htlc::{create_near_to_ethereum_order, OrderHTLCExt},
    price_oracle::{MockPriceOracle, PriceConverter},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== NEAR-Limit Order Integration Example ===\n");

    // 1. 価格オラクルの設定
    println!("1. Setting up price oracle...");
    let oracle = MockPriceOracle::new();
    let price_converter = PriceConverter::new(oracle);

    // 現在の価格を表示
    // Note: 価格は内部的に使用されるため、直接アクセスはできません
    println!("   Using mock prices: NEAR=$5, ETH=$2000");

    // 2. HTLCシークレットの生成
    println!("\n2. Generating HTLC secret...");
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);
    println!("   Secret hash: 0x{}", hex::encode(secret_hash));

    // 3. NEAR→Ethereum オーダーの作成
    println!("\n3. Creating NEAR to Ethereum order...");
    let near_amount = 10_000_000_000_000_000_000_000_000; // 10 NEAR

    // 価格変換（10 NEAR → USDC）
    let usdc_amount = price_converter
        .convert_amount(
            near_amount,
            "NEAR",
            24, // NEAR decimals
            "USDC",
            6, // USDC decimals
        )
        .await?;

    println!("   Making: 10 NEAR");
    println!("   Taking: {} USDC", usdc_amount as f64 / 1_000_000.0);

    let order = create_near_to_ethereum_order(
        "alice.near",
        "0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0",
        near_amount,
        usdc_amount,
        secret_hash,
        3600, // 1 hour timeout
    )?;

    println!("   Order created successfully!");

    // 4. HTLC情報の確認
    println!("\n4. Verifying HTLC data in order...");
    if order.has_htlc_data() {
        let htlc_data = order.extract_htlc_data()?;
        println!("   ✓ HTLC data embedded in order");
        println!("   - Recipient chain: {}", htlc_data.recipient_chain);
        println!("   - Recipient address: {}", htlc_data.recipient_address);
        println!("   - Timeout: {} seconds", htlc_data.timeout);
    }

    // 5. クロスチェーン実行の準備
    println!("\n5. Preparing cross-chain execution...");
    let _executor = CrossChainExecutor::new(
        "https://base-sepolia.infura.io/v3/YOUR_KEY",
        "0x171C87724E720F2806fc29a010a62897B30fdb62", // Base Sepolia factory
        "https://rpc.testnet.near.org",
    )?;

    let _execution_params = ExecutionParams {
        order: order.clone(),
        limit_order_protocol: "0x171C87724E720F2806fc29a010a62897B30fdb62".to_string(),
        near_htlc_contract: "htlc.testnet".to_string(),
    };

    println!("   Execution parameters configured");

    // 6. オーダーの詳細を表示
    println!("\n6. Order details:");
    println!("   Salt: 0x{}", hex::encode(order.salt));
    println!("   Maker: {}", order.maker());
    println!("   Receiver: {}", order.receiver);
    println!(
        "   Making amount: {} NEAR",
        near_amount as f64 / 10f64.powi(24)
    );
    println!(
        "   Taking amount: {} USDC",
        usdc_amount as f64 / 10f64.powi(6)
    );
    println!("   Interactions (HTLC data): {}", order.interactions);

    // 7. 実行フローの説明
    println!("\n7. Execution flow:");
    println!("   Step 1: User signs and submits order to 1inch API");
    println!("   Step 2: Resolver fills order on Base Sepolia");
    println!("   Step 3: System detects fill event and creates HTLC on NEAR");
    println!("   Step 4: Resolver claims with secret on Ethereum");
    println!("   Step 5: System uses revealed secret to claim on NEAR");
    println!("   Step 6: Swap complete! 🎉");

    println!("\n=== Example completed successfully! ===");

    Ok(())
}

/// デモ用: オーダーのEIP-712署名を表示
#[allow(dead_code)]
fn demonstrate_order_signing(order: &fusion_core::order::Order) {
    use fusion_core::eip712::OrderEIP712;

    let chain_id = 84532; // Base Sepolia
    let verifying_contract = "0x171C87724E720F2806fc29a010a62897B30fdb62";

    let typed_data = order.to_eip712(chain_id, verifying_contract);
    let hash = typed_data.hash();

    println!("\nEIP-712 Signing Details:");
    println!("  Domain:");
    println!("    Name: {}", typed_data.domain.name);
    println!("    Version: {}", typed_data.domain.version);
    println!("    Chain ID: {}", typed_data.domain.chain_id);
    println!(
        "    Verifying Contract: {}",
        typed_data.domain.verifying_contract
    );
    println!("  Message hash: 0x{}", hex::encode(hash));
}
