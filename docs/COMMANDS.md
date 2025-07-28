# コマンドリファレンス

## 🛠️ 開発環境セットアップ

### 依存関係インストール
```bash
# ルートディレクトリで
npm install
cargo build

# NEARコントラクトビルド
cargo build -p near-htlc --target wasm32-unknown-unknown --release
cargo build -p test-token --target wasm32-unknown-unknown --release
```

### 環境変数設定
```bash
# .env.exampleをコピー
cp .env.example .env

# 必要な値を設定
NEAR_NETWORK=testnet
NEAR_ACCOUNT_ID=your-account.testnet
EVM_RPC_URL=https://...
EVM_PRIVATE_KEY=0x...
```

## 🚀 デプロイコマンド

### NEARコントラクト
```bash
# テストネット
near deploy \
  --wasmFile target/wasm32-unknown-unknown/release/near_htlc.wasm \
  --accountId your-contract.testnet

# ローカル（sandbox）
near dev-deploy target/wasm32-unknown-unknown/release/near_htlc.wasm
```

### EVMコントラクト
```bash
# Monadテストネット
npx hardhat run scripts/deploy.js --network monad-testnet

# ローカル
npx hardhat run scripts/deploy.js --network localhost
```

## 🧪 テストコマンド

### ユニットテスト
```bash
# Rustテスト（NEAR）
cargo test --package near-htlc

# Solidityテスト（EVM）
npx hardhat test

# 特定のテストのみ
cargo test test_full_htlc_flow -- --nocapture
```

### 統合テスト
```bash
# E2Eテスト
npm run test:e2e

# セキュリティテスト
cd contracts/near-htlc && ./scripts/run_security_tests.sh
```

## 📡 リレイヤー操作

### 開発モード
```bash
# TypeScriptで実行
npm run relayer:dev

# ログ付き
DEBUG=relayer:* npm run relayer:dev
```

### 本番モード
```bash
# ビルド
npm run relayer:build

# 起動
npm run relayer:start

# PM2で管理
pm2 start ecosystem.config.js
```

## 🔍 デバッグコマンド

### NEARコントラクト状態確認
```bash
# エスクロー詳細を確認
near view your-contract.testnet get_escrow '{"escrow_id": "fusion_0"}'

# 全エスクロー一覧
near view your-contract.testnet get_all_escrows '{}'
```

### EVMコントラクト確認
```bash
# Hardhat Console
npx hardhat console --network monad-testnet

# コントラクト状態を確認
> const resolver = await ethers.getContractAt("FusionResolver", "0x...")
> await resolver.getEscrow("0x...")
```

## 🎯 よく使うワンライナー

### クリーンビルド
```bash
cargo clean && cargo build -p near-htlc --target wasm32-unknown-unknown --release
```

### 全テスト実行
```bash
make test
```

### ログ監視
```bash
# NEARトランザクション監視
near logs your-contract.testnet --follow

# EVMイベント監視
npx hardhat run scripts/watch-events.js --network monad-testnet
```

## 🆘 トラブルシューティング

### WASM Deserializationエラー
```bash
# WASMサイズ確認
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# 最適化ビルド
cargo build --release --target wasm32-unknown-unknown
```

### ガス不足エラー
```bash
# NEARでガスを増やす
near call ... --gas 300000000000000

# EVMでガスを指定
const tx = await contract.method({ gasLimit: 1000000 });
```

### Permission Deniedエラー
```bash
# スクリプトに実行権限を付与
chmod +x contracts/near-htlc/scripts/*.sh
```

## 📚 参考リンク

- [NEAR CLI Docs](https://docs.near.org/tools/near-cli)
- [Hardhat Docs](https://hardhat.org/hardhat-runner/docs/getting-started)
- [Cargo Docs](https://doc.rust-lang.org/cargo/)