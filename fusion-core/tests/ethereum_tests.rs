use fusion_core::chains::ethereum::EthereumConnector;
use fusion_core::htlc::{generate_secret, hash_secret};
use ethers::types::{Address, U256};
use std::str::FromStr;

#[tokio::test]
async fn test_create_escrow() {
    // テスト用のRPCとファクトリーアドレス
    let rpc_url = "http://localhost:8545";
    let factory_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    
    let connector = EthereumConnector::new(rpc_url, factory_address)
        .expect("Failed to create connector");
    
    // テスト用のプライベートキー（Hardhatのデフォルトアカウント）
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let connector = connector.with_signer(private_key)
        .expect("Failed to add signer");
    
    // Escrowパラメータの準備
    let token = Address::zero(); // ETHの場合
    let amount = U256::from(1_000_000_000_000_000_000u64); // 1 ETH
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret);
    let timeout = U256::from(3600); // 1時間
    let recipient = Address::from_str("0x70997970C51812dc3A010C7d01b50e0d17dc79C8")
        .expect("Invalid recipient address");
    
    // create_escrowを呼び出し
    let escrow_address = connector.create_escrow(
        token,
        amount,
        secret_hash,
        timeout,
        recipient,
    ).await;
    
    assert!(escrow_address.is_ok());
    let escrow_address = escrow_address.unwrap();
    assert_ne!(escrow_address, Address::zero());
}

#[tokio::test]
async fn test_claim_escrow() {
    // テスト用のRPCとファクトリーアドレス
    let rpc_url = "http://localhost:8545";
    let factory_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    
    let connector = EthereumConnector::new(rpc_url, factory_address)
        .expect("Failed to create connector");
    
    // 受取人のプライベートキー（Hardhatのデフォルトアカウント2）
    let private_key = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
    let connector = connector.with_signer(private_key)
        .expect("Failed to add signer");
    
    // 事前に作成されたEscrowのアドレス（実際のテストでは先にcreate_escrowを呼ぶ）
    let escrow_address = Address::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512")
        .expect("Invalid escrow address");
    
    // テスト用のシークレット
    let secret = [1u8; 32]; // 実際のテストでは、create時と同じシークレットを使用
    
    // claim_escrowを呼び出し
    let receipt = connector.claim_escrow(escrow_address, secret).await;
    
    assert!(receipt.is_ok());
    let receipt = receipt.unwrap();
    assert_eq!(receipt.status, Some(U256::from(1))); // 成功ステータス
}

#[tokio::test]
async fn test_refund_escrow() {
    // テスト用のRPCとファクトリーアドレス
    let rpc_url = "http://localhost:8545";
    let factory_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    
    let connector = EthereumConnector::new(rpc_url, factory_address)
        .expect("Failed to create connector");
    
    // 送信者のプライベートキー（Hardhatのデフォルトアカウント）
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let connector = connector.with_signer(private_key)
        .expect("Failed to add signer");
    
    // 事前に作成されたEscrowのアドレス（実際のテストでは先にcreate_escrowを呼ぶ）
    let escrow_address = Address::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512")
        .expect("Invalid escrow address");
    
    // refund_escrowを呼び出し（タイムアウト後を想定）
    let receipt = connector.refund_escrow(escrow_address).await;
    
    assert!(receipt.is_ok());
    let receipt = receipt.unwrap();
    assert_eq!(receipt.status, Some(U256::from(1))); // 成功ステータス
}