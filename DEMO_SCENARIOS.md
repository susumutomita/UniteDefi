# UniteSwap Demo Scenarios 🎬

ETHGlobal Unite ハッカソン用のデモシナリオ集

## 🚀 Quick Demo (5分)

### 準備
```bash
git clone https://github.com/susumutomita/UniteDefi.git
cd UniteDefi
cargo build -p fusion-cli --release
```

## デモシナリオ 1: HTLC基本フロー ⚡

### Step 1: HTLC作成
```bash
./target/release/fusion-cli create-htlc \
  --sender 0x1234567890123456789012345678901234567890 \
  --recipient 0x9876543210987654321098765432109876543210 \
  --amount 1000
```

**期待される出力:**
```json
{
  "htlc_id": "htlc_xxxxxxxxxxxxxxxx",
  "secret": "秘密値（32バイト）",
  "secret_hash": "秘密のハッシュ値",
  "status": "Pending"
}
```

### Step 2: HTLC クレーム試行（エラーケース）
```bash
./target/release/fusion-cli claim \
  --htlc-id htlc_6c2c0d83023b6dba \
  --secret 27eddfe62b6a8a7787b2bfe30694d334500ed8f134b5f3f9b7a047605c7a9518
```

**期待される出力:**
```json
{
  "error": "HTLC not found",
  "htlc_id": "htlc_6c2c0d83023b6dba"
}
```

> 💡 **Note**: メモリ内ストレージのため、コマンド間でHTLCは保持されません

## デモシナリオ 2: Limit Order作成 📊

### Step 1: Ethereum Base Limit Order
```bash
./target/release/fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash 0x6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263 \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62
```

**期待される出力:**
- EIP712ハッシュの生成
- 1inch Limit Order Protocol準拠の注文データ
- HTLC情報の埋め込み

### Step 2: NEAR to Ethereum Order
```bash
./target/release/fusion-cli order create-near \
  --near-account alice.near \
  --ethereum-address 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --near-amount 1000000000000000000000000 \
  --secret-hash 0x6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263
```

**期待される出力:**
- NEAR価格の自動取得
- クロスチェーン注文の生成
- 実行手順のガイダンス

## デモシナリオ 3: Cross-Chain リレー 🌉

### Relay Order Execution
```bash
./target/release/fusion-cli relay-order \
  --order-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
  --to-chain near \
  --htlc-secret 0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba
```

**期待される出力:**
- NEAR HTLC作成のシミュレーション
- リレー処理の詳細ログ
- 次のステップのガイダンス

## デモシナリオ 4: 管理操作 🛠️

### Order Management
```bash
# オーダーブック確認
./target/release/fusion-cli orderbook --chain ethereum

# 注文ステータス確認（存在しない注文）
./target/release/fusion-cli order status --order-id nonexistent

# 注文キャンセル試行（存在しない注文）
./target/release/fusion-cli order cancel --order-id nonexistent
```

## デモシナリオ 5: エラーハンドリング ⚠️

### バリデーション確認
```bash
# 不正なアドレス
./target/release/fusion-cli order create \
  --maker-asset invalid_address \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash 0x6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263 \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62
```

**期待される出力:**
```
Error: Invalid address: must be 40 hex characters (excluding 0x prefix)
```

## 🏆 ハッカソン要件デモ

### Hashlock and Timelock Preservation
- ✅ HTLC作成でハッシュロックとタイムロックが適切に設定される
- ✅ 秘密値とハッシュ値の対応が正確

### Bidirectional Swaps
- ✅ Ethereum → NEAR: `order create-near`
- ✅ EVM → 非EVM: `relay-order`

### 1inch Fusion+ Integration
- ✅ EIP712ハッシュ生成
- ✅ 公式Limit Order Protocol準拠
- ✅ 正式なコントラクトアドレス使用

## 📊 パフォーマンス実証

### レスポンス時間測定
```bash
time ./target/release/fusion-cli order create \
  --maker-asset 0x4200000000000000000000000000000000000006 \
  --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 \
  --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 \
  --making-amount 1000000000000000000 \
  --taking-amount 3000000000 \
  --htlc-secret-hash 0x6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263 \
  --htlc-timeout 3600 \
  --chain-id 84532 \
  --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62
```

**期待結果**: < 2秒での完了

## 🎯 審査員向けクイックチェック

```bash
# 1. ビルド確認
cargo build -p fusion-cli --release

# 2. テスト実行
cargo test --workspace

# 3. 基本機能確認
./target/release/fusion-cli --help

# 4. HTLC作成
./target/release/fusion-cli create-htlc --sender 0x1234567890123456789012345678901234567890 --recipient 0x9876543210987654321098765432109876543210 --amount 1000

# 5. 注文作成
./target/release/fusion-cli order create --maker-asset 0x4200000000000000000000000000000000000006 --taker-asset 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 --maker 0x7aD8317e9aB4837AEF734e23d1C62F4938a6D950 --making-amount 1000000000000000000 --taking-amount 3000000000 --htlc-secret-hash 0x6c2c0d83023b6dba52903a91952ab0cde4a0ce554d80a9f07ec815e54438a263 --htlc-timeout 3600 --chain-id 84532 --verifying-contract 0x171C87724E720F2806fc29a010a62897B30fdb62
```

## 📈 技術的ハイライト

- **Rust製高性能CLI**: サブ秒レスポンス
- **完全なETHType型安全性**: コンパイル時エラー検出
- **1inch Fusion+完全準拠**: 公式プロトコル使用
- **クロスチェーン対応**: Ethereum, NEAR, Base
- **エラーハンドリング**: 包括的バリデーション
- **テスト網羅**: 100+テストケース

---

*デモ実行時間: 約5-10分*  
*必要な前提知識: Rust基礎, ブロックチェーン概念*