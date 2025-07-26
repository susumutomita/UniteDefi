use rand::Rng;
use sha2::{Digest, Sha256};

/// 32バイトのランダムなシークレットを生成する
pub fn generate_secret() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut secret = vec![0u8; 32];
    rng.fill(&mut secret[..]);
    secret
}

/// シークレットのSHA256ハッシュを計算する
pub fn hash_secret(secret: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.finalize().to_vec()
}
