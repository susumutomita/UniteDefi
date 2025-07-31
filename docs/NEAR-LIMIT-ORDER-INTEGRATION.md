# NEAR-Limit Order統合ガイド

## 概要

NEAR-Limit Order統合により、NEARチェーンのHTLCコントラクトと1inch Limit Order Protocolを接続し、EVM・非EVM間でのアトミックスワップが可能になります。

## アーキテクチャ

### コンポーネント

1. **NEARチェーン側**
   - HTLCコントラクト（`fusion_htlc.rs`）
   - タイムロック機能とシークレットハッシュによる保護

2. **Ethereumチェーン側**
   - 1inch Limit Order Protocol（Base Sepolia）
   - Escrowコントラクト

3. **ブリッジロジック**
   - HTLCDataエンコーディング
   - クロスチェーン実行フロー
   - 価格オラクル統合

## HTLCData構造

```rust
pub struct HTLCData {
    pub secret_hash: [u8; 32],      // シークレットハッシュ
    pub timeout: u64,               // タイムアウト（秒）
    pub recipient_chain: String,    // 受信チェーン（"near"）
    pub recipient_address: String,  // NEARアドレス
}
```

この構造体はLimit OrderのinteractionsフィールドにエンコードされてEthereum上に保存されます。

## 使用方法

### 1. CLIを使用したNEAR→Ethereumオーダー作成

```bash
# 新しいシークレットを生成してオーダーを作成
fusion-cli order create-near \
  --near-account alice.near \
  --ethereum-address 0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0 \
  --near-amount 10.0 \
  --generate-secret \
  --timeout 3600 \
  --slippage-bps 100

# 既存のシークレットハッシュを使用
fusion-cli order create-near \
  --near-account alice.near \
  --ethereum-address 0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0 \
  --near-amount 10.0 \
  --secret-hash 0x1234567890abcdef... \
  --timeout 3600
```

### 2. プログラマティックな使用

```rust
use fusion_core::{
    htlc::{generate_secret, hash_secret},
    limit_order_htlc::create_near_to_ethereum_order,
};

// シークレットの生成
let secret = generate_secret();
let secret_hash = hash_secret(&secret);

// オーダーの作成
let order = create_near_to_ethereum_order(
    "alice.near",
    "0x742d35Cc6634C0532925a3b844Bc9e7595f8b4e0",
    10_000_000_000_000_000_000_000_000, // 10 NEAR
    50_000_000, // 50 USDC
    secret_hash,
    3600, // 1時間
)?;
```

## 実行フロー

1. **オーダー作成**
   - ユーザーがNEARトークンを売り、USDCを買うオーダーを作成
   - HTLCデータがLimit Orderに埋め込まれる

2. **オーダー署名・送信**
   - NEARウォレットでオーダーに署名
   - 1inch Fusion APIに送信

3. **リゾルバーによるフィル**
   - リゾルバーがBase SepoliaでUSDCを入金
   - Limit Orderがフィルされる

4. **NEAR側でHTLC作成**
   - システムがフィルイベントを検知
   - NEARチェーンでHTLCを作成

5. **シークレット公開**
   - リゾルバーがEthereum側でシークレットを公開してクレーム
   - シークレットがブロックチェーンに記録される

6. **NEAR側でクレーム**
   - システムが公開されたシークレットを使用
   - NEAR側でHTLCをクレーム
   - スワップ完了

## 価格オラクル

現在の実装では`MockPriceOracle`を使用していますが、本番環境では以下のオラクルと統合予定：

- Chainlink Price Feeds
- Pyth Network

## セキュリティ考慮事項

1. **タイムロック**
   - 適切なタイムアウト期間の設定が重要
   - チェーン間の確認時間を考慮

2. **シークレット管理**
   - シークレットは安全に保管する必要がある
   - 公開前に漏洩しないよう注意

3. **価格スリッページ**
   - 価格変動に対する保護のため、スリッページ許容値を設定

## トラブルシューティング

### よくある問題

1. **"Invalid NEAR account format"**
   - NEARアカウントは`.near`または`.testnet`で終わる必要があります

2. **"Secret hash must be exactly 32 bytes"**
   - シークレットハッシュは正確に32バイト（64文字の16進数）である必要があります

3. **価格変換エラー**
   - オラクルが正しく設定されているか確認してください

## 例：統合デモ

```bash
# デモプログラムの実行
cargo run --example near_limit_order_integration
```

このデモでは以下を実行します：
- 価格オラクルの設定
- HTLCシークレットの生成
- NEAR→Ethereumオーダーの作成
- HTLC情報の検証
- 実行フローの説明