use fusion_core::htlc::{generate_secret, hash_secret, Htlc};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== HTLC デモ ===\n");

    // 1. シークレットを生成
    let secret = generate_secret();
    println!("1. シークレットを生成しました（32バイト）");
    println!("   シークレット: 0x{}", hex::encode(&secret));

    // 2. シークレットハッシュを計算
    let secret_hash = hash_secret(&secret);
    println!("\n2. シークレットハッシュを計算しました");
    println!("   ハッシュ: 0x{}", hex::encode(&secret_hash));

    // 3. HTLCを作成（Alice → Bob）
    let amount = 1000u64;
    let timeout = Duration::from_secs(5); // 5秒のタイムアウト

    let mut htlc = match Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        amount,
        secret_hash,
        timeout,
    ) {
        Ok(htlc) => htlc,
        Err(e) => {
            println!("HTLC作成エラー: {}", e);
            return;
        }
    };

    println!("\n3. HTLCを作成しました");
    println!("   送信者: {}", htlc.sender());
    println!("   受信者: {}", htlc.recipient());
    println!("   金額: {}", htlc.amount());
    println!("   タイムアウト: 5秒");
    println!("   現在の状態: {:?}", htlc.state());

    // 4. 正しいシークレットでクレーム
    println!("\n4. Bobが正しいシークレットでクレームを試みます...");
    match htlc.claim(&secret) {
        Ok(_) => {
            println!("   ✓ クレーム成功！");
            println!("   現在の状態: {:?}", htlc.state());
        }
        Err(e) => {
            println!("   ✗ クレーム失敗: {}", e);
        }
    }

    // 5. 別のHTLCでタイムアウトのシナリオをデモ
    println!("\n=== タイムアウトシナリオ ===");

    let mut htlc2 = match Htlc::new(
        "Alice".to_string(),
        "Bob".to_string(),
        amount,
        hash_secret(&generate_secret()), // 別のシークレットハッシュ
        Duration::from_secs(2),          // 2秒のタイムアウト
    ) {
        Ok(htlc) => htlc,
        Err(e) => {
            println!("HTLC作成エラー: {}", e);
            return;
        }
    };

    println!("\n5. 新しいHTLCを作成しました（2秒のタイムアウト）");
    println!("   現在の状態: {:?}", htlc2.state());

    // タイムアウト前のリファンドを試みる
    println!("\n6. タイムアウト前にリファンドを試みます...");
    match htlc2.refund() {
        Ok(_) => println!("   ✓ リファンド成功"),
        Err(e) => println!("   ✗ リファンド失敗（期待通り）: {}", e),
    }

    // 3秒待つ
    println!("\n7. 3秒待ちます...");
    thread::sleep(Duration::from_secs(3));

    // タイムアウト後のリファンド
    println!("\n8. タイムアウト後にリファンドを試みます...");
    match htlc2.refund() {
        Ok(_) => {
            println!("   ✓ リファンド成功！");
            println!("   現在の状態: {:?}", htlc2.state());
        }
        Err(e) => {
            println!("   ✗ リファンド失敗: {}", e);
        }
    }

    println!("\n=== デモ終了 ===");
}
