use fusion_core::htlc::{generate_secret, hash_secret};

fn main() {
    println!("=== HTLC Secret Demo ===\n");

    // シークレットを生成
    let secret = generate_secret();
    println!("生成されたシークレット（16進数）:");
    println!("{}\n", hex::encode(secret));

    // ハッシュを計算
    let hash = hash_secret(&secret);
    println!("シークレットのSHA256ハッシュ:");
    println!("{}\n", hex::encode(hash));

    // 同じシークレットから同じハッシュが生成されることを確認
    let hash2 = hash_secret(&secret);
    println!("再計算したハッシュ（同じはず）:");
    println!("{}\n", hex::encode(hash2));

    // 別のシークレットを生成
    let another_secret = generate_secret();
    let another_hash = hash_secret(&another_secret);
    println!("別のシークレットとハッシュ:");
    println!("シークレット: {}", hex::encode(another_secret));
    println!("ハッシュ: {}", hex::encode(another_hash));
}
