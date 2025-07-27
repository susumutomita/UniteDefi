# QAエンジニア向けテスト計画書

## 🎯 目的

本文書は、1inch Fusion+ NEAR HTLCクロスチェインスワップ実装のQAテスト計画を定義します。

## 📊 テスト対象

### 1. NEARスマートコントラクト
- `/contracts/near-htlc/src/fusion_htlc.rs`
- HTLCエスクロー機能
- タイムロック管理
- セーフティデポジット

### 2. TypeScriptオーケストレーター
- `/contracts/near-htlc/examples/fusion-orchestrator.ts`
- クロスチェイン調整
- エラーハンドリング

### 3. デプロイメントスクリプト
- `/contracts/near-htlc/scripts/deploy_testnet.sh`
- 自動デプロイ機能

## 🔒 セキュリティテスト項目

### 1. ハッシュ検証テスト
#### テストケース
- [ ] バイナリデータのハッシュ検証（hex形式）
- [ ] 不正なhex文字列の拒否
- [ ] 空のシークレットの拒否
- [ ] 長すぎるシークレット（>32バイト）の処理

#### 検証方法
```bash
# テスト実行
cargo test test_hash_verification_security

# 手動検証
near call htlc.testnet claim '{"escrow_id": "fusion_0", "secret": "not_hex"}' --accountId alice.testnet
# Expected: パニックまたはエラー
```

### 2. タイムスタンプオーバーフローテスト
#### テストケース
- [ ] 最大値に近いタイムスタンプでの動作
- [ ] 10年以上の期間設定の拒否
- [ ] ナノ秒精度の正確性
- [ ] ブロックタイムスタンプとの整合性

#### 検証方法
```bash
# オーバーフローテスト
cargo test test_timestamp_overflow_protection

# 手動検証（10年以上の期間）
near call htlc.testnet create_escrow '{
    "beneficiary": "bob.testnet",
    "secret_hash": "...",
    "token_id": null,
    "amount": "1000000000000000000000000",
    "safety_deposit": "100000000000000000000000",
    "safety_deposit_beneficiary": null,
    "finality_period": 315360001,
    "cancel_period": 630720000,
    "public_cancel_period": 946080000
}' --accountId alice.testnet --deposit 1.1
# Expected: エラー "Time period too long"
```

### 3. ガス制限テスト
#### テストケース
- [ ] 大量のエスクロー作成でのガス消費
- [ ] batch_cancelでの最大処理数
- [ ] NEP-141トークン転送のガス消費
- [ ] コールバック処理のガス消費

#### 検証方法
```bash
# ガス制限テスト
cargo test test_gas_limit_stress

# 手動検証（100個のバッチキャンセル）
# テストスクリプトを実行
node scripts/test_batch_cancel.js
```

### 4. リエントランシー攻撃テスト
#### テストケース
- [ ] batch_cancelでの重複ID処理
- [ ] 並行トランザクションでの状態整合性
- [ ] 外部コントラクト呼び出し中の状態変更
- [ ] コールバック中の再入可能性

#### 検証方法
```bash
# リエントランシーテスト
cargo test test_reentrancy_protection

# 手動検証
# 同じIDを複数回含むバッチキャンセル
near call htlc.testnet batch_cancel '{"escrow_ids": ["fusion_0", "fusion_0", "fusion_1"]}' --accountId resolver.testnet
```

### 5. クロスコントラクト失敗処理テスト
#### テストケース
- [ ] NEP-141転送失敗時のロールバック
- [ ] NEAR転送失敗時のロールバック
- [ ] コールバック失敗時の状態復元
- [ ] ネットワーク障害時の動作

#### 検証方法
```bash
# 統合テスト実行
cargo test --test integration_tests

# 手動検証（存在しないトークンコントラクト）
near call htlc.testnet create_escrow '{
    "beneficiary": "bob.testnet",
    "secret_hash": "...",
    "token_id": "nonexistent.testnet",
    ...
}' --accountId alice.testnet
```

## 🧪 機能テスト項目

### 1. エスクロー作成
- [ ] 正常なエスクロー作成
- [ ] 不足デポジットでの失敗
- [ ] 無効なパラメータでの失敗
- [ ] タイムアウト順序の検証

### 2. クレーム（請求）
- [ ] 正しいシークレットでのクレーム
- [ ] 間違ったシークレットでの失敗
- [ ] 権限のないユーザーのクレーム失敗
- [ ] タイムアウト後のクレーム失敗

### 3. キャンセル
- [ ] リゾルバーによるキャンセル
- [ ] パブリック期間でのキャンセル
- [ ] 早すぎるキャンセルの失敗
- [ ] 二重キャンセルの防止

### 4. 照会機能
- [ ] アクティブエスクローの取得
- [ ] クレーム可能エスクローの取得
- [ ] キャンセル可能エスクローの取得
- [ ] ページネーション機能

## 🔧 パフォーマンステスト

### 1. スケーラビリティ
- [ ] 1000個のエスクローでの動作
- [ ] 検索機能のレスポンス時間
- [ ] ストレージ使用量の測定

### 2. 並行性
- [ ] 同時トランザクション処理
- [ ] 状態の一貫性維持

## 📝 テスト環境

### 必要なツール
```bash
# Rust環境
rustup target add wasm32-unknown-unknown

# NEAR CLI
npm install -g near-cli

# テスト依存関係
npm install

# テストアカウント
near create-account qa-test-1.testnet --useFaucet
near create-account qa-test-2.testnet --useFaucet
near create-account qa-test-3.testnet --useFaucet
```

### テストネットワーク
- NEAR Testnet: https://rpc.testnet.near.org
- Ethereum Sepolia: https://sepolia.infura.io/v3/YOUR_KEY

## 🚀 テスト実行手順

### 1. 自動テスト実行
```bash
# すべてのセキュリティテスト
npm run test:security

# Rustユニットテスト
cargo test

# 統合テスト
cargo test --test integration_tests

# TypeScriptテスト
npm test
```

### 2. 手動テスト実行
```bash
# デプロイ
./scripts/deploy_testnet.sh

# エンドツーエンドテスト
node scripts/e2e_test.js
```

## 📊 テスト結果報告

### 報告フォーマット
```markdown
## テスト実行結果 - [日付]

### 環境
- NEAR SDK: 5.0.0
- Rust: 1.70.0
- Node.js: 18.x

### 結果サマリー
- 総テスト数: XX
- 成功: XX
- 失敗: XX
- スキップ: XX

### 失敗したテスト
1. テスト名: [詳細]
   - エラー内容
   - 再現手順
   - 予想される原因

### パフォーマンス指標
- エスクロー作成: XXms
- クレーム実行: XXms
- バッチキャンセル（100件）: XXms

### セキュリティ問題
- 発見された脆弱性: [詳細]
- 推奨される修正: [提案]
```

## 🎯 受け入れ基準

- [ ] すべてのセキュリティテストが合格
- [ ] すべての機能テストが合格
- [ ] パフォーマンステストが基準値を満たす
- [ ] ドキュメントが最新
- [ ] コードカバレッジが80％以上

## 📅 テストスケジュール

1. **フェーズ1（1日目）**: セキュリティテスト
2. **フェーズ2（2日目）**: 機能テスト
3. **フェーズ3（3日目）**: パフォーマンステスト
4. **フェーズ4（4日目）**: 統合テスト

このテスト計画に従って、HTLCクロスチェインスワップの品質を保証します。